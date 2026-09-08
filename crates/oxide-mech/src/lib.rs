//! Oxide-3D Mechanisms, Kinematic Joints, and Rigid Body Dynamics.

use oxide_core::id::EntityKey;
use serde::{Deserialize, Serialize};

/// Type of kinematic joint constraint connecting mechanism components.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JointKind {
    /// Completely rigid fix.
    Fixed,
    /// 1-DOF Revolute hinge joint around an axis.
    Revolute {
        /// Axis vector.
        axis: [f64; 3],
        /// Optional min/max limits in radians.
        limits: Option<[f64; 2]>,
    },
    /// 1-DOF Prismatic slider joint along an axis.
    Prismatic {
        /// Translation axis.
        axis: [f64; 3],
        /// Optional min/max limits in distance.
        limits: Option<[f64; 2]>,
    },
    /// Gear pair coupling angular velocities.
    Gear {
        /// Ratio (output / input).
        ratio: f64,
        /// Driving entity.
        driver: EntityKey,
        /// Driven entity.
        driven: EntityKey,
    },
}
