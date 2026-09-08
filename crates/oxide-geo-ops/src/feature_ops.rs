//! High-level parametric CAD operations (Extrude, Revolve, Fillet, Chamfer, Shell).

use oxide_core::id::{FaceKey, ShellKey, SolidKey, VertexKey};
use oxide_geo::surface::Surface3d;
use oxide_geo::topology::TopologyDatabase;
use serde::{Deserialize, Serialize};

/// Parametric options for 3D extrusion.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExtrudeOptions {
    /// Linear distance to extrude along normal.
    pub distance: f64,
    /// Draft angle in radians (0.0 = no draft).
    pub draft_angle: f64,
    /// Symmetric bidirectional extrusion flag.
    pub symmetric: bool,
}

impl Default for ExtrudeOptions {
    fn default() -> Self {
        Self {
            distance: 10.0,
            draft_angle: 0.0,
            symmetric: false,
        }
    }
}

/// Parametric options for edge filleting.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FilletOptions {
    /// Constant or start fillet radius.
    pub radius: f64,
}

impl Default for FilletOptions {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}

/// Extrude a planar B-Rep face along a vector into a closed 3D solid body.
pub fn extrude_face(
    db: &mut TopologyDatabase,
    face_key: FaceKey,
    direction: [f64; 3],
    opts: ExtrudeOptions,
) -> Option<SolidKey> {
    let face = db.faces.get(face_key)?;
    let wire = db.wires.get(face.outer_wire)?;

    let mut base_vertices = Vec::new();
    for &edge_key in &wire.edges {
        let edge = db.edges.get(edge_key)?;
        base_vertices.push(edge.start);
    }

    if base_vertices.len() < 3 {
        return None;
    }

    let dir_len = (direction[0] * direction[0]
        + direction[1] * direction[1]
        + direction[2] * direction[2])
        .sqrt();
    let norm_dir = if dir_len > 1e-12 {
        [
            direction[0] / dir_len,
            direction[1] / dir_len,
            direction[2] / dir_len,
        ]
    } else {
        [0.0, 0.0, 1.0]
    };

    let offset_dist = opts.distance;
    let offset_vec = [
        norm_dir[0] * offset_dist,
        norm_dir[1] * offset_dist,
        norm_dir[2] * offset_dist,
    ];

    // Create extruded top vertices
    let mut top_vertices = Vec::with_capacity(base_vertices.len());
    for &v_base in &base_vertices {
        let pt = db.vertices.get(v_base)?.point;
        let top_pt = [pt[0] + offset_vec[0], pt[1] + offset_vec[1], pt[2] + offset_vec[2]];
        let v_top = db.add_vertex(top_pt);
        top_vertices.push(v_top);
    }

    let n = base_vertices.len();
    let mut lateral_faces = Vec::with_capacity(n);

    // Create lateral quad faces connecting base and top
    for i in 0..n {
        let next_i = (i + 1) % n;
        let b0 = base_vertices[i];
        let b1 = base_vertices[next_i];
        let t0 = top_vertices[i];
        let t1 = top_vertices[next_i];

        let e_bottom = db.add_edge(b0, b1, None);
        let e_right = db.add_edge(b1, t1, None);
        let e_top = db.add_edge(t1, t0, None);
        let e_left = db.add_edge(t0, b0, None);

        let quad_wire = db.add_wire([e_bottom, e_right, e_top, e_left]);
        let pt0 = db.vertices.get(b0)?.point;
        let lat_face = db.add_face(
            quad_wire,
            Surface3d::Plane {
                origin: pt0,
                normal: [norm_dir[1], -norm_dir[0], 0.0],
            },
        );
        lateral_faces.push(lat_face);
    }

    // Top cap face
    let mut top_edges = Vec::with_capacity(n);
    for i in 0..n {
        let next_i = (i + 1) % n;
        let e = db.add_edge(top_vertices[i], top_vertices[next_i], None);
        top_edges.push(e);
    }
    let top_wire = db.add_wire(top_edges);
    let top_pt = db.vertices.get(top_vertices[0])?.point;
    let top_face = db.add_face(
        top_wire,
        Surface3d::Plane {
            origin: top_pt,
            normal: norm_dir,
        },
    );

    let mut all_faces = vec![face_key, top_face];
    all_faces.extend(lateral_faces);

    let shell = db.add_shell(all_faces, true);
    Some(db.add_solid(shell))
}
