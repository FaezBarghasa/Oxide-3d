//! Geometric mathematics, interval arithmetic, tolerance-aware predicates, and transforms.

pub mod interval;
pub mod predicates;
pub mod tolerance;
pub mod transform;

pub use interval::Interval;
pub use predicates::{orient2d, orient3d};
pub use tolerance::Tolerance;
pub use transform::Transform3;
