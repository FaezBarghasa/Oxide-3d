//! Oxide-3D Topology Optimization (SIMP Density Method & Level Set).

use serde::{Deserialize, Serialize};

/// Configuration parameters for SIMP Topology Optimization.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TopOptConfig {
    /// Target volume fraction (e.g. 0.30 = 30% material retained).
    pub target_volume_fraction: f64,
    /// SIMP penalization power (typically 3.0).
    pub penalization_power: f64,
    /// Filter radius for minimum member size control.
    pub filter_radius: f64,
    /// Maximum optimization iterations.
    pub max_iterations: usize,
    /// Move limit for Optimality Criteria (OC) updates.
    pub move_limit: f64,
}

impl Default for TopOptConfig {
    fn default() -> Self {
        Self {
            target_volume_fraction: 0.30,
            penalization_power: 3.0,
            filter_radius: 1.5,
            max_iterations: 100,
            move_limit: 0.2,
        }
    }
}

/// SIMP (Solid Isotropic Material with Penalization) Topology Optimizer.
#[derive(Debug)]
pub struct SimpOptimizer {
    /// Number of elements along X and Y.
    pub grid_size: (usize, usize),
    /// Configuration settings.
    pub config: TopOptConfig,
    /// Material densities x_e in [0.001, 1.0].
    pub densities: Vec<f64>,
}

impl SimpOptimizer {
    /// Initialize a SIMP optimizer with uniform density satisfying target volume fraction.
    pub fn new(nelx: usize, nely: usize, config: TopOptConfig) -> Self {
        let num_elements = nelx * nely;
        let initial_density = config.target_volume_fraction;
        Self {
            grid_size: (nelx, nely),
            config,
            densities: vec![initial_density; num_elements],
        }
    }

    /// Perform an Optimality Criteria (OC) density update step given element strain energy sensitivities.
    pub fn update_densities(&mut self, sensitivities: &[f64]) {
        let (nelx, nely) = self.grid_size;
        let total_elements = nelx * nely;
        if sensitivities.len() != total_elements {
            return;
        }

        // Bisection search for Lagrange multiplier lambda
        let mut l1 = 0.0;
        let mut l2 = 1e9;
        let move_limit = self.config.move_limit;
        let target_vol = self.config.target_volume_fraction * total_elements as f64;

        let mut updated = vec![0.0; total_elements];

        for _ in 0..50 {
            let lmid = 0.5 * (l1 + l2);
            let mut current_vol = 0.0;

            for i in 0..total_elements {
                let x = self.densities[i];
                let b_e = (sensitivities[i] / lmid).sqrt();
                let x_new = if x * b_e <= x - move_limit {
                    x - move_limit
                } else if x * b_e >= x + move_limit {
                    x + move_limit
                } else {
                    x * b_e
                };
                let x_clamped = x_new.clamp(0.001, 1.0);
                updated[i] = x_clamped;
                current_vol += x_clamped;
            }

            if current_vol > target_vol {
                l1 = lmid;
            } else {
                l2 = lmid;
            }

            if (l2 - l1) / (l1 + l2 + 1e-12) < 1e-3 {
                break;
            }
        }

        self.densities = updated;
    }

    /// Calculate active volume fraction of the current density distribution.
    pub fn current_volume_fraction(&self) -> f64 {
        let sum: f64 = self.densities.iter().sum();
        sum / self.densities.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simp_density_update() {
        let config = TopOptConfig {
            target_volume_fraction: 0.50,
            penalization_power: 3.0,
            filter_radius: 1.5,
            max_iterations: 20,
            move_limit: 0.2,
        };

        let mut opt = SimpOptimizer::new(10, 10, config);
        assert!((opt.current_volume_fraction() - 0.50).abs() < 1e-4);

        // Simulated sensitivities: high strain energy in lower half
        let mut sensitivities = vec![0.1; 100];
        for i in 50..100 {
            sensitivities[i] = 2.0;
        }

        opt.update_densities(&sensitivities);

        // Elements with high sensitivity should have higher density than low sensitivity
        assert!(opt.densities[75] > opt.densities[25]);
        // Total volume fraction should remain conserved close to 0.50
        assert!((opt.current_volume_fraction() - 0.50).abs() < 0.05);
    }
}
