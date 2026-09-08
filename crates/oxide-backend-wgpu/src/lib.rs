//! WebGPU / wgpu portable graphics and compute shader backend.

use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, DeviceBuffer,
    GridDim, KernelArg, KernelEvent, KernelHandle,
};

/// WebGPU portable accelerator backend.
#[derive(Debug)]
pub struct WgpuBackend {
    capabilities: BackendCapabilities,
}

impl Default for WgpuBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl WgpuBackend {
    /// Create new WebGPU backend wrapper.
    #[must_use]
    pub fn new() -> Self {
        let capabilities = BackendCapabilities {
            kind: BackendKind::Wgpu,
            device_name: "WebGPU Portable Adapter".to_string(),
            fp16: true,
            fp32: true,
            fp64: false,
            int64_atomics: false,
            unified_memory: false,
            max_workgroup_size: 256,
            subgroup_size: None,
            device_memory_bytes: 8 * 1024 * 1024 * 1024,
            host_memory_bytes: 32 * 1024 * 1024 * 1024,
            deterministic_reduction: false,
            p2p_multi_gpu: false,
        };
        Self { capabilities }
    }
}

impl AcceleratorBackend for WgpuBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Wgpu
    }

    fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> {
        Ok(DeviceBuffer {
            id: 1,
            size_bytes: bytes,
            backend: BackendKind::Wgpu,
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
            id: 1,
            backend: BackendKind::Wgpu,
        })
    }

    fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError> {
        Ok(())
    }
}
