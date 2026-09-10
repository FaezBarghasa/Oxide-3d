//! B-Rep Topology to Polygon Mesh tessellation and synchronization bridge.

use crate::topology::TopologyDatabase;
use oxide_core::id::SolidKey;
use serde::{Deserialize, Serialize};

/// Lightweight tessellated polygon mesh generated from exact B-Rep topology.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TessellatedMesh {
    /// 3D Vertex positions [x, y, z].
    pub positions: Vec<[f32; 3]>,
    /// 3D Vertex normals [nx, ny, nz].
    pub normals: Vec<[f32; 3]>,
    /// Triangle index buffer.
    pub indices: Vec<u32>,
}

impl TessellatedMesh {
    /// Create a new empty tessellated mesh.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        }
    }
}

/// Real-time B-Rep to Polygon Mesh tessellator and synchronization bridge.
#[derive(Debug, Clone, Copy, Default)]
pub struct BrepTessellator {
    /// Chordal tolerance for curve/surface sampling (linear error).
    pub tolerance: f64,
    /// Angular deflection tolerance in radians.
    pub angular_tolerance: f64,
}

impl BrepTessellator {
    /// Create a new tessellator with given linear tolerance.
    #[must_use]
    pub const fn new(tolerance: f64) -> Self {
        Self {
            tolerance,
            angular_tolerance: 0.1,
        }
    }

    /// Tessellate a Solid from the topology database into a triangle mesh.
    #[must_use]
    pub fn tessellate_solid(&self, db: &TopologyDatabase, solid_key: SolidKey) -> TessellatedMesh {
        let mut mesh = TessellatedMesh::new();

        let solid = match db.solids.get(solid_key) {
            Some(s) => s,
            None => return mesh,
        };

        let shell = match db.shells.get(solid.outer_shell) {
            Some(sh) => sh,
            None => return mesh,
        };

        for &face_key in &shell.faces {
            let face = match db.faces.get(face_key) {
                Some(f) => f,
                None => continue,
            };

            let wire = match db.wires.get(face.outer_wire) {
                Some(w) => w,
                None => continue,
            };

            let mut face_pts = Vec::new();
            for &edge_key in &wire.edges {
                if let Some(edge) = db.edges.get(edge_key) {
                    if let Some(v_start) = db.vertices.get(edge.start) {
                        face_pts.push(v_start.point);
                    }
                }
            }

            if face_pts.len() < 3 {
                continue;
            }

            // Normal from analytical surface at parametric midpoint (0.5, 0.5)
            let norm = face.surface.normal(0.5, 0.5);
            let n_f32 = [norm[0] as f32, norm[1] as f32, norm[2] as f32];

            let base_idx = mesh.positions.len() as u32;
            for p in &face_pts {
                mesh.positions.push([p[0] as f32, p[1] as f32, p[2] as f32]);
                mesh.normals.push(n_f32);
            }

            // Fan triangulation for convex/planar face boundaries
            for i in 1..(face_pts.len() - 1) {
                mesh.indices.push(base_idx);
                mesh.indices.push(base_idx + i as u32);
                mesh.indices.push(base_idx + (i + 1) as u32);
            }
        }

        mesh
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brep_tessellation() {
        let mut db = TopologyDatabase::new();
        let box_solid = db.make_box(10.0, 20.0, 30.0);

        let tessellator = BrepTessellator::new(0.01);
        let mesh = tessellator.tessellate_solid(&db, box_solid);

        // A box with 6 quad faces triangulated = 12 triangles = 36 indices
        assert_eq!(mesh.indices.len(), 36);
        assert!(!mesh.positions.is_empty());
        assert_eq!(mesh.positions.len(), mesh.normals.len());
    }
}
