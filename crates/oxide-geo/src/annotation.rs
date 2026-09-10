//! CAD Dimensioning, Annotations, Multileaders, and Paper Space Layouts.
//!
//! Provides production-ready data structures for:
//! - Linear, Aligned, Angular, Radial, and Diameter Dimensions
//! - `DimStyle` parameters (text height, arrow size, extension line offsets, tolerances)
//! - Multileader (`MLeader`) with leader lines, landing, and dogleg
//! - Paper Space Layouts (`PaperLayout`) with viewports (`Viewport2D`) and scaling (1:1, 1:50, 1:100)

use crate::drafting_2d::Point2D;
use serde::{Deserialize, Serialize};

/// Dimension Type classification matching AutoCAD / OpenCADStudio standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionType {
    /// Linear horizontal or vertical dimension.
    Linear,
    /// Aligned along the measured line direction.
    Aligned,
    /// Angle between two vectors or 3 points.
    Angular,
    /// Radius of arc or circle (e.g. `R 25.0`).
    Radius,
    /// Diameter of arc or circle (e.g. `Ø 50.0`).
    Diameter,
    /// Ordinate dimension relative to datum origin.
    Ordinate,
    /// Arc length dimension.
    ArcLength,
}

/// CAD Dimensioning Style (`DIMSTYLE`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimStyle {
    /// Style name (e.g. "Standard", "ISO-25", "Architectural").
    pub name: String,
    /// Dimension text height in drawing units (mm).
    pub text_height: f64,
    /// Arrowhead length.
    pub arrow_size: f64,
    /// Extension line overshoot past dimension line.
    pub extension_overshoot: f64,
    /// Extension line offset from measured entity point.
    pub extension_offset: f64,
    /// Gap between dimension line and text.
    pub text_gap: f64,
    /// Decimal precision places (e.g. 2 for 0.00).
    pub precision: usize,
    /// Prefix (e.g. "R", "Ø").
    pub prefix: String,
    /// Suffix (e.g. " mm", "°").
    pub suffix: String,
}

impl Default for DimStyle {
    fn default() -> Self {
        Self {
            name: "ISO-25".to_string(),
            text_height: 2.5,
            arrow_size: 2.5,
            extension_overshoot: 1.25,
            extension_offset: 0.625,
            text_gap: 0.625,
            precision: 2,
            prefix: String::new(),
            suffix: String::new(),
        }
    }
}

/// Dimension Entity in 2D Space.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DimensionEntity {
    /// Dimension classification.
    pub dim_type: DimensionType,
    /// First reference point.
    pub def_point1: Point2D,
    /// Second reference point.
    pub def_point2: Point2D,
    /// Dimension line placement point.
    pub text_point: Point2D,
    /// Override text (empty string means auto-computed measurement).
    pub text_override: String,
    /// Active style name.
    pub style_name: String,
}

impl DimensionEntity {
    /// Compute actual measurement value between reference points.
    #[must_use]
    pub fn measurement(&self) -> f64 {
        match self.dim_type {
            DimensionType::Linear => (self.def_point2.x - self.def_point1.x)
                .abs()
                .max((self.def_point2.y - self.def_point1.y).abs()),
            DimensionType::Aligned => self.def_point1.distance_to(&self.def_point2),
            DimensionType::Radius => self.def_point1.distance_to(&self.def_point2),
            DimensionType::Diameter => self.def_point1.distance_to(&self.def_point2) * 2.0,
            _ => self.def_point1.distance_to(&self.def_point2),
        }
    }

    /// Format displayed measurement string according to style.
    #[must_use]
    pub fn formatted_text(&self, style: &DimStyle) -> String {
        if !self.text_override.is_empty() {
            return self.text_override.clone();
        }
        let val = self.measurement();
        let formatted = format!("{:.1$}", val, style.precision);
        match self.dim_type {
            DimensionType::Radius => format!("R{formatted}"),
            DimensionType::Diameter => format!("Ø{formatted}"),
            _ => format!("{}{formatted}{}", style.prefix, style.suffix),
        }
    }
}

