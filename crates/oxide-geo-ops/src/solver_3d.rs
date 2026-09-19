//! 3D Variational Geometric & Assembly Constraint Solver.
//!
//! Uses unit Dual Quaternions on SE(3) Riemannian manifolds, analytical and numerical
//! Jacobians, and Levenberg-Marquardt damping powered by `faer`.

use faer::{Mat, prelude::*};
use serde::{Deserialize, Serialize};

/// Unit Dual Quaternion representing a rigid body pose in SE(3).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DualQuaternion {
    /// Real quaternion [w, x, y, z] representing rotation.
    pub real: [f64; 4],
    /// Dual quaternion [w, x, y, z] representing translation.
    pub dual: [f64; 4],
}

impl Default for DualQuaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl DualQuaternion {
    /// Identity pose (zero translation, zero rotation).
    pub const fn identity() -> Self {
        Self {
            real: [1.0, 0.0, 0.0, 0.0],
            dual: [0.0, 0.0, 0.0, 0.0],
        }
    }

    /// Construct from translation vector [tx, ty, tz] and unit quaternion [qw, qx, qy, qz].
    pub fn from_translation_rotation(t: [f64; 3], q: [f64; 4]) -> Self {
        let real = q;
        let dual = [
            -0.5 * (t[0] * q[1] + t[1] * q[2] + t[2] * q[3]),
            0.5 * (t[0] * q[0] + t[1] * q[3] - t[2] * q[2]),
            0.5 * (-t[0] * q[3] + t[1] * q[0] + t[2] * q[1]),
            0.5 * (t[0] * q[2] - t[1] * q[1] + t[2] * q[0]),
        ];
        Self { real, dual }
    }

    /// Construct from pure translation [tx, ty, tz].
    pub fn from_translation(t: [f64; 3]) -> Self {
        Self::from_translation_rotation(t, [1.0, 0.0, 0.0, 0.0])
    }

    /// Construct from rotation axis [ax, ay, az] and angle in radians.
    pub fn from_axis_angle(axis: [f64; 3], angle_rad: f64) -> Self {
        let half = angle_rad * 0.5;
        let s = half.sin();
        let norm = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let [ax, ay, az] = if norm > 1e-12 {
            [axis[0] / norm, axis[1] / norm, axis[2] / norm]
        } else {
            [0.0, 0.0, 1.0]
        };
        let q = [half.cos(), ax * s, ay * s, az * s];
        Self::from_translation_rotation([0.0, 0.0, 0.0], q)
    }

    /// Extract translation vector [tx, ty, tz].
    pub fn translation(&self) -> [f64; 3] {
        let rw = self.real[0];
        let rx = self.real[1];
        let ry = self.real[2];
        let rz = self.real[3];

        let dw = self.dual[0];
        let dx = self.dual[1];
        let dy = self.dual[2];
        let dz = self.dual[3];

        [
            2.0 * (-dw * rx + dx * rw - dy * rz + dz * ry),
            2.0 * (-dw * ry + dx * rz + dy * rw - dz * rx),
            2.0 * (-dw * rz - dx * ry + dy * rx + dz * rw),
        ]
    }

    /// Transform a 3D point using this dual quaternion.
    pub fn transform_point(&self, p: [f64; 3]) -> [f64; 3] {
        let t = self.translation();
        let [qw, qx, qy, qz] = self.real;
        let px = p[0];
        let py = p[1];
        let pz = p[2];

        let ix = qw * px + qy * pz - qz * py;
        let iy = qw * py + qz * px - qx * pz;
        let iz = qw * pz + qx * py - qy * px;
        let iw = -qx * px - qy * py - qz * pz;

        [
            ix * qw + iw * -qx + iy * -qz - iz * -qy + t[0],
            iy * qw + iw * -qy + iz * -qx - ix * -qz + t[1],
            iz * qw + iw * -qz + ix * -qy - iy * -qx + t[2],
        ]
    }

    /// Transform a 3D direction vector (rotation only).
    pub fn transform_vector(&self, v: [f64; 3]) -> [f64; 3] {
        let [qw, qx, qy, qz] = self.real;
        let [vx, vy, vz] = v;

        let ix = qw * vx + qy * vz - qz * vy;
        let iy = qw * vy + qz * vx - qx * vz;
        let iz = qw * vz + qx * vy - qy * px_dummy(vx, vy);
        let iw = -qx * vx - qy * vy - qz * vz;

        [
            ix * qw + iw * -qx + iy * -qz - iz * -qy,
            iy * qw + iw * -qy + iz * -qx - ix * -qz,
            iz * qw + iw * -qz + (qw * vz + qx * vy - qy * vx) * -qy - (qw * vy + qz * vx - qx * vz) * -qx,
        ]
    }

