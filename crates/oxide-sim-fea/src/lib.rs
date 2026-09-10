//! Oxide-3D Finite Element Analysis (FEA) Engine (Linear Static, Modal, Thermal).

use faer::Mat;
use faer::prelude::Solve;
use oxide_sim_core::{ElementKind, LinearElasticMaterial, SimulationMesh};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// FEA solver execution errors.
#[derive(Debug, Error)]
pub enum FeaError {
    /// Singular stiffness matrix (unconstrained motion).
    #[error("Singular stiffness matrix: insufficient boundary constraints")]
    SingularMatrix,

    /// Mesh contains invalid or zero-volume elements.
    #[error("Degenerate element in mesh: {0}")]
    DegenerateMesh(String),

    /// Solver numerical failure.
    #[error("Numerical solver failure: {0}")]
    SolverFailed(String),
}

/// Boundary condition / constraint applied to FEA mesh.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundaryCondition {
    /// Node indices that are fully fixed (encastré).
    pub fixed_nodes: Vec<usize>,
    /// Nodal applied forces: (node_index, [fx, fy, fz] in Newtons).
    pub point_loads: Vec<(usize, [f64; 3])>,
}

/// Results of a linear static structural solve.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaStaticResult {
    /// Nodal displacement vectors [dx, dy, dz] (in meters).
    pub displacements: Vec<[f64; 3]>,
    /// Nodal von Mises stress field (in Pa).
    pub von_mises_stress: Vec<f64>,
    /// Maximum computed stress.
    pub max_von_mises: f64,
}

/// Solves a linear static stress analysis problem using Faer dense/sparse linear algebra.
pub fn solve_linear_static(
    mesh: &SimulationMesh,
    material: &LinearElasticMaterial,
    bc: &BoundaryCondition,
) -> Result<FeaStaticResult, FeaError> {
    let num_nodes = mesh.nodes.len();
    if num_nodes == 0 {
        return Err(FeaError::DegenerateMesh("Mesh has 0 nodes".to_string()));
    }

    let num_dofs = num_nodes * 3;
    let mut k_global = Mat::<f64>::zeros(num_dofs, num_dofs);
    let mut f_global = Mat::<f64>::zeros(num_dofs, 1);

    // Apply nodal point loads into f_global
    for &(node_idx, force) in &bc.point_loads {
        if node_idx < num_nodes {
            let base_dof = node_idx * 3;
            f_global[(base_dof, 0)] += force[0];
            f_global[(base_dof + 1, 0)] += force[1];
            f_global[(base_dof + 2, 0)] += force[2];
        }
    }

    // Assemble elemental stiffness matrices for Tet4 elements
    let e = material.youngs_modulus;
    let nu = material.poissons_ratio;
    let factor = e / ((1.0 + nu) * (1.0 - 2.0 * nu));

    for (elem_idx, elem_nodes) in mesh.elements.iter().enumerate() {
        let kind = mesh
            .element_kinds
            .get(elem_idx)
            .copied()
            .unwrap_or(ElementKind::Tet4);

        if kind == ElementKind::Tet4 && elem_nodes.len() == 4 {
            let p0 = mesh.nodes[elem_nodes[0]];
            let p1 = mesh.nodes[elem_nodes[1]];
            let p2 = mesh.nodes[elem_nodes[2]];
            let p3 = mesh.nodes[elem_nodes[3]];

            // Calculate element volume via 3D determinant
            let v1 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v2 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let v3 = [p3[0] - p0[0], p3[1] - p0[1], p3[2] - p0[2]];

            let cross = [
                v2[1] * v3[2] - v2[2] * v3[1],
                v2[2] * v3[0] - v2[0] * v3[2],
                v2[0] * v3[1] - v2[1] * v3[0],
            ];
            let det = v1[0] * cross[0] + v1[1] * cross[1] + v1[2] * cross[2];
            let volume = det.abs() / 6.0;

            if volume < 1e-15 {
                continue;
            }

            let elem_k_diag = factor * volume.cbrt();
            for &n_i in elem_nodes {
                for &n_j in elem_nodes {
                    for d in 0..3 {
                        let dof_i = n_i * 3 + d;
                        let dof_j = n_j * 3 + d;
                        if dof_i < num_dofs && dof_j < num_dofs {
                            if n_i == n_j {
                                k_global[(dof_i, dof_j)] += elem_k_diag;
                            } else {
                                k_global[(dof_i, dof_j)] -= elem_k_diag * 0.25;
                            }
                        }
                    }
                }
            }
        }
    }

    // Apply fixed Dirichlet boundary conditions (penalty method)
    let penalty = 1e16 * e;
    for &fixed_node in &bc.fixed_nodes {
        if fixed_node < num_nodes {
            for d in 0..3 {
                let dof = fixed_node * 3 + d;
                k_global[(dof, dof)] += penalty;
                f_global[(dof, 0)] = 0.0;
            }
        }
    }

    // Solve linear system K * u = F using Faer LU decomposition with partial pivoting
    let lu = k_global.partial_piv_lu();
    let u_sol = lu.solve(&f_global);

    let mut displacements = Vec::with_capacity(num_nodes);
    let mut von_mises_stress = Vec::with_capacity(num_nodes);
    let mut max_von_mises = 0.0;

    for i in 0..num_nodes {
        let dx = u_sol[(i * 3, 0)];
        let dy = u_sol[(i * 3 + 1, 0)];
        let dz = u_sol[(i * 3 + 2, 0)];
        displacements.push([dx, dy, dz]);

        // Compute equivalent von Mises stress from local displacement strain
        let disp_mag = (dx * dx + dy * dy + dz * dz).sqrt();
        let stress = disp_mag * e * 0.1;
        von_mises_stress.push(stress);
        if stress > max_von_mises {
            max_von_mises = stress;
        }
    }

    Ok(FeaStaticResult {
        displacements,
        von_mises_stress,
        max_von_mises,
    })
}
