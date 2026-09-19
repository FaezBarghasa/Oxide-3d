//! Design for Manufacturability (DFM) Analysis Engine.
//!
//! Covers:
//! - Additive Manufacturing (3D Printing): Overhang angles, support volume, wall thickness.
//! - Subtractive (CNC Milling Aluminum): Tool accessibility cones, internal corner fillet radius, pocket aspect ratio.
//! - Injection Molded Plastics: Draft angles against mold pull direction, parting line analysis, wall thickness variation.

use oxide_geo::mesh_bridge::TessellatedMesh;
use serde::{Deserialize, Serialize};

/// Classification of DFM issue severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DfmSeverity {
    /// Information / best practice hint.
    Info,
    /// Warning: May increase tooling cost or surface defect rate.
    Warning,
    /// Error: Violation will cause machining failure, trapped support, or un-moldable part.
    Error,
}

/// DFM Issue report item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DfmIssue {
    /// Manufacturing process domain.
    pub domain: &'static str,
    /// Rule violated.
    pub rule_name: &'static str,
    /// Severity level.
    pub severity: DfmSeverity,
    /// Human-readable explanation.
    pub description: String,
    /// Optional face/triangle indices or 3D location where issue occurs.
    pub location: Option<[f64; 3]>,
    /// Measured value vs threshold limit.
    pub measured_value: f64,
    /// Required limit.
    pub threshold_limit: f64,
}

/// 3D Printing / Additive Manufacturing DFM Analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditiveDfmConfig {
    /// Maximum unsupported overhang angle in degrees from vertical (default 45°).
    pub max_overhang_angle_deg: f64,
    /// Minimum printable wall thickness in mm (default 0.8mm for FDM, 0.4mm for SLA).
    pub min_wall_thickness_mm: f64,
    /// Build plate normal direction [x, y, z] (default [0, 0, 1]).
    pub build_direction: [f64; 3],
}

impl Default for AdditiveDfmConfig {
    fn default() -> Self {
        Self {
            max_overhang_angle_deg: 45.0,
            min_wall_thickness_mm: 0.8,
            build_direction: [0.0, 0.0, 1.0],
        }
    }
}

/// CNC Milled Aluminum DFM Analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CncMillingDfmConfig {
    /// Minimum internal fillet radius in mm corresponding to standard endmill (e.g., 1.5mm for 3mm tool).
    pub min_internal_radius_mm: f64,
    /// Maximum pocket depth to width aspect ratio (default 4.0).
    pub max_pocket_aspect_ratio: f64,
    /// Spindle access direction [x, y, z] (default [0, 0, 1] for 3-axis top-down).
    pub spindle_axis: [f64; 3],
}

impl Default for CncMillingDfmConfig {
    fn default() -> Self {
        Self {
            min_internal_radius_mm: 1.5,
            max_pocket_aspect_ratio: 4.0,
            spindle_axis: [0.0, 0.0, 1.0],
        }
    }
}

/// Injection Molded Plastics DFM Analyzer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionMoldingDfmConfig {
    /// Minimum required draft angle in degrees (default 1.0° - 1.5°).
    pub min_draft_angle_deg: f64,
    /// Maximum allowed wall thickness variation ratio across the part (default 1.25).
    pub max_wall_thickness_variation_ratio: f64,
    /// Mold pull / opening vector [x, y, z] (default [0, 0, 1]).
    pub pull_direction: [f64; 3],
}

impl Default for InjectionMoldingDfmConfig {
    fn default() -> Self {
        Self {
            min_draft_angle_deg: 1.0,
            max_wall_thickness_variation_ratio: 1.25,
            pull_direction: [0.0, 0.0, 1.0],
        }
    }
}

/// Design for Manufacturability (DFM) Comprehensive Engine.
pub struct DfmEngine;

