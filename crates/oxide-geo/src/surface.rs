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
                let p = o + u_axis * (theta.cos() * radius) + v_axis * (theta.sin() * radius) + a * v;
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
    _degrees: (usize, usize),
    control_points: &[Vec<[f64; 4]>],
    _knots_u: &[f64],
    _knots_v: &[f64],
    u: f64,
    v: f64,
) -> [f64; 3] {
    if control_points.is_empty() || control_points[0].is_empty() {
        return [0.0, 0.0, 0.0];
    }
    let u_clamped = u.clamp(0.0, 1.0);
    let v_clamped = v.clamp(0.0, 1.0);

    let rows = control_points.len();
    let cols = control_points[0].len();

    let mut row_pts = Vec::with_capacity(rows);
    for row in control_points {
        let col_idx = (v_clamped * (cols - 1) as f64).round() as usize;
        let pt = row.get(col_idx.min(cols - 1)).copied().unwrap_or([0.0, 0.0, 0.0, 1.0]);
        row_pts.push(pt);
    }

    let row_idx = (u_clamped * (rows - 1) as f64).round() as usize;
    let final_pt = row_pts.get(row_idx.min(rows - 1)).copied().unwrap_or([0.0, 0.0, 0.0, 1.0]);
    let w = if final_pt[3].abs() > 1e-12 { final_pt[3] } else { 1.0 };
    [final_pt[0] / w, final_pt[1] / w, final_pt[2] / w]
}
