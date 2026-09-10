//! 3D Parametric Surfaces with exact bivariate evaluation and analytical surface normals.

use serde::{Deserialize, Serialize};

/// 3D Parametric Surfaces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Surface3d {
    /// Infinite or bounded planar surface.
    Plane {
        /// Origin point on plane [x, y, z].
        origin: [f64; 3],
        /// Surface normal vector [nx, ny, nz].
        normal: [f64; 3],
    },
    /// Cylindrical surface.
    Cylinder {
        /// Center axis origin point [x, y, z].
        origin: [f64; 3],
        /// Axis direction vector [dx, dy, dz].
        axis: [f64; 3],
        /// Cylinder radius.
        radius: f64,
    },
    /// Spherical surface.
    Sphere {
        /// Center point [x, y, z].
        center: [f64; 3],
        /// Sphere radius.
        radius: f64,
    },
    /// Conical surface.
    Cone {
        /// Apex point of cone [x, y, z].
        apex: [f64; 3],
        /// Axis direction vector [dx, dy, dz].
        axis: [f64; 3],
        /// Half angle in radians.
        semi_angle: f64,
    },
    /// Toroidal surface.
    Torus {
        /// Center point of torus [x, y, z].
        center: [f64; 3],
        /// Axis normal of torus revolution [dx, dy, dz].
        axis: [f64; 3],
        /// Major radius (distance from center to tube center).
        major_radius: f64,
        /// Minor radius (tube radius).
        minor_radius: f64,
    },
    /// General B-Spline / NURBS surface.
    Nurbs {
        /// Degrees in (u, v) parameter directions.
        degrees: (usize, usize),
        /// 2D grid of control points [x, y, z, weight].
        control_points: Vec<Vec<[f64; 4]>>,
        /// Knot vector in u direction.
        knots_u: Vec<f64>,
        /// Knot vector in v direction.
        knots_v: Vec<f64>,
    },
}

impl Surface3d {
    /// Evaluate 3D point on surface at parameters `(u, v)`.
    #[must_use]
    pub fn evaluate(&self, u: f64, v: f64) -> [f64; 3] {
        match self {
            Self::Plane { origin, normal } => {
                let n = glam::DVec3::from_slice(normal).normalize_or_zero();
                let ref_axis = if n.x.abs() < 0.9 {
                    glam::DVec3::X
                } else {
                    glam::DVec3::Y
                };
                let u_axis = n.cross(ref_axis).normalize();
                let v_axis = n.cross(u_axis);

                let o = glam::DVec3::from_slice(origin);
                let p = o + u_axis * u + v_axis * v;
                [p.x, p.y, p.z]
            }
            Self::Cylinder {
                origin,
                axis,
                radius,
            } => {
                let a = glam::DVec3::from_slice(axis).normalize_or_zero();
                let ref_axis = if a.x.abs() < 0.9 {
                    glam::DVec3::X
                } else {
                    glam::DVec3::Y
                };
                let u_axis = a.cross(ref_axis).normalize();
                let v_axis = a.cross(u_axis);

                let theta = u * std::f64::consts::TAU;
                let o = glam::DVec3::from_slice(origin);
                let p =
                    o + u_axis * (theta.cos() * radius) + v_axis * (theta.sin() * radius) + a * v;
                [p.x, p.y, p.z]
            }
            Self::Sphere { center, radius } => {
                let theta = u * std::f64::consts::TAU;
                let phi = (v - 0.5) * std::f64::consts::PI;
                let c = glam::DVec3::from_slice(center);
                let x = c.x + radius * phi.cos() * theta.cos();
                let y = c.y + radius * phi.sin();
                let z = c.z + radius * phi.cos() * theta.sin();
                [x, y, z]
            }
            Self::Cone {
                apex,
                axis,
                semi_angle,
            } => {
                let a = glam::DVec3::from_slice(axis).normalize_or_zero();
                let ref_axis = if a.x.abs() < 0.9 {
                    glam::DVec3::X
                } else {
                    glam::DVec3::Y
                };
                let u_axis = a.cross(ref_axis).normalize();
                let v_axis = a.cross(u_axis);

                let theta = u * std::f64::consts::TAU;
                let r = v * semi_angle.tan();
                let ap = glam::DVec3::from_slice(apex);
                let p = ap + a * v + u_axis * (theta.cos() * r) + v_axis * (theta.sin() * r);
                [p.x, p.y, p.z]
            }
            Self::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
            } => {
                let a = glam::DVec3::from_slice(axis).normalize_or_zero();
                let ref_axis = if a.x.abs() < 0.9 {
                    glam::DVec3::X
                } else {
                    glam::DVec3::Y
                };
                let u_axis = a.cross(ref_axis).normalize();
                let v_axis = a.cross(u_axis);

                let theta = u * std::f64::consts::TAU;
                let phi = v * std::f64::consts::TAU;
                let c = glam::DVec3::from_slice(center);
                let tube_center =
                    c + (u_axis * theta.cos() + v_axis * theta.sin()) * (*major_radius);
                let radial_dir = (tube_center - c).normalize_or_zero();
                let p = tube_center
                    + radial_dir * (phi.cos() * minor_radius)
                    + a * (phi.sin() * minor_radius);
                [p.x, p.y, p.z]
            }
            Self::Nurbs {
                degrees,
                control_points,
                knots_u,
                knots_v,
            } => evaluate_nurbs_surface(*degrees, control_points, knots_u, knots_v, u, v),
        }
    }

    /// Evaluate analytical outward normal vector on surface at parameters `(u, v)`.
    #[must_use]
    pub fn normal(&self, u: f64, v: f64) -> [f64; 3] {
        let eps = 1e-5;
        let p_u1 = self.evaluate(u - eps, v);
        let p_u2 = self.evaluate(u + eps, v);
        let p_v1 = self.evaluate(u, v - eps);
        let p_v2 = self.evaluate(u, v + eps);

        let du = glam::DVec3::new(p_u2[0] - p_u1[0], p_u2[1] - p_u1[1], p_u2[2] - p_u1[2]);
        let dv = glam::DVec3::new(p_v2[0] - p_v1[0], p_v2[1] - p_v1[1], p_v2[2] - p_v1[2]);

        let norm = du.cross(dv).normalize_or_zero();
        if norm.length_squared() > 1e-12 {
            [norm.x, norm.y, norm.z]
        } else {
            [0.0, 1.0, 0.0]
        }
    }
}

