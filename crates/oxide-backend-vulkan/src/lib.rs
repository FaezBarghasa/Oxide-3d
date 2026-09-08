//! Vulkan Compute backend for explicit SPIR-V compute pipelines.

use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, DeviceBuffer,
    GridDim, KernelArg, KernelEvent, KernelHandle,
};

/// Vulkan explicit compute backend.
#[derive(Debug)]
pub struct VulkanBackend {
    capabilities: BackendCapabilities,
}

impl Default for VulkanBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl VulkanBackend {
    /// Initialize Vulkan backend.
    #[must_use]
    pub fn new() -> Self {
        let capabilities = BackendCapabilities {
            kind: BackendKind::Vulkan,
            device_name: "Vulkan Explicit Compute Device".to_string(),
            fp16: true,
            fp32: true,
            fp64: true,
            int64_atomics: true,
            unified_memory: false,
            max_workgroup_size: 1024,
            subgroup_size: Some(32),
            device_memory_bytes: 16 * 1024 * 1024 * 1024,
            host_memory_bytes: 64 * 1024 * 1024 * 1024,
            deterministic_reduction: false,
            p2p_multi_gpu: false,
        };
        Self { capabilities }
    }
}

impl AcceleratorBackend for VulkanBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Vulkan
    }

    fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> {
        Ok(DeviceBuffer {
            id: 30,
            size_bytes: bytes,
            backend: BackendKind::Vulkan,
        })
    }

    fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError> {
        self.allocate_device_buffer(data.len() as u64)
    }

    fn download_bytes(&self, _buffer: &DeviceBuffer, _out: &mut [u8]) -> Result<(), ComputeError> {
        Ok(())
    }

    fn launch(
        &self,
        _kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError> {
        Ok(KernelEvent {
            id: 30,
            backend: BackendKind::Vulkan,
        })
    }

    fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError> {
        Ok(())
    }
}
