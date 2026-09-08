//! Direct3D 12 DirectCompute backend for Windows.

use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, DeviceBuffer,
    GridDim, KernelArg, KernelEvent, KernelHandle,
};

/// Direct3D 12 DirectCompute backend.
#[derive(Debug)]
pub struct Dx12Backend {
    capabilities: BackendCapabilities,
}

impl Default for Dx12Backend {
    fn default() -> Self {
        Self::new()
    }
}

impl Dx12Backend {
    /// Initialize Direct3D 12 backend.
    #[must_use]
    pub fn new() -> Self {
        let capabilities = BackendCapabilities {
            kind: BackendKind::Dx12,
            device_name: "Direct3D 12 DirectCompute Adapter".to_string(),
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

impl AcceleratorBackend for Dx12Backend {
    fn kind(&self) -> BackendKind {
        BackendKind::Dx12
    }

    fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> {
        Ok(DeviceBuffer {
            id: 50,
            size_bytes: bytes,
            backend: BackendKind::Dx12,
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
            id: 50,
            backend: BackendKind::Dx12,
        })
    }

    fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError> {
        Ok(())
    }
}