/// Multileader (`MLEADER`) annotation with pointer arrow and text block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MultiLeader {
    /// Arrowhead target point.
    pub arrow_target: Point2D,
    /// Landing joint point.
    pub landing_point: Point2D,
    /// Length of horizontal dogleg landing line.
    pub landing_distance: f64,
    /// Text annotation contents.
    pub text: String,
    /// Text height in drawing units.
    pub text_height: f64,
}

impl MultiLeader {
    /// Create new multileader annotation.
    #[must_use]
    pub fn new(arrow_target: Point2D, landing_point: Point2D, text: impl Into<String>) -> Self {
        Self {
            arrow_target,
            landing_point,
            landing_distance: 5.0,
            text: text.into(),
            text_height: 2.5,
        }
    }
}

/// Paper Space Viewport (`MVIEW`) mapped onto a sheet layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutViewport {
    /// Viewport ID.
    pub id: u32,
    /// Paper sheet center coordinate.
    pub center_paper: Point2D,
    /// Viewport width in sheet mm.
    pub width_mm: f64,
    /// Viewport height in sheet mm.
    pub height_mm: f64,
    /// Target model center coordinate.
    pub target_model: Point2D,
    /// Viewport zoom scale (e.g. 0.02 for 1:50, 0.01 for 1:100, 1.0 for 1:1).
    pub scale: f64,
    /// Whether viewport zoom is locked.
    pub is_locked: bool,
    /// Layer visibility overrides inside this viewport.
    pub frozen_layers: Vec<String>,
}

/// Paper Space Layout Sheet (e.g. A4, A3, A1, A0, Arch D).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaperLayout {
    /// Layout sheet tab name (e.g. "Layout1", "A3-Assembly-Sheet").
    pub name: String,
    /// Paper width in mm (e.g. 420.0 for A3).
    pub paper_width_mm: f64,
    /// Paper height in mm (e.g. 297.0 for A3).
    pub paper_height_mm: f64,
    /// Margin offsets [left, bottom, right, top] in mm.
    pub margins_mm: [f64; 4],
    /// Embedded viewports displaying model space geometry.
    pub viewports: Vec<LayoutViewport>,
}

impl Default for PaperLayout {
    fn default() -> Self {
        Self {
            name: "Layout1 (A3)".to_string(),
            paper_width_mm: 420.0,
            paper_height_mm: 297.0,
            margins_mm: [10.0, 10.0, 10.0, 10.0],
            viewports: vec![LayoutViewport {
                id: 1,
                center_paper: Point2D::new(210.0, 148.5),
                width_mm: 380.0,
                height_mm: 260.0,
                target_model: Point2D::new(0.0, 0.0),
                scale: 1.0,
                is_locked: false,
                frozen_layers: Vec::new(),
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dimension_measurements_and_formatting() {
        let style = DimStyle::default();
        let dim_aligned = DimensionEntity {
            dim_type: DimensionType::Aligned,
            def_point1: Point2D::new(0.0, 0.0),
            def_point2: Point2D::new(30.0, 40.0),
            text_point: Point2D::new(15.0, 25.0),
            text_override: String::new(),
            style_name: "ISO-25".to_string(),
        };

        assert!((dim_aligned.measurement() - 50.0).abs() < 1e-4);
        assert_eq!(dim_aligned.formatted_text(&style), "50.00");

        let dim_dia = DimensionEntity {
            dim_type: DimensionType::Diameter,
            def_point1: Point2D::new(0.0, 0.0),
            def_point2: Point2D::new(25.0, 0.0),
            text_point: Point2D::new(12.5, 5.0),
            text_override: String::new(),
            style_name: "ISO-25".to_string(),
        };

        assert!((dim_dia.measurement() - 50.0).abs() < 1e-4);
        assert_eq!(dim_dia.formatted_text(&style), "Ø50.00");
    }

    #[test]
    fn test_paper_layout_defaults() {
        let layout = PaperLayout::default();
        assert_eq!(layout.paper_width_mm, 420.0);
        assert_eq!(layout.paper_height_mm, 297.0);
        assert_eq!(layout.viewports.len(), 1);
        assert_eq!(layout.viewports[0].scale, 1.0);
    }
}
