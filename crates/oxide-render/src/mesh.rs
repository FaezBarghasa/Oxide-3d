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

    /// Generate a 3D UV sphere centered at origin.
    #[must_use]
    pub fn sphere(radius: f32, rings: u32, sectors: u32, color: [f32; 4]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let r_step = std::f32::consts::PI / rings.max(2) as f32;
        let s_step = 2.0 * std::f32::consts::PI / sectors.max(3) as f32;

        for r in 0..=rings {
            let phi = std::f32::consts::FRAC_PI_2 - (r as f32) * r_step;
            let xy = radius * phi.cos();
            let z = radius * phi.sin();

            for s in 0..=sectors {
                let theta = (s as f32) * s_step;
                let x = xy * theta.cos();
                let y = xy * theta.sin();
                let len = (x * x + y * y + z * z).sqrt();
                let normal = if len > 1e-6 {
                    [x / len, y / len, z / len]
                } else {
                    [0.0, 0.0, 1.0]
                };
                vertices.push(Vertex::new([x, y, z], normal, color));
            }
        }

        for r in 0..rings {
            let k1 = r * (sectors + 1);
            let k2 = k1 + sectors + 1;

            for s in 0..sectors {
                if r != 0 {
                    indices.extend_from_slice(&[k1 + s, k2 + s, k1 + s + 1]);
                }
                if r != (rings - 1) {
                    indices.extend_from_slice(&[k1 + s + 1, k2 + s, k2 + s + 1]);
                }
            }
        }

        Self { vertices, indices }
    }

    /// Generate a 3D cylinder centered at origin along the Z axis.
    #[must_use]
    pub fn cylinder(radius: f32, height: f32, segments: u32, color: [f32; 4]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let half_h = height * 0.5;
        let segments = segments.max(3);
        let step = 2.0 * std::f32::consts::PI / segments as f32;

        // Side vertices
        for i in 0..=segments {
            let theta = (i as f32) * step;
            let cos_t = theta.cos();
            let sin_t = theta.sin();
            let x = radius * cos_t;
            let y = radius * sin_t;
            let norm = [cos_t, sin_t, 0.0];

            vertices.push(Vertex::new([x, y, -half_h], norm, color));
            vertices.push(Vertex::new([x, y, half_h], norm, color));
        }

        for i in 0..segments {
            let base = i * 2;
            indices.extend_from_slice(&[base, base + 1, base + 2, base + 1, base + 3, base + 2]);
        }

        // Top and bottom caps
        let top_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new([0.0, 0.0, half_h], [0.0, 0.0, 1.0], color));
        for i in 0..=segments {
            let theta = (i as f32) * step;
            let x = radius * theta.cos();
            let y = radius * theta.sin();
            vertices.push(Vertex::new([x, y, half_h], [0.0, 0.0, 1.0], color));
        }
        for i in 0..segments {
            let p1 = top_center_idx + 1 + i;
            let p2 = p1 + 1;
            indices.extend_from_slice(&[top_center_idx, p1, p2]);
        }

        let bot_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new([0.0, 0.0, -half_h], [0.0, 0.0, -1.0], color));
        for i in 0..=segments {
            let theta = (i as f32) * step;
            let x = radius * theta.cos();
            let y = radius * theta.sin();
            vertices.push(Vertex::new([x, y, -half_h], [0.0, 0.0, -1.0], color));
        }
        for i in 0..segments {
            let p1 = bot_center_idx + 1 + i;
            let p2 = p1 + 1;
            indices.extend_from_slice(&[bot_center_idx, p2, p1]);
        }

        Self { vertices, indices }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mesh_primitives() {
        let cube = TriMesh::cube(1.0, [1.0, 0.0, 0.0, 1.0]);
        assert_eq!(cube.vertices.len(), 24);
        assert_eq!(cube.indices.len(), 36);

        let sphere = TriMesh::sphere(1.0, 8, 16, [0.0, 1.0, 0.0, 1.0]);
        assert!(!sphere.vertices.is_empty());
        assert!(!sphere.indices.is_empty());

        let cyl = TriMesh::cylinder(0.5, 2.0, 16, [0.0, 0.0, 1.0, 1.0]);
        assert!(!cyl.vertices.is_empty());
        assert!(!cyl.indices.is_empty());
    }
}

