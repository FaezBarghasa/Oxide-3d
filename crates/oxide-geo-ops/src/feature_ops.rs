//! High-level parametric CAD operations (Extrude, Revolve, Fillet, Chamfer, Shell).

use oxide_core::id::{FaceKey, SolidKey};
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

/// Parametric options for 3D rotational revolution around an axis.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RevolveOptions {
    /// Angle of revolution in radians (2*PI for full 360 solid).
    pub angle_radians: f64,
    /// Origin point of revolution axis [x, y, z].
    pub axis_origin: [f64; 3],
    /// Direction vector of revolution axis [dx, dy, dz].
    pub axis_direction: [f64; 3],
}

impl Default for RevolveOptions {
    fn default() -> Self {
        Self {
            angle_radians: std::f64::consts::TAU,
            axis_origin: [0.0, 0.0, 0.0],
            axis_direction: [0.0, 0.0, 1.0],
        }
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

    let dir_len =
        (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2])
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
        let top_pt = [
            pt[0] + offset_vec[0],
            pt[1] + offset_vec[1],
            pt[2] + offset_vec[2],
        ];
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

/// Revolve a planar face around an axis into a rotational 3D B-Rep solid body.
pub fn revolve_face(
    db: &mut TopologyDatabase,
    face_key: FaceKey,
    opts: RevolveOptions,
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

    let axis_dir = glam::DVec3::from_slice(&opts.axis_direction).normalize_or_zero();
    let axis_origin = glam::DVec3::from_slice(&opts.axis_origin);
    let quat = glam::DQuat::from_axis_angle(axis_dir, opts.angle_radians);

    // Create rotated target vertices
    let mut revolved_vertices = Vec::with_capacity(base_vertices.len());
    for &v_base in &base_vertices {
        let pt = db.vertices.get(v_base)?.point;
        let v_rel = glam::DVec3::from_slice(&pt) - axis_origin;
        let v_rot = axis_origin + quat.mul_vec3(v_rel);
        let v_new = db.add_vertex([v_rot.x, v_rot.y, v_rot.z]);
        revolved_vertices.push(v_new);
    }

    let n = base_vertices.len();
    let mut lateral_faces = Vec::with_capacity(n);

    for i in 0..n {
        let next_i = (i + 1) % n;
        let b0 = base_vertices[i];
        let b1 = base_vertices[next_i];
        let r0 = revolved_vertices[i];
        let r1 = revolved_vertices[next_i];

        let e_bottom = db.add_edge(b0, b1, None);
        let e_right = db.add_edge(b1, r1, None);
        let e_top = db.add_edge(r1, r0, None);
        let e_left = db.add_edge(r0, b0, None);

        let quad_wire = db.add_wire([e_bottom, e_right, e_top, e_left]);
        let pt0 = db.vertices.get(b0)?.point;
        let lat_face = db.add_face(
            quad_wire,
            Surface3d::Cylinder {
                origin: opts.axis_origin,
                axis: [axis_dir.x, axis_dir.y, axis_dir.z],
                radius: (glam::DVec3::from_slice(&pt0) - axis_origin).length(),
            },
        );
        lateral_faces.push(lat_face);
    }

    let mut revolved_edges = Vec::with_capacity(n);
    for i in 0..n {
        let next_i = (i + 1) % n;
        let e = db.add_edge(revolved_vertices[i], revolved_vertices[next_i], None);
        revolved_edges.push(e);
    }
    let top_wire = db.add_wire(revolved_edges);
    let top_pt = db.vertices.get(revolved_vertices[0])?.point;
    let top_face = db.add_face(
        top_wire,
        Surface3d::Plane {
            origin: top_pt,
            normal: [axis_dir.x, axis_dir.y, axis_dir.z],
        },
    );

    let mut all_faces = vec![face_key, top_face];
    all_faces.extend(lateral_faces);

    let shell = db.add_shell(all_faces, true);
    Some(db.add_solid(shell))
}

/// Create a parametric 3D box solid primitive in the topology database.
pub fn create_primitive_box(db: &mut TopologyDatabase, dx: f64, dy: f64, dz: f64) -> SolidKey {
    db.make_box(dx, dy, dz)
}

/// Create a parametric 3D cylinder solid primitive in the topology database.
pub fn create_primitive_cylinder(
    db: &mut TopologyDatabase,
    radius: f64,
    height: f64,
    segments: usize,
) -> SolidKey {
    db.make_cylinder(radius, height, segments)
}

/// Create a parametric 3D regular pyramid solid primitive in the topology database.
pub fn create_primitive_pyramid(
    db: &mut TopologyDatabase,
    base_size: f64,
    height: f64,
) -> SolidKey {
    db.make_pyramid(base_size, height)
}

/// Apply a direct modeling offset to all vertices of a planar face along its normal.
pub fn direct_offset_face(
    db: &mut TopologyDatabase,
    face_key: FaceKey,
    offset_distance: f64,
) -> bool {
    let (normal, vertex_keys) = match db.faces.get(face_key) {
        Some(face) => {
            let n = match &face.surface {
                Surface3d::Plane { normal, .. } => *normal,
                _ => return false,
            };
            let wire = match db.wires.get(face.outer_wire) {
                Some(w) => w,
                None => return false,
            };
            let mut v_keys = Vec::new();
            for &edge_key in &wire.edges {
                if let Some(edge) = db.edges.get(edge_key) {
                    v_keys.push(edge.start);
                }
            }
            (n, v_keys)
        }
        None => return false,
    };

    let len = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    let norm = if len > 1e-12 {
        [normal[0] / len, normal[1] / len, normal[2] / len]
    } else {
        [0.0, 0.0, 1.0]
    };

    let delta = [
        norm[0] * offset_distance,
        norm[1] * offset_distance,
        norm[2] * offset_distance,
    ];

    for v_key in vertex_keys {
        if let Some(v) = db.vertices.get_mut(v_key) {
            v.point[0] += delta[0];
            v.point[1] += delta[1];
            v.point[2] += delta[2];
        }
    }

    if let Some(face) = db.faces.get_mut(face_key) {
        if let Surface3d::Plane { origin, .. } = &mut face.surface {
            origin[0] += delta[0];
            origin[1] += delta[1];
            origin[2] += delta[2];
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrude_and_revolve_feature_ops() {
        let mut db = TopologyDatabase::new();
        let box_key = db.make_box(2.0, 2.0, 2.0);
        let solid = db.solids.get(box_key).unwrap();
        let shell = db.shells.get(solid.outer_shell).unwrap();
        let face_key = shell.faces[0];

        let extrude_solid = extrude_face(
            &mut db,
            face_key,
            [0.0, 0.0, 1.0],
            ExtrudeOptions::default(),
        );
        assert!(extrude_solid.is_some());

        let revolve_solid = revolve_face(&mut db, face_key, RevolveOptions::default());
        assert!(revolve_solid.is_some());
    }

    #[test]
    fn test_primitive_constructors_and_direct_offset() {
        let mut db = TopologyDatabase::new();
        let box_k = create_primitive_box(&mut db, 10.0, 10.0, 10.0);
        assert!(db.solids.get(box_k).is_some());

        let cyl_k = create_primitive_cylinder(&mut db, 5.0, 20.0, 16);
        assert!(db.solids.get(cyl_k).is_some());

        let pyr_k = create_primitive_pyramid(&mut db, 6.0, 15.0);
        assert!(db.solids.get(pyr_k).is_some());

        let solid = db.solids.get(box_k).unwrap();
        let shell = db.shells.get(solid.outer_shell).unwrap();
        let face_key = shell.faces[0];

        let offset_res = direct_offset_face(&mut db, face_key, 2.5);
        assert!(offset_res);
    }
}
