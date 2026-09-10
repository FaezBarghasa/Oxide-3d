//! Oxide-3D B-Rep topology, parametric curves, surfaces, and geometric entities.

pub mod curve;
pub mod drafting_2d;
pub mod mesh_bridge;
pub mod surface;
pub mod topology;

pub use curve::Curve3d;
pub use drafting_2d::{CadLayer, DraftingDatabase2D, DraftingEntity2D, OsnapCandidate, OsnapMode, Point2D};
pub use mesh_bridge::{BrepTessellator, TessellatedMesh};
pub use surface::Surface3d;
pub use topology::{Edge, Face, Shell, Solid, TopologyDatabase, Vertex, Wire};
