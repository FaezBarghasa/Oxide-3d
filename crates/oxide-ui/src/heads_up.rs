//! Heads-up View Toolbar, Task Pane, and Shortcut Bar models.

use serde::{Deserialize, Serialize};

/// Display Style modes for the 3D Viewport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DisplayStyle {
    /// Shaded with visible polygon/feature edges.
    #[default]
    ShadedWithEdges,
    /// Shaded surface without black edges.
    Shaded,
    /// Hidden Lines Removed (HLR).
    HiddenLinesRemoved,
    /// Hidden Lines Visible as dashed curves (HLV).
    HiddenLinesVisible,
    /// Pure wireframe line display.
    Wireframe,
}

/// Heads-up View Toolbar State.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadsUpToolbarModel {
    /// Current display style.
    pub display_style: DisplayStyle,
    /// Perspective camera vs Orthographic projection.
    pub is_perspective: bool,
    /// Shadows in shaded mode.
    pub shadows: bool,
    /// Ambient occlusion pass.
    pub ambient_occlusion: bool,
    /// Show reference planes in viewport.
    pub show_planes: bool,
    /// Show reference axes.
    pub show_axes: bool,
    /// Show coordinate origin.
    pub show_origin: bool,
    /// Show 2D/3D sketches.
    pub show_sketches: bool,
    /// Active section view clipping plane.
    pub is_section_view: bool,
}

impl Default for HeadsUpToolbarModel {
    fn default() -> Self {
        Self {
            display_style: DisplayStyle::ShadedWithEdges,
            is_perspective: false,
            shadows: true,
            ambient_occlusion: true,
            show_planes: true,
            show_axes: false,
            show_origin: true,
            show_sketches: true,
            is_section_view: false,
        }
    }
}

/// Task Pane Tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TaskPaneTab {
    /// Standard parts & fasteners library.
    #[default]
    DesignLibrary,
    /// Local file explorer.
    FileExplorer,
    /// Orthographic drawing views palette.
    ViewPalette,
    /// Materials, Appearances, Scenes, Decals.
    AppearancesScenesDecals,
    /// Custom metadata properties.
    CustomProperties,
    /// SOLIDWORKS Forum / Community.
    Community,
}

/// Task Pane State Model.
#[derive(Debug, Clone, Default)]
pub struct TaskPaneModel {
    /// Active selected tab in Task Pane.
    pub active_tab: TaskPaneTab,
    /// Whether task pane is expanded/pinned open.
    pub is_expanded: bool,
}

/// Quick "S" Shortcut Bar Model.
#[derive(Debug, Clone)]
pub struct ShortcutBarModel {
    /// Is shortcut bar popup currently open.
    pub is_open: bool,
    /// Screen coordinate position where "S" was pressed.
    pub position: [f32; 2],
    /// Quick command IDs shown in shortcut grid.
    pub command_ids: Vec<&'static str>,
}

impl Default for ShortcutBarModel {
    fn default() -> Self {
        Self {
            is_open: false,
            position: [0.0, 0.0],
            command_ids: vec![
                "sketch.line",
                "sketch.rect",
                "sketch.circle",
                "sketch.smart_dim",
                "cmd.extrude",
                "cmd.revolve",
                "cmd.fillet",
                "cmd.chamfer",
                "eval.measure",
            ],
        }
    }
}