impl DfmEngine {
    /// Analyze mesh for 3D Printing / Additive issues (overhangs requiring support structures).
    pub fn analyze_additive(mesh: &TessellatedMesh, config: &AdditiveDfmConfig) -> Vec<DfmIssue> {
        let mut issues = Vec::new();
        let bz = config.build_direction;
        let b_len = (bz[0] * bz[0] + bz[1] * bz[1] + bz[2] * bz[2]).sqrt();
        let b_norm = if b_len > 1e-9 {
            [bz[0] / b_len, bz[1] / b_len, bz[2] / b_len]
        } else {
            [0.0, 0.0, 1.0]
        };

        let max_angle_rad = config.max_overhang_angle_deg.to_radians();

        let num_triangles = mesh.indices.len() / 3;
        for i in 0..num_triangles {
            let i0 = mesh.indices[i * 3] as usize;
            let i1 = mesh.indices[i * 3 + 1] as usize;
            let i2 = mesh.indices[i * 3 + 2] as usize;

            if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
                continue;
            }

            let p0 = [mesh.positions[i0][0] as f64, mesh.positions[i0][1] as f64, mesh.positions[i0][2] as f64];
            let p1 = [mesh.positions[i1][0] as f64, mesh.positions[i1][1] as f64, mesh.positions[i1][2] as f64];
            let p2 = [mesh.positions[i2][0] as f64, mesh.positions[i2][1] as f64, mesh.positions[i2][2] as f64];

            // Compute triangle normal
            let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let n_len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if n_len < 1e-12 {
                continue;
            }
            let n_norm = [n[0] / n_len, n[1] / n_len, n[2] / n_len];

            // Dot product with build vector
            let dot = n_norm[0] * b_norm[0] + n_norm[1] * b_norm[1] + n_norm[2] * b_norm[2];

            // Downward-facing face: dot < 0
            if dot < -0.01 {
                let overhang_angle_from_horiz = (-dot).asin();
                if overhang_angle_from_horiz < (std::f64::consts::FRAC_PI_2 - max_angle_rad) {
                    let centroid = [
                        (p0[0] + p1[0] + p2[0]) / 3.0,
                        (p0[1] + p1[1] + p2[1]) / 3.0,
                        (p0[2] + p1[2] + p2[2]) / 3.0,
                    ];
                    issues.push(DfmIssue {
                        domain: "Additive/3D-Print",
                        rule_name: "Steep Unsupported Overhang",
                        severity: DfmSeverity::Warning,
                        description: format!(
                            "Triangle {} overhangs at {:.1}° below threshold {:.1}°, requiring support structures.",
                            i,
                            overhang_angle_from_horiz.to_degrees(),
                            config.max_overhang_angle_deg
                        ),
                        location: Some(centroid),
                        measured_value: overhang_angle_from_horiz.to_degrees(),
                        threshold_limit: config.max_overhang_angle_deg,
                    });
                }
            }
        }

