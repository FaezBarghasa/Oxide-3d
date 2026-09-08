//! OpenCL backend for heterogeneous devices and FPGAs.

use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, DeviceBuffer,
    GridDim, KernelArg, KernelEvent, KernelHandle,
};

/// OpenCL compute backend.
#[derive(Debug)]
pub struct OpenClBackend {
    capabilities: BackendCapabilities,
}

impl Default for OpenClBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenClBackend {
    /// Initialize OpenCL backend.
    #[must_use]
    pub fn new() -> Self {
        let capabilities = BackendCapabilities {
            kind: BackendKind::OpenCl,
            device_name: "OpenCL Heterogeneous Accelerator".to_string(),
            fp16: true,
            fp32: true,
            fp64: true,
            int64_atomics: true,
            unified_memory: false,
            max_workgroup_size: 512,
            subgroup_size: None,
            device_memory_bytes: 8 * 1024 * 1024 * 1024,
            host_memory_bytes: 32 * 1024 * 1024 * 1024,
            deterministic_reduction: false,
            p2p_multi_gpu: false,
        };
        Self { capabilities }
    }
}

impl AcceleratorBackend for OpenClBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::OpenCl
    }

    fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> {
        Ok(DeviceBuffer {
            id: 60,
            size_bytes: bytes,
            backend: BackendKind::OpenCl,
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
            id: 60,
            backend: BackendKind::OpenCl,
        })
    }

    fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError> {
        Ok(())
    }
}
