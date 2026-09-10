//! Oxide-3D Product Lifecycle Management (PLM), Item Masters, Revisions, and BOM hierarchies.

use petgraph::Directed;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Unique Item Identifier in PLM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ItemId(pub Uuid);

impl Default for ItemId {
    fn default() -> Self {
        Self(Uuid::now_v7())
    }
}

/// Product lifecycle approval state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LifecycleState {
    /// In engineering design/modification.
    InWork,
    /// Frozen for review.
    InReview,
    /// Formally released for manufacturing.
    Released,
    /// Deprecated/Obsolete.
    Obsolete,
}

impl Default for LifecycleState {
    fn default() -> Self {
        Self::InWork
    }
}

/// Item master entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// PLM ID.
    pub id: ItemId,
    /// Official part number (e.g. "OX-100234-A").
    pub part_number: String,
    /// Descriptive title.
    pub name: String,
    /// Revision code (e.g. "A", "B.1").
    pub revision: String,
    /// Lifecycle state.
    pub lifecycle: LifecycleState,
    /// Unit cost estimate (USD).
    pub unit_cost: f64,
}

/// Bill of Materials (BOM) parent-child entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomEntry {
    /// Parent assembly item ID.
    pub parent: ItemId,
    /// Child component item ID.
    pub child: ItemId,
    /// Quantity required per parent assembly.
    pub quantity: f64,
    /// Find number on engineering drawing.
    pub find_number: u32,
}

/// Flat roll-up entry for indented and aggregated BOM exports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomRollupEntry {
    /// Item master record.
    pub item: Item,
    /// Total aggregated quantity across the entire assembly hierarchy.
    pub total_quantity: f64,
    /// Total extended cost (total_quantity * unit_cost).
    pub extended_cost: f64,
}

/// Engineering Change Order (ECO) tracking.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringChangeOrder {
    /// ECO number (e.g. "ECO-2026-0012").
    pub eco_number: String,
    /// Description of engineering modifications.
    pub description: String,
    /// Items affected by this change.
    pub affected_items: Vec<ItemId>,
    /// Closed/Approved status.
    pub approved: bool,
}

/// PLM Database Manager.
#[derive(Debug, Default)]
pub struct PlmDatabase {
    items: HashMap<ItemId, Item>,
    bom_graph: petgraph::graph::Graph<ItemId, f64, Directed>,
    node_map: HashMap<ItemId, NodeIndex>,
}

impl PlmDatabase {
    /// Create a new empty PLM database.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
            bom_graph: petgraph::graph::Graph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Register a new item in the item master.
    pub fn register_item(&mut self, item: Item) -> ItemId {
        let id = item.id;
        self.items.insert(id, item);
        if !self.node_map.contains_key(&id) {
            let idx = self.bom_graph.add_node(id);
            self.node_map.insert(id, idx);
        }
        id
    }

    /// Add a child component dependency into an assembly BOM.
    pub fn add_bom_child(&mut self, parent: ItemId, child: ItemId, quantity: f64) {
        let p_idx = *self
            .node_map
            .entry(parent)
            .or_insert_with(|| self.bom_graph.add_node(parent));
        let c_idx = *self
            .node_map
            .entry(child)
            .or_insert_with(|| self.bom_graph.add_node(child));
        self.bom_graph.add_edge(p_idx, c_idx, quantity);
    }

    /// Recursively calculate total aggregated bill of materials roll-up for a root assembly.
    pub fn calculate_bom_rollup(&self, root: ItemId) -> Vec<BomRollupEntry> {
        let mut quantities: HashMap<ItemId, f64> = HashMap::new();
        self.accumulate_children(root, 1.0, &mut quantities);

        let mut results = Vec::new();
        for (item_id, total_qty) in quantities {
            if let Some(item) = self.items.get(&item_id) {
                let extended_cost = total_qty * item.unit_cost;
                results.push(BomRollupEntry {
                    item: item.clone(),
                    total_quantity: total_qty,
                    extended_cost,
                });
            }
        }

        results.sort_by(|a, b| a.item.part_number.cmp(&b.item.part_number));
        results
    }

    fn accumulate_children(
        &self,
        current: ItemId,
        multiplier: f64,
        accum: &mut HashMap<ItemId, f64>,
    ) {
        if let Some(&node_idx) = self.node_map.get(&current) {
            for edge in self
                .bom_graph
                .edges_directed(node_idx, petgraph::Direction::Outgoing)
            {
                let child_id = self.bom_graph[edge.target()];
                let edge_qty = *edge.weight();
                let effective_qty = edge_qty * multiplier;

                *accum.entry(child_id).or_insert(0.0) += effective_qty;
                self.accumulate_children(child_id, effective_qty, accum);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plm_bom_hierarchy_and_cost_rollup() {
        let mut plm = PlmDatabase::new();

        let top_assy = Item {
            id: ItemId::default(),
            part_number: "ASY-001".into(),
            name: "Main Robotic Arm".into(),
            revision: "A".into(),
            lifecycle: LifecycleState::InWork,
            unit_cost: 0.0,
        };

        let sub_assy = Item {
            id: ItemId::default(),
            part_number: "ASY-002".into(),
            name: "Actuator Subassembly".into(),
            revision: "A".into(),
            lifecycle: LifecycleState::InWork,
            unit_cost: 50.0,
        };

        let motor = Item {
            id: ItemId::default(),
            part_number: "PRT-101".into(),
            name: "NEMA 23 Stepper Motor".into(),
            revision: "B".into(),
            lifecycle: LifecycleState::Released,
            unit_cost: 35.0,
        };

        let bolt = Item {
            id: ItemId::default(),
            part_number: "STD-M4".into(),
            name: "M4x16 Socket Head Cap Screw".into(),
            revision: "A".into(),
            lifecycle: LifecycleState::Released,
            unit_cost: 0.25,
        };

        let top_id = plm.register_item(top_assy);
        let sub_id = plm.register_item(sub_assy);
        let motor_id = plm.register_item(motor);
        let bolt_id = plm.register_item(bolt);

        // Main assembly has 2 Actuator subassemblies and 4 mounting bolts
        plm.add_bom_child(top_id, sub_id, 2.0);
        plm.add_bom_child(top_id, bolt_id, 4.0);

        // Each Actuator subassembly has 1 motor and 4 bolts
        plm.add_bom_child(sub_id, motor_id, 1.0);
        plm.add_bom_child(sub_id, bolt_id, 4.0);

        let rollup = plm.calculate_bom_rollup(top_id);

        let bolt_entry = rollup
            .iter()
            .find(|e| e.item.part_number == "STD-M4")
            .unwrap();
        // 4 top bolts + (2 actuators * 4 bolts) = 12 total bolts
        assert_eq!(bolt_entry.total_quantity, 12.0);
        assert!((bolt_entry.extended_cost - 3.0).abs() < 1e-4);

        let motor_entry = rollup
            .iter()
            .find(|e| e.item.part_number == "PRT-101")
            .unwrap();
        // 2 actuators * 1 motor = 2 motors
        assert_eq!(motor_entry.total_quantity, 2.0);
        assert!((motor_entry.extended_cost - 70.0).abs() < 1e-4);
    }
}
