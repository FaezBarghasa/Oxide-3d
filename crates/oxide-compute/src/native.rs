//! Native GPU compute backend stubs for CUDA, ROCm/HIP, Vulkan, Metal, DX12, and OpenCL.

use crate::traits::{
    BackendKind, BufferId, BufferUsage, ComputeDevice, ComputeError, DeviceCapabilities, KernelId,
};

/// Generic native accelerator device wrapper providing dynamic dispatch across hardware vendors.
pub struct NativeComputeDevice {
    capabilities: DeviceCapabilities,
}

impl std::fmt::Debug for NativeComputeDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeComputeDevice")
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl NativeComputeDevice {
    /// Create a new native device representation for a given backend kind.
    #[must_use]
    pub fn new(backend: BackendKind, name: &str, total_mem: u64, is_unified: bool) -> Self {
        let capabilities = DeviceCapabilities {
            name: name.to_string(),
            backend,
            total_memory_bytes: total_mem,
            is_unified_memory: is_unified,
            supports_fp64: true,
            supports_fp16: true,
            supports_tensor_cores: matches!(backend, BackendKind::Cuda | BackendKind::RocmHip | BackendKind::Metal),
            max_workgroups: [65535, 65535, 65535],
            max_workgroup_size: 1024,
        };
        Self { capabilities }
    }
}

impl ComputeDevice for NativeComputeDevice {
    fn capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    fn allocate_buffer(&self, _size_bytes: usize, _usage: BufferUsage) -> Result<BufferId, ComputeError> {
        Ok(BufferId(100))
    }

    fn free_buffer(&self, _buffer: BufferId) -> Result<(), ComputeError> {
        Ok(())
    }

    fn write_buffer(&self, _buffer: BufferId, _offset_bytes: usize, _data: &[u8]) -> Result<(), ComputeError> {
        Ok(())
    }

    fn read_buffer(&self, _buffer: BufferId, _offset_bytes: usize, _out: &mut [u8]) -> Result<(), ComputeError> {
        Ok(())
    }

    fn compile_kernel(&self, _name: &str, _source: &str) -> Result<KernelId, ComputeError> {
        Ok(KernelId(200))
    }

    fn dispatch(
        &self,
        _kernel: KernelId,
        _grid_size: [u32; 3],
        _workgroup_size: [u32; 3],
        _bindings: &[BufferId],
    ) -> Result<(), ComputeError> {
        Ok(())
    }

    fn synchronize(&self) -> Result<(), ComputeError> {
        Ok(())
    }
}
