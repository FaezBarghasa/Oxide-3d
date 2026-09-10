//! Oxide-3D B-Rep topology, parametric curves, surfaces, and geometric entities.

pub mod curve;
pub mod mesh_bridge;
pub mod surface;
pub mod topology;

pub use curve::Curve3d;
pub use mesh_bridge::{BrepTessellator, TessellatedMesh};
pub use surface::Surface3d;
pub use topology::{Edge, Face, Shell, Solid, TopologyDatabase, Vertex, Wire};
