//! B-Rep Topology entities (Vertex, Edge, Wire, Face, Shell, Solid) and storage arena.

use crate::curve::Curve3d;
use crate::surface::Surface3d;
use oxide_core::id::{EdgeKey, FaceKey, ShellKey, SolidKey, VertexKey, WireKey};
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;
use smallvec::SmallVec;

/// 0D B-Rep Vertex.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    /// 3D Cartesian coordinates [x, y, z].
    pub point: [f64; 3],
}

impl Vertex {
    /// Create a new vertex at Cartesian coordinates.
    #[must_use]
    pub const fn new(point: [f64; 3]) -> Self {
        Self { point }
    }
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

impl Edge {
    /// Create a new edge connecting start and end vertices.
    #[must_use]
    pub const fn new(start: VertexKey, end: VertexKey, curve: Option<Curve3d>) -> Self {
        Self { start, end, curve }
    }
}

/// 1D Closed B-Rep Wire of contiguous edges.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wire {
    /// Ordered sequence of oriented edge keys.
    pub edges: SmallVec<[EdgeKey; 4]>,
}

impl Wire {
    /// Create a new wire from an ordered list of edge keys.
    #[must_use]
    pub fn new(edges: impl IntoIterator<Item = EdgeKey>) -> Self {
        Self {
            edges: edges.into_iter().collect(),
        }
    }
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

impl Face {
    /// Create a new face on a parametric surface bounded by an outer wire.
    #[must_use]
    pub fn new(outer_wire: WireKey, surface: Surface3d) -> Self {
        Self {
            outer_wire,
            inner_wires: SmallVec::new(),
            surface,
        }
    }
}

/// 2D Connected Shell of contiguous faces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shell {
    /// Component faces.
    pub faces: Vec<FaceKey>,
    /// Closed watertight flag.
    pub is_closed: bool,
}

impl Shell {
    /// Create a new shell from a list of face keys.
    #[must_use]
    pub fn new(faces: Vec<FaceKey>, is_closed: bool) -> Self {
        Self { faces, is_closed }
    }
}

/// 3D B-Rep Solid bounded by outer shell and optional internal cavity shells.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solid {
    /// Outer boundary shell key.
    pub outer_shell: ShellKey,
    /// Internal void shells.
    pub void_shells: Vec<ShellKey>,
}

impl Solid {
    /// Create a new solid body bounded by an outer shell.
    #[must_use]
    pub fn new(outer_shell: ShellKey) -> Self {
        Self {
            outer_shell,
            void_shells: Vec::new(),
        }
    }
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

    /// Add a 3D vertex to the database.
    pub fn add_vertex(&mut self, point: [f64; 3]) -> VertexKey {
        self.vertices.insert(Vertex::new(point))
    }

    /// Add an edge connecting two vertices.
    pub fn add_edge(
        &mut self,
        start: VertexKey,
        end: VertexKey,
        curve: Option<Curve3d>,
    ) -> EdgeKey {
        self.edges.insert(Edge::new(start, end, curve))
    }

    /// Add a closed boundary wire.
    pub fn add_wire(&mut self, edges: impl IntoIterator<Item = EdgeKey>) -> WireKey {
        self.wires.insert(Wire::new(edges))
    }

    /// Add a bounded face.
    pub fn add_face(&mut self, outer_wire: WireKey, surface: Surface3d) -> FaceKey {
        self.faces.insert(Face::new(outer_wire, surface))
    }

    /// Add a shell of connected faces.
    pub fn add_shell(&mut self, faces: Vec<FaceKey>, is_closed: bool) -> ShellKey {
        self.shells.insert(Shell::new(faces, is_closed))
    }

    /// Add a solid bounded by a shell.
    pub fn add_solid(&mut self, outer_shell: ShellKey) -> SolidKey {
        self.solids.insert(Solid::new(outer_shell))
    }

    /// Create an exact B-Rep solid box with dimensions (dx, dy, dz) centered at origin.
    pub fn make_box(&mut self, dx: f64, dy: f64, dz: f64) -> SolidKey {
        let hx = dx * 0.5;
        let hy = dy * 0.5;
        let hz = dz * 0.5;

        // 8 Vertices
        let v000 = self.add_vertex([-hx, -hy, -hz]);
        let v100 = self.add_vertex([hx, -hy, -hz]);
        let v110 = self.add_vertex([hx, hy, -hz]);
        let v010 = self.add_vertex([-hx, hy, -hz]);
        let v001 = self.add_vertex([-hx, -hy, hz]);
        let v101 = self.add_vertex([hx, -hy, hz]);
        let v111 = self.add_vertex([hx, hy, hz]);
        let v011 = self.add_vertex([-hx, hy, hz]);

        // Helper to create a planar quad face
        let mut make_quad =
            |p_verts: [VertexKey; 4], normal: [f64; 3], origin: [f64; 3]| -> FaceKey {
                let e0 = self.add_edge(p_verts[0], p_verts[1], None);
                let e1 = self.add_edge(p_verts[1], p_verts[2], None);
                let e2 = self.add_edge(p_verts[2], p_verts[3], None);
                let e3 = self.add_edge(p_verts[3], p_verts[0], None);
                let wire = self.add_wire([e0, e1, e2, e3]);
                self.add_face(wire, Surface3d::Plane { origin, normal })
            };

        // 6 Faces
        let f_bottom = make_quad([v000, v100, v110, v010], [0.0, 0.0, -1.0], [0.0, 0.0, -hz]);
        let f_top = make_quad([v001, v011, v111, v101], [0.0, 0.0, 1.0], [0.0, 0.0, hz]);
        let f_front = make_quad([v000, v001, v101, v100], [0.0, -1.0, 0.0], [0.0, -hy, 0.0]);
        let f_back = make_quad([v010, v110, v111, v011], [0.0, 1.0, 0.0], [0.0, hy, 0.0]);
        let f_left = make_quad([v000, v010, v011, v001], [-1.0, 0.0, 0.0], [-hx, 0.0, 0.0]);
        let f_right = make_quad([v100, v101, v111, v110], [1.0, 0.0, 0.0], [hx, 0.0, 0.0]);

        let shell = self.add_shell(
            vec![f_bottom, f_top, f_front, f_back, f_left, f_right],
            true,
        );
        self.add_solid(shell)
    }
}
