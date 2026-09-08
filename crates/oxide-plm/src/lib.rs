//! Oxide-3D Product Lifecycle Management (PLM), Item Masters, Revisions, and BOM hierarchies.

use serde::{Deserialize, Serialize};
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
