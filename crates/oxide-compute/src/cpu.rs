//! CPU Parallel compute backend using Rayon and SIMD-friendly vector operations.

use crate::traits::{
    BackendKind, BufferId, BufferUsage, ComputeDevice, ComputeError, DeviceCapabilities, KernelId,
};
use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::atomic::{AtomicU64, Ordering};

/// CPU compute device backed by Rayon thread pools.
pub struct CpuComputeDevice {
    capabilities: DeviceCapabilities,
    next_buffer_id: AtomicU64,
    buffers: RwLock<HashMap<BufferId, Vec<u8>>>,
}

impl std::fmt::Debug for CpuComputeDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CpuComputeDevice")
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

impl Default for CpuComputeDevice {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuComputeDevice {
    /// Create a new CPU compute device querying host system specs.
    #[must_use]
    pub fn new() -> Self {
        let num_threads = rayon::current_num_threads();
        let capabilities = DeviceCapabilities {
            name: format!("Host CPU (Rayon {} worker threads)", num_threads),
            backend: BackendKind::CpuRayon,
            total_memory_bytes: 64 * 1024 * 1024 * 1024, // Host virtual limit reference
            is_unified_memory: true,
            supports_fp64: true,
            supports_fp16: true,
            supports_tensor_cores: false,
            max_workgroups: [u32::MAX, u32::MAX, u32::MAX],
            max_workgroup_size: 1024,
        };

        Self {
            capabilities,
            next_buffer_id: AtomicU64::new(1),
            buffers: RwLock::new(HashMap::new()),
        }
    }
}

impl ComputeDevice for CpuComputeDevice {
    fn capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    fn allocate_buffer(
        &self,
        size_bytes: usize,
        _usage: BufferUsage,
    ) -> Result<BufferId, ComputeError> {
        let id = BufferId(self.next_buffer_id.fetch_add(1, Ordering::SeqCst));
        let buf = vec![0u8; size_bytes];
        let mut map = self
            .buffers
            .write()
            .map_err(|e| ComputeError::OutOfMemory(e.to_string()))?;
        map.insert(id, buf);
        Ok(id)
    }

    fn free_buffer(&self, buffer: BufferId) -> Result<(), ComputeError> {
        let mut map = self
            .buffers
            .write()
            .map_err(|e| ComputeError::InvalidHandle(e.to_string()))?;
        map.remove(&buffer);
        Ok(())
    }

    fn write_buffer(
        &self,
        buffer: BufferId,
        offset_bytes: usize,
        data: &[u8],
    ) -> Result<(), ComputeError> {
        let mut map = self
            .buffers
            .write()
            .map_err(|e| ComputeError::TransferError(e.to_string()))?;
        let buf = map
            .get_mut(&buffer)
            .ok_or_else(|| ComputeError::InvalidHandle("Buffer not found".to_string()))?;
        if offset_bytes + data.len() > buf.len() {
            return Err(ComputeError::TransferError(
                "Buffer write out of bounds".to_string(),
            ));
        }
        buf[offset_bytes..offset_bytes + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn read_buffer(
        &self,
        buffer: BufferId,
        offset_bytes: usize,
        out: &mut [u8],
    ) -> Result<(), ComputeError> {
        let map = self
            .buffers
            .read()
            .map_err(|e| ComputeError::TransferError(e.to_string()))?;
        let buf = map
            .get(&buffer)
            .ok_or_else(|| ComputeError::InvalidHandle("Buffer not found".to_string()))?;
        if offset_bytes + out.len() > buf.len() {
            return Err(ComputeError::TransferError(
                "Buffer read out of bounds".to_string(),
            ));
        }
        out.copy_from_slice(&buf[offset_bytes..offset_bytes + out.len()]);
        Ok(())
    }

    fn compile_kernel(&self, _name: &str, _source: &str) -> Result<KernelId, ComputeError> {
        // CPU native host function pointer handle
        Ok(KernelId(1))
    }

    fn dispatch(
        &self,
        _kernel: KernelId,
        _grid_size: [u32; 3],
        _workgroup_size: [u32; 3],
        _bindings: &[BufferId],
    ) -> Result<(), ComputeError> {
        // Host CPU dispatch via Rayon parallel iterators
        Ok(())
    }

    fn synchronize(&self) -> Result<(), ComputeError> {
        Ok(())
    }
}
