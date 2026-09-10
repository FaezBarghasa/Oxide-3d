//! Graphite Modeling Tools (Ribbon) for Oxide-3D DCC System.

use serde::{Deserialize, Serialize};

/// Graphite Ribbon Tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RibbonTab {
    #[default]
    Modeling,
    Freeform,
    Selection,
    ObjectPaint,
    Populate,
}

impl RibbonTab {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Modeling => "Modeling",
            Self::Freeform => "Freeform",
            Self::Selection => "Selection",
            Self::ObjectPaint => "Object Paint",
            Self::Populate => "Populate",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Modeling,
            Self::Freeform,
            Self::Selection,
            Self::ObjectPaint,
            Self::Populate,
        ]
    }
}

/// Sub-Object Selection Level in Polygon Modeling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SubObjectLevel {
    #[default]
    Object,
    Vertex,
    Edge,
    Border,
    Polygon,
    Element,
}

/// PolyDraw Brush Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PolyDrawMode {
    #[default]
    Off,
    DrawOnSurface,
    DrawOnGrid,
    Shapes,
    Strips,
    Branches,
    Surface,
}

/// Paint Deform Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PaintDeformMode {
    #[default]
    PushPull,
    Relax,
    Smudge,
    Flatten,
    Revert,
}

/// Complete Graphite Ribbon Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphiteRibbonModel {
    pub active_tab: RibbonTab,
    pub sub_object_level: SubObjectLevel,
    pub polydraw_mode: PolyDrawMode,
    pub paint_deform: PaintDeformMode,
    pub preserve_uvs: bool,
    pub repeat_last_tool: Option<String>,
    pub brush_size: f32,
    pub brush_strength: f32,
}

impl Default for GraphiteRibbonModel {
    fn default() -> Self {
        Self {
            active_tab: RibbonTab::Modeling,
            sub_object_level: SubObjectLevel::Object,
            polydraw_mode: PolyDrawMode::Off,
            paint_deform: PaintDeformMode::PushPull,
            preserve_uvs: false,
            repeat_last_tool: None,
            brush_size: 20.0,
            brush_strength: 0.5,
        }
    }
}

impl GraphiteRibbonModel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
