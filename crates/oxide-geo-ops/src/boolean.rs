use serde::{Deserialize, Serialize};

/// Type of 3D Boolean operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BooleanKind {
    /// Union of two solids (A ∪ B).
    Union,
    /// Difference of two solids (A \ B).
    Difference,
    /// Intersection of two solids (A ∩ B).
    Intersection,
}

/// Execution options for boolean operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanOptions {
    /// Operation kind.
    pub kind: BooleanKind,
    /// Linear geometric tolerance.
    pub tolerance: f64,
}

impl Default for BooleanOptions {
    fn default() -> Self {
        Self {
            kind: BooleanKind::Union,
            tolerance: 1e-6,
        }
    }
}
