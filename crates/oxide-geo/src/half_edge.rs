//! Manifold Half-Edge Data Structure for mesh editing, subdivisions, and sculpting.

use glam::DVec3;
use serde::{Deserialize, Serialize};
use slotmap::{SlotMap, new_key_type};

new_key_type! {
    /// Key representing a vertex in the half-edge mesh.
    pub struct HeVertexKey;
    /// Key representing a directed half-edge.
    pub struct HalfEdgeKey;
    /// Key representing a face/polygon in the half-edge mesh.
    pub struct HeFaceKey;
}

/// A vertex stored in the half-edge mesh.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeVertex {
    /// 3D position of vertex in f64 precision.
    pub position: [f64; 3],
    /// Outgoing half-edge originating from this vertex.
    pub half_edge: Option<HalfEdgeKey>,
}

/// Directed half-edge structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HalfEdge {
    /// Opposite (twin) half-edge in adjacent face or boundary.
    pub twin: Option<HalfEdgeKey>,
    /// Next half-edge around current face (counter-clockwise).
    pub next: Option<HalfEdgeKey>,
    /// Previous half-edge around current face.
    pub prev: Option<HalfEdgeKey>,
    /// Origin vertex from which this half-edge starts.
    pub vertex: HeVertexKey,
    /// Incident face bounded by this half-edge.
    pub face: Option<HeFaceKey>,
}

/// A polygonal face bounded by half-edges.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeFace {
    /// One half-edge on the boundary of this face.
    pub half_edge: HalfEdgeKey,
}

/// Manifold Half-Edge Mesh data structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HalfEdgeMesh {
    /// Vertex arena storage.
    pub vertices: SlotMap<HeVertexKey, HeVertex>,
    /// Directed half-edge arena storage.
    pub half_edges: SlotMap<HalfEdgeKey, HalfEdge>,
    /// Face arena storage.
    pub faces: SlotMap<HeFaceKey, HeFace>,
}

impl HalfEdgeMesh {
    /// Create a new empty half-edge mesh.
    #[must_use]
    pub fn new() -> Self {
        Self {
            vertices: SlotMap::with_key(),
            half_edges: SlotMap::with_key(),
            faces: SlotMap::with_key(),
        }
    }

    /// Add a vertex with 3D coordinate.
    pub fn add_vertex(&mut self, position: [f64; 3]) -> HeVertexKey {
        self.vertices.insert(HeVertex {
            position,
            half_edge: None,
        })
    }

    /// Add a triangle face given 3 ordered vertex keys.
    pub fn add_triangle(
        &mut self,
        v0: HeVertexKey,
        v1: HeVertexKey,
        v2: HeVertexKey,
    ) -> Option<HeFaceKey> {
        let h0 = self.half_edges.insert(HalfEdge {
            twin: None,
            next: None,
            prev: None,
            vertex: v0,
            face: None,
        });
        let h1 = self.half_edges.insert(HalfEdge {
            twin: None,
            next: None,
            prev: None,
            vertex: v1,
            face: None,
        });
        let h2 = self.half_edges.insert(HalfEdge {
            twin: None,
            next: None,
            prev: None,
            vertex: v2,
            face: None,
        });

        // Set next and prev
        if let Some(edge) = self.half_edges.get_mut(h0) {
            edge.next = Some(h1);
            edge.prev = Some(h2);
        }
        if let Some(edge) = self.half_edges.get_mut(h1) {
            edge.next = Some(h2);
            edge.prev = Some(h0);
        }
        if let Some(edge) = self.half_edges.get_mut(h2) {
            edge.next = Some(h0);
            edge.prev = Some(h1);
        }

        // Set vertex outgoing edges if unset
        if let Some(v) = self.vertices.get_mut(v0) {
            if v.half_edge.is_none() {
                v.half_edge = Some(h0);
            }
        }
        if let Some(v) = self.vertices.get_mut(v1) {
            if v.half_edge.is_none() {
                v.half_edge = Some(h1);
            }
        }
        if let Some(v) = self.vertices.get_mut(v2) {
            if v.half_edge.is_none() {
                v.half_edge = Some(h2);
            }
        }

        let face_key = self.faces.insert(HeFace { half_edge: h0 });

        if let Some(edge) = self.half_edges.get_mut(h0) {
            edge.face = Some(face_key);
        }
        if let Some(edge) = self.half_edges.get_mut(h1) {
            edge.face = Some(face_key);
        }
        if let Some(edge) = self.half_edges.get_mut(h2) {
            edge.face = Some(face_key);
        }

        Some(face_key)
    }

    /// Perform Laplacian smoothing for sculpt operations.
    pub fn laplacian_smooth(&mut self, factor: f64) {
        let mut new_positions: Vec<(HeVertexKey, [f64; 3])> = Vec::with_capacity(self.vertices.len());

        for (v_key, v) in &self.vertices {
            let mut neighbor_sum = DVec3::ZERO;
            let mut count = 0.0;

            // Iterate 1-ring neighbors
            if let Some(start_he_key) = v.half_edge {
                let mut current_he = start_he_key;
                loop {
                    if let Some(he) = self.half_edges.get(current_he) {
                        if let Some(next_he_key) = he.next {
                            if let Some(next_he) = self.half_edges.get(next_he_key) {
                                if let Some(neighbor) = self.vertices.get(next_he.vertex) {
                                    neighbor_sum += DVec3::from_slice(&neighbor.position);
                                    count += 1.0;
                                }
                            }
                        }

                        if let Some(twin_key) = he.twin {
                            if let Some(twin_he) = self.half_edges.get(twin_key) {
                                if let Some(next_from_twin) = twin_he.next {
                                    current_he = next_from_twin;
                                    if current_he == start_he_key {
                                        break;
                                    }
                                    continue;
                                }
                            }
                        }
                    }
                    break;
                }
            }

            if count > 0.0 {
                let current_pos = DVec3::from_slice(&v.position);
                let avg = neighbor_sum / count;
                let smoothed = current_pos + (avg - current_pos) * factor;
                new_positions.push((v_key, [smoothed.x, smoothed.y, smoothed.z]));
            }
        }

        for (v_key, pos) in new_positions {
            if let Some(v) = self.vertices.get_mut(v_key) {
                v.position = pos;
            }
        }
    }
}
