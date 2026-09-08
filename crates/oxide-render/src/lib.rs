//! Oxide-3D wgpu-based Viewport Renderer, Shaders, GPU Picking, and Gizmo Overlays.

/// Camera models, projection matrices, and orbital viewport navigation.
pub mod camera;
/// Triangle mesh representations, vertex buffers, and primitive generators.
pub mod mesh;
/// wgpu rendering pipeline, shaders, and GPU uniform buffers.
pub mod pipeline;

pub use camera::Camera;
pub use mesh::{TriMesh, Vertex};
pub use pipeline::{GpuMesh, RenderPipelineManager, Uniforms};
