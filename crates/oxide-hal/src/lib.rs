//! Hardware Abstraction Layer (HAL) for heterogeneous computing in Oxide-3D.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Enumeration of supported hardware acceleration backends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BackendKind {
    /// Host CPU using Rayon, SIMD vectorization, and faer.
    Cpu,
    /// WebGPU / wgpu portable graphics and compute shaders.
    Wgpu,
    /// NVIDIA CUDA (cuBLAS, cuSPARSE, cuSOLVER, PTX).
    Cuda,
    /// AMD ROCm / HIP (rocBLAS, rocSPARSE, rocSOLVER, HSACO).
    Rocm,
    /// Vulkan Compute via explicit SPIR-V pipelines.
    Vulkan,
    /// Apple Metal for Apple Silicon (Metal Shading Language).
    Metal,
    /// Microsoft Direct3D 12 DirectCompute.
    Dx12,
    /// OpenCL for heterogeneous devices and FPGAs.
    OpenCl,
}

impl BackendKind {
    /// Human-readable name.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Cpu => "CPU (Rayon + SIMD)",
            Self::Wgpu => "WebGPU (wgpu)",
            Self::Cuda => "NVIDIA CUDA",
            Self::Rocm => "AMD ROCm / HIP",
            Self::Vulkan => "Vulkan Compute",
            Self::Metal => "Apple Metal",
            Self::Dx12 => "Direct3D 12",
            Self::OpenCl => "OpenCL",
        }
    }
}

/// Compute accelerator error definitions.
#[derive(Debug, Error)]
pub enum ComputeError {
    /// Backend is not available on this platform or system.
    #[error("Backend not available: {0:?}")]
    BackendUnavailable(BackendKind),

    /// Device memory allocation failure.
    #[error("Out of device memory: {0}")]
    OutOfMemory(String),

    /// Driver initialization or call failure.
    #[error("Driver error: {0}")]
    DriverError(String),

    /// Data transfer failed between host and device.
    #[error("Transfer error: {0}")]
    TransferError(String),

    /// Kernel launch or compilation failure.
    #[error("Kernel error: {0}")]
    KernelError(String),

    /// Operation cancelled by user or scheduler.
    #[error("Compute task cancelled")]
    Cancelled,

    /// Tokio join error.
    #[error("Task join error: {0}")]
    JoinError(String),
}

/// Comprehensive hardware capability profile of an accelerator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendCapabilities {
    /// Backend kind.
    pub kind: BackendKind,
    /// Descriptive hardware device name (e.g. "NVIDIA GeForce RTX 4090").
    pub device_name: String,
    /// Supports FP16 half-precision math.
    pub fp16: bool,
    /// Supports standard FP32 single-precision math.
    pub fp32: bool,
    /// Supports IEEE-754 FP64 double-precision math (crucial for exact FEA/CFD).
    pub fp64: bool,
    /// Supports 64-bit atomic integer memory operations.
    pub int64_atomics: bool,
    /// Unified memory architecture (zero-copy shared CPU/GPU address space).
    pub unified_memory: bool,
    /// Maximum workgroup/thread block size.
    pub max_workgroup_size: u32,
    /// Subgroup/warp/wavefront execution width (e.g. 32 for CUDA, 64 for AMD).
    pub subgroup_size: Option<u32>,
    /// Total VRAM on device in bytes.
    pub device_memory_bytes: u64,
    /// Accessible host RAM in bytes.
    pub host_memory_bytes: u64,
    /// Guarantees bit-for-bit deterministic parallel reduction.
    pub deterministic_reduction: bool,
    /// Supports Peer-to-Peer direct memory transfers between multiple GPUs.
    pub p2p_multi_gpu: bool,
}

/// Minimum hardware requirements for a specific simulation or geometric kernel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelRequirement {
    /// Requires native 64-bit floating point hardware support.
    pub fp64_required: bool,
    /// Minimum required device memory in bytes.
    pub min_memory_bytes: u64,
    /// Requires strictly deterministic execution for repeatable engineering certification.
    pub deterministic_required: bool,
    /// Preferred backend if available.
    pub preferred_backend: Option<BackendKind>,
}

/// Handle to allocated device memory.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceBuffer {
    /// Unique buffer ID.
    pub id: u64,
    /// Size in bytes.
    pub size_bytes: u64,
    /// Hosting backend.
    pub backend: BackendKind,
}

/// Grid dimensions for compute dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct GridDim {
    /// X dimension count.
    pub x: u32,
    /// Y dimension count.
    pub y: u32,
    /// Z dimension count.
    pub z: u32,
}

impl Default for GridDim {
    fn default() -> Self {
        Self { x: 1, y: 1, z: 1 }
    }
}

/// Block/Workgroup dimensions for compute dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockDim {
    /// Threads in X.
    pub x: u32,
    /// Threads in Y.
    pub y: u32,
    /// Threads in Z.
    pub z: u32,
}

impl Default for BlockDim {
    fn default() -> Self {
        Self { x: 256, y: 1, z: 1 }
    }
}

/// Kernel launch argument variant.
#[derive(Debug, Clone)]
pub enum KernelArg {
    /// Buffer binding.
    Buffer(DeviceBuffer),
    /// F64 scalar value.
    ScalarF64(f64),
    /// F32 scalar value.
    ScalarF32(f32),
    /// U64 scalar value.
    ScalarU64(u64),
    /// U32 scalar value.
    ScalarU32(u32),
}

/// Abstract kernel handle.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KernelHandle {
    /// Kernel name.
    pub name: String,
    /// Target backend.
    pub backend: BackendKind,
}

/// Asynchronous completion event for launched compute kernels.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KernelEvent {
    /// Event ID.
    pub id: u64,
    /// Originating backend.
    pub backend: BackendKind,
}

/// Core safe hardware accelerator trait.
pub trait AcceleratorBackend: Send + Sync {
    /// Backend kind.
    fn kind(&self) -> BackendKind;

    /// Retrieve device capability descriptor.
    fn capabilities(&self) -> &BackendCapabilities;

    /// Allocate device memory.
    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError>;

    /// Upload host slice to device buffer.
    fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError>;

    /// Download device buffer contents to host slice.
    fn download_bytes(&self, buffer: &DeviceBuffer, out: &mut [u8]) -> Result<(), ComputeError>;

    /// Launch a compute kernel across the specified grid and block layout.
    fn launch(
        &self,
        kernel: &KernelHandle,
        grid: GridDim,
        block: BlockDim,
        args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError>;

    /// Block or wait for kernel completion event.
    fn wait(&self, event: &KernelEvent) -> Result<(), ComputeError>;
}
