//! Oxide-3D Geometric operations: Booleans, Extrusions, Fillets, Chamfers, and Kernel traits.

pub mod boolean;
pub mod dfm;
pub mod drawing_sheet;
pub mod feature_ops;
pub mod kernel;
pub mod sketch_solver;
pub mod solver_3d;

pub use boolean::{BooleanKind, BooleanOptions};
pub use dfm::{
    AdditiveDfmConfig, CncMillingDfmConfig, DfmEngine, DfmIssue, DfmSeverity,
    InjectionMoldingDfmConfig,
};
pub use drawing_sheet::{
    DrawingSheet, DrawingViewKind, DrawingViewport, ProjectedEdge2D,
};
pub use feature_ops::{
    ExtrudeOptions, FilletOptions, RevolveOptions, create_primitive_box, create_primitive_cylinder,
    create_primitive_pyramid, direct_offset_face, extrude_face, revolve_face,
};
pub use kernel::{GeometryKernel, KernelError, KernelResult, NativeGeometryKernel};
pub use sketch_solver::{SketchConstraint, SketchPoint2d, SketchSolver};
pub use solver_3d::{
    Constraint3D, ConstraintSolver3D, DualQuaternion, SolverConfig, SolverStatus,
};

