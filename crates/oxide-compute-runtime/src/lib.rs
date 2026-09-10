//! Oxide-3D Safe Async Compute Runtime, Job Scheduler, and Backend Auto-Selection.

use oxide_backend_cpu::CpuBackend;
use oxide_backend_cuda::CudaBackend;
use oxide_backend_dx12::Dx12Backend;
use oxide_backend_metal::MetalBackend;
use oxide_backend_opencl::OpenClBackend;
use oxide_backend_rocm::RocmBackend;
use oxide_backend_vulkan::VulkanBackend;
use oxide_backend_wgpu::WgpuBackend;
use oxide_hal::{
    AcceleratorBackend, BackendCapabilities, BackendKind, BlockDim, ComputeError, GridDim,
    KernelArg, KernelRequirement,
};
use oxide_kernels::{KernelCategory, KernelRegistry};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

/// High-level compute job specification.
#[derive(Debug, Clone)]
pub struct ComputeJob {
    /// Kernel domain category.
    pub category: KernelCategory,
    /// Hardware requirement.
    pub requirement: KernelRequirement,
    /// Grid dimensions.
    pub grid: GridDim,
    /// Workgroup/block dimensions.
    pub block: BlockDim,
}

/// Compute runtime orchestrator.
pub struct ComputeRuntime {
    backends: Vec<Arc<dyn AcceleratorBackend>>,
    kernel_registry: Arc<KernelRegistry>,
}

impl std::fmt::Debug for ComputeRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ComputeRuntime")
            .field("backends_count", &self.backends.len())
            .finish()
    }
}

impl Default for ComputeRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl ComputeRuntime {
    /// Initialize compute runtime and discover all available hardware accelerators.
    #[must_use]
    pub fn new() -> Self {
        let mut backends: Vec<Arc<dyn AcceleratorBackend>> = Vec::new();

        // 1. CUDA (NVIDIA)
        backends.push(Arc::new(CudaBackend::new()));
        // 2. ROCm / HIP (AMD)
        backends.push(Arc::new(RocmBackend::new()));
        // 3. Apple Metal (macOS / Apple Silicon)
        backends.push(Arc::new(MetalBackend::new()));
        // 4. Vulkan Compute (Cross-Platform)
        backends.push(Arc::new(VulkanBackend::new()));
        // 5. Direct3D 12 (Windows)
        backends.push(Arc::new(Dx12Backend::new()));
        // 6. WebGPU (Portable)
        backends.push(Arc::new(WgpuBackend::new()));
        // 7. OpenCL (Heterogeneous)
        backends.push(Arc::new(OpenClBackend::new()));
        // 8. CPU (Rayon Fallback - Always Available)
        backends.push(Arc::new(CpuBackend::new()));

        Self {
            backends,
            kernel_registry: Arc::new(KernelRegistry::new()),
        }
    }

    /// Select optimal backend satisfying the kernel requirements.
    pub fn select_backend(
        &self,
        req: &KernelRequirement,
    ) -> Result<Arc<dyn AcceleratorBackend>, ComputeError> {
        let best = self
            .backends
            .iter()
            .filter(|b| Self::satisfies(b.capabilities(), req))
            .max_by_key(|b| Self::score_backend(b.capabilities()));

        best.cloned()
            .ok_or(ComputeError::BackendUnavailable(BackendKind::Cpu))
    }

    fn satisfies(cap: &BackendCapabilities, req: &KernelRequirement) -> bool {
        if req.fp64_required && !cap.fp64 {
            return false;
        }
        if req.deterministic_required && !cap.deterministic_reduction {
            return false;
        }
        if cap.device_memory_bytes < req.min_memory_bytes {
            return false;
        }
        if let Some(preferred) = req.preferred_backend {
            if cap.kind == preferred {
                return true;
            }
        }
        true
    }

    fn score_backend(cap: &BackendCapabilities) -> u64 {
        let mut score = match cap.kind {
            BackendKind::Cuda => 1000,
            BackendKind::Rocm => 900,
            BackendKind::Metal => 800,
            BackendKind::Vulkan => 700,
            BackendKind::Dx12 => 650,
            BackendKind::Wgpu => 600,
            BackendKind::OpenCl => 300,
            BackendKind::Cpu => 50,
        };
        if cap.fp64 {
            score += 500;
        }
        if cap.deterministic_reduction {
            score += 100;
        }
        score += cap.device_memory_bytes / (1024 * 1024 * 1024);
        score
    }

    /// Submit a compute job with async execution and cancellation safety.
    pub async fn submit(
        &self,
        cancel: CancellationToken,
        job: ComputeJob,
        args: &[KernelArg],
    ) -> Result<(), ComputeError> {
        let backend = self.select_backend(&job.requirement)?;
        let kernel = self.kernel_registry.select(job.category, backend.kind());

        tokio::select! {
            _ = cancel.cancelled() => Err(ComputeError::Cancelled),
            res = async {
                let event = backend.launch(&kernel, job.grid, job.block, args)?;
                backend.wait(&event)?;
                Ok(())
            } => res,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compute_runtime_backend_selection_and_execution() {
        let runtime = ComputeRuntime::new();
        let req = KernelRequirement {
            fp64_required: false,
            deterministic_required: false,
            min_memory_bytes: 1024,
            preferred_backend: None,
        };

        let backend = runtime
            .select_backend(&req)
            .expect("CPU backend always available");
        assert!(backend.capabilities().device_memory_bytes >= 1024);

        let job = ComputeJob {
            category: KernelCategory::LinearAlgebra,
            requirement: req,
            grid: GridDim { x: 1, y: 1, z: 1 },
            block: BlockDim { x: 64, y: 1, z: 1 },
        };

        let cancel = CancellationToken::new();
        let res = runtime.submit(cancel, job, &[]).await;
        assert!(res.is_ok());
    }
}
