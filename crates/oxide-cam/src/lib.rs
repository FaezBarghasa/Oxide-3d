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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolpathPoint {
    /// Rapid transit (G0).
    Rapid {
        /// Target 3D coordinate.
        position: [f64; 3],
    },
    /// Linear cutting feed (G1).
    LinearFeed {
        /// Target 3D coordinate.
        position: [f64; 3],
        /// Feedrate in mm/min.
        feedrate_mm_min: f64,
    },
}

/// CAM Operation parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PocketOperation {
    /// Cutting tool used.
    pub tool: CuttingTool,
    /// Clearance plane Z (mm).
    pub clearance_z: f64,
    /// Retract plane Z (mm).
    pub retract_z: f64,
    /// Top of stock Z (mm).
    pub stock_top_z: f64,
    /// Final pocket floor depth Z (mm).
    pub target_depth_z: f64,
    /// Maximum axial stepdown per pass (mm).
    pub stepdown_mm: f64,
    /// Stepover distance (mm).
    pub stepover_mm: f64,
    /// Cutting feedrate (mm/min).
    pub feedrate_mm_min: f64,
    /// Plunge feedrate (mm/min).
    pub plunge_feedrate_mm_min: f64,
    /// Spindle speed (RPM).
    pub spindle_rpm: f64,
}

/// Toolpath Generator for 2.5D Pocketing and Contouring operations.
#[derive(Debug, Default)]
pub struct ToolpathGenerator;

impl ToolpathGenerator {
    /// Create a new toolpath generator.
    pub fn new() -> Self {
        Self
    }

    /// Generate a multi-pass zigzag pocketing toolpath for a rectangular boundary.
    pub fn generate_rectangular_pocket(
        &self,
        op: &PocketOperation,
        min_pt: [f64; 2],
        max_pt: [f64; 2],
    ) -> Vec<ToolpathPoint> {
        let mut path = Vec::new();
        let tool_radius = op.tool.diameter_mm * 0.5;

        // Offset pocket boundary inwards by tool radius
        let x_min = min_pt[0] + tool_radius;
        let x_max = max_pt[0] - tool_radius;
        let y_min = min_pt[1] + tool_radius;
        let y_max = max_pt[1] - tool_radius;

        if x_min >= x_max || y_min >= y_max {
            return path; // Tool too large for pocket
        }

        // Calculate depth passes
        let total_depth = (op.stock_top_z - op.target_depth_z).abs();
        let num_passes = (total_depth / op.stepdown_mm).ceil() as usize;
        let actual_stepdown = if num_passes > 0 {
            total_depth / num_passes as f64
        } else {
            total_depth
        };

        let mut current_z = op.stock_top_z;

        // Start at clearance plane above initial point
        path.push(ToolpathPoint::Rapid {
            position: [x_min, y_min, op.clearance_z],
        });

        for _ in 0..num_passes {
            current_z -= actual_stepdown;

            // Rapid to retract plane above start
            path.push(ToolpathPoint::Rapid {
                position: [x_min, y_min, op.retract_z],
            });

            // Plunge feed to current cut depth
            path.push(ToolpathPoint::LinearFeed {
                position: [x_min, y_min, current_z],
                feedrate_mm_min: op.plunge_feedrate_mm_min,
            });

            // Zigzag raster back and forth along Y with stepover in X
            let mut curr_y = y_min;
            let mut going_up = true;

            while curr_y <= y_max {
                let target_x = if going_up { x_max } else { x_min };
                path.push(ToolpathPoint::LinearFeed {
                    position: [target_x, curr_y, current_z],
                    feedrate_mm_min: op.feedrate_mm_min,
                });

                curr_y += op.stepover_mm;
                if curr_y <= y_max {
                    path.push(ToolpathPoint::LinearFeed {
                        position: [target_x, curr_y, current_z],
                        feedrate_mm_min: op.feedrate_mm_min,
                    });
                }
                going_up = !going_up;
            }

            // Retract tool to safety plane
            path.push(ToolpathPoint::Rapid {
                position: [x_max, y_max, op.retract_z],
            });
        }

        // Final rapid return to clearance height
        path.push(ToolpathPoint::Rapid {
            position: [x_min, y_min, op.clearance_z],
        });

        path
    }
}

