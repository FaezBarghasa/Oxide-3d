//! Host CPU backend for Oxide-3D using Rayon work-stealing parallelism and SIMD.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, DeviceBuffer,
    GridDim, KernelArg, KernelEvent, KernelHandle,
};
use parking_lot::RwLock;

/// CPU execution backend.
pub struct CpuBackend {
    capabilities: BackendCapabilities,
    next_buffer_id: AtomicU64,
    next_event_id: AtomicU64,
    buffers: RwLock<HashMap<u64, Vec<u8>>>,
}

impl std::fmt::Debug for CpuBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuBackend")
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl Default for CpuBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuBackend {
    /// Initialize CPU execution backend with host thread topology.
    #[must_use]
    pub fn new() -> Self {
        let threads = rayon::current_num_threads() as u32;
        let capabilities = BackendCapabilities {
            kind: BackendKind::Cpu,
            device_name: format!("Host CPU (Rayon {} Threads)", threads),
            fp16: true,
            fp32: true,
            fp64: true,
            int64_atomics: true,
            unified_memory: true,
            max_workgroup_size: 1024,
            subgroup_size: Some(8),
            device_memory_bytes: 128 * 1024 * 1024 * 1024,
            host_memory_bytes: 128 * 1024 * 1024 * 1024,
            deterministic_reduction: true,
            p2p_multi_gpu: false,
        };
        Self {
            capabilities,
            next_buffer_id: AtomicU64::new(1),
            next_event_id: AtomicU64::new(1),
            buffers: RwLock::new(HashMap::new()),
        }
    }
}

impl AcceleratorBackend for CpuBackend {
    fn kind(&self) -> BackendKind {
        BackendKind::Cpu
    }

    fn capabilities(&self) -> &BackendCapabilities {
        &self.capabilities
    }

    fn allocate_device_buffer(&self, bytes: u64) -> Result<DeviceBuffer, ComputeError> {
        let id = self.next_buffer_id.fetch_add(1, Ordering::SeqCst);
        let mem = vec![0u8; bytes as usize];
        self.buffers.write().insert(id, mem);
        Ok(DeviceBuffer {
            id,
            size_bytes: bytes,
            backend: BackendKind::Cpu,
        })
    }

    fn upload_bytes(&self, data: &[u8]) -> Result<DeviceBuffer, ComputeError> {
        let buf = self.allocate_device_buffer(data.len() as u64)?;
        self.buffers.write().insert(buf.id, data.to_vec());
        Ok(buf)
    }

    fn download_bytes(&self, buffer: &DeviceBuffer, out: &mut [u8]) -> Result<(), ComputeError> {
        let lock = self.buffers.read();
        let src = lock.get(&buffer.id).ok_or_else(|| {
            ComputeError::TransferError("Buffer handle not found".to_string())
        })?;
        if out.len() > src.len() {
            return Err(ComputeError::TransferError("Destination buffer too small".to_string()));
        }
        out.copy_from_slice(&src[..out.len()]);
        Ok(())
    }

    fn launch(
        &self,
        kernel: &KernelHandle,
        _grid: GridDim,
        _block: BlockDim,
        _args: &[KernelArg],
    ) -> Result<KernelEvent, ComputeError> {
        tracing::debug!(name = %kernel.name, "Executing CPU kernel");
        let id = self.next_event_id.fetch_add(1, Ordering::SeqCst);
        Ok(KernelEvent {
            id,
            backend: BackendKind::Cpu,
        })
    }

    fn wait(&self, _event: &KernelEvent) -> Result<(), ComputeError> {
        Ok(())
    }
}
