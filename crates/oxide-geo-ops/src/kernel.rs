use oxide_core::id::{EdgeKey, FaceKey, SolidKey};
use thiserror::Error;
use crate::boolean::BooleanOptions;
use crate::feature_ops::{ExtrudeOptions, FilletOptions};

/// Geometric kernel execution errors.
#[derive(Debug, Error)]
pub enum KernelError {
    /// Non-manifold geometry detected.
    #[error("Non-manifold geometry error: {0}")]
    NonManifold(String),

    /// Boolean operation failed to evaluate cleanly.
    #[error("Boolean operation failure: {0}")]
    BooleanFailed(String),

    /// Fillet radius exceeds topology limit.
    #[error("Fillet radius out of range: {0}")]
    InvalidFilletRadius(String),

    /// General internal kernel error.
    #[error("Kernel error: {0}")]
    Internal(String),
}

/// Standard result for kernel operations.
pub type KernelResult<T> = Result<T, KernelError>;

/// Pluggable CAD geometry kernel trait abstraction.
pub trait GeometryKernel: Send + Sync {
    /// Execute a 3D boolean operation between two solid bodies.
    fn boolean(&self, a: SolidKey, b: SolidKey, opts: BooleanOptions) -> KernelResult<SolidKey>;

    /// Extrude a closed profile face into a solid body.
    fn extrude(&self, profile: FaceKey, direction: [f64; 3], opts: ExtrudeOptions) -> KernelResult<SolidKey>;

    /// Apply round fillets to specified edges.
    fn fillet(&self, solid: SolidKey, edges: &[EdgeKey], opts: FilletOptions) -> KernelResult<SolidKey>;
}
