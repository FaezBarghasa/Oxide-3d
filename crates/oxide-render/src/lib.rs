//! Oxide-3D wgpu-based Viewport Renderer, Shaders, GPU Picking, and Gizmo Overlays.

pub mod camera;
pub mod mesh;
pub mod pipeline;

pub use camera::Camera;
pub use mesh::{TriMesh, Vertex};
pub use pipeline::{GpuMesh, RenderPipelineManager, Uniforms};
