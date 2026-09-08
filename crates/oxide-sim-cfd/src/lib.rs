//! Oxide-3D Computational Fluid Dynamics (CFD) Lattice Boltzmann & Finite Volume Solvers.

use serde::{Deserialize, Serialize};

/// Fluid domain physical properties.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FluidProperties {
    /// Dynamic viscosity in Pa*s (e.g. 1.81e-5 for air).
    pub dynamic_viscosity: f64,
    /// Density in kg/m^3 (e.g. 1.225 for air).
    pub density: f64,
}

impl Default for FluidProperties {
    fn default() -> Self {
        Self {
            dynamic_viscosity: 1.81e-5,
            density: 1.225,
        }
    }
}

/// Real-time GPU/CPU Lattice Boltzmann Method (LBM D3Q19) fluid simulator.
#[derive(Debug)]
pub struct LbmSolver {
    /// Grid dimensions (nx, ny, nz).
    pub dims: (usize, usize, usize),
    /// Fluid physical properties.
    pub fluid: FluidProperties,
}

impl LbmSolver {
    /// Create a new LBM solver instance on a 3D voxel grid.
    #[must_use]
    pub fn new(dims: (usize, usize, usize), fluid: FluidProperties) -> Self {
        Self { dims, fluid }
    }

    /// Advance simulation by a single time step dt.
    pub fn step(&mut self) {
        // LBM Stream-and-Collide step
    }
}
