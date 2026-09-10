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
            (
                [0.0, 0.0, 1.0],
                [[-h, h, h], [-h, -h, h], [h, -h, h], [h, h, h]],
            ), // +Z Front
            (
                [0.0, 0.0, -1.0],
                [[h, h, -h], [h, -h, -h], [-h, -h, -h], [-h, h, -h]],
            ), // -Z Back
            (
                [1.0, 0.0, 0.0],
                [[h, h, h], [h, -h, h], [h, -h, -h], [h, h, -h]],
            ), // +X Right
            (
                [-1.0, 0.0, 0.0],
                [[-h, h, -h], [-h, -h, -h], [-h, -h, h], [-h, h, h]],
            ), // -X Left
            (
                [0.0, 1.0, 0.0],
                [[-h, h, -h], [-h, h, h], [h, h, h], [h, h, -h]],
            ), // +Y Top
            (
                [0.0, -1.0, 0.0],
                [[-h, -h, h], [-h, -h, -h], [h, -h, -h], [h, -h, h]],
            ), // -Y Bottom
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

    /// Generate a 3D plane (quad) centered at origin facing +Z.
    #[must_use]
    pub fn plane(size: f32, color: [f32; 4]) -> Self {
        let h = size * 0.5;
        let vertices = vec![
            Vertex::new([-h, -h, 0.0], [0.0, 0.0, 1.0], color),
            Vertex::new([h, -h, 0.0], [0.0, 0.0, 1.0], color),
            Vertex::new([h, h, 0.0], [0.0, 0.0, 1.0], color),
            Vertex::new([-h, h, 0.0], [0.0, 0.0, 1.0], color),
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];
        Self { vertices, indices }
    }

    /// Generate a 3D cone centered at origin along the Z axis.
    #[must_use]
    pub fn cone(radius: f32, height: f32, segments: u32, color: [f32; 4]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let half_h = height * 0.5;
        let segments = segments.max(3);
        let step = 2.0 * std::f32::consts::PI / segments as f32;

        let apex_idx = 0;
        vertices.push(Vertex::new([0.0, 0.0, half_h], [0.0, 0.0, 1.0], color));

        for i in 0..=segments {
            let theta = (i as f32) * step;
            let x = radius * theta.cos();
            let y = radius * theta.sin();
            let norm = [theta.cos(), theta.sin(), radius / height];
            let len = (norm[0] * norm[0] + norm[1] * norm[1] + norm[2] * norm[2]).sqrt();
            let n = [norm[0] / len, norm[1] / len, norm[2] / len];
            vertices.push(Vertex::new([x, y, -half_h], n, color));
        }

        for i in 0..segments {
            let p1 = i + 1;
            let p2 = p1 + 1;
            indices.extend_from_slice(&[apex_idx, p1, p2]);
        }

        // Base cap
        let base_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new([0.0, 0.0, -half_h], [0.0, 0.0, -1.0], color));
        for i in 0..=segments {
            let theta = (i as f32) * step;
            let x = radius * theta.cos();
            let y = radius * theta.sin();
            vertices.push(Vertex::new([x, y, -half_h], [0.0, 0.0, -1.0], color));
        }
        for i in 0..segments {
            let p1 = base_center_idx + 1 + i;
            let p2 = p1 + 1;
            indices.extend_from_slice(&[base_center_idx, p2, p1]);
        }

        Self { vertices, indices }
    }

    /// Generate a 3D torus centered at origin.
    #[must_use]
    pub fn torus(
        major_radius: f32,
        minor_radius: f32,
        major_segments: u32,
        minor_segments: u32,
        color: [f32; 4],
    ) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let maj_segs = major_segments.max(3);
        let min_segs = minor_segments.max(3);

        for i in 0..=maj_segs {
            let u = (i as f32 / maj_segs as f32) * std::f32::consts::TAU;
            let cos_u = u.cos();
            let sin_u = u.sin();

            for j in 0..=min_segs {
                let v = (j as f32 / min_segs as f32) * std::f32::consts::TAU;
                let cos_v = v.cos();
                let sin_v = v.sin();

                let x = (major_radius + minor_radius * cos_v) * cos_u;
                let y = (major_radius + minor_radius * cos_v) * sin_u;
                let z = minor_radius * sin_v;

                let nx = cos_v * cos_u;
                let ny = cos_v * sin_u;
                let nz = sin_v;

                vertices.push(Vertex::new([x, y, z], [nx, ny, nz], color));
            }
        }

        for i in 0..maj_segs {
            for j in 0..min_segs {
                let a = i * (min_segs + 1) + j;
                let b = (i + 1) * (min_segs + 1) + j;
                let c = b + 1;
                let d = a + 1;

                indices.extend_from_slice(&[a, b, d]);
                indices.extend_from_slice(&[b, c, d]);
            }
        }

        Self { vertices, indices }
    }

    /// Generate a 2D planar grid (XY plane).
    #[must_use]
    pub fn grid(size_x: f32, size_y: f32, subdiv_x: u32, subdiv_y: u32, color: [f32; 4]) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let sx = subdiv_x.max(1);
        let sy = subdiv_y.max(1);

        let hx = size_x * 0.5;
        let hy = size_y * 0.5;

        for j in 0..=sy {
            let y = -hy + (j as f32 / sy as f32) * size_y;
            for i in 0..=sx {
                let x = -hx + (i as f32 / sx as f32) * size_x;
                vertices.push(Vertex::new([x, y, 0.0], [0.0, 0.0, 1.0], color));
            }
        }

        for j in 0..sy {
            for i in 0..sx {
                let row1 = j * (sx + 1);
                let row2 = (j + 1) * (sx + 1);

                let a = row1 + i;
                let b = row1 + i + 1;
                let c = row2 + i + 1;
                let d = row2 + i;

                indices.extend_from_slice(&[a, b, c]);
                indices.extend_from_slice(&[a, c, d]);
            }
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

        let plane = TriMesh::plane(2.0, [1.0, 1.0, 1.0, 1.0]);
        assert_eq!(plane.vertices.len(), 4);
        assert_eq!(plane.indices.len(), 6);

        let cone = TriMesh::cone(1.0, 2.0, 16, [1.0, 1.0, 0.0, 1.0]);
        assert!(!cone.vertices.is_empty());
        assert!(!cone.indices.is_empty());

        let torus = TriMesh::torus(1.0, 0.25, 16, 8, [0.0, 1.0, 1.0, 1.0]);
        assert!(!torus.vertices.is_empty());
        assert!(!torus.indices.is_empty());

        let grid = TriMesh::grid(10.0, 10.0, 4, 4, [0.5, 0.5, 0.5, 1.0]);
        assert_eq!(grid.vertices.len(), 25);
        assert_eq!(grid.indices.len(), 96);
    }
}
