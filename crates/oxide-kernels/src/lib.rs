//! Central Kernel Registry and multi-backend kernel source / binary definitions.

use oxide_hal::{BackendKind, KernelHandle};
use serde::{Deserialize, Serialize};

/// High-level engineering kernel category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KernelCategory {
    /// Dense/Sparse Linear Algebra & Solvers.
    LinearAlgebra,
    /// FEA Stiffness Matrix Assembly & CG Solver.
    SparseSolve,
    /// Lattice Boltzmann Method (LBM) Fluid Solver.
    CfdLbm,
    /// SIMP Topology Optimization.
    TopOpt,
    /// GPU Voxelization & SDF field generation.
    Voxelization,
    /// Mesh processing & Tessellation.
    MeshProcessing,
    /// GPU Sculpting Brushes (Draw, Clay, Smooth, Flatten).
    SculptBrush,
    /// Voxel Remeshing & SDF reconstruction.
    Remesh,
    /// Real-time rigid-body mechanism updates.
    Mechanism,
    /// Viewport PBR Shading & Picking.
    Rendering,
}

/// Central registry mapping engineering operations to hardware-specific compute kernels.
#[derive(Debug, Default)]
pub struct KernelRegistry;

impl KernelRegistry {
    /// Create new kernel registry.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Select optimal kernel for a given category and target hardware backend.
    #[must_use]
    pub fn select(&self, category: KernelCategory, backend: BackendKind) -> KernelHandle {
        let name = match category {
            KernelCategory::LinearAlgebra => "saxpy_f64",
            KernelCategory::SparseSolve => "sparse_cg_solve_f64",
            KernelCategory::CfdLbm => "lbm_d3q19_stream_collide",
            KernelCategory::TopOpt => "simp_filter_f64",
            KernelCategory::Voxelization => "voxelize_mesh",
            KernelCategory::MeshProcessing => "mesh_tessellate",
            KernelCategory::SculptBrush => "sculpt_brush_deform",
            KernelCategory::Remesh => "voxel_remesh_openvdb",
            KernelCategory::Mechanism => "joint_solve_f64",
            KernelCategory::Rendering => "pbr_viewport",
        };
        KernelHandle {
            name: name.to_string(),
            backend,
        }
    }
}
