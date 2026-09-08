use serde::{Deserialize, Serialize};

/// Standard physical unit systems supported across CAD, simulation, and CAM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnitSystem {
    /// Metric SI standard (meters, kilograms, seconds, Newtons).
    MetricSi,
    /// Metric Engineering standard (millimeters, kilograms, seconds, Newtons, Megapascals).
    MetricMm,
    /// Imperial standard (inches, pounds-mass, seconds, pounds-force, psi).
    ImperialInches,
}

impl Default for UnitSystem {
    fn default() -> Self {
        Self::MetricMm
    }
}