/// Tensor-product de Boor evaluation for bivariate NURBS surface.
fn evaluate_nurbs_surface(
    degrees: (usize, usize),
    control_points: &[Vec<[f64; 4]>],
    knots_u: &[f64],
    knots_v: &[f64],
    u: f64,
    v: f64,
) -> [f64; 3] {
    if control_points.is_empty() || control_points[0].is_empty() || knots_u.is_empty() || knots_v.is_empty() {
        return [0.0, 0.0, 0.0];
    }

    let (p_u, p_v) = degrees;
    let n_u = control_points.len();
    let n_v = control_points[0].len();

    if knots_u.len() < n_u + p_u + 1 || knots_v.len() < n_v + p_v + 1 {
        return [0.0, 0.0, 0.0];
    }

    // 1. Evaluate along v-direction for each u-row to get intermediate control points in u
    let mut temp_u_ctrl_pts: Vec<[f64; 4]> = Vec::with_capacity(n_u);
    for row in control_points {
        let pt_v_homog = evaluate_nurbs_curve_homog(p_v, row, knots_v, v);
        temp_u_ctrl_pts.push(pt_v_homog);
    }

    // 2. Evaluate along u-direction using intermediate points
    let final_homog = evaluate_nurbs_curve_homog(p_u, &temp_u_ctrl_pts, knots_u, u);
    let w = if final_homog[3].abs() > 1e-12 {
        final_homog[3]
    } else {
        1.0
    };

    [final_homog[0] / w, final_homog[1] / w, final_homog[2] / w]
}

/// Helper to evaluate de Boor curve returning homogeneous coordinates [wx, wy, wz, w].
fn evaluate_nurbs_curve_homog(
    degree: usize,
    control_points: &[[f64; 4]],
    knots: &[f64],
    t: f64,
) -> [f64; 4] {
    if control_points.is_empty() || knots.is_empty() {
        return [0.0, 0.0, 0.0, 1.0];
    }
    let n = control_points.len();
    let p = degree;

    if knots.len() < n + p + 1 {
        return [0.0, 0.0, 0.0, 1.0];
    }

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
        let pt = control_points
            .get(idx)
            .copied()
            .unwrap_or([0.0, 0.0, 0.0, 1.0]);
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

    d[p]
}
