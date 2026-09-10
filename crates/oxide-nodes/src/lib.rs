//! Oxide-3D Procedural Geometry Nodes Dataflow Engine.

use petgraph::Directed;
use petgraph::graph::NodeIndex;
use petgraph::visit::{EdgeRef, Topo};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Geometry Nodes evaluation errors.
#[derive(Debug, Error)]
pub enum NodeError {
    /// Type mismatch on connected sockets.
    #[error("Type mismatch: expected {expected}, got {found}")]
    TypeMismatch {
        /// Expected socket type name.
        expected: String,
        /// Actual socket type name received.
        found: String,
    },

    /// Cycle detected in node graph.
    #[error("Cycle detected in node execution DAG")]
    CycleDetected,

    /// Node not found.
    #[error("Node {0} not found in graph")]
    NodeNotFound(usize),

    /// Missing required input.
    #[error("Missing input socket {socket} on node {node_id}")]
    MissingInput {
        /// ID of target node.
        node_id: usize,
        /// Socket index on node.
        socket: usize,
    },

    /// General node execution error.
    #[error("Node evaluation failed: {0}")]
    EvaluationFailed(String),
}

/// Dynamic value flowing through node graph sockets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeSocketValue {
    /// Floating-point scalar.
    Float(f64),
    /// Integer scalar.
    Int(i64),
    /// Boolean flag.
    Bool(bool),
    /// 3D Vector.
    Vector([f64; 3]),
    /// Text string.
    String(String),
    /// Triangle Mesh data.
    Mesh(NodeMeshData),
}

/// Triangle Mesh payload passed through geometry node pipelines.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeMeshData {
    /// 3D Vertex positions.
    pub positions: Vec<[f32; 3]>,
    /// 3D Vertex normals.
    pub normals: Vec<[f32; 3]>,
    /// Triangle index list.
    pub indices: Vec<u32>,
}

impl NodeMeshData {
    /// Create a unit box/cube mesh with width, height, depth.
    pub fn cube(dx: f32, dy: f32, dz: f32) -> Self {
        let hx = dx * 0.5;
        let hy = dy * 0.5;
        let hz = dz * 0.5;

        #[rustfmt::skip]
        let positions = vec![
            // Front face (+Z)
            [-hx, -hy,  hz], [ hx, -hy,  hz], [ hx,  hy,  hz], [-hx,  hy,  hz],
            // Back face (-Z)
            [ hx, -hy, -hz], [-hx, -hy, -hz], [-hx,  hy, -hz], [ hx,  hy, -hz],
            // Top face (+Y)
            [-hx,  hy,  hz], [ hx,  hy,  hz], [ hx,  hy, -hz], [-hx,  hy, -hz],
            // Bottom face (-Y)
            [-hx, -hy, -hz], [ hx, -hy, -hz], [ hx, -hy,  hz], [-hx, -hy,  hz],
            // Right face (+X)
            [ hx, -hy,  hz], [ hx, -hy, -hz], [ hx,  hy, -hz], [ hx,  hy,  hz],
            // Left face (-X)
            [-hx, -hy, -hz], [-hx, -hy,  hz], [-hx,  hy,  hz], [-hx,  hy, -hz],
        ];

        let mut normals = Vec::with_capacity(24);
        for n in &[
            [0.0, 0.0, 1.0],  // Front
            [0.0, 0.0, -1.0], // Back
            [0.0, 1.0, 0.0],  // Top
            [0.0, -1.0, 0.0], // Bottom
            [1.0, 0.0, 0.0],  // Right
            [-1.0, 0.0, 0.0], // Left
        ] {
            normals.extend_from_slice(&[*n; 4]);
        }

        #[rustfmt::skip]
        let indices = vec![
            0, 1, 2,  0, 2, 3,       // Front
            4, 5, 6,  4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,     // Top
            12, 13, 14, 12, 14, 15,  // Bottom
            16, 17, 18, 16, 18, 19,  // Right
            20, 21, 22, 20, 22, 23,  // Left
        ];

        Self {
            positions,
            normals,
            indices,
        }
    }

