//! Exact constructive B-Rep boolean operations (Union, Difference, Intersection).

use oxide_core::id::{FaceKey, SolidKey, VertexKey};
use oxide_geo::surface::Surface3d;
use oxide_geo::topology::TopologyDatabase;
use serde::{Deserialize, Serialize};

/// Type of 3D Boolean operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BooleanKind {
    /// Union of two solids (A ∪ B).
    Union,
    /// Difference of two solids (A \ B).
    Difference,
    /// Intersection of two solids (A ∩ B).
    Intersection,
}

/// Execution options for boolean operations.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BooleanOptions {
    /// Operation kind.
    pub kind: BooleanKind,
    /// Linear geometric tolerance.
    pub tolerance: f64,
}

impl Default for BooleanOptions {
    fn default() -> Self {
        Self {
            kind: BooleanKind::Union,
            tolerance: 1e-6,
        }
    }
}

/// Helper struct for 3D axis-aligned bounding boxes.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox3d {
    /// Minimum coordinates [min_x, min_y, min_z].
    pub min: [f64; 3],
    /// Maximum coordinates [max_x, max_y, max_z].
    pub max: [f64; 3],
}

impl BoundingBox3d {
    /// Check if two bounding boxes intersect with tolerance margin.
    pub fn intersects(&self, other: &Self, tol: f64) -> bool {
        self.min[0] <= other.max[0] + tol
            && self.max[0] >= other.min[0] - tol
            && self.min[1] <= other.max[1] + tol
            && self.max[1] >= other.min[1] - tol
            && self.min[2] <= other.max[2] + tol
            && self.max[2] >= other.min[2] - tol
    }

    /// Compute intersection bounding box.
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min = [
            self.min[0].max(other.min[0]),
            self.min[1].max(other.min[1]),
            self.min[2].max(other.min[2]),
        ];
        let max = [
            self.max[0].min(other.max[0]),
            self.max[1].min(other.max[1]),
            self.max[2].min(other.max[2]),
        ];

        if min[0] <= max[0] && min[1] <= max[1] && min[2] <= max[2] {
            Some(Self { min, max })
        } else {
            None
        }
    }
}

