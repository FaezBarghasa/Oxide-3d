use serde::{Deserialize, Serialize};

/// 3D Parametric Surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Surface3d {
    /// Infinite or bounded planar surface.
    Plane {
        /// Origin point on plane.
        origin: [f64; 3],
        /// Surface normal vector.
        normal: [f64; 3],
    },
    /// Cylindrical surface.
    Cylinder {
        /// Center axis origin point.
        origin: [f64; 3],
        /// Axis direction vector.
        axis: [f64; 3],
        /// Radius.
        radius: f64,
    },
    /// Spherical surface.
    Sphere {
        /// Center point.
        center: [f64; 3],
        /// Radius.
        radius: f64,
    },
    /// General B-Spline / NURBS surface.
    Nurbs {
        /// Degrees in u and v.
        degrees: (usize, usize),
        /// 2D grid of control points with weights.
        control_points: Vec<Vec<[f64; 4]>>,
        /// Knot vectors in u and v.
        knots_u: Vec<f64>,
        knots_v: Vec<f64>,
    },
}