    /// Create a cylinder mesh given radius, height, and circumferential segments.
    pub fn cylinder(radius: f32, height: f32, segments: usize) -> Self {
        let segs = segments.max(3);
        let hh = height * 0.5;
        let mut positions = Vec::with_capacity((segs + 1) * 2 + 2);
        let mut normals = Vec::with_capacity((segs + 1) * 2 + 2);
        let mut indices = Vec::new();

        // Side vertices
        for i in 0..=segs {
            let theta = (i as f32 / segs as f32) * std::f32::consts::TAU;
            let cos_t = theta.cos();
            let sin_t = theta.sin();
            let n = [cos_t, 0.0, sin_t];

            positions.push([cos_t * radius, -hh, sin_t * radius]);
            normals.push(n);

            positions.push([cos_t * radius, hh, sin_t * radius]);
            normals.push(n);
        }

        for i in 0..segs {
            let base = (i * 2) as u32;
            indices.extend_from_slice(&[base, base + 1, base + 3, base, base + 3, base + 2]);
        }

        Self {
            positions,
            normals,
            indices,
        }
    }

    /// Create a UV sphere mesh.
    pub fn sphere(radius: f32, rings: usize, sectors: usize) -> Self {
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        let rings = rings.max(2);
        let sectors = sectors.max(3);

        let r_step = std::f32::consts::PI / rings as f32;
        let s_step = std::f32::consts::TAU / sectors as f32;

        for r in 0..=rings {
            let phi = std::f32::consts::FRAC_PI_2 - (r as f32) * r_step;
            let xy = radius * phi.cos();
            let z = radius * phi.sin();

            for s in 0..=sectors {
                let theta = (s as f32) * s_step;
                let x = xy * theta.cos();
                let y = xy * theta.sin();
                let len = (x * x + y * y + z * z).sqrt();
                let norm = if len > 1e-6 {
                    [x / len, y / len, z / len]
                } else {
                    [0.0, 0.0, 1.0]
                };

                positions.push([x, y, z]);
                normals.push(norm);
            }
        }

        for r in 0..rings {
            let k1 = (r * (sectors + 1)) as u32;
            let k2 = k1 + sectors as u32 + 1;

            for s in 0..sectors {
                let su = s as u32;
                if r != 0 {
                    indices.extend_from_slice(&[k1 + su, k2 + su, k1 + su + 1]);
                }
                if r != (rings - 1) {
                    indices.extend_from_slice(&[k1 + su + 1, k2 + su, k2 + su + 1]);
                }
            }
        }

        Self {
            positions,
            normals,
            indices,
        }
    }

    /// Create a cone mesh given radius, height, and circumferential segments.
    pub fn cone(radius: f32, height: f32, segments: usize) -> Self {
        let segs = segments.max(3);
        let hh = height * 0.5;
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        let apex = [0.0, hh, 0.0];
        positions.push(apex);
        normals.push([0.0, 1.0, 0.0]);

        for i in 0..=segs {
            let theta = (i as f32 / segs as f32) * std::f32::consts::TAU;
            let x = theta.cos() * radius;
            let z = theta.sin() * radius;
            let norm = [theta.cos(), radius / height, theta.sin()];
            let len = (norm[0] * norm[0] + norm[1] * norm[1] + norm[2] * norm[2]).sqrt();
            let n = [norm[0] / len, norm[1] / len, norm[2] / len];

            positions.push([x, -hh, z]);
            normals.push(n);
        }

        for i in 0..segs {
            let p1 = (i + 1) as u32;
            let p2 = p1 + 1;
            indices.extend_from_slice(&[0, p1, p2]);
        }

        Self {
            positions,
            normals,
            indices,
        }
    }

