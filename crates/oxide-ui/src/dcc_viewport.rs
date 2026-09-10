//! DCC Viewport Navigation, Layouts, Configuration and Shading Modes.

use serde::{Deserialize, Serialize};

/// Viewport Shading Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DccShadingMode {
    #[default]
    Realistic,
    Shaded,
    ConsistentColors,
    Clay,
    XRay,
    EdgedFaces,
    Wireframe,
    BoundingBox,
    HiddenLine,
}

impl DccShadingMode {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Realistic => "Realistic",
            Self::Shaded => "Shaded",
            Self::ConsistentColors => "Consistent Colors",
            Self::Clay => "Clay",
            Self::XRay => "X-Ray",
            Self::EdgedFaces => "Edged Faces",
            Self::Wireframe => "Wireframe",
            Self::BoundingBox => "Bounding Box",
            Self::HiddenLine => "Hidden Line",
        }
    }
}

/// Viewport Layout presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ViewportLayoutPreset {
    Single,
    #[default]
    Quad4,
    SplitHorizontal,
    SplitVertical,
    OneTopTwoBottom,
    OneLeftTwoRight,
}

/// Viewport View Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ViewportViewType {
    #[default]
    Perspective,
    Orthographic,
    Top,
    Bottom,
    Front,
    Back,
    Left,
    Right,
    Camera,
    Light,
}

/// Viewport Navigation Tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ViewportNavTool {
    #[default]
    Select,
    Zoom,
    ZoomAll,
    ZoomExtents,
    ZoomExtentsSelected,
    Pan,
    Orbit,
    OrbitSubObject,
    WalkThrough,
    FlyThrough,
    FieldOfView,
}

/// Safe Frame Configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeFrameConfig {
    pub show_safe_frame: bool,
    pub show_live_area: bool,
    pub show_action_safe: bool,
    pub show_title_safe: bool,
    pub show_user_safe: bool,
}

impl Default for SafeFrameConfig {
    fn default() -> Self {
        Self {
            show_safe_frame: false,
            show_live_area: true,
            show_action_safe: true,
            show_title_safe: true,
            show_user_safe: false,
        }
    }
}

/// DCC Viewport State & Configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DccViewportModel {
    pub layout: ViewportLayoutPreset,
    pub active_view: ViewportViewType,
    pub shading_mode: DccShadingMode,
    pub active_nav_tool: ViewportNavTool,
    pub is_maximized: bool,
    pub show_viewcube: bool,
    pub show_steeringwheels: bool,
    pub show_grid: bool,
    pub show_statistics: bool,
    pub adaptive_degradation: bool,
    pub safe_frames: SafeFrameConfig,
}

impl Default for DccViewportModel {
    fn default() -> Self {
        Self::new()
    }
}

impl DccViewportModel {
    #[must_use]
    pub fn new() -> Self {
        Self {
            layout: ViewportLayoutPreset::Quad4,
            active_view: ViewportViewType::Perspective,
            shading_mode: DccShadingMode::Realistic,
            active_nav_tool: ViewportNavTool::Select,
            is_maximized: false,
            show_viewcube: true,
            show_steeringwheels: false,
            show_grid: true,
            show_statistics: true,
            adaptive_degradation: false,
            safe_frames: SafeFrameConfig::default(),
        }
    }

    /// Toggle maximize viewport (Alt+W).
    pub fn toggle_maximize(&mut self) {
        self.is_maximized = !self.is_maximized;
    }
}
