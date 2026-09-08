//! Multi-backend compute abstraction traits and device capability descriptors.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors arising from compute device discovery, buffer allocation, or kernel execution.
#[derive(Debug, Error)]
pub enum ComputeError {
    /// The requested backend is not available on this platform or hardware.
    #[error("Compute backend '{0:?}' is not available")]
    BackendUnavailable(BackendKind),

    /// Out of host or device memory.
    #[error("Out of compute memory: {0}")]
    OutOfMemory(String),

    /// Kernel compilation or pipeline creation failure.
    #[error("Kernel compilation failed: {0}")]
    KernelCompilation(String),

    /// Kernel dispatch or execution failure.
    #[error("Kernel execution failed: {0}")]
    ExecutionFailed(String),

    /// Data transfer failed between host and device.
    #[error("Buffer transfer error: {0}")]
    TransferError(String),

    /// Invalid buffer or kernel handle.
    #[error("Invalid resource handle: {0}")]
    InvalidHandle(String),
}

/// Enumeration of supported heterogeneous compute backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BackendKind {
    /// NVIDIA CUDA (cuBLAS, cuSPARSE, Tensor Cores).
    Cuda,
    /// AMD ROCm / HIP (rocBLAS, rocSPARSE).
    RocmHip,
    /// Vulkan Compute (SPIR-V pipelines).
    Vulkan,
    /// Apple Metal Compute (Metal Shading Language).
    Metal,
    /// Microsoft Direct3D 12 / DirectCompute (HLSL CS).
    Direct3D12,
    /// OpenCL (Heterogeneous CPUs, GPUs, DSPs, FPGAs).
    OpenCl,
    /// WebGPU / wgpu portable compute shaders (WGSL).
    Wgpu,
    /// CPU Parallel execution via Rayon + AVX-512 / AVX2 / NEON SIMD.
    CpuRayon,
}

impl BackendKind {
    /// Human-readable name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Cuda => "NVIDIA CUDA",
            Self::RocmHip => "AMD ROCm / HIP",
            Self::Vulkan => "Vulkan Compute",
            Self::Metal => "Apple Metal",
            Self::Direct3D12 => "Direct3D 12 DirectCompute",
            Self::OpenCl => "OpenCL",
            Self::Wgpu => "WebGPU (wgpu)",
            Self::CpuRayon => "CPU Multi-Threaded (Rayon)",
        }
    }
}

/// Hardware and capability profile of a compute device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCapabilities {
    /// Device name (e.g. "NVIDIA RTX 4090", "Apple M3 Max", "AMD Radeon RX 7900 XTX").
    pub name: String,
    /// Architecture backend kind.
    pub backend: BackendKind,
    /// Total device memory in bytes.
    pub total_memory_bytes: u64,
    /// Unified memory architecture (zero-copy CPU/GPU sharing like Apple Silicon).
    pub is_unified_memory: bool,
    /// Native 64-bit floating point compute support (crucial for CAD/FEA solvers).
    pub supports_fp64: bool,
    /// Native 16-bit floating point compute support (half-precision).
    pub supports_fp16: bool,
    /// Matrix / Tensor Core acceleration support.
    pub supports_tensor_cores: bool,
    /// Maximum number of workgroups per dispatch [x, y, z].
    pub max_workgroups: [u32; 3],
    /// Maximum workgroup invocations size.
    pub max_workgroup_size: u32,
}

/// Buffer allocation usage flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BufferUsage {
    /// Storage buffer for general compute read/write.
    Storage,
    /// Uniform parameter buffer.
    Uniform,
    /// Staging buffer for host-to-device transfers.
    StagingToDevice,
    /// Staging buffer for device-to-host readbacks.
    StagingToHost,
}

/// Handle to an allocated device memory buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub u64);

/// Handle to a compiled compute kernel / pipeline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KernelId(pub u64);

/// Abstract compute device trait implemented by hardware backends.
pub trait ComputeDevice: Send + Sync {
    /// Retrieve device metadata and hardware capabilities.
    fn capabilities(&self) -> &DeviceCapabilities;

    /// Allocate a device buffer.
    fn allocate_buffer(&self, size_bytes: usize, usage: BufferUsage) -> Result<BufferId, ComputeError>;

    /// Free an allocated device buffer.
    fn free_buffer(&self, buffer: BufferId) -> Result<(), ComputeError>;

    /// Copy data from host slice to device buffer.
    fn write_buffer(&self, buffer: BufferId, offset_bytes: usize, data: &[u8]) -> Result<(), ComputeError>;

    /// Read data from device buffer back to host.
    fn read_buffer(&self, buffer: BufferId, offset_bytes: usize, out: &mut [u8]) -> Result<(), ComputeError>;

    /// Compile a compute kernel from source or binary SPIR-V / PTX / MSL / HLSL / OpenCL C.
    fn compile_kernel(&self, name: &str, source: &str) -> Result<KernelId, ComputeError>;

    /// Dispatch a compute kernel over an 3D invocation grid [x, y, z].
    fn dispatch(
        &self,
        kernel: KernelId,
        grid_size: [u32; 3],
        workgroup_size: [u32; 3],
        bindings: &[BufferId],
    ) -> Result<(), ComputeError>;

    /// Synchronize and wait for all pending operations on this device.
    fn synchronize(&self) -> Result<(), ComputeError>;
}