    /// Create a torus mesh.
    pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: usize,
        minor_segments: usize,
    ) -> Self {
        let maj_segs = major_segments.max(3);
        let min_segs = minor_segments.max(3);
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for i in 0..=maj_segs {
            let u = (i as f32 / maj_segs as f32) * std::f32::consts::TAU;
            let cos_u = u.cos();
            let sin_u = u.sin();

            for j in 0..=min_segs {
                let v = (j as f32 / min_segs as f32) * std::f32::consts::TAU;
                let cos_v = v.cos();
                let sin_v = v.sin();

                let x = (major_radius + minor_radius * cos_v) * cos_u;
                let y = minor_radius * sin_v;
                let z = (major_radius + minor_radius * cos_v) * sin_u;

                let nx = cos_v * cos_u;
                let ny = sin_v;
                let nz = cos_v * sin_u;

                positions.push([x, y, z]);
                normals.push([nx, ny, nz]);
            }
        }

        for i in 0..maj_segs {
            for j in 0..min_segs {
                let a = (i * (min_segs + 1) + j) as u32;
                let b = ((i + 1) * (min_segs + 1) + j) as u32;
                let c = b + 1;
                let d = a + 1;

                indices.extend_from_slice(&[a, b, d]);
                indices.extend_from_slice(&[b, c, d]);
            }
        }

        Self {
            positions,
            normals,
            indices,
        }
    }

    /// Create a planar grid mesh.
    pub fn grid(size_x: f32, size_z: f32, subdiv_x: usize, subdiv_z: usize) -> Self {
        let sx = subdiv_x.max(1);
        let sz = subdiv_z.max(1);
        let hx = size_x * 0.5;
        let hz = size_z * 0.5;

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        for j in 0..=sz {
            let z = -hz + (j as f32 / sz as f32) * size_z;
            for i in 0..=sx {
                let x = -hx + (i as f32 / sx as f32) * size_x;
                positions.push([x, 0.0, z]);
                normals.push([0.0, 1.0, 0.0]);
            }
        }

        for j in 0..sz {
            for i in 0..sx {
                let row1 = (j * (sx + 1)) as u32;
                let row2 = ((j + 1) * (sx + 1)) as u32;

                let a = row1 + i as u32;
                let b = row1 + i as u32 + 1;
                let c = row2 + i as u32 + 1;
                let d = row2 + i as u32;

                indices.extend_from_slice(&[a, b, c]);
                indices.extend_from_slice(&[a, c, d]);
            }
        }

        Self {
            positions,
            normals,
            indices,
        }
    }

    /// Transform mesh vertices by translation offset and uniform scale.
    pub fn transform(&mut self, translation: [f32; 3], scale: [f32; 3]) {
        for pos in &mut self.positions {
            pos[0] = pos[0] * scale[0] + translation[0];
            pos[1] = pos[1] * scale[1] + translation[1];
            pos[2] = pos[2] * scale[2] + translation[2];
        }
    }

    /// Join another mesh into this mesh.
    pub fn join(&mut self, other: &Self) {
        let offset = self.positions.len() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        for idx in &other.indices {
            self.indices.push(idx + offset);
        }
    }
}

/// Socket definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketDef {
    /// Socket name.
    pub name: String,
    /// Default socket type.
    pub socket_type: String,
}

/// Trait for procedural geometry nodes.
pub trait OxideNode: Send + Sync {
    /// Human-readable node type name.
    fn name(&self) -> &'static str;
    /// Input socket definitions.
    fn inputs(&self) -> Vec<SocketDef>;
    /// Output socket definitions.
    fn outputs(&self) -> Vec<SocketDef>;
    /// Evaluate the node's mathematical/geometric logic.
    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError>;
}

// ==========================================
// Built-in Geometry Node Implementations
// ==========================================