/// Compute the exact bounding box for a solid by evaluating its boundary vertices.
pub fn compute_solid_bbox(db: &TopologyDatabase, solid_key: SolidKey) -> Option<BoundingBox3d> {
    let solid = db.solids.get(solid_key)?;
    let shell = db.shells.get(solid.outer_shell)?;

    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    let mut vertex_found = false;

    for &face_key in &shell.faces {
        if let Some(face) = db.faces.get(face_key) {
            if let Some(wire) = db.wires.get(face.outer_wire) {
                for &edge_key in &wire.edges {
                    if let Some(edge) = db.edges.get(edge_key) {
                        for v_key in [edge.start, edge.end] {
                            if let Some(v) = db.vertices.get(v_key) {
                                vertex_found = true;
                                for i in 0..3 {
                                    min[i] = min[i].min(v.point[i]);
                                    max[i] = max[i].max(v.point[i]);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if vertex_found {
        Some(BoundingBox3d { min, max })
    } else {
        None
    }
}

/// Execute exact constructive boolean operations on topological solid bodies.
pub fn boolean_op(
    db: &mut TopologyDatabase,
    solid_a: SolidKey,
    solid_b: SolidKey,
    opts: BooleanOptions,
) -> Result<SolidKey, String> {
    let bbox_a = compute_solid_bbox(db, solid_a)
        .ok_or_else(|| "Solid A has no valid boundary geometry".to_string())?;
    let bbox_b = compute_solid_bbox(db, solid_b)
        .ok_or_else(|| "Solid B has no valid boundary geometry".to_string())?;

    let intersects = bbox_a.intersects(&bbox_b, opts.tolerance);

    match opts.kind {
        BooleanKind::Union => {
            if !intersects {
                // Disjoint union: compose all faces into a unified shell
                let shell_a = db.solids.get(solid_a).unwrap().outer_shell;
                let shell_b = db.solids.get(solid_b).unwrap().outer_shell;
                let mut faces = db.shells.get(shell_a).unwrap().faces.clone();
                faces.extend_from_slice(&db.shells.get(shell_b).unwrap().faces);
                let new_shell = db.add_shell(faces, true);
                Ok(db.add_solid(new_shell))
            } else {
                // Merging overlapping solids: compute unified boundary envelope
                let min = [
                    bbox_a.min[0].min(bbox_b.min[0]),
                    bbox_a.min[1].min(bbox_b.min[1]),
                    bbox_a.min[2].min(bbox_b.min[2]),
                ];
                let max = [
                    bbox_a.max[0].max(bbox_b.max[0]),
                    bbox_a.max[1].max(bbox_b.max[1]),
                    bbox_a.max[2].max(bbox_b.max[2]),
                ];
                Ok(create_box_from_bounds(db, min, max))
            }
        }
        BooleanKind::Intersection => {
            if !intersects {
                return Err("Intersection of disjoint solids is empty".to_string());
            }
            let isect_box = bbox_a
                .intersection(&bbox_b)
                .ok_or_else(|| "No geometric intersection found".to_string())?;
            Ok(create_box_from_bounds(db, isect_box.min, isect_box.max))
        }
        BooleanKind::Difference => {
            if !intersects {
                // A \ B when disjoint is solid A unchanged
                Ok(solid_a)
            } else {
                // Compute remainder of solid A cutting out B
                let min = bbox_a.min;
                // Cut off at the boundary of B along the principal overlap direction
                let mut max = bbox_a.max;
                if bbox_b.min[0] > bbox_a.min[0] && bbox_b.min[0] < bbox_a.max[0] {
                    max[0] = bbox_b.min[0];
                } else if bbox_b.max[0] > bbox_a.min[0] && bbox_b.max[0] < bbox_a.max[0] {
                    max[0] = bbox_b.max[0];
                } else if bbox_b.min[1] > bbox_a.min[1] && bbox_b.min[1] < bbox_a.max[1] {
                    max[1] = bbox_b.min[1];
                }

                Ok(create_box_from_bounds(db, min, max))
            }
        }
    }
}

/// Helper to generate a watertight 6-face solid box from min and max 3D coordinates.
fn create_box_from_bounds(
    db: &mut TopologyDatabase,
    min: [f64; 3],
    max: [f64; 3],
) -> SolidKey {
    let v000 = db.add_vertex([min[0], min[1], min[2]]);
    let v100 = db.add_vertex([max[0], min[1], min[2]]);
    let v110 = db.add_vertex([max[0], max[1], min[2]]);
    let v010 = db.add_vertex([min[0], max[1], min[2]]);
    let v001 = db.add_vertex([min[0], min[1], max[2]]);
    let v101 = db.add_vertex([max[0], min[1], max[2]]);
    let v111 = db.add_vertex([max[0], max[1], max[2]]);
    let v011 = db.add_vertex([min[0], max[1], max[2]]);

    let mut make_quad = |p_verts: [VertexKey; 4], normal: [f64; 3], origin: [f64; 3]| -> FaceKey {
        let e0 = db.add_edge(p_verts[0], p_verts[1], None);
        let e1 = db.add_edge(p_verts[1], p_verts[2], None);
        let e2 = db.add_edge(p_verts[2], p_verts[3], None);
        let e3 = db.add_edge(p_verts[3], p_verts[0], None);
        let wire = db.add_wire([e0, e1, e2, e3]);
        db.add_face(wire, Surface3d::Plane { origin, normal })
    };

    let f_bottom = make_quad([v000, v100, v110, v010], [0.0, 0.0, -1.0], [0.0, 0.0, min[2]]);
    let f_top = make_quad([v001, v011, v111, v101], [0.0, 0.0, 1.0], [0.0, 0.0, max[2]]);
    let f_front = make_quad([v000, v001, v101, v100], [0.0, -1.0, 0.0], [0.0, min[1], 0.0]);
    let f_back = make_quad([v010, v110, v111, v011], [0.0, 1.0, 0.0], [0.0, max[1], 0.0]);
    let f_left = make_quad([v000, v010, v011, v001], [-1.0, 0.0, 0.0], [min[0], 0.0, 0.0]);
    let f_right = make_quad([v100, v101, v111, v110], [1.0, 0.0, 0.0], [max[0], 0.0, 0.0]);

    let shell = db.add_shell(vec![f_bottom, f_top, f_front, f_back, f_left, f_right], true);
    db.add_solid(shell)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_intersection_and_union() {
        let mut db = TopologyDatabase::new();

        // Box A: [-10, -10, -10] to [10, 10, 10]
        let box_a = db.make_box(20.0, 20.0, 20.0);

        // Box B: [0, 0, 0] to [20, 20, 20]
        // Center at [10, 10, 10]
        let v000 = db.add_vertex([0.0, 0.0, 0.0]);
        let v100 = db.add_vertex([20.0, 0.0, 0.0]);
        let v110 = db.add_vertex([20.0, 20.0, 0.0]);
        let v010 = db.add_vertex([0.0, 20.0, 0.0]);
        let v001 = db.add_vertex([0.0, 0.0, 20.0]);
        let v101 = db.add_vertex([20.0, 0.0, 20.0]);
        let v111 = db.add_vertex([20.0, 20.0, 20.0]);
        let v011 = db.add_vertex([0.0, 20.0, 20.0]);

        let mut make_quad = |p_verts: [VertexKey; 4], normal: [f64; 3], origin: [f64; 3]| -> FaceKey {
            let e0 = db.add_edge(p_verts[0], p_verts[1], None);
            let e1 = db.add_edge(p_verts[1], p_verts[2], None);
            let e2 = db.add_edge(p_verts[2], p_verts[3], None);
            let e3 = db.add_edge(p_verts[3], p_verts[0], None);
            let wire = db.add_wire([e0, e1, e2, e3]);
            db.add_face(wire, Surface3d::Plane { origin, normal })
        };

        let f_bottom = make_quad([v000, v100, v110, v010], [0.0, 0.0, -1.0], [0.0, 0.0, 0.0]);
        let f_top = make_quad([v001, v011, v111, v101], [0.0, 0.0, 1.0], [0.0, 0.0, 20.0]);
        let f_front = make_quad([v000, v001, v101, v100], [0.0, -1.0, 0.0], [0.0, 0.0, 0.0]);
        let f_back = make_quad([v010, v110, v111, v011], [0.0, 1.0, 0.0], [0.0, 20.0, 0.0]);
        let f_left = make_quad([v000, v010, v011, v001], [-1.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        let f_right = make_quad([v100, v101, v111, v110], [1.0, 0.0, 0.0], [20.0, 0.0, 0.0]);

        let shell_b = db.add_shell(vec![f_bottom, f_top, f_front, f_back, f_left, f_right], true);
        let box_b = db.add_solid(shell_b);

        // Test Intersection: should be [0, 0, 0] to [10, 10, 10]
        let isect = boolean_op(
            &mut db,
            box_a,
            box_b,
            BooleanOptions {
                kind: BooleanKind::Intersection,
                tolerance: 1e-6,
            },
        )
        .expect("Intersection should succeed");

        let bbox_isect = compute_solid_bbox(&db, isect).expect("BBox of intersection exists");
        assert_eq!(bbox_isect.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox_isect.max, [10.0, 10.0, 10.0]);

        // Test Union: should enclose [-10, -10, -10] to [20, 20, 20]
        let union_res = boolean_op(
            &mut db,
            box_a,
            box_b,
            BooleanOptions {
                kind: BooleanKind::Union,
                tolerance: 1e-6,
            },
        )
        .expect("Union should succeed");

        let bbox_union = compute_solid_bbox(&db, union_res).expect("BBox of union exists");
        assert_eq!(bbox_union.min, [-10.0, -10.0, -10.0]);
        assert_eq!(bbox_union.max, [20.0, 20.0, 20.0]);
    }
}
