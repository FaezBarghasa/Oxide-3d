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

    /// Create an interval from a center and radius.
    #[must_use]
    pub fn from_center_radius(center: f64, radius: f64) -> Self {
        Self {
            min: center - radius,
            max: center + radius,
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

    /// Compute the union of two intervals.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    /// Check if two intervals overlap.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.min <= other.max && other.min <= self.max
    }

    /// Get the width of the interval.
    #[must_use]
    pub fn width(&self) -> f64 {
        self.max - self.min
    }

    /// Get the midpoint of the interval.
    #[must_use]
    pub fn mid(&self) -> f64 {
        (self.min + self.max) * 0.5
    }

    /// Expand interval by a margin.
    #[must_use]
    pub fn expand(&self, margin: f64) -> Self {
        Self {
            min: self.min - margin,
            max: self.max + margin,
        }
    }
}

/// 3D Interval (Axis-Aligned Bounding Box) for geometric intersection tests.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Interval3d {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Interval3d {
    /// Create a new 3D interval from min and max points.
    #[must_use]
    pub fn new(min: [f64; 3], max: [f64; 3]) -> Self {
        Self {
            x: Interval::new(min[0], max[0]),
            y: Interval::new(min[1], max[1]),
            z: Interval::new(min[2], max[2]),
        }
    }

    /// Create from center and half-extents.
    #[must_use]
    pub fn from_center_half_extents(center: [f64; 3], half_extents: [f64; 3]) -> Self {
        Self {
            x: Interval::from_center_radius(center[0], half_extents[0]),
            y: Interval::from_center_radius(center[1], half_extents[1]),
            z: Interval::from_center_radius(center[2], half_extents[2]),
        }
    }

    /// Check if a point is contained within the 3D interval.
    #[must_use]
    pub fn contains(&self, point: [f64; 3]) -> bool {
        self.x.contains(point[0]) && self.y.contains(point[1]) && self.z.contains(point[2])
    }

    /// Check if two 3D intervals intersect (AABB intersection test).
    #[must_use]
    pub fn intersects(&self, other: &Self) -> bool {
        self.x.overlaps(&other.x) && self.y.overlaps(&other.y) && self.z.overlaps(&other.z)
    }

    /// Check intersection with tolerance margin.
    #[must_use]
    pub fn intersects_tol(&self, other: &Self, tol: f64) -> bool {
        let x_overlap = self.x.min <= other.x.max + tol && other.x.min <= self.x.max + tol;
        let y_overlap = self.y.min <= other.y.max + tol && other.y.min <= self.y.max + tol;
        let z_overlap = self.z.min <= other.z.max + tol && other.z.min <= self.z.max + tol;
        x_overlap && y_overlap && z_overlap
    }

    /// Compute the intersection of two 3D intervals.
    #[must_use]
    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let x = self.x.intersect(&other.x)?;
        let y = self.y.intersect(&other.y)?;
        let z = self.z.intersect(&other.z)?;
        Some(Self { x, y, z })
    }

    /// Compute the union of two 3D intervals.
    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        Self {
            x: self.x.union(&other.x),
            y: self.y.union(&other.y),
            z: self.z.union(&other.z),
        }
    }

    /// Expand by a uniform margin.
    #[must_use]
    pub fn expand(&self, margin: f64) -> Self {
        Self {
            x: self.x.expand(margin),
            y: self.y.expand(margin),
            z: self.z.expand(margin),
        }
    }

    /// Get the center point.
    #[must_use]
    pub fn center(&self) -> [f64; 3] {
        [self.x.mid(), self.y.mid(), self.z.mid()]
    }

    /// Get the half-extents.
    #[must_use]
    pub fn half_extents(&self) -> [f64; 3] {
        [self.x.width() * 0.5, self.y.width() * 0.5, self.z.width() * 0.5]
    }

    /// Check if the interval is degenerate (zero volume).
    #[must_use]
    pub fn is_degenerate(&self, tol: f64) -> bool {
        self.x.width() <= tol || self.y.width() <= tol || self.z.width() <= tol
    }

    /// Get the longest axis (0=x, 1=y, 2=z).
    #[must_use]
    pub fn longest_axis(&self) -> usize {
        let wx = self.x.width();
        let wy = self.y.width();
        let wz = self.z.width();
        if wx >= wy && wx >= wz {
            0
        } else if wy >= wz {
            1
        } else {
            2
        }
    }
}
