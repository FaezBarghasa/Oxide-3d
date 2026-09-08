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

/// GD&T Feature Control Frame specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureControlFrame {
    /// Characteristic symbol.
    pub characteristic: GeometricCharacteristic,
    /// Total tolerance zone value.
    pub tolerance_value: f64,
    /// Primary datum reference.
    pub datum_primary: Option<String>,
    /// Secondary datum reference.
    pub datum_secondary: Option<String>,
    /// Tertiary datum reference.
    pub datum_tertiary: Option<String>,
}
