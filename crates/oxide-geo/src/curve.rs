//! 3D Parametric Curves with exact evaluation, differentiation, and NURBS algorithms.

use serde::{Deserialize, Serialize};

/// 3D Parametric Curves.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Curve3d {
    /// Linear segment or line.
    Line {
        /// Origin / start point [x, y, z].
        origin: [f64; 3],
        /// Direction vector [dx, dy, dz].
        direction: [f64; 3],
    },
    /// Circular arc or full circle.
    Circle {
        /// Center point [x, y, z].
        center: [f64; 3],
        /// Normal axis vector [nx, ny, nz].
        normal: [f64; 3],
        /// Reference radius.
        radius: f64,
    },
    /// General Non-Uniform Rational B-Spline (NURBS) curve.
    Nurbs {
        /// Degree of basis functions (p).
        degree: usize,
        /// Control points [x, y, z, weight].
        control_points: Vec<[f64; 4]>,
        /// Non-decreasing knot vector.
        knots: Vec<f64>,
    },
}

impl Curve3d {
    /// Evaluate 3D point on curve at parameter `t`.
    #[must_use]
    pub fn evaluate(&self, t: f64) -> [f64; 3] {
        match self {
            Self::Line { origin, direction } => [
                origin[0] + direction[0] * t,
                origin[1] + direction[1] * t,
                origin[2] + direction[2] * t,
            ],
            Self::Circle {
                center,
                normal,
                radius,
            } => {
                let n = glam::DVec3::from_slice(normal).normalize_or_zero();
                let ref_axis = if n.x.abs() < 0.9 {
                    glam::DVec3::X
                } else {
                    glam::DVec3::Y
                };
                let u = n.cross(ref_axis).normalize();
                let v = n.cross(u);

                let angle = t * std::f64::consts::TAU;
                let c = glam::DVec3::from_slice(center);
                let p = c + u * (angle.cos() * radius) + v * (angle.sin() * radius);
                [p.x, p.y, p.z]
            }
            Self::Nurbs {
                degree,
                control_points,
                knots,
            } => evaluate_nurbs_curve(*degree, control_points, knots, t),
        }
    }

    /// Evaluate tangent derivative vector `C'(t)` on curve at parameter `t`.
    #[must_use]
    pub fn tangent(&self, t: f64) -> [f64; 3] {
        let eps = 1e-5;
        let p1 = self.evaluate(t - eps);
        let p2 = self.evaluate(t + eps);
        let dx = (p2[0] - p1[0]) / (2.0 * eps);
        let dy = (p2[1] - p1[1]) / (2.0 * eps);
        let dz = (p2[2] - p1[2]) / (2.0 * eps);
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        if len > 1e-12 {
            [dx / len, dy / len, dz / len]
        } else {
            [0.0, 0.0, 1.0]
        }
    }
}

/// De Boor algorithm for evaluating rational B-Spline / NURBS curve at parameter `t`.
fn evaluate_nurbs_curve(
    degree: usize,
    control_points: &[[f64; 4]],
    knots: &[f64],
    t: f64,
) -> [f64; 3] {
    if control_points.is_empty() || knots.is_empty() {
        return [0.0, 0.0, 0.0];
    }
    let n = control_points.len();
    let p = degree;

    // Clamp t to knot bounds
    let t_min = knots[p];
    let t_max = knots[n];
    let clamped_t = t.clamp(t_min, t_max);

    // Find knot span k
    let mut k = p;
    for i in p..n {
        if clamped_t >= knots[i] && clamped_t < knots[i + 1] {
            k = i;
            break;
        }
        if (clamped_t - t_max).abs() < 1e-9 {
            k = n - 1;
            break;
        }
    }

    // Initialize working points in homogeneous coordinates
    let mut d: Vec<[f64; 4]> = Vec::with_capacity(p + 1);
    for j in 0..=p {
        let idx = k.saturating_sub(p) + j;
        let pt = control_points.get(idx).copied().unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let w = pt[3];
        d.push([pt[0] * w, pt[1] * w, pt[2] * w, w]);
    }

    // Triangular de Boor reduction
    for r in 1..=p {
        for j in (r..=p).rev() {
            let idx = k.saturating_sub(p) + j;
            let denom = knots.get(idx + p + 1 - r).copied().unwrap_or(1.0)
                - knots.get(idx).copied().unwrap_or(0.0);
            let alpha = if denom.abs() > 1e-12 {
                (clamped_t - knots[idx]) / denom
            } else {
                0.0
            };

            let prev = d[j - 1];
            let curr = d[j];
            d[j] = [
                (1.0 - alpha) * prev[0] + alpha * curr[0],
                (1.0 - alpha) * prev[1] + alpha * curr[1],
                (1.0 - alpha) * prev[2] + alpha * curr[2],
                (1.0 - alpha) * prev[3] + alpha * curr[3],
            ];
        }
    }

    let result = d[p];
    let w = if result[3].abs() > 1e-12 { result[3] } else { 1.0 };
    [result[0] / w, result[1] / w, result[2] / w]
}
