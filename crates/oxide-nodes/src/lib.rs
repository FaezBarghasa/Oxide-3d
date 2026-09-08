//! Oxide-3D Procedural Geometry Nodes Dataflow Engine.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Geometry Nodes evaluation errors.
#[derive(Debug, Error)]
pub enum NodeError {
    /// Type mismatch on connected sockets.
    #[error("Type mismatch: expected {expected}, got {found}")]
    TypeMismatch { expected: String, found: String },

    /// Cycle detected in node graph.
    #[error("Cycle detected in node execution DAG")]
    CycleDetected,

    /// General node execution error.
    #[error("Node evaluation failed: {0}")]
    EvaluationFailed(String),
}

/// Dynamic value flowing through node graph sockets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeSocketValue {
    /// Floating-point scalar.
    Float(f64),
    /// Integer scalar.
    Int(i64),
    /// Boolean flag.
    Bool(bool),
    /// 3D Vector.
    Vector([f64; 3]),
    /// Text string.
    String(String),
}

/// Socket definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketDef {
    /// Socket name.
    pub name: String,
    /// Default socket type.
    pub socket_type: String,
}

/// Trait for procedural geometry nodes.
pub trait OxideNode: Send + Sync {
    /// Human-readable node type name.
    fn name(&self) -> &'static str;
    /// Input socket definitions.
    fn inputs(&self) -> Vec<SocketDef>;
    /// Output socket definitions.
    fn outputs(&self) -> Vec<SocketDef>;
    /// Evaluate the node's mathematical/geometric logic.
    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError>;
}
