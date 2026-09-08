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
}

impl Default for TopOptConfig {
    fn default() -> Self {
        Self {
            target_volume_fraction: 0.30,
            penalization_power: 3.0,
            filter_radius: 1.5,
            max_iterations: 100,
        }
    }
}
