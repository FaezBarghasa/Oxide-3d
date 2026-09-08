//! Oxide-3D Computer-Aided Manufacturing (CAM), Toolpaths, and G-Code Postprocessing.

use serde::{Deserialize, Serialize};

/// Type of CNC milling operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MillingOperationKind {
    /// 2.5D Adaptive / Zigzag Pocketing.
    Pocketing,
    /// 2D/3D Contour Profile Milling.
    Contouring,
    /// Hole Drilling / Tapping.
    Drilling,
    /// 3D Surface Ball-End Finishing.
    SurfaceFinishing,
}

/// Cutting tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuttingTool {
    /// Tool name/number.
    pub name: String,
    /// Tool diameter in mm.
    pub diameter_mm: f64,
    /// Number of flutes.
    pub flutes: u32,
    /// Maximum spindle RPM.
    pub max_rpm: f64,
}

/// Linear/Arc motion command in generated toolpath.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolpathPoint {
    /// Rapid transit (G0).
    Rapid { position: [f64; 3] },
    /// Linear cutting feed (G1).
    LinearFeed { position: [f64; 3], feedrate_mm_min: f64 },
}
