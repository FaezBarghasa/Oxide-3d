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

/// 2D/3D Lattice Boltzmann (D2Q9) fluid dynamics simulator.
#[derive(Debug, Clone)]
pub struct LbmSolver2D {
    /// Grid width and height (nx, ny).
    pub dims: (usize, usize),
    /// Relaxation time parameter tau.
    pub tau: f64,
    /// Discrete velocity distribution functions f_i for each cell.
    pub f: Vec<[f64; 9]>,
    /// Temporary stream buffer.
    pub f_temp: Vec<[f64; 9]>,
    /// Obstacle mask (true = solid bounce-back boundary).
    pub solid_mask: Vec<bool>,
    /// Inflow velocity [ux, uy].
    pub inflow_velocity: [f64; 2],
}

// D2Q9 lattice velocities e_i
const C_X: [i32; 9] = [0, 1, 0, -1, 0, 1, -1, -1, 1];
const C_Y: [i32; 9] = [0, 0, 1, 0, -1, 1, 1, -1, -1];
// Lattice weights w_i
const W: [f64; 9] = [
    4.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 9.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
    1.0 / 36.0,
];
// Opposite direction indices for bounce-back
const OPPOSITE: [usize; 9] = [0, 3, 4, 1, 2, 7, 8, 5, 6];

impl LbmSolver2D {
    /// Create a new 2D Lattice Boltzmann fluid solver.
    pub fn new(nx: usize, ny: usize, tau: f64, inflow_velocity: [f64; 2]) -> Self {
        let size = nx * ny;
        let mut f = Vec::with_capacity(size);
        for _ in 0..size {
            f.push(W); // Initial resting distribution
        }

        Self {
            dims: (nx, ny),
            tau,
            f_temp: f.clone(),
            f,
            solid_mask: vec![false; size],
            inflow_velocity,
        }
    }

    /// Add a circular solid obstruction (e.g. cylinder in crossflow).
    pub fn add_cylinder_obstacle(&mut self, cx: f64, cy: f64, radius: f64) {
        let (nx, ny) = self.dims;
        let r2 = radius * radius;
        for y in 0..ny {
            for x in 0..nx {
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;
                if dx * dx + dy * dy <= r2 {
                    self.solid_mask[y * nx + x] = true;
                }
            }
        }
    }

    /// Calculate macroscopic density rho and velocity (ux, uy) for cell (x, y).
    pub fn macroscopic(&self, x: usize, y: usize) -> (f64, [f64; 2]) {
        let idx = y * self.dims.0 + x;
        let fi = self.f[idx];
        let mut rho = 0.0;
        let mut ux = 0.0;
        let mut uy = 0.0;

        for i in 0..9 {
            rho += fi[i];
            ux += fi[i] * C_X[i] as f64;
            uy += fi[i] * C_Y[i] as f64;
        }

        if rho > 1e-12 {
            ux /= rho;
            uy /= rho;
        }

        (rho, [ux, uy])
    }

    /// Advance simulation by a single time step dt using BGK collision and streaming.
    pub fn step(&mut self) {
        let (nx, ny) = self.dims;
        let omega = 1.0 / self.tau;

        // Collision step (BGK relaxation towards equilibrium)
        for y in 0..ny {
            for x in 0..nx {
                let idx = y * nx + x;
                if self.solid_mask[idx] {
                    continue;
                }

                let (rho, [ux, uy]) = self.macroscopic(x, y);
                let u2 = ux * ux + uy * uy;

                for i in 0..9 {
                    let eu = C_X[i] as f64 * ux + C_Y[i] as f64 * uy;
                    let feq = W[i] * rho * (1.0 + 3.0 * eu + 4.5 * eu * eu - 1.5 * u2);
                    self.f_temp[idx][i] = self.f[idx][i] * (1.0 - omega) + feq * omega;
                }
            }
        }

        // Streaming step with boundary conditions
        for y in 0..ny {
            for x in 0..nx {
                let curr_idx = y * nx + x;
                if self.solid_mask[curr_idx] {
                    continue;
                }

                for i in 0..9 {
                    let src_x = (x as i32 - C_X[i]).rem_euclid(nx as i32) as usize;
                    let src_y = (y as i32 - C_Y[i]).rem_euclid(ny as i32) as usize;
                    let src_idx = src_y * nx + src_x;

                    if self.solid_mask[src_idx] {
                        // Half-way bounce-back on solid obstacles
                        self.f[curr_idx][i] = self.f_temp[curr_idx][OPPOSITE[i]];
                    } else {
                        self.f[curr_idx][i] = self.f_temp[src_idx][i];
                    }
                }
            }
        }

        // Inflow boundary condition at x = 0
        let [u_in, v_in] = self.inflow_velocity;
        let u2_in = u_in * u_in + v_in * v_in;
        for y in 0..ny {
            let idx = y * nx;
            let rho = 1.0;
            for i in 0..9 {
                let eu = C_X[i] as f64 * u_in + C_Y[i] as f64 * v_in;
                self.f[idx][i] = W[i] * rho * (1.0 + 3.0 * eu + 4.5 * eu * eu - 1.5 * u2_in);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lbm_cfd_simulation_step() {
        let mut solver = LbmSolver2D::new(40, 20, 0.6, [0.05, 0.0]);
        solver.add_cylinder_obstacle(15.0, 10.0, 3.0);

        // Run 20 simulation time steps
        for _ in 0..20 {
            solver.step();
        }

        let (rho_in, [ux_in, _]) = solver.macroscopic(0, 10);
        assert!((rho_in - 1.0).abs() < 1e-3);
        assert!(ux_in > 0.0, "Inflow velocity should be positive along X");

        // The center of the obstacle should remain blocked
        let obs_idx = 10 * 40 + 15;
        assert!(
            solver.solid_mask[obs_idx],
            "Cylinder obstacle mask should be set"
        );
    }
}
