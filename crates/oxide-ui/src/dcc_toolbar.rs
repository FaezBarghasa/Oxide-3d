//! Main Toolbar System for DCC / 3D Digital Content Creation in Oxide-3D.

use serde::{Deserialize, Serialize};

/// Selection Filter Categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectionFilter {
    #[default]
    All,
    Geometry,
    Shapes,
    Lights,
    Cameras,
    Helpers,
    SpaceWarps,
    IoObjects,
    Bones,
    IkChains,
    Points,
    Custom,
}

impl SelectionFilter {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Geometry => "Geometry",
            Self::Shapes => "Shapes",
            Self::Lights => "Lights",
            Self::Cameras => "Cameras",
            Self::Helpers => "Helpers",
            Self::SpaceWarps => "Space Warps",
            Self::IoObjects => "I/O Objects",
            Self::Bones => "Bone",
            Self::IkChains => "IK Chain",
            Self::Points => "Point",
            Self::Custom => "Custom Filter...",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::All,
            Self::Geometry,
            Self::Shapes,
            Self::Lights,
            Self::Cameras,
            Self::Helpers,
            Self::SpaceWarps,
            Self::IoObjects,
            Self::Bones,
            Self::IkChains,
            Self::Points,
            Self::Custom,
        ]
    }
}

/// Selection Region Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SelectionRegionMode {
    #[default]
    Rectangular,
    Circular,
    Fence,
    Lasso,
    Paint,
}

impl SelectionRegionMode {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Rectangular => "Rectangular Region",
            Self::Circular => "Circular Region",
            Self::Fence => "Fence Region",
            Self::Lasso => "Lasso Region",
            Self::Paint => "Paint Selection Region",
        }
    }
}

/// Transform Gizmo / Tool Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TransformToolMode {
    #[default]
    Select,
    Move,
    Rotate,
    ScaleUniform,
    ScaleNonUniform,
    ScaleSquash,
    SelectAndPlace,
    SelectAndManipulate,
}

/// Reference Coordinate System options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CoordSystem {
    #[default]
    View,
    Screen,
    World,
    Parent,
    Local,
    Gimbal,
    Grid,
    Working,
}

impl CoordSystem {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::View => "View",
            Self::Screen => "Screen",
            Self::World => "World",
            Self::Parent => "Parent",
            Self::Local => "Local",
            Self::Gimbal => "Gimbal",
            Self::Grid => "Grid",
            Self::Working => "Working Pivot",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::View,
            Self::Screen,
            Self::World,
            Self::Parent,
            Self::Local,
            Self::Gimbal,
            Self::Grid,
            Self::Working,
        ]
    }
}

/// Transform Center Pivot Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TransformCenterMode {
    #[default]
    PivotPointCenter,
    SelectionCenter,
    TransformCoordCenter,
}

/// Snaps Configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapsState {
    pub grid_snap: bool,
    pub angle_snap: bool,
    pub percent_snap: bool,
    pub spinner_snap: bool,
}

impl Default for SnapsState {
    fn default() -> Self {
        Self {
            grid_snap: false,
            angle_snap: true,
            percent_snap: false,
            spinner_snap: true,
        }
    }
}

/// Main Toolbar State Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DccMainToolbarModel {
    pub selection_filter: SelectionFilter,
    pub region_mode: SelectionRegionMode,
    pub window_crossing_toggle: bool,
    pub transform_tool: TransformToolMode,
    pub coord_system: CoordSystem,
    pub center_mode: TransformCenterMode,
    pub snaps: SnapsState,
    pub active_selection_set: Option<String>,
    pub named_selection_sets: Vec<String>,
    pub isolate_selection_active: bool,
    pub working_pivot_active: bool,
    pub show_massfx_toolbar: bool,
    pub show_anim_layers_toolbar: bool,
    pub show_brush_presets_toolbar: bool,
}

impl Default for DccMainToolbarModel {
    fn default() -> Self {
        Self::new()
    }
}

impl DccMainToolbarModel {
    /// Create new DCC Main Toolbar Model.
    #[must_use]
    pub fn new() -> Self {
        Self {
            selection_filter: SelectionFilter::All,
            region_mode: SelectionRegionMode::Rectangular,
            window_crossing_toggle: false,
            transform_tool: TransformToolMode::Select,
            coord_system: CoordSystem::View,
            center_mode: TransformCenterMode::PivotPointCenter,
            snaps: SnapsState::default(),
            active_selection_set: None,
            named_selection_sets: vec![
                "Main_Body".to_string(),
                "Wheels".to_string(),
                "Lights".to_string(),
            ],
            isolate_selection_active: false,
            working_pivot_active: false,
            show_massfx_toolbar: false,
            show_anim_layers_toolbar: false,
            show_brush_presets_toolbar: false,
        }
    }
}