/// Cube Primitive Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct CubeNode;
impl OxideNode for CubeNode {
    fn name(&self) -> &'static str {
        "Cube Primitive"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Size X".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Size Y".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Size Z".into(),
                socket_type: "Float".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let dx = match inputs.first() {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let dy = match inputs.get(1) {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let dz = match inputs.get(2) {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };

        let mesh = NodeMeshData::cube(dx, dy, dz);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Cylinder Primitive Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct CylinderNode;
impl OxideNode for CylinderNode {
    fn name(&self) -> &'static str {
        "Cylinder Primitive"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Radius".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Height".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Segments".into(),
                socket_type: "Int".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let r = match inputs.first() {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let h = match inputs.get(1) {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 2.0,
        };
        let segs = match inputs.get(2) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 16,
        };

        let mesh = NodeMeshData::cylinder(r, h, segs);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Sphere Primitive Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct SphereNode;
impl OxideNode for SphereNode {
    fn name(&self) -> &'static str {
        "Sphere Primitive"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Radius".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Rings".into(),
                socket_type: "Int".into(),
            },
            SocketDef {
                name: "Sectors".into(),
                socket_type: "Int".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let r = match inputs.first() {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let rings = match inputs.get(1) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 16,
        };
        let sectors = match inputs.get(2) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 32,
        };

        let mesh = NodeMeshData::sphere(r, rings, sectors);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Cone Primitive Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct ConeNode;
impl OxideNode for ConeNode {
    fn name(&self) -> &'static str {
        "Cone Primitive"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Radius".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Height".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Segments".into(),
                socket_type: "Int".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let r = match inputs.first() {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let h = match inputs.get(1) {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 2.0,
        };
        let segs = match inputs.get(2) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 16,
        };

        let mesh = NodeMeshData::cone(r, h, segs);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Torus Primitive Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct TorusNode;
impl OxideNode for TorusNode {
    fn name(&self) -> &'static str {
        "Torus Primitive"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Major Radius".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Minor Radius".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Major Segments".into(),
                socket_type: "Int".into(),
            },
            SocketDef {
                name: "Minor Segments".into(),
                socket_type: "Int".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let r_maj = match inputs.first() {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 1.0,
        };
        let r_min = match inputs.get(1) {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 0.25,
        };
        let segs_maj = match inputs.get(2) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 24,
        };
        let segs_min = match inputs.get(3) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 12,
        };

        let mesh = NodeMeshData::torus(r_maj, r_min, segs_maj, segs_min);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Grid Primitive Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct GridNode;
impl OxideNode for GridNode {
    fn name(&self) -> &'static str {
        "Grid Primitive"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Size X".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Size Z".into(),
                socket_type: "Float".into(),
            },
            SocketDef {
                name: "Subdiv X".into(),
                socket_type: "Int".into(),
            },
            SocketDef {
                name: "Subdiv Z".into(),
                socket_type: "Int".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let sx = match inputs.first() {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 2.0,
        };
        let sz = match inputs.get(1) {
            Some(NodeSocketValue::Float(f)) => *f as f32,
            _ => 2.0,
        };
        let sub_x = match inputs.get(2) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 4,
        };
        let sub_z = match inputs.get(3) {
            Some(NodeSocketValue::Int(i)) => *i as usize,
            _ => 4,
        };

        let mesh = NodeMeshData::grid(sx, sz, sub_x, sub_z);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Transform Geometry Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct TransformGeometryNode;
impl OxideNode for TransformGeometryNode {
    fn name(&self) -> &'static str {
        "Transform Geometry"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Geometry".into(),
                socket_type: "Mesh".into(),
            },
            SocketDef {
                name: "Translation".into(),
                socket_type: "Vector".into(),
            },
            SocketDef {
                name: "Scale".into(),
                socket_type: "Vector".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let mut mesh = match inputs.first() {
            Some(NodeSocketValue::Mesh(m)) => m.clone(),
            _ => {
                return Err(NodeError::EvaluationFailed(
                    "Transform requires Mesh input".into(),
                ));
            }
        };

        let trans = match inputs.get(1) {
            Some(NodeSocketValue::Vector(v)) => [v[0] as f32, v[1] as f32, v[2] as f32],
            _ => [0.0, 0.0, 0.0],
        };

        let scale = match inputs.get(2) {
            Some(NodeSocketValue::Vector(v)) => [v[0] as f32, v[1] as f32, v[2] as f32],
            _ => [1.0, 1.0, 1.0],
        };

        mesh.transform(trans, scale);
        Ok(vec![NodeSocketValue::Mesh(mesh)])
    }
}

/// Join Geometry Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct JoinGeometryNode;
impl OxideNode for JoinGeometryNode {
    fn name(&self) -> &'static str {
        "Join Geometry"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Geometry A".into(),
                socket_type: "Mesh".into(),
            },
            SocketDef {
                name: "Geometry B".into(),
                socket_type: "Mesh".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let mut mesh_a = match inputs.first() {
            Some(NodeSocketValue::Mesh(m)) => m.clone(),
            _ => NodeMeshData::cube(0.0, 0.0, 0.0),
        };

        if let Some(NodeSocketValue::Mesh(mesh_b)) = inputs.get(1) {
            mesh_a.join(mesh_b);
        }

        Ok(vec![NodeSocketValue::Mesh(mesh_a)])
    }
}

/// Subdivide Mesh Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct SubdivideMeshNode;
impl OxideNode for SubdivideMeshNode {
    fn name(&self) -> &'static str {
        "Subdivide Mesh"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Geometry".into(),
                socket_type: "Mesh".into(),
            },
            SocketDef {
                name: "Cuts".into(),
                socket_type: "Int".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Geometry".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let mesh = match inputs.first() {
            Some(NodeSocketValue::Mesh(m)) => m.clone(),
            _ => {
                return Err(NodeError::EvaluationFailed(
                    "Subdivide requires Mesh input".into(),
                ));
            }
        };

        let cuts = match inputs.get(1) {
            Some(NodeSocketValue::Int(c)) => (*c).max(0) as usize,
            _ => 1,
        };

        if cuts == 0 {
            return Ok(vec![NodeSocketValue::Mesh(mesh)]);
        }

        // Subdivide each triangle into 4 smaller triangles by splitting edge midpoints
        let mut new_positions = mesh.positions.clone();
        let mut new_normals = mesh.normals.clone();
        let mut new_indices = Vec::with_capacity(mesh.indices.len() * 4);

        for chunk in mesh.indices.chunks_exact(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let p0 = mesh.positions[i0];
            let p1 = mesh.positions[i1];
            let p2 = mesh.positions[i2];

            let n0 = mesh.normals[i0];
            let n1 = mesh.normals[i1];
            let n2 = mesh.normals[i2];

            // Edge midpoints
            let m01 = [
                (p0[0] + p1[0]) * 0.5,
                (p0[1] + p1[1]) * 0.5,
                (p0[2] + p1[2]) * 0.5,
            ];
            let m12 = [
                (p1[0] + p2[0]) * 0.5,
                (p1[1] + p2[1]) * 0.5,
                (p1[2] + p2[2]) * 0.5,
            ];
            let m20 = [
                (p2[0] + p0[0]) * 0.5,
                (p2[1] + p0[1]) * 0.5,
                (p2[2] + p0[2]) * 0.5,
            ];

            let nm01 = [
                (n0[0] + n1[0]) * 0.5,
                (n0[1] + n1[1]) * 0.5,
                (n0[2] + n1[2]) * 0.5,
            ];
            let nm12 = [
                (n1[0] + n2[0]) * 0.5,
                (n1[1] + n2[1]) * 0.5,
                (n1[2] + n2[2]) * 0.5,
            ];
            let nm20 = [
                (n2[0] + n0[0]) * 0.5,
                (n2[1] + n0[1]) * 0.5,
                (n2[2] + n0[2]) * 0.5,
            ];

            let idx_m01 = new_positions.len() as u32;
            new_positions.push(m01);
            new_normals.push(nm01);

            let idx_m12 = new_positions.len() as u32;
            new_positions.push(m12);
            new_normals.push(nm12);

            let idx_m20 = new_positions.len() as u32;
            new_positions.push(m20);
            new_normals.push(nm20);

            let idx0 = i0 as u32;
            let idx1 = i1 as u32;
            let idx2 = i2 as u32;

            // 4 sub-triangles
            new_indices.extend_from_slice(&[idx0, idx_m01, idx_m20]);
            new_indices.extend_from_slice(&[idx1, idx_m12, idx_m01]);
            new_indices.extend_from_slice(&[idx2, idx_m20, idx_m12]);
            new_indices.extend_from_slice(&[idx_m01, idx_m12, idx_m20]);
        }

        let subdivided = NodeMeshData {
            positions: new_positions,
            normals: new_normals,
            indices: new_indices,
        };

        Ok(vec![NodeSocketValue::Mesh(subdivided)])
    }
}

