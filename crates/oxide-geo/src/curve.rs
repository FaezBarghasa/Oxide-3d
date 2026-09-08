use serde::{Deserialize, Serialize};

/// 3D Parametric Curves.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Curve3d {
    /// Infinite or segment line.
    Line {
        /// Origin point.
        origin: [f64; 3],
        /// Unit direction vector.
        direction: [f64; 3],
    },
    /// Circular arc or full circle.
    Circle {
        /// Center point.
        center: [f64; 3],
        /// Normal axis of circle plane.
        normal: [f64; 3],
        /// Radius.
        radius: f64,
    },
    /// General Non-Uniform Rational B-Spline (NURBS) curve.
    Nurbs {
        /// Degree of polynomial basis functions.
        degree: usize,
        /// Control points [x, y, z, weight].
        control_points: Vec<[f64; 4]>,
        /// Knot vector.
        knots: Vec<f64>,
    },
}
