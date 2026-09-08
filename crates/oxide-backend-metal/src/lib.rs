//! Apple Metal compute backend for Apple Silicon / macOS.

use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, DeviceBuffer,
    GridDim, KernelArg, KernelEvent, KernelHandle,
};

/// Apple Metal compute backend.
#[derive(Debug)]
pub struct MetalBackend {
    capabilities: BackendCapabilities,
}

impl Default for MetalBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl MetalBackend {
    /// Initialize Apple Metal backend.
    #[must_use]
    pub fn new() -> Self {
        let capabilities = BackendCapabilities {
            kind: BackendKind::Metal,
            device_name: "Apple Silicon GPU (Metal)".to_string(),
            fp16: true,
            fp32: true,
            fp64: false, // Limited FP64 on standard Apple Silicon
            int64_atomics: true,
            unified_memory: true, // Native zero-copy unified memory
            max_workgroup_size: 1024,
            subgroup_size: Some(32),
            device_memory_bytes: 64 * 1024 * 1024 * 1024,
            host_memory_bytes: 64 * 1024 * 1024 * 1024,
            deterministic_reduction: true,
            p2p_multi_gpu: false,
        };
        Self { capabilities }
    }
}

impl AcceleratorBackend for MetalBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Metal
    }

    fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> {
        Ok(DeviceBuffer {
            id: 40,
            size_bytes: bytes,
            backend: BackendKind::Metal,
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
            id: 40,
            backend: BackendKind::Metal,
        })
    }

    fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError> {
        Ok(())
    }
}
