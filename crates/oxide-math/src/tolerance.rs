use serde::{Deserialize, Serialize};

/// Modeling and manufacturing tolerances with explicit context for geometric queries.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ToleranceContext {
    /// Linear distance coincidence tolerance (in default modeling units, typically 1e-7 m).
    pub linear_epsilon: f64,
    /// Angular parallelism/perpendicularity tolerance in radians (e.g., 1e-6 rad).
    pub angular_epsilon: f64,
    /// UV space tolerance on surface domains for parametric operations.
    pub parametric_epsilon: f64,
}

impl Default for ToleranceContext {
    fn default() -> Self {
        Self {
            linear_epsilon: 1e-7,
            angular_epsilon: 1e-6,
            parametric_epsilon: 1e-8,
        }
    }
}

impl ToleranceContext {
    /// Create a new tolerance context with custom values.
    #[must_use]
    pub const fn new(linear: f64, angular: f64, parametric: f64) -> Self {
        Self {
            linear_epsilon: linear,
            angular_epsilon: angular,
            parametric_epsilon: parametric,
        }
    }

    /// High precision context for exact CAD operations.
    #[must_use]
    pub const fn high_precision() -> Self {
        Self {
            linear_epsilon: 1e-9,
            angular_epsilon: 1e-8,
            parametric_epsilon: 1e-10,
        }
    }

    /// Loose tolerance for interactive/DCC operations.
    #[must_use]
    pub const fn loose() -> Self {
        Self {
            linear_epsilon: 1e-4,
            angular_epsilon: 1e-3,
            parametric_epsilon: 1e-5,
        }
    }

    /// Check if two 3D points are coincident within linear tolerance.
    #[must_use]
    pub fn points_equal(&self, p1: [f64; 3], p2: [f64; 3]) -> bool {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        let dz = p1[2] - p2[2];
        (dx * dx + dy * dy + dz * dz) <= (self.linear_epsilon * self.linear_epsilon)
    }

    /// Check if two 2D points are coincident within linear tolerance.
    #[must_use]
    pub fn points_equal_2d(&self, p1: [f64; 2], p2: [f64; 2]) -> bool {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        (dx * dx + dy * dy) <= (self.linear_epsilon * self.linear_epsilon)
    }

    /// Check if two angles are equal within angular tolerance.
    #[must_use]
    pub fn angles_equal(&self, a1: f64, a2: f64) -> bool {
        let diff = (a1 - a2).abs();
        diff <= self.angular_epsilon || (std::f64::consts::TAU - diff) <= self.angular_epsilon
    }

    /// Check if a value is effectively zero within linear tolerance.
    #[must_use]
    pub fn is_zero(&self, val: f64) -> bool {
        val.abs() <= self.linear_epsilon
    }

    /// Check if two vectors are parallel within angular tolerance.
    #[must_use]
    pub fn vectors_parallel(&self, v1: [f64; 3], v2: [f64; 3]) -> bool {
        let dot = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
        let norm1 = (v1[0] * v1[0] + v1[1] * v1[1] + v1[2] * v1[2]).sqrt();
        let norm2 = (v2[0] * v2[0] + v2[1] * v2[1] + v2[2] * v2[2]).sqrt();
        if norm1 < self.linear_epsilon || norm2 < self.linear_epsilon {
            return false;
        }
        let cos_angle = (dot / (norm1 * norm2)).clamp(-1.0, 1.0);
        cos_angle.abs() >= 1.0 - self.angular_epsilon
    }

    /// Check if two vectors are perpendicular within angular tolerance.
    #[must_use]
    pub fn vectors_perpendicular(&self, v1: [f64; 3], v2: [f64; 3]) -> bool {
        let dot = v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2];
        let norm1 = (v1[0] * v1[0] + v1[1] * v1[1] + v1[2] * v1[2]).sqrt();
        let norm2 = (v2[0] * v2[0] + v2[1] * v2[1] + v2[2] * v2[2]).sqrt();
        if norm1 < self.linear_epsilon || norm2 < self.linear_epsilon {
            return false;
        }
        let cos_angle = (dot / (norm1 * norm2)).abs();
        cos_angle <= self.angular_epsilon
    }
}

/// Simple Tolerance struct for backward compatibility.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tolerance {
    pub linear: f64,
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
    #[must_use]
    pub fn points_equal(&self, p1: [f64; 3], p2: [f64; 3]) -> bool {
        let dx = p1[0] - p2[0];
        let dy = p1[1] - p2[1];
        let dz = p1[2] - p2[2];
        (dx * dx + dy * dy + dz * dz) <= (self.linear * self.linear)
    }

    /// Convert to ToleranceContext.
    #[must_use]
    pub fn to_context(&self) -> ToleranceContext {
        ToleranceContext {
            linear_epsilon: self.linear,
            angular_epsilon: self.angular,
            parametric_epsilon: self.linear,
        }
    }
}

impl From<Tolerance> for ToleranceContext {
    fn from(t: Tolerance) -> Self {
        t.to_context()
    }
}
