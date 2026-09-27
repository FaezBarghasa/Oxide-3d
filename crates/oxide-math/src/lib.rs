//! Geometric mathematics, interval arithmetic, tolerance-aware predicates, and transforms.

/// Interval arithmetic for bound analysis.
pub mod interval;
/// Robust geometric orientation predicates.
pub mod predicates;
/// Precision tolerances and numerical epsilon comparisons.
pub mod tolerance;
/// 3D affine transformations and rigid body matrices.
pub mod transform;

pub use interval::{Interval, Interval3d};
pub use predicates::{orient2d, orient2d_tol, orient3d, orient3d_tol, incircle, incircle_tol, insphere, insphere_tol};
pub use tolerance::{Tolerance, ToleranceContext};
pub use transform::Transform3;
