//! Oxide-3D Geometric operations: Booleans, Extrusions, Fillets, Chamfers, and Kernel traits.

pub mod boolean;
pub mod feature_ops;
pub mod kernel;
pub mod sketch_solver;

pub use boolean::{BooleanKind, BooleanOptions};
pub use feature_ops::{extrude_face, revolve_face, ExtrudeOptions, FilletOptions, RevolveOptions};
pub use kernel::{GeometryKernel, KernelError, KernelResult, NativeGeometryKernel};
pub use sketch_solver::{SketchConstraint, SketchPoint2d, SketchSolver};