/// Instance on Points Node.
#[derive(Debug, Clone, Copy, Default)]
pub struct InstanceOnPointsNode;
impl OxideNode for InstanceOnPointsNode {
    fn name(&self) -> &'static str {
        "Instance on Points"
    }

    fn inputs(&self) -> Vec<SocketDef> {
        vec![
            SocketDef {
                name: "Points Mesh".into(),
                socket_type: "Mesh".into(),
            },
            SocketDef {
                name: "Instance".into(),
                socket_type: "Mesh".into(),
            },
            SocketDef {
                name: "Scale".into(),
                socket_type: "Float".into(),
            },
        ]
    }

    fn outputs(&self) -> Vec<SocketDef> {
        vec![SocketDef {
            name: "Instances".into(),
            socket_type: "Mesh".into(),
        }]
    }

    fn evaluate(&self, inputs: &[NodeSocketValue]) -> Result<Vec<NodeSocketValue>, NodeError> {
        let points_mesh = match inputs.first() {
            Some(NodeSocketValue::Mesh(m)) => m,
            _ => {
                return Err(NodeError::EvaluationFailed(
                    "Instance on Points requires points mesh".into(),
                ));
            }
        };

        let instance_mesh = match inputs.get(1) {
            Some(NodeSocketValue::Mesh(m)) => m,
            _ => {
                return Err(NodeError::EvaluationFailed(
                    "Instance on Points requires instance geometry".into(),
                ));
            }
        };

        let scale = match inputs.get(2) {
            Some(NodeSocketValue::Float(s)) => *s as f32,
            _ => 1.0,
        };

        let mut output = NodeMeshData {
            positions: Vec::new(),
            normals: Vec::new(),
            indices: Vec::new(),
        };

        for pt in &points_mesh.positions {
            let mut copy = instance_mesh.clone();
            copy.transform(*pt, [scale, scale, scale]);
            output.join(&copy);
        }

        Ok(vec![NodeSocketValue::Mesh(output)])
    }
}

