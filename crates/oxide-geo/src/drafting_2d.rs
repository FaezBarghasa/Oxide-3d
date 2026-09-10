//! 2D Drafting Entity Models, Object Snaps (OSNAP), Polar Tracking, Layers & Blocks.

use serde::{Deserialize, Serialize};

/// 2D Vector Point.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub fn distance_to(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Supported 2D Drafting Entities.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DraftingEntity2D {
    Line {
        start: Point2D,
        end: Point2D,
    },
    Polyline {
        vertices: Vec<Point2D>,
        closed: bool,
    },
    Circle {
        center: Point2D,
        radius: f64,
    },
    Arc {
        center: Point2D,
        radius: f64,
        start_angle_rad: f64,
        end_angle_rad: f64,
    },
    Ellipse {
        center: Point2D,
        major_axis: Point2D,
        radius_ratio: f64,
    },
    Hatch {
        pattern_name: String,
        boundary_loops: Vec<Vec<Point2D>>,
        scale: f64,
        angle_deg: f64,
    },
    Text {
        position: Point2D,
        content: String,
        height: f64,
    },
}

/// Object Snap (OSNAP) Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OsnapMode {
    Endpoint,
    Midpoint,
    Center,
    GeometricCenter,
    Quadrant,
    Intersection,
    Perpendicular,
    Tangent,
    Nearest,
    Parallel,
}

impl OsnapMode {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Endpoint => "Endpoint",
            Self::Midpoint => "Midpoint",
            Self::Center => "Center",
            Self::GeometricCenter => "Geometric Center",
            Self::Quadrant => "Quadrant",
            Self::Intersection => "Intersection",
            Self::Perpendicular => "Perpendicular",
            Self::Tangent => "Tangent",
            Self::Nearest => "Nearest",
            Self::Parallel => "Parallel",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Endpoint,
            Self::Midpoint,
            Self::Center,
            Self::GeometricCenter,
            Self::Quadrant,
            Self::Intersection,
            Self::Perpendicular,
            Self::Tangent,
            Self::Nearest,
            Self::Parallel,
        ]
    }
}

/// Snapped Candidate Point Result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OsnapCandidate {
    pub point: Point2D,
    pub mode: OsnapMode,
    pub distance_to_cursor: f64,
}

/// CAD Layer Definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CadLayer {
    pub name: String,
    pub color_rgb: [u8; 3],
    pub linetype: String,
    pub lineweight_mm: f32,
    pub is_frozen: bool,
    pub is_locked: bool,
    pub is_plottable: bool,
}

impl CadLayer {
    #[must_use]
    pub fn new(name: impl Into<String>, color: [u8; 3]) -> Self {
        Self {
            name: name.into(),
            color_rgb: color,
            linetype: "Continuous".to_string(),
            lineweight_mm: 0.25,
            is_frozen: false,
            is_locked: false,
            is_plottable: true,
        }
    }
}

/// Complete 2D Drafting Database / Document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftingDatabase2D {
    pub active_layer_name: String,
    pub layers: Vec<CadLayer>,
    pub entities: Vec<DraftingEntity2D>,
    pub active_osnaps: Vec<OsnapMode>,
    pub ortho_mode: bool,
    pub polar_tracking: bool,
    pub polar_increment_deg: f64,
}

impl Default for DraftingDatabase2D {
    fn default() -> Self {
        Self {
            active_layer_name: "0".to_string(),
            layers: vec![
                CadLayer::new("0", [255, 255, 255]),
                CadLayer::new("Dimensions", [0, 200, 255]),
                CadLayer::new("Hidden", [255, 128, 0]),
                CadLayer::new("Centerlines", [255, 0, 0]),
            ],
            entities: Vec::new(),
            active_osnaps: vec![
                OsnapMode::Endpoint,
                OsnapMode::Midpoint,
                OsnapMode::Center,
                OsnapMode::Intersection,
            ],
            ortho_mode: false,
            polar_tracking: true,
            polar_increment_deg: 45.0,
        }
    }
}

impl DraftingDatabase2D {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Find closest OSNAP candidate for cursor position.
    #[must_use]
    pub fn find_osnap(&self, cursor: Point2D, tolerance: f64) -> Option<OsnapCandidate> {
        let mut best: Option<OsnapCandidate> = None;

        for entity in &self.entities {
            match entity {
                DraftingEntity2D::Line { start, end } => {
                    if self.active_osnaps.contains(&OsnapMode::Endpoint) {
                        let d_start = cursor.distance_to(start);
                        if d_start <= tolerance
                            && best.as_ref().is_none_or(|b| d_start < b.distance_to_cursor)
                        {
                            best = Some(OsnapCandidate {
                                point: *start,
                                mode: OsnapMode::Endpoint,
                                distance_to_cursor: d_start,
                            });
                        }
                        let d_end = cursor.distance_to(end);
                        if d_end <= tolerance
                            && best.as_ref().is_none_or(|b| d_end < b.distance_to_cursor)
                        {
                            best = Some(OsnapCandidate {
                                point: *end,
                                mode: OsnapMode::Endpoint,
                                distance_to_cursor: d_end,
                            });
                        }
                    }
                    if self.active_osnaps.contains(&OsnapMode::Midpoint) {
                        let mid = Point2D::new((start.x + end.x) * 0.5, (start.y + end.y) * 0.5);
                        let d_mid = cursor.distance_to(&mid);
                        if d_mid <= tolerance
                            && best.as_ref().is_none_or(|b| d_mid < b.distance_to_cursor)
                        {
                            best = Some(OsnapCandidate {
                                point: mid,
                                mode: OsnapMode::Midpoint,
                                distance_to_cursor: d_mid,
                            });
                        }
                    }
                }
                DraftingEntity2D::Circle { center, radius } => {
                    if self.active_osnaps.contains(&OsnapMode::Center) {
                        let d_center = cursor.distance_to(center);
                        if d_center <= tolerance
                            && best
                                .as_ref()
                                .is_none_or(|b| d_center < b.distance_to_cursor)
                        {
                            best = Some(OsnapCandidate {
                                point: *center,
                                mode: OsnapMode::Center,
                                distance_to_cursor: d_center,
                            });
                        }
                    }
                    if self.active_osnaps.contains(&OsnapMode::Quadrant) {
                        let quads = [
                            Point2D::new(center.x + radius, center.y),
                            Point2D::new(center.x - radius, center.y),
                            Point2D::new(center.x, center.y + radius),
                            Point2D::new(center.x, center.y - radius),
                        ];
                        for q in quads {
                            let d = cursor.distance_to(&q);
                            if d <= tolerance
                                && best.as_ref().is_none_or(|b| d < b.distance_to_cursor)
                            {
                                best = Some(OsnapCandidate {
                                    point: q,
                                    mode: OsnapMode::Quadrant,
                                    distance_to_cursor: d,
                                });
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        best
    }
}
