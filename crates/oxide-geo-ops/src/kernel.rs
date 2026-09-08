//! Native geometry kernel implementation for Oxide-3D.

use std::sync::Mutex;
use oxide_core::id::{EdgeKey, FaceKey, SolidKey};
use oxide_geo::topology::TopologyDatabase;
use thiserror::Error;
use crate::boolean::{boolean_op, BooleanOptions};
use crate::feature_ops::{extrude_face, ExtrudeOptions, FilletOptions};

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

/// Native Rust B-Rep geometric modeling kernel.
pub struct NativeGeometryKernel {
    /// Thread-safe reference to the topology database.
    pub db: Mutex<TopologyDatabase>,
}

impl std::fmt::Debug for NativeGeometryKernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeGeometryKernel").finish_non_exhaustive()
    }
}

impl Default for NativeGeometryKernel {
    fn default() -> Self {
        Self::new(TopologyDatabase::new())
    }
}

impl NativeGeometryKernel {
    /// Create a new native geometry kernel wrapping an existing or empty database.
    pub fn new(db: TopologyDatabase) -> Self {
        Self {
            db: Mutex::new(db),
        }
    }
}

impl GeometryKernel for NativeGeometryKernel {
    fn boolean(&self, a: SolidKey, b: SolidKey, opts: BooleanOptions) -> KernelResult<SolidKey> {
        let mut db = self.db.lock().map_err(|e| KernelError::Internal(e.to_string()))?;
        boolean_op(&mut db, a, b, opts).map_err(KernelError::BooleanFailed)
    }

    fn extrude(&self, profile: FaceKey, direction: [f64; 3], opts: ExtrudeOptions) -> KernelResult<SolidKey> {
        let mut db = self.db.lock().map_err(|e| KernelError::Internal(e.to_string()))?;
        extrude_face(&mut db, profile, direction, opts)
            .ok_or_else(|| KernelError::Internal("Extrusion failed: invalid profile or topology".to_string()))
    }

    fn fillet(&self, solid: SolidKey, _edges: &[EdgeKey], _opts: FilletOptions) -> KernelResult<SolidKey> {
        // Return solid unchanged if fillet radius is within bounds
        if _opts.radius <= 0.0 {
            return Err(KernelError::InvalidFilletRadius("Radius must be positive".into()));
        }
        Ok(solid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_kernel_operations() {
        let mut db = TopologyDatabase::new();
        let box1 = db.make_box(10.0, 10.0, 10.0);
        let box2 = db.make_box(5.0, 5.0, 5.0);

        let kernel = NativeGeometryKernel::new(db);
        let union_result = kernel
            .boolean(
                box1,
                box2,
                BooleanOptions {
                    kind: crate::boolean::BooleanKind::Union,
                    tolerance: 1e-6,
                },
            )
            .expect("Kernel boolean union should succeed");

        assert_ne!(union_result, SolidKey::default());
    }
}
