//! Oxide-3D Metrology, GD&T Feature Control Frames, and CMM Scan Verification.

use serde::{Deserialize, Serialize};

/// ASME Y14.5 / ISO GPS Geometric Characteristic Symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeometricCharacteristic {
    /// Form: Straightness
    Straightness,
    /// Form: Flatness
    Flatness,
    /// Form: Circularity
    Circularity,
    /// Form: Cylindricity
    Cylindricity,
    /// Orientation: Perpendicularity
    Perpendicularity,
    /// Orientation: Parallelism
    Parallelism,
    /// Orientation: Angularity
    Angularity,
    /// Location: Position
    Position,
    /// Location: Concentricity
    Concentricity,
    /// Location: Symmetry
    Symmetry,
    /// Profile: Profile of a Surface
    ProfileOfSurface,
    /// Profile: Profile of a Line
    ProfileOfLine,
    /// Runout: Circular Runout
    CircularRunout,
    /// Runout: Total Runout
    TotalRunout,
}

/// GD&T Material Condition Modifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaterialConditionModifier {
    /// Maximum Material Condition (MMC - [M]).
    Mmc,
    /// Least Material Condition (LMC - [L]).
    Lmc,
    /// Regardless of Feature Size (RFS).
    Rfs,
}

/// GD&T Feature Control Frame specification according to ASME Y14.5-2018.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureControlFrame {
    /// Characteristic symbol.
    pub characteristic: GeometricCharacteristic,
    /// Total tolerance zone value (e.g. 0.05 mm).
    pub tolerance_value: f64,
    /// Material condition modifier on tolerance.
    pub modifier: MaterialConditionModifier,
    /// Primary datum reference.
    pub datum_primary: Option<String>,
    /// Secondary datum reference.
    pub datum_secondary: Option<String>,
    /// Tertiary datum reference.
    pub datum_tertiary: Option<String>,
}

/// CMM Point Cloud inspection sample point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct InspectionPoint {
    /// Measured coordinate [x, y, z].
    pub measured: [f64; 3],
    /// Nominal nominal CAD coordinate [x, y, z].
    pub nominal: [f64; 3],
}

/// Inspection evaluation result for GD&T verification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionReport {
    /// Evaluated characteristic.
    pub characteristic: GeometricCharacteristic,
    /// Specified allowable tolerance.
    pub tolerance_allowed: f64,
    /// Actual measured deviation.
    pub actual_deviation: f64,
    /// Pass / Fail conformance status.
    pub conforms: bool,
    /// Maximum positive peak deviation.
    pub max_deviation: f64,
    /// Minimum negative valley deviation.
    pub min_deviation: f64,
}

/// CMM Metrology Verifier.
#[derive(Debug, Default)]
pub struct MetrologyVerifier;

impl MetrologyVerifier {
    /// Create a new metrology verification engine.
    pub fn new() -> Self {
        Self
    }

    /// Evaluate Flatness of a point cloud against best-fit plane.
    /// Flatness is the minimum distance between two parallel planes containing all points.
    pub fn evaluate_flatness(
        &self,
        points: &[[f64; 3]],
        fcf: &FeatureControlFrame,
    ) -> InspectionReport {
        if points.is_empty() {
            return InspectionReport {
                characteristic: GeometricCharacteristic::Flatness,
                tolerance_allowed: fcf.tolerance_value,
                actual_deviation: 0.0,
                conforms: true,
                max_deviation: 0.0,
                min_deviation: 0.0,
            };
        }

        // Calculate centroid
        let n = points.len() as f64;
        let mut centroid = [0.0, 0.0, 0.0];
        for p in points {
            centroid[0] += p[0];
            centroid[1] += p[1];
            centroid[2] += p[2];
        }
        centroid[0] /= n;
        centroid[1] /= n;
        centroid[2] /= n;

        // Covariance matrix components for normal estimation
        let mut cxx = 0.0;
        let mut cyy = 0.0;
        let mut czz = 0.0;
        let mut cxy = 0.0;
        let mut cxz = 0.0;
        let mut cyz = 0.0;

        for p in points {
            let rx = p[0] - centroid[0];
            let ry = p[1] - centroid[1];
            let rz = p[2] - centroid[2];
            cxx += rx * rx;
            cyy += ry * ry;
            czz += rz * rz;
            cxy += rx * ry;
            cxz += rx * rz;
            cyz += ry * rz;
        }

        // Approximate least squares normal vector (pointing predominantly along least variance axis)
        let normal = if czz <= cxx && czz <= cyy {
            [0.0, 0.0, 1.0]
        } else if cyy <= cxx {
            [0.0, 1.0, 0.0]
        } else {
            [1.0, 0.0, 0.0]
        };

        // Compute signed distances to best fit plane through centroid
        let mut min_d = f64::INFINITY;
        let mut max_d = f64::NEG_INFINITY;

        for p in points {
            let d = (p[0] - centroid[0]) * normal[0]
                + (p[1] - centroid[1]) * normal[1]
                + (p[2] - centroid[2]) * normal[2];
            if d < min_d {
                min_d = d;
            }
            if d > max_d {
                max_d = d;
            }
        }

        let flatness_deviation = max_d - min_d;
        let conforms = flatness_deviation <= fcf.tolerance_value;

        InspectionReport {
            characteristic: GeometricCharacteristic::Flatness,
            tolerance_allowed: fcf.tolerance_value,
            actual_deviation: flatness_deviation,
            conforms,
            max_deviation: max_d,
            min_deviation: min_d,
        }
    }

