//! Oxide-3D Procedural Geometry Nodes Dataflow Engine.

use petgraph::graph::NodeIndex;
use petgraph::visit::{EdgeRef, Topo};
use petgraph::Directed;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// Geometry Nodes evaluation errors.
#[derive(Debug, Error)]
pub enum NodeError {
    /// Type mismatch on connected sockets.
    #[error("Type mismatch: expected {expected}, got {found}")]
    TypeMismatch { expected: String, found: String },

    /// Cycle detected in node graph.
    #[error("Cycle detected in node execution DAG")]
    CycleDetected,

    /// Node not found.
    #[error("Node {0} not found in graph")]
    NodeNotFound(usize),

    /// Missing required input.
    #[error("Missing input socket {socket} on node {node_id}")]
    MissingInput { node_id: usize, socket: usize },

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
            [0.0, 0.0, 1.0],   // Front
            [0.0, 0.0, -1.0],  // Back
            [0.0, 1.0, 0.0],   // Top
            [0.0, -1.0, 0.0],  // Bottom
            [1.0, 0.0, 0.0],   // Right
            [-1.0, 0.0, 0.0],  // Left
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

/// Transform Geometry Node.
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
                ))
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
}
