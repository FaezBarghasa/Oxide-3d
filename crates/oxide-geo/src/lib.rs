//! Oxide-3D B-Rep topology, parametric curves, surfaces, and geometric entities.

pub mod annotation;
pub mod curve;
pub mod drafting_2d;
pub mod half_edge;
pub mod mesh_bridge;
pub mod surface;
pub mod topology;

pub use annotation::{
    DimStyle, DimensionEntity, DimensionType, LayoutViewport, MultiLeader, PaperLayout,
};
pub use curve::Curve3d;
pub use drafting_2d::{
    CadLayer, DraftingDatabase2D, DraftingEntity2D, OsnapCandidate, OsnapMode, Point2D,
};
pub use half_edge::{HalfEdge, HalfEdgeKey, HalfEdgeMesh, HeFace, HeFaceKey, HeVertex, HeVertexKey};
pub use mesh_bridge::{BrepTessellator, TessellatedMesh};
pub use surface::Surface3d;
pub use topology::{Edge, Face, Shell, Solid, TopologyDatabase, Vertex, Wire};
