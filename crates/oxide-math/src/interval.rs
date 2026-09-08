use serde::{Deserialize, Serialize};

/// 1D Interval arithmetic for bounding and uncertainty propagation.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Interval {
    /// Lower bound.
    pub min: f64,
    /// Upper bound.
    pub max: f64,
}

impl Interval {
    /// Create a new interval with bounds check.
    #[must_use]
    pub fn new(a: f64, b: f64) -> Self {
        if a <= b {
            Self { min: a, max: b }
        } else {
            Self { min: b, max: a }
        }
    }

    /// Check if a value is contained within the interval.
    #[must_use]
    pub fn contains(&self, val: f64) -> bool {
        val >= self.min && val <= self.max
    }

    /// Compute the intersection of two intervals.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);
        if min <= max {
            Some(Self { min, max })
        } else {
            None
        }
    }
}