    /// Retract tangent vector xi in se(3) to SE(3) manifold via exponential map.
    pub fn retract(&self, xi: &[f64; 6]) -> Self {
        let w = [xi[0], xi[1], xi[2]];
        let v = [xi[3], xi[4], xi[5]];

        let theta_sq = w[0] * w[0] + w[1] * w[1] + w[2] * w[2];
        let theta = theta_sq.sqrt();

        let (sin_half, cos_half) = if theta < 1e-8 {
            (0.5, 1.0)
        } else {
            ((theta * 0.5).sin() / theta, (theta * 0.5).cos())
        };

        let delta_r = [cos_half, w[0] * sin_half, w[1] * sin_half, w[2] * sin_half];
        let delta_dq = Self::from_translation_rotation(v, delta_r);

        self.multiply(&delta_dq)
    }

    /// Dual quaternion multiplication: self * other.
    pub fn multiply(&self, other: &Self) -> Self {
        let [aw, ax, ay, az] = self.real;
        let [bw, bx, by, bz] = other.real;

        let real = [
            aw * bw - ax * bx - ay * by - az * bz,
            aw * bx + ax * bw + ay * bz - az * by,
            aw * by - ax * bz + ay * bw + az * bx,
            aw * bz + ax * by - ay * bx + az * bw,
        ];

        let [adw, adx, ady, adz] = self.dual;
        let [bdw, bdx, bdy, bdz] = other.dual;

        let dual = [
            aw * bdw - ax * bdx - ay * bdy - az * bdz + adw * bw - adx * bx - ady * by - adz * bz,
            aw * bdx + ax * bdw + ay * bdz - az * bdy + adw * bx + adx * bw + ady * bz - adz * by,
            aw * bdy - ax * bdz + ay * bdw + az * bdx + adw * by - adx * bz + ady * bw + adz * bx,
            aw * bdz + ax * bdy - ay * bdx + az * bdw + adw * bz + adx * by - ady * bx + adz * bw,
        ];

        // Normalize real quaternion to maintain unit length
        let r_norm_sq = real[0] * real[0] + real[1] * real[1] + real[2] * real[2] + real[3] * real[3];
        if r_norm_sq > 1e-12 {
            let inv_norm = 1.0 / r_norm_sq.sqrt();
            Self {
                real: [real[0] * inv_norm, real[1] * inv_norm, real[2] * inv_norm, real[3] * inv_norm],
                dual: [dual[0] * inv_norm, dual[1] * inv_norm, dual[2] * inv_norm, dual[3] * inv_norm],
            }
        } else {
            Self { real, dual }
        }
    }
}

#[inline(always)]
fn px_dummy(vx: f64, _vy: f64) -> f64 {
    vx
}

/// 3D Geometric Constraint between entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constraint3D {
    /// Lock body pose to fixed world coordinates.
    FixBody {
        body: usize,
        target: DualQuaternion,
    },
    /// Point-on-Point coincidence constraint (removes 3 DOF).
    PointCoincident {
        body_a: usize,
        local_point_a: [f64; 3],
        body_b: usize,
        local_point_b: [f64; 3],
    },
    /// Coaxial / Concentric constraint (aligns axis and ensures collinear points, removes 4 DOF).
    Concentric {
        body_a: usize,
        local_axis_a: [f64; 3],
        local_point_a: [f64; 3],
        body_b: usize,
        local_axis_b: [f64; 3],
        local_point_b: [f64; 3],
    },
    /// Parallel axes constraint (removes 2 DOF).
    ParallelAxes {
        body_a: usize,
        local_axis_a: [f64; 3],
        body_b: usize,
        local_axis_b: [f64; 3],
    },
    /// Perpendicular axes constraint (dot product = 0, removes 1 DOF).
    PerpendicularAxes {
        body_a: usize,
        local_axis_a: [f64; 3],
        body_b: usize,
        local_axis_b: [f64; 3],
    },
    /// Fixed Euclidean distance between two points (removes 1 DOF).
    Distance {
        body_a: usize,
        local_point_a: [f64; 3],
        body_b: usize,
        local_point_b: [f64; 3],
        target_distance: f64,
    },
    /// Fixed Angle in radians between two direction axes (removes 1 DOF).
    Angle {
        body_a: usize,
        local_axis_a: [f64; 3],
        body_b: usize,
        local_axis_b: [f64; 3],
        target_angle_rad: f64,
    },
}

