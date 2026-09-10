//! 2D Geometric Sketch Constraint Solver using Newton-Raphson nonlinear Jacobian iterations.

use faer::Mat;
use faer::prelude::Solve;
use serde::{Deserialize, Serialize};

/// 2D Point parameter in sketch solver.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SketchPoint2d {
    /// X coordinate.
    pub x: f64,
    /// Y coordinate.
    pub y: f64,
}

/// Geometric constraint between 2D sketch entities.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SketchConstraint {
    /// Fix point position exactly.
    FixPoint {
        /// Point index.
        point: usize,
        /// Target X coordinate.
        target_x: f64,
        /// Target Y coordinate.
        target_y: f64,
    },
    /// Coincident constraint: point A must equal point B.
    Coincident {
        /// Point index A.
        p1: usize,
        /// Point index B.
        p2: usize,
    },
    /// Horizontal constraint: two points have same Y.
    Horizontal {
        /// Point index A.
        p1: usize,
        /// Point index B.
        p2: usize,
    },
    /// Vertical constraint: two points have same X.
    Vertical {
        /// Point index A.
        p1: usize,
        /// Point index B.
        p2: usize,
    },
    /// Fixed Euclidean distance between two points.
    Distance {
        /// Point index A.
        p1: usize,
        /// Point index B.
        p2: usize,
        /// Target distance.
        target_dist: f64,
    },
}

/// 2D Sketch constraint solver state.
#[derive(Debug, Clone, Default)]
pub struct SketchSolver {
    /// Parameter points [x0, y0, x1, y1, ...].
    pub points: Vec<SketchPoint2d>,
    /// Constraints applied on points.
    pub constraints: Vec<SketchConstraint>,
}

impl SketchSolver {
    /// Create a new empty sketch solver.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Add a 2D point and return its index.
    pub fn add_point(&mut self, x: f64, y: f64) -> usize {
        let idx = self.points.len();
        self.points.push(SketchPoint2d { x, y });
        idx
    }

    /// Add a constraint.
    pub fn add_constraint(&mut self, constraint: SketchConstraint) {
        self.constraints.push(constraint);
    }

    /// Count total scalar residual equations.
    fn count_equations(&self) -> usize {
        let mut count = 0;
        for c in &self.constraints {
            match c {
                SketchConstraint::FixPoint { .. } | SketchConstraint::Coincident { .. } => {
                    count += 2;
                }
                SketchConstraint::Horizontal { .. }
                | SketchConstraint::Vertical { .. }
                | SketchConstraint::Distance { .. } => {
                    count += 1;
                }
            }
        }
        count
    }

