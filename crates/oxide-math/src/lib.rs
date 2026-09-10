//! Geometric mathematics, interval arithmetic, tolerance-aware predicates, and transforms.

/// Interval arithmetic for bound analysis.
pub mod interval;
/// Robust geometric orientation predicates.
pub mod predicates;
/// Precision tolerances and numerical epsilon comparisons.
pub mod tolerance;
/// 3D affine transformations and rigid body matrices.
pub mod transform;

pub use interval::Interval;
pub use predicates::{orient2d, orient3d};
pub use tolerance::Tolerance;
pub use transform::Transform3;
