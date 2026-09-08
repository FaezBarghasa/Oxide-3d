//! Oxide-3D Geometric operations: Booleans, Extrusions, Fillets, Chamfers, and Kernel traits.

pub mod boolean;
pub mod feature_ops;
pub mod kernel;

pub use boolean::{BooleanKind, BooleanOptions};
pub use feature_ops::{ExtrudeOptions, FilletOptions};
pub use kernel::{GeometryKernel, KernelResult};
