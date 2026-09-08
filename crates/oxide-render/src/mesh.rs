//! Triangle mesh and vertex representation for GPU upload and rendering.

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

/// Standard vertex structure uploaded to GPU vertex buffers.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Vertex {
    /// 3D position [x, y, z].
    pub position: [f32; 3],
    /// Normal vector [nx, ny, nz].
    pub normal: [f32; 3],
    /// Color [r, g, b, a].
    pub color: [f32; 4],
}

impl Vertex {
    /// Create a new vertex with position, normal, and color.
    #[must_use]
    pub const fn new(position: [f32; 3], normal: [f32; 3], color: [f32; 4]) -> Self {
        Self {
            position,
            normal,
            color,
        }
    }

    /// Return the wgpu vertex buffer layout descriptor.
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// CPU-side triangle mesh.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TriMesh {
    /// Vertices list.
    pub vertices: Vec<Vertex>,
    /// Index buffer list.
    pub indices: Vec<u32>,
}

impl TriMesh {
    /// Create a new empty triangle mesh.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate a 3D unit cube centered at origin with width, height, depth = size.
    #[must_use]
    pub fn cube(size: f32, color: [f32; 4]) -> Self {
        let h = size * 0.5;
        let mut vertices = Vec::with_capacity(24);
        let mut indices = Vec::with_capacity(36);

        let faces = [
            // Normal, 4 corners (top-left, bottom-left, bottom-right, top-right)
            ([0.0, 0.0, 1.0], [[-h, h, h], [-h, -h, h], [h, -h, h], [h, h, h]]),   // +Z Front
            ([0.0, 0.0, -1.0], [[h, h, -h], [h, -h, -h], [-h, -h, -h], [-h, h, -h]]), // -Z Back
            ([1.0, 0.0, 0.0], [[h, h, h], [h, -h, h], [h, -h, -h], [h, h, -h]]),   // +X Right
            ([-1.0, 0.0, 0.0], [[-h, h, -h], [-h, -h, -h], [-h, -h, h], [-h, h, h]]), // -X Left
            ([0.0, 1.0, 0.0], [[-h, h, -h], [-h, h, h], [h, h, h], [h, h, -h]]),   // +Y Top
            ([0.0, -1.0, 0.0], [[-h, -h, h], [-h, -h, -h], [h, -h, -h], [h, -h, h]]), // -Y Bottom
        ];

        for (normal, corners) in faces {
            let base = vertices.len() as u32;
            for corner in corners {
                vertices.push(Vertex::new(corner, normal, color));
            }
            // Two triangles per face
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }

        Self { vertices, indices }
    }
}
