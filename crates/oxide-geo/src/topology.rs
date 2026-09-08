use oxide_core::id::{EdgeKey, FaceKey, ShellKey, SolidKey, VertexKey, WireKey};
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use smallvec::SmallVec;
use crate::curve::Curve3d;
use crate::surface::Surface3d;

/// 0D B-Rep Vertex.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    /// 3D Cartesian coordinates.
    pub point: [f64; 3],
}

/// 1D B-Rep Edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Start vertex key.
    pub start: VertexKey,
    /// End vertex key.
    pub end: VertexKey,
    /// Underlying parametric 3D curve.
    pub curve: Option<Curve3d>,
}

/// 1D Closed B-Rep Wire of contiguous edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wire {
    /// Ordered sequence of oriented edge keys.
    pub edges: SmallVec<[EdgeKey; 4]>,
}

/// 2D B-Rep Face bounded by outer/inner wires on a surface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    /// Outer boundary wire key.
    pub outer_wire: WireKey,
    /// Inner boundary holes.
    pub inner_wires: SmallVec<[WireKey; 2]>,
    /// Underlying parametric 3D surface.
    pub surface: Surface3d,
}

/// 2D Connected Shell of contiguous faces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shell {
    /// Component faces.
    pub faces: Vec<FaceKey>,
    /// Closed watertight flag.
    pub is_closed: bool,
}

/// 3D B-Rep Solid bounded by outer shell and optional internal cavity shells.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solid {
    /// Outer boundary shell key.
    pub outer_shell: ShellKey,
    /// Internal void shells.
    pub void_shells: Vec<ShellKey>,
}

/// High-performance arena and slotmap database for B-Rep topology entities.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TopologyDatabase {
    /// Vertices pool.
    pub vertices: SlotMap<VertexKey, Vertex>,
    /// Edges pool.
    pub edges: SlotMap<EdgeKey, Edge>,
    /// Wires pool.
    pub wires: SlotMap<WireKey, Wire>,
    /// Faces pool.
    pub faces: SlotMap<FaceKey, Face>,
    /// Shells pool.
    pub shells: SlotMap<ShellKey, Shell>,
    /// Solids pool.
    pub solids: SlotMap<SolidKey, Solid>,
}

impl TopologyDatabase {
    /// Create a new empty topology database.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