/// Status of the constraint system.
#[derive(Debug, Clone, PartialEq)]
pub enum SolverStatus {
    /// Solved within tolerance.
    Converged { iterations: usize, final_residual: f64 },
    /// Residual failed to drop below tolerance within max iterations.
    MaxIterationsReached { iterations: usize, final_residual: f64 },
    /// Overconstrained conflicting constraint detected.
    OverConstrainedConflict { residual_norm: f64 },
}

/// Configuration for Levenberg-Marquardt solver.
#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub initial_damping: f64,
}

impl Default for SolverConfig {
    fn default() -> Self {
        Self {
            max_iterations: 60,
            tolerance: 1e-7,
            initial_damping: 1e-3,
        }
    }
}

/// Variational 3D Assembly & Sketch Constraint Solver.
#[derive(Debug, Clone, Default)]
pub struct ConstraintSolver3D {
    pub poses: Vec<DualQuaternion>,
    pub is_fixed: Vec<bool>,
    pub constraints: Vec<Constraint3D>,
}

impl ConstraintSolver3D {
    /// Create a new 3D constraint solver.
    pub fn new() -> Self {
        Self {
            poses: Vec::new(),
            is_fixed: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Add a body with initial pose and fixed status.
    pub fn add_body(&mut self, initial_pose: DualQuaternion, fixed: bool) -> usize {
        let idx = self.poses.len();
        self.poses.push(initial_pose);
        self.is_fixed.push(fixed);
        idx
    }

    /// Add a 3D geometric constraint.
    pub fn add_constraint(&mut self, constraint: Constraint3D) {
        self.constraints.push(constraint);
    }

    /// Count total scalar residual equations across all constraints.
    pub fn count_equations(&self) -> usize {
        let mut count = 0;
        for c in &self.constraints {
            match c {
                Constraint3D::FixBody { .. } => count += 6,
                Constraint3D::PointCoincident { .. } => count += 3,
                Constraint3D::Concentric { .. } => count += 5,
                Constraint3D::ParallelAxes { .. } => count += 2,
                Constraint3D::PerpendicularAxes { .. } => count += 1,
                Constraint3D::Distance { .. } => count += 1,
                Constraint3D::Angle { .. } => count += 1,
            }
        }
        count
    }

    /// Evaluate scalar residual vector across all constraints.
    pub fn compute_residuals(&self) -> Vec<f64> {
        let mut residuals = Vec::new();
        for c in &self.constraints {
            match c {
                Constraint3D::FixBody { body, target } => {
                    let cur_t = self.poses[*body].translation();
                    let tgt_t = target.translation();
                    residuals.push(cur_t[0] - tgt_t[0]);
                    residuals.push(cur_t[1] - tgt_t[1]);
                    residuals.push(cur_t[2] - tgt_t[2]);

                    let cr = self.poses[*body].real;
                    let tr = target.real;
                    // Rotation difference residuals
                    residuals.push(cr[1] - tr[1]);
                    residuals.push(cr[2] - tr[2]);
                    residuals.push(cr[3] - tr[3]);
                }
                Constraint3D::PointCoincident { body_a, local_point_a, body_b, local_point_b } => {
                    let pa = self.poses[*body_a].transform_point(*local_point_a);
                    let pb = self.poses[*body_b].transform_point(*local_point_b);
                    residuals.push(pa[0] - pb[0]);
                    residuals.push(pa[1] - pb[1]);
                    residuals.push(pa[2] - pb[2]);
                }
                Constraint3D::Distance { body_a, local_point_a, body_b, local_point_b, target_distance } => {
                    let pa = self.poses[*body_a].transform_point(*local_point_a);
                    let pb = self.poses[*body_b].transform_point(*local_point_b);
                    let dx = pa[0] - pb[0];
                    let dy = pa[1] - pb[1];
                    let dz = pa[2] - pb[2];
                    let current_dist = (dx * dx + dy * dy + dz * dz).sqrt();
                    residuals.push(current_dist - *target_distance);
                }
                Constraint3D::ParallelAxes { body_a, local_axis_a, body_b, local_axis_b } => {
                    let va = self.poses[*body_a].transform_vector(*local_axis_a);
                    let vb = self.poses[*body_b].transform_vector(*local_axis_b);
                    // Cross product components (2 independent constraints for unit vectors)
                    residuals.push(va[1] * vb[2] - va[2] * vb[1]);
                    residuals.push(va[2] * vb[0] - va[0] * vb[2]);
                }
                Constraint3D::PerpendicularAxes { body_a, local_axis_a, body_b, local_axis_b } => {
                    let va = self.poses[*body_a].transform_vector(*local_axis_a);
                    let vb = self.poses[*body_b].transform_vector(*local_axis_b);
                    // Dot product must equal 0
                    residuals.push(va[0] * vb[0] + va[1] * vb[1] + va[2] * vb[2]);
                }
                Constraint3D::Angle { body_a, local_axis_a, body_b, local_axis_b, target_angle_rad } => {
                    let va = self.poses[*body_a].transform_vector(*local_axis_a);
                    let vb = self.poses[*body_b].transform_vector(*local_axis_b);
                    let dot = (va[0] * vb[0] + va[1] * vb[1] + va[2] * vb[2]).clamp(-1.0, 1.0);
                    residuals.push(dot - target_angle_rad.cos());
                }
                Constraint3D::Concentric { body_a, local_axis_a, local_point_a, body_b, local_axis_b, local_point_b } => {
                    let va = self.poses[*body_a].transform_vector(*local_axis_a);
                    let vb = self.poses[*body_b].transform_vector(*local_axis_b);
                    residuals.push(va[1] * vb[2] - va[2] * vb[1]);
                    residuals.push(va[2] * vb[0] - va[0] * vb[2]);

                    let pa = self.poses[*body_a].transform_point(*local_point_a);
                    let pb = self.poses[*body_b].transform_point(*local_point_b);
                    let d = [pa[0] - pb[0], pa[1] - pb[1], pa[2] - pb[2]];
                    // Distance perpendicular to axis must be zero: d x va = 0
                    residuals.push(d[1] * va[2] - d[2] * va[1]);
                    residuals.push(d[2] * va[0] - d[0] * va[2]);
                    residuals.push(d[0] * va[1] - d[1] * va[0]);
                }
            }
        }
        residuals
    }

    /// Solve the 3D constraint system using Levenberg-Marquardt with adaptive damping.
    pub fn solve(&mut self, config: &SolverConfig) -> SolverStatus {
        let num_bodies = self.poses.len();
        let num_dofs = num_bodies * 6;
        let mut lambda = config.initial_damping;

        for iter in 0..config.max_iterations {
            let res = self.compute_residuals();
            let m = res.len();
            if m == 0 {
                return SolverStatus::Converged { iterations: iter, final_residual: 0.0 };
            }

            let residual_norm_sq: f64 = res.iter().map(|r| r * r).sum();
            let residual_norm = residual_norm_sq.sqrt();
            if residual_norm < config.tolerance {
                return SolverStatus::Converged { iterations: iter, final_residual: residual_norm };
            }

            // Numerical Jacobian computation (m x num_dofs) with central finite difference
            let mut j_mat = Mat::<f64>::zeros(m, num_dofs);
            let eps = 1e-7;

            for b in 0..num_bodies {
                if self.is_fixed[b] {
                    continue;
                }
                for dof in 0..6 {
                    let mut xi_pos = [0.0; 6];
                    let mut xi_neg = [0.0; 6];
                    xi_pos[dof] = eps;
                    xi_neg[dof] = -eps;

                    let orig_pose = self.poses[b];
                    self.poses[b] = orig_pose.retract(&xi_pos);
                    let res_pos = self.compute_residuals();

                    self.poses[b] = orig_pose.retract(&xi_neg);
                    let res_neg = self.compute_residuals();

                    self.poses[b] = orig_pose;

                    for i in 0..m {
                        j_mat[(i, b * 6 + dof)] = (res_pos[i] - res_neg[i]) / (2.0 * eps);
                    }
                }
            }

            // Normal equations: (J^T * J + lambda * diag(J^T * J)) * delta = -J^T * res
            let jt = j_mat.transpose();
            let mut jtj = &jt * &j_mat;

            for i in 0..num_dofs {
                jtj[(i, i)] += lambda * (1.0 + jtj[(i, i)].max(1.0));
            }

            let mut rhs = Mat::<f64>::zeros(num_dofs, 1);
            for i in 0..num_dofs {
                let mut sum = 0.0;
                for k in 0..m {
                    sum += jt[(i, k)] * res[k];
                }
                rhs[(i, 0)] = -sum;
            }

            // Solve normal equations via faer Cholesky
            let chol = match jtj.cholesky(faer::Side::Lower) {
                Ok(c) => c,
                Err(_) => {
                    lambda *= 10.0;
                    continue;
                }
            };
            let delta = chol.solve(&rhs);

            // Trial update
            let mut trial_poses = self.poses.clone();
            for b in 0..num_bodies {
                if self.is_fixed[b] {
                    continue;
                }
                let mut xi = [0.0; 6];
                for dof in 0..6 {
                    xi[dof] = delta[(b * 6 + dof, 0)];
                }
                trial_poses[b] = trial_poses[b].retract(&xi);
            }

            // Evaluate trial residual
            let old_poses = std::mem::replace(&mut self.poses, trial_poses);
            let trial_res = self.compute_residuals();
            let trial_norm_sq: f64 = trial_res.iter().map(|r| r * r).sum();

            if trial_norm_sq < residual_norm_sq {
                // Step accepted
                lambda = (lambda * 0.3).max(1e-9);
            } else {
                // Reject step, increase damping
                self.poses = old_poses;
                lambda = (lambda * 10.0).min(1e8);
            }
        }

        let final_res = self.compute_residuals();
        let final_residual: f64 = final_res.iter().map(|r| r * r).sum::<f64>().sqrt();
        if final_residual < config.tolerance {
            SolverStatus::Converged { iterations: config.max_iterations, final_residual }
        } else {
            SolverStatus::MaxIterationsReached { iterations: config.max_iterations, final_residual }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_coincidence_solve() {
        let mut solver = ConstraintSolver3D::new();

        // Body 0: Fixed at origin
        let b0 = solver.add_body(DualQuaternion::identity(), true);

        // Body 1: Free, initial position at [5.0, 5.0, 5.0]
        let initial_pose_1 = DualQuaternion::from_translation([5.0, 5.0, 5.0]);
        let b1 = solver.add_body(initial_pose_1, false);

        // Constraint: Point [0,0,0] on body 1 must coincide with [10, 0, 0] on body 0
        solver.add_constraint(Constraint3D::PointCoincident {
            body_a: b0,
            local_point_a: [10.0, 0.0, 0.0],
            body_b: b1,
            local_point_b: [0.0, 0.0, 0.0],
        });

        let config = SolverConfig::default();
        let status = solver.solve(&config);

        match status {
            SolverStatus::Converged { iterations, final_residual } => {
                assert!(final_residual < 1e-6);
                assert!(iterations < 20);
                let t1 = solver.poses[b1].translation();
                assert!((t1[0] - 10.0).abs() < 1e-4);
                assert!(t1[1].abs() < 1e-4);
                assert!(t1[2].abs() < 1e-4);
            }
            other => panic!("Expected convergence, got {:?}", other),
        }
    }

    #[test]
    fn test_distance_and_parallel_axes() {
        let mut solver = ConstraintSolver3D::new();

        let b0 = solver.add_body(DualQuaternion::identity(), true);
        let b1 = solver.add_body(DualQuaternion::from_translation([2.0, 1.0, 0.0]), false);

        // Distance constraint of 15.0 mm between body origins
        solver.add_constraint(Constraint3D::Distance {
            body_a: b0,
            local_point_a: [0.0, 0.0, 0.0],
            body_b: b1,
            local_point_b: [0.0, 0.0, 0.0],
            target_distance: 15.0,
        });

        // Parallel Z axes
        solver.add_constraint(Constraint3D::ParallelAxes {
            body_a: b0,
            local_axis_a: [0.0, 0.0, 1.0],
            body_b: b1,
            local_axis_b: [0.0, 0.0, 1.0],
        });

        let config = SolverConfig::default();
        let status = solver.solve(&config);

        match status {
            SolverStatus::Converged { final_residual, .. } => {
                assert!(final_residual < 1e-6);
                let t = solver.poses[b1].translation();
                let dist = (t[0] * t[0] + t[1] * t[1] + t[2] * t[2]).sqrt();
                assert!((dist - 15.0).abs() < 1e-4);
            }
            other => panic!("Expected convergence, got {:?}", other),
        }
    }
}