/// Node Graph Connection Edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeConnection {
    /// Source output socket index.
    pub from_socket: usize,
    /// Destination input socket index.
    pub to_socket: usize,
}

/// Procedural Node Graph Evaluation DAG.
pub struct NodeGraph {
    graph: petgraph::graph::Graph<Arc<dyn OxideNode>, NodeConnection, Directed>,
    node_indices: Vec<NodeIndex>,
}

impl std::fmt::Debug for NodeGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NodeGraph")
            .field("node_count", &self.graph.node_count())
            .field("edge_count", &self.graph.edge_count())
            .finish()
    }
}

impl Default for NodeGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeGraph {
    /// Create a new empty node graph.
    pub fn new() -> Self {
        Self {
            graph: petgraph::graph::Graph::new(),
            node_indices: Vec::new(),
        }
    }

    /// Add a node to the graph and return its ID.
    pub fn add_node(&mut self, node: Arc<dyn OxideNode>) -> usize {
        let idx = self.graph.add_node(node);
        let id = self.node_indices.len();
        self.node_indices.push(idx);
        id
    }

    /// Connect output socket of source node to input socket of destination node.
    pub fn connect(
        &mut self,
        from_node: usize,
        from_socket: usize,
        to_node: usize,
        to_socket: usize,
    ) -> Result<(), NodeError> {
        let src = self
            .node_indices
            .get(from_node)
            .copied()
            .ok_or(NodeError::NodeNotFound(from_node))?;
        let dst = self
            .node_indices
            .get(to_node)
            .copied()
            .ok_or(NodeError::NodeNotFound(to_node))?;

        self.graph.add_edge(
            src,
            dst,
            NodeConnection {
                from_socket,
                to_socket,
            },
        );
        Ok(())
    }