    /// Solve the sketch constraints using damped Newton-Raphson iteration.
    pub fn solve(&mut self, max_iterations: usize, tolerance: f64) -> Result<usize, String> {
        let n_vars = self.points.len() * 2;
        let n_eqs = self.count_equations();

        if n_vars == 0 || n_eqs == 0 {
            return Ok(0);
        }

        for iter in 0..max_iterations {
            let mut residuals = Mat::<f64>::zeros(n_eqs, 1);
            let mut jacobian = Mat::<f64>::zeros(n_eqs, n_vars);

            let mut eq_idx = 0;

            for c in &self.constraints {
                match *c {
                    SketchConstraint::FixPoint {
                        point,
                        target_x,
                        target_y,
                    } => {
                        let p = self.points[point];
                        residuals[(eq_idx, 0)] = p.x - target_x;
                        residuals[(eq_idx + 1, 0)] = p.y - target_y;

                        jacobian[(eq_idx, point * 2)] = 1.0;
                        jacobian[(eq_idx + 1, point * 2 + 1)] = 1.0;
                        eq_idx += 2;
                    }
                    SketchConstraint::Coincident { p1, p2 } => {
                        let pt1 = self.points[p1];
                        let pt2 = self.points[p2];
                        residuals[(eq_idx, 0)] = pt1.x - pt2.x;
                        residuals[(eq_idx + 1, 0)] = pt1.y - pt2.y;

                        jacobian[(eq_idx, p1 * 2)] = 1.0;
                        jacobian[(eq_idx, p2 * 2)] = -1.0;
                        jacobian[(eq_idx + 1, p1 * 2 + 1)] = 1.0;
                        jacobian[(eq_idx + 1, p2 * 2 + 1)] = -1.0;
                        eq_idx += 2;
                    }
                    SketchConstraint::Horizontal { p1, p2 } => {
                        let pt1 = self.points[p1];
                        let pt2 = self.points[p2];
                        residuals[(eq_idx, 0)] = pt1.y - pt2.y;

                        jacobian[(eq_idx, p1 * 2 + 1)] = 1.0;
                        jacobian[(eq_idx, p2 * 2 + 1)] = -1.0;
                        eq_idx += 1;
                    }
                    SketchConstraint::Vertical { p1, p2 } => {
                        let pt1 = self.points[p1];
                        let pt2 = self.points[p2];
                        residuals[(eq_idx, 0)] = pt1.x - pt2.x;

                        jacobian[(eq_idx, p1 * 2)] = 1.0;
                        jacobian[(eq_idx, p2 * 2)] = -1.0;
                        eq_idx += 1;
                    }
                    SketchConstraint::Distance {
                        p1,
                        p2,
                        target_dist,
                    } => {
                        let pt1 = self.points[p1];
                        let pt2 = self.points[p2];
                        let dx = pt2.x - pt1.x;
                        let dy = pt2.y - pt1.y;
                        let curr_dist = (dx * dx + dy * dy).sqrt();

                        residuals[(eq_idx, 0)] = curr_dist - target_dist;

                        if curr_dist > 1e-12 {
                            jacobian[(eq_idx, p1 * 2)] = -dx / curr_dist;
                            jacobian[(eq_idx, p1 * 2 + 1)] = -dy / curr_dist;
                            jacobian[(eq_idx, p2 * 2)] = dx / curr_dist;
                            jacobian[(eq_idx, p2 * 2 + 1)] = dy / curr_dist;
                        }
                        eq_idx += 1;
                    }
                }
            }

            // Check residual norm
            let mut max_res: f64 = 0.0;
            for i in 0..n_eqs {
                max_res = max_res.max(residuals[(i, 0)].abs());
            }

            if max_res < tolerance {
                return Ok(iter);
            }

            // Solve normal equations J^T * J * delta = - J^T * residuals with Levenberg-Marquardt regularization
            let jt = jacobian.transpose();
            let mut jtj = &jt * &jacobian;
            let jtr = &jt * &residuals;

            // Add Tikhonov damping lambda * I to diagonal
            let lambda = 1e-4;
            for i in 0..n_vars {
                jtj[(i, i)] += lambda;
            }

            let delta = jtj.partial_piv_lu().solve(&jtr);

            // Update point parameters: x = x - delta
            for (p_idx, pt) in self.points.iter_mut().enumerate() {
                pt.x -= delta[(p_idx * 2, 0)];
                pt.y -= delta[(p_idx * 2 + 1, 0)];
            }
        }

        Err(format!(
            "Sketch solver failed to converge in {max_iterations} iterations"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sketch_horizontal_distance_solver() {
        let mut solver = SketchSolver::new();

        // Point 0 at origin
        let p0 = solver.add_point(0.0, 0.0);
        // Point 1 roughly along X
        let p1 = solver.add_point(8.0, 3.0);

        // Fix Point 0 at (0, 0)
        solver.add_constraint(SketchConstraint::FixPoint {
            point: p0,
            target_x: 0.0,
            target_y: 0.0,
        });

        // Horizontal constraint between p0 and p1
        solver.add_constraint(SketchConstraint::Horizontal { p1: p0, p2: p1 });

        // Distance constraint of 10.0 mm
        solver.add_constraint(SketchConstraint::Distance {
            p1: p0,
            p2: p1,
            target_dist: 10.0,
        });

        let iterations = solver.solve(20, 1e-6).expect("Solver should converge");
        assert!(iterations < 20);

        let pt0 = solver.points.get(p0).expect("Point 0 exists");
        let pt1 = solver.points.get(p1).expect("Point 1 exists");

        assert!((pt0.x - 0.0).abs() < 1e-5);
        assert!((pt0.y - 0.0).abs() < 1e-5);
        assert!((pt1.x - 10.0).abs() < 1e-5);
        assert!((pt1.y - 0.0).abs() < 1e-5);
    }
}