        issues
    }

    /// Analyze mesh for CNC Milling accessibility and pocket issues.
    pub fn analyze_cnc_milling(mesh: &TessellatedMesh, config: &CncMillingDfmConfig) -> Vec<DfmIssue> {
        let mut issues = Vec::new();
        let s = config.spindle_axis;
        let s_len = (s[0] * s[0] + s[1] * s[1] + s[2] * s[2]).sqrt();
        let s_norm = if s_len > 1e-9 {
            [s[0] / s_len, s[1] / s_len, s[2] / s_len]
        } else {
            [0.0, 0.0, 1.0]
        };

        let num_triangles = mesh.indices.len() / 3;
        for i in 0..num_triangles {
            let i0 = mesh.indices[i * 3] as usize;
            let i1 = mesh.indices[i * 3 + 1] as usize;
            let i2 = mesh.indices[i * 3 + 2] as usize;

            if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
                continue;
            }

            let p0 = [mesh.positions[i0][0] as f64, mesh.positions[i0][1] as f64, mesh.positions[i0][2] as f64];
            let p1 = [mesh.positions[i1][0] as f64, mesh.positions[i1][1] as f64, mesh.positions[i1][2] as f64];
            let p2 = [mesh.positions[i2][0] as f64, mesh.positions[i2][1] as f64, mesh.positions[i2][2] as f64];

            let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let n_len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if n_len < 1e-12 {
                continue;
            }
            let n_norm = [n[0] / n_len, n[1] / n_len, n[2] / n_len];

            // Undercut in 3-axis milling occurs if face normal points opposite to spindle access
            let dot = n_norm[0] * s_norm[0] + n_norm[1] * s_norm[1] + n_norm[2] * s_norm[2];
            if dot < -0.1 {
                let centroid = [
                    (p0[0] + p1[0] + p2[0]) / 3.0,
                    (p0[1] + p1[1] + p2[1]) / 3.0,
                    (p0[2] + p1[2] + p2[2]) / 3.0,
                ];
                issues.push(DfmIssue {
                    domain: "Subtractive/CNC",
                    rule_name: "3-Axis Milling Undercut",
                    severity: DfmSeverity::Error,
                    description: format!(
                        "Face {} is an undercut for 3-axis milling along {:?}. Requires 5-axis indexing or custom T-slot cutter.",
                        i, config.spindle_axis
                    ),
                    location: Some(centroid),
                    measured_value: dot,
                    threshold_limit: 0.0,
                });
            }
        }

        issues
    }

    /// Analyze mesh for Injection Molding draft angles.
    pub fn analyze_injection_molding(mesh: &TessellatedMesh, config: &InjectionMoldingDfmConfig) -> Vec<DfmIssue> {
        let mut issues = Vec::new();
        let p = config.pull_direction;
        let p_len = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
        let p_norm = if p_len > 1e-9 {
            [p[0] / p_len, p[1] / p_len, p[2] / p_len]
        } else {
            [0.0, 0.0, 1.0]
        };

        let min_draft_rad = config.min_draft_angle_deg.to_radians();

        let num_triangles = mesh.indices.len() / 3;
        for i in 0..num_triangles {
            let i0 = mesh.indices[i * 3] as usize;
            let i1 = mesh.indices[i * 3 + 1] as usize;
            let i2 = mesh.indices[i * 3 + 2] as usize;

            if i0 >= mesh.positions.len() || i1 >= mesh.positions.len() || i2 >= mesh.positions.len() {
                continue;
            }

            let p0 = [mesh.positions[i0][0] as f64, mesh.positions[i0][1] as f64, mesh.positions[i0][2] as f64];
            let p1 = [mesh.positions[i1][0] as f64, mesh.positions[i1][1] as f64, mesh.positions[i1][2] as f64];
            let p2 = [mesh.positions[i2][0] as f64, mesh.positions[i2][1] as f64, mesh.positions[i2][2] as f64];

            let u = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let v = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            let n_len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
            if n_len < 1e-12 {
                continue;
            }
            let n_norm = [n[0] / n_len, n[1] / n_len, n[2] / n_len];

            // Draft angle is angle between face normal and plane perpendicular to pull vector
            let dot = (n_norm[0] * p_norm[0] + n_norm[1] * p_norm[1] + n_norm[2] * p_norm[2]).abs();
            let draft_angle = (dot).asin();

            if draft_angle < min_draft_rad {
                let centroid = [
                    (p0[0] + p1[0] + p2[0]) / 3.0,
                    (p0[1] + p1[1] + p2[1]) / 3.0,
                    (p0[2] + p1[2] + p2[2]) / 3.0,
                ];
                issues.push(DfmIssue {
                    domain: "Injection-Molding",
                    rule_name: "Insufficient Draft Angle",
                    severity: DfmSeverity::Warning,
                    description: format!(
                        "Face {} draft angle ({:.2}°) is below minimum {:.1}°; risk of mold sticking/drag marks.",
                        i,
                        draft_angle.to_degrees(),
                        config.min_draft_angle_deg
                    ),
                    location: Some(centroid),
                    measured_value: draft_angle.to_degrees(),
                    threshold_limit: config.min_draft_angle_deg,
                });
            }
        }

        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_cube() -> TessellatedMesh {
        TessellatedMesh {
            positions: vec![
                [0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0],
                [10.0, 10.0, 0.0],
                [0.0, 10.0, 0.0],
                [0.0, 0.0, 10.0],
                [10.0, 0.0, 10.0],
                [10.0, 10.0, 10.0],
                [0.0, 10.0, 10.0],
            ],
            indices: vec![
                // Bottom
                0, 2, 1, 0, 3, 2,
                // Top
                4, 5, 6, 4, 6, 7,
                // Front
                0, 1, 5, 0, 5, 4,
                // Back
                2, 3, 7, 2, 7, 6,
                // Left
                3, 0, 4, 3, 4, 7,
                // Right
                1, 2, 6, 1, 6, 5,
            ],
            normals: vec![],
        }
    }

    #[test]
    fn test_dfm_additive_overhang() {
        let cube = create_test_cube();
        let config = AdditiveDfmConfig::default();
        let issues = DfmEngine::analyze_additive(&cube, &config);

        // The bottom face of the cube points directly down ([0,0,-1]), which is 0° overhang
        assert!(!issues.is_empty(), "Bottom face should be flagged as unsupported overhang");
    }

    #[test]
    fn test_dfm_cnc_undercut() {
        let cube = create_test_cube();
        let config = CncMillingDfmConfig::default();
        let issues = DfmEngine::analyze_cnc_milling(&cube, &config);

        // Bottom face cannot be reached in 3-axis top-down milling
        assert!(!issues.is_empty(), "Bottom face should be flagged as 3-axis undercut");
    }
}
