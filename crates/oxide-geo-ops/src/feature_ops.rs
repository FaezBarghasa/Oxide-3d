use serde::{Deserialize, Serialize};

/// Parametric options for 3D extrusion.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ExtrudeOptions {
    /// Linear distance to extrude along normal.
    pub distance: f64,
    /// Draft angle in radians (0.0 = no draft).
    pub draft_angle: f64,
    /// Symmetric bidirectional extrusion flag.
    pub symmetric: bool,
}

impl Default for ExtrudeOptions {
    fn default() -> Self {
        Self {
            distance: 10.0,
            draft_angle: 0.0,
            symmetric: false,
        }
    }
}

/// Parametric options for edge filleting.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct FilletOptions {
    /// Constant or start fillet radius.
    pub radius: f64,
}

impl Default for FilletOptions {
    fn default() -> Self {
        Self { radius: 1.0 }
    }
}