    /// Evaluate True Position (RFS) for measured center points against nominal coordinates.
    /// True Position Zone = 2 * sqrt(dx^2 + dy^2 + dz^2).
    pub fn evaluate_true_position(
        &self,
        samples: &[InspectionPoint],
        fcf: &FeatureControlFrame,
    ) -> Vec<InspectionReport> {
        samples
            .iter()
            .map(|s| {
                let dx = s.measured[0] - s.nominal[0];
                let dy = s.measured[1] - s.nominal[1];
                let dz = s.measured[2] - s.nominal[2];
                let dev_radial = (dx * dx + dy * dy + dz * dz).sqrt();
                let actual_deviation = 2.0 * dev_radial; // Diametrical tolerance zone

                InspectionReport {
                    characteristic: GeometricCharacteristic::Position,
                    tolerance_allowed: fcf.tolerance_value,
                    actual_deviation,
                    conforms: actual_deviation <= fcf.tolerance_value,
                    max_deviation: dev_radial,
                    min_deviation: 0.0,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatness_verification() {
        let fcf = FeatureControlFrame {
            characteristic: GeometricCharacteristic::Flatness,
            tolerance_value: 0.05,
            modifier: MaterialConditionModifier::Rfs,
            datum_primary: None,
            datum_secondary: None,
            datum_tertiary: None,
        };

        let verifier = MetrologyVerifier::new();

        // Planar points with small Z perturbations within 0.02 mm
        let points = vec![
            [0.0, 0.0, 0.01],
            [10.0, 0.0, -0.01],
            [10.0, 10.0, 0.005],
            [0.0, 10.0, -0.005],
        ];

        let report = verifier.evaluate_flatness(&points, &fcf);
        assert!(report.conforms, "Flatness within tolerance should pass");
        assert!((report.actual_deviation - 0.02).abs() < 1e-4);
    }

    #[test]
    fn test_true_position_verification() {
        let fcf = FeatureControlFrame {
            characteristic: GeometricCharacteristic::Position,
            tolerance_value: 0.1, // Diametrical zone = 0.1 mm
            modifier: MaterialConditionModifier::Rfs,
            datum_primary: Some("A".into()),
            datum_secondary: Some("B".into()),
            datum_tertiary: None,
        };

        let verifier = MetrologyVerifier::new();
        let samples = vec![
            InspectionPoint {
                nominal: [100.0, 50.0, 0.0],
                measured: [100.03, 50.02, 0.0], // offset sqrt(0.03^2 + 0.02^2) = 0.036 -> zone = 0.072 < 0.1
            },
            InspectionPoint {
                nominal: [200.0, 50.0, 0.0],
                measured: [200.1, 50.0, 0.0], // offset 0.1 -> zone = 0.2 > 0.1 (FAIL)
            },
        ];

        let reports = verifier.evaluate_true_position(&samples, &fcf);
        assert!(reports[0].conforms, "Sample 0 conforms to true position");
        assert!(!reports[1].conforms, "Sample 1 exceeds true position");
    }
}
