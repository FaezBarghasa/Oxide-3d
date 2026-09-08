use serde::{Deserialize, Serialize};

/// Modeling and manufacturing tolerances.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tolerance {
    /// Linear distance coincidence tolerance (in default modeling units, typically 1e-6 mm).
    pub linear: f64,
    /// Angular parallelism/perpendicularity tolerance in radians.
    pub angular: f64,
}

impl Default for Tolerance {
    fn default() -> Self {
        Self {
            linear: 1e-6,
            angular: 1e-8,
        }
    }
}

impl Tolerance {
    /// Check if two points are coincident within linear tolerance.
    #[must_use]
    pub fn points_equal(&self, p1: [f64; 3], p2: [f64; 3]) -> bool {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        let dz = p1[2] - p2[2];
        (dx * dx + dy * dy + dz * dz) <= (self.linear * self.linear)
    }
}
