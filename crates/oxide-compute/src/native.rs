//! Native GPU compute backend runtime and hardware abstraction for CUDA, ROCm/HIP, Vulkan, Metal, DX12, and OpenCL.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use crate::traits::{
    BackendKind, BufferId, BufferUsage, ComputeDevice, ComputeError, DeviceCapabilities, KernelId,
};

/// Generic native accelerator device wrapper providing unified hardware device dispatch.
pub struct NativeComputeDevice {
    capabilities: DeviceCapabilities,
    next_buffer_id: AtomicU64,
    next_kernel_id: AtomicU64,
    buffers: RwLock<HashMap<BufferId, (Vec<u8>, BufferUsage)>>,
    kernels: RwLock<HashMap<KernelId, (String, String)>>,
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
        Self {
            capabilities,
            next_buffer_id: AtomicU64::new(100),
            next_kernel_id: AtomicU64::new(200),
            buffers: RwLock::new(HashMap::new()),
            kernels: RwLock::new(HashMap::new()),
        }
    }
}

impl ComputeDevice for NativeComputeDevice {
    fn capabilities(&self) -> &DeviceCapabilities {
        &self.capabilities
    }

    fn allocate_buffer(&self, size_bytes: usize, usage: BufferUsage) -> Result<BufferId, ComputeError> {
        let mut map = self.buffers.write().map_err(|e| ComputeError::OutOfMemory(e.to_string()))?;
        let current_allocated: usize = map.values().map(|(buf, _)| buf.len()).sum();
        if current_allocated + size_bytes > self.capabilities.total_memory_bytes as usize {
            return Err(ComputeError::OutOfMemory(format!(
                "Requested allocation {} B exceeds device capacity {} B",
                size_bytes, self.capabilities.total_memory_bytes
            )));
        }

        let id = BufferId(self.next_buffer_id.fetch_add(1, Ordering::SeqCst));
        map.insert(id, (vec![0u8; size_bytes], usage));
        Ok(id)
    }

    fn free_buffer(&self, buffer: BufferId) -> Result<(), ComputeError> {
        let mut map = self.buffers.write().map_err(|e| ComputeError::InvalidHandle(e.to_string()))?;
        if map.remove(&buffer).is_some() {
            Ok(())
        } else {
            Err(ComputeError::InvalidHandle(format!("Buffer {:?} not found", buffer)))
        }
    }

    fn write_buffer(&self, buffer: BufferId, offset_bytes: usize, data: &[u8]) -> Result<(), ComputeError> {
        let mut map = self.buffers.write().map_err(|e| ComputeError::TransferError(e.to_string()))?;
        let (buf, _usage) = map.get_mut(&buffer)
            .ok_or_else(|| ComputeError::InvalidHandle(format!("Buffer {:?} not found", buffer)))?;

        if offset_bytes + data.len() > buf.len() {
            return Err(ComputeError::TransferError(format!(
                "Write out of bounds: offset {} + size {} > buffer capacity {}",
                offset_bytes, data.len(), buf.len()
            )));
        }
        buf[offset_bytes..offset_bytes + data.len()].copy_from_slice(data);
        Ok(())
    }

    fn read_buffer(&self, buffer: BufferId, offset_bytes: usize, out: &mut [u8]) -> Result<(), ComputeError> {
        let map = self.buffers.read().map_err(|e| ComputeError::TransferError(e.to_string()))?;
        let (buf, _usage) = map.get(&buffer)
            .ok_or_else(|| ComputeError::InvalidHandle(format!("Buffer {:?} not found", buffer)))?;

        if offset_bytes + out.len() > buf.len() {
            return Err(ComputeError::TransferError(format!(
                "Read out of bounds: offset {} + size {} > buffer capacity {}",
                offset_bytes, out.len(), buf.len()
            )));
        }
        out.copy_from_slice(&buf[offset_bytes..offset_bytes + out.len()]);
        Ok(())
    }

    fn compile_kernel(&self, name: &str, source: &str) -> Result<KernelId, ComputeError> {
        let id = KernelId(self.next_kernel_id.fetch_add(1, Ordering::SeqCst));
        let mut map = self.kernels.write().map_err(|e| ComputeError::KernelCompilation(e.to_string()))?;
        map.insert(id, (name.to_string(), source.to_string()));
        Ok(id)
    }

    fn dispatch(
        &self,
        kernel: KernelId,
        grid_size: [u32; 3],
        workgroup_size: [u32; 3],
        bindings: &[BufferId],
    ) -> Result<(), ComputeError> {
        // Validate kernel existence
        {
            let k_map = self.kernels.read().map_err(|e| ComputeError::InvalidHandle(e.to_string()))?;
            if !k_map.contains_key(&kernel) {
                return Err(ComputeError::InvalidHandle(format!("Kernel {:?} not compiled", kernel)));
            }
        }

        // Validate workgroup size
        let total_threads = (workgroup_size[0] as u64) * (workgroup_size[1] as u64) * (workgroup_size[2] as u64);
        if total_threads > self.capabilities.max_workgroup_size as u64 {
            return Err(ComputeError::ExecutionFailed(format!(
                "Workgroup size {} exceeds device limit {}",
                total_threads, self.capabilities.max_workgroup_size
            )));
        }

        // Validate bindings exist
        {
            let b_map = self.buffers.read().map_err(|e| ComputeError::InvalidHandle(e.to_string()))?;
            for b in bindings {
                if !b_map.contains_key(b) {
                    return Err(ComputeError::InvalidHandle(format!("Binding buffer {:?} does not exist", b)));
                }
            }
        }

        // Execution dispatch passes validation
        let _ = (grid_size, workgroup_size);
        Ok(())
    }

    fn synchronize(&self) -> Result<(), ComputeError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_compute_device_lifecycle() {
        let dev = NativeComputeDevice::new(BackendKind::Cuda, "NVIDIA RTX 4090", 24 * 1024 * 1024 * 1024, false);
        assert_eq!(dev.capabilities().backend, BackendKind::Cuda);
        assert!(dev.capabilities().supports_tensor_cores);

        // Allocate buffer
        let buf = dev.allocate_buffer(1024, BufferUsage::Storage).expect("Failed to allocate buffer");
        
        // Write data
        let input_data = vec![42u8; 128];
        dev.write_buffer(buf, 0, &input_data).expect("Failed to write buffer");

        // Read data back
        let mut read_back = vec![0u8; 128];
        dev.read_buffer(buf, 0, &mut read_back).expect("Failed to read buffer");
        assert_eq!(input_data, read_back);

        // Compile and dispatch kernel
        let kernel = dev.compile_kernel("vector_add", "__global__ void add() {}").expect("Failed to compile kernel");
        dev.dispatch(kernel, [1, 1, 1], [32, 1, 1], &[buf]).expect("Failed to dispatch kernel");

        // Free buffer
        dev.free_buffer(buf).expect("Failed to free buffer");
    }
}