    /// Topologically evaluate the entire graph and return the output sockets for each node.
    pub fn evaluate(&self) -> Result<HashMap<usize, Vec<NodeSocketValue>>, NodeError> {
        let mut topo = Topo::new(&self.graph);
        let mut node_outputs: HashMap<NodeIndex, Vec<NodeSocketValue>> = HashMap::new();
        let mut evaluated_count = 0;

        while let Some(node_idx) = topo.next(&self.graph) {
            evaluated_count += 1;
            let node = &self.graph[node_idx];
            let num_inputs = node.inputs().len();
            let mut inputs = vec![NodeSocketValue::Float(0.0); num_inputs];

            // Gather inputs from incoming edges
            for edge in self
                .graph
                .edges_directed(node_idx, petgraph::Direction::Incoming)
            {
                let source_idx = edge.source();
                let connection = edge.weight();
                if let Some(src_outputs) = node_outputs.get(&source_idx) {
                    if let Some(val) = src_outputs.get(connection.from_socket) {
                        if connection.to_socket < inputs.len() {
                            inputs[connection.to_socket] = val.clone();
                        }
                    }
                }
            }

            let outputs = node.evaluate(&inputs)?;
            node_outputs.insert(node_idx, outputs);
        }

        if evaluated_count < self.graph.node_count() {
            return Err(NodeError::CycleDetected);
        }

        // Map petgraph NodeIndex back to usize IDs
        let mut result = HashMap::new();
        for (id, &idx) in self.node_indices.iter().enumerate() {
            if let Some(outputs) = node_outputs.remove(&idx) {
                result.insert(id, outputs);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_graph_evaluation() {
        let mut graph = NodeGraph::new();

        let cube1 = graph.add_node(Arc::new(CubeNode));
        let xform = graph.add_node(Arc::new(TransformGeometryNode));
        let cube2 = graph.add_node(Arc::new(CubeNode));
        let join = graph.add_node(Arc::new(JoinGeometryNode));

        // Connect Cube1 -> Transform
        graph.connect(cube1, 0, xform, 0).unwrap();

        // Connect Transform -> Join A
        graph.connect(xform, 0, join, 0).unwrap();

        // Connect Cube2 -> Join B
        graph.connect(cube2, 0, join, 1).unwrap();

        let results = graph.evaluate().expect("Graph evaluation failed");
        let join_output = results.get(&join).expect("Join outputs found");

        if let Some(NodeSocketValue::Mesh(mesh)) = join_output.first() {
            // Cube has 24 vertices, joining two cubes = 48 vertices
            assert_eq!(mesh.positions.len(), 48);
            assert_eq!(mesh.indices.len(), 72);
        } else {
            panic!("Expected mesh output from Join Geometry node");
        }
    }

    #[test]
    fn test_subdivide_and_instance_nodes() {
        let mut graph = NodeGraph::new();

        let cube = graph.add_node(Arc::new(CubeNode));
        let subdiv = graph.add_node(Arc::new(SubdivideMeshNode));
        let instance = graph.add_node(Arc::new(InstanceOnPointsNode));

        graph.connect(cube, 0, subdiv, 0).unwrap();
        graph.connect(subdiv, 0, instance, 0).unwrap();
        graph.connect(cube, 0, instance, 1).unwrap();

        let results = graph.evaluate().expect("Graph evaluation failed");
        let inst_output = results.get(&instance).expect("Instance output found");

        if let Some(NodeSocketValue::Mesh(mesh)) = inst_output.first() {
            // Subdivided cube points x instance cube
            assert!(!mesh.positions.is_empty());
            assert!(!mesh.indices.is_empty());
        } else {
            panic!("Expected mesh output from Instance on Points node");
        }
    }

    #[test]
    fn test_primitive_geometry_nodes() {
        let mut graph = NodeGraph::new();

        let sphere = graph.add_node(Arc::new(SphereNode));
        let cone = graph.add_node(Arc::new(ConeNode));
        let torus = graph.add_node(Arc::new(TorusNode));
        let grid = graph.add_node(Arc::new(GridNode));

        let results = graph.evaluate().expect("Graph evaluation failed");

        let sphere_mesh = match results.get(&sphere).and_then(|v| v.first()) {
            Some(NodeSocketValue::Mesh(m)) => m,
            _ => panic!("Expected sphere mesh"),
        };
        assert!(!sphere_mesh.positions.is_empty());

        let cone_mesh = match results.get(&cone).and_then(|v| v.first()) {
            Some(NodeSocketValue::Mesh(m)) => m,
            _ => panic!("Expected cone mesh"),
        };
        assert!(!cone_mesh.positions.is_empty());

        let torus_mesh = match results.get(&torus).and_then(|v| v.first()) {
            Some(NodeSocketValue::Mesh(m)) => m,
            _ => panic!("Expected torus mesh"),
        };
        assert!(!torus_mesh.positions.is_empty());

        let grid_mesh = match results.get(&grid).and_then(|v| v.first()) {
            Some(NodeSocketValue::Mesh(m)) => m,
            _ => panic!("Expected grid mesh"),
        };
        assert!(!grid_mesh.positions.is_empty());
    }
}