/// CNC Dialect Postprocessor formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PostProcessorDialect {
    /// Standard Fanuc / Haas ISO G-code.
    Fanuc,
    /// Open-source GRBL format.
    Grbl,
    /// Siemens Sinumerik 840D.
    Siemens,
}

/// G-Code Postprocessor emitting standardized CNC programs.
#[derive(Debug)]
pub struct GCodePostProcessor;

impl GCodePostProcessor {
    /// Format a list of toolpath points into a CNC G-code program string.
    pub fn post_process(
        dialect: PostProcessorDialect,
        program_name: &str,
        spindle_rpm: f64,
        points: &[ToolpathPoint],
    ) -> String {
        let mut gcode = String::new();

        // Header
        match dialect {
            PostProcessorDialect::Fanuc | PostProcessorDialect::Grbl => {
                gcode.push_str(&format!("(Program: {program_name})\n"));
                gcode.push_str("G21 (Metric units)\n");
                gcode.push_str("G90 (Absolute coordinates)\n");
                gcode.push_str("G17 (XY plane)\n");
                gcode.push_str(&format!("M03 S{:.0} (Spindle CW)\n", spindle_rpm));
            }
            PostProcessorDialect::Siemens => {
                gcode.push_str(&format!("; Program: {program_name}\n"));
                gcode.push_str("G71 (Metric units)\n");
                gcode.push_str("G90\n");
                gcode.push_str(&format!("M3 S{:.0}\n", spindle_rpm));
            }
        }

        // Emit motion lines
        for pt in points {
            match pt {
                ToolpathPoint::Rapid { position } => {
                    gcode.push_str(&format!(
                        "G00 X{:.3} Y{:.3} Z{:.3}\n",
                        position[0], position[1], position[2]
                    ));
                }
                ToolpathPoint::LinearFeed {
                    position,
                    feedrate_mm_min,
                } => {
                    gcode.push_str(&format!(
                        "G01 X{:.3} Y{:.3} Z{:.3} F{:.1}\n",
                        position[0], position[1], position[2], feedrate_mm_min
                    ));
                }
            }
        }

        // Footer
        gcode.push_str("M05 (Spindle Stop)\n");
        gcode.push_str("G00 Z50.000 (Safe Z retract)\n");
        gcode.push_str("M30 (Program End)\n");

        gcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pocket_toolpath_and_gcode() {
        let tool = CuttingTool {
            name: "6mm Flat Endmill".into(),
            diameter_mm: 6.0,
            flutes: 3,
            max_rpm: 12000.0,
        };

        let op = PocketOperation {
            tool,
            clearance_z: 10.0,
            retract_z: 2.0,
            stock_top_z: 0.0,
            target_depth_z: -5.0,
            stepdown_mm: 2.5,
            stepover_mm: 3.0,
            feedrate_mm_min: 1200.0,
            plunge_feedrate_mm_min: 300.0,
            spindle_rpm: 8000.0,
        };

        let generator = ToolpathGenerator::new();
        let points = generator.generate_rectangular_pocket(&op, [0.0, 0.0], [50.0, 50.0]);

        assert!(!points.is_empty(), "Toolpath points should be generated");

        let gcode = GCodePostProcessor::post_process(
            PostProcessorDialect::Fanuc,
            "POCKET_TEST",
            op.spindle_rpm,
            &points,
        );

        assert!(gcode.contains("G21"));
        assert!(gcode.contains("M03 S8000"));
        assert!(gcode.contains("G01"));
        assert!(gcode.contains("M30"));
    }
}
