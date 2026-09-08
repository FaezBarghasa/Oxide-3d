//! Oxide-3D Parametric Feature Tree and Dependency Graph.

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};

/// Type of parametric CAD feature in the model history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureKind {
    /// Datum reference plane or coordinate system.
    DatumPlane { name: String },
    /// 2D sketch constraint plane.
    Sketch { name: String },
    /// Extrude feature.
    Extrude { depth: f64 },
    /// Revolve feature.
    Revolve { angle_rad: f64 },
    /// Fillet feature.
    Fillet { radius: f64 },
    /// Chamfer feature.
    Chamfer { distance: f64 },
}

/// A node in the parametric feature tree.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureNode {
    /// Human-readable feature name (e.g. "Extrude.1").
    pub name: String,
    /// Feature parameters and kind.
    pub kind: FeatureKind,
    /// Suppressed/unsuppressed flag.
    pub is_suppressed: bool,
}

/// Dependency relationship edge between features.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FeatureDependency {
    /// Direct geometric reference (e.g. sketch depends on plane).
    Reference,
    /// Upstream topology consumption.
    Consumes,
}

/// Parametric feature history dependency DAG.
#[derive(Debug, Default)]
pub struct FeatureGraph {
    /// Directed graph connecting features in dependency order.
    pub graph: DiGraph<FeatureNode, FeatureDependency>,
}

impl FeatureGraph {
    /// Create a new empty feature tree.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a feature node to the tree.
    pub fn add_feature(&mut self, node: FeatureNode) -> NodeIndex {
        self.graph.add_node(node)
    }
}
