//! Preferences Dialog, Quad Menus, and Complete Keyboard Shortcuts Catalog for Oxide-3D DCC.

use serde::{Deserialize, Serialize};

/// 13 Tabs of the DCC Preferences Dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PreferencesTab {
    #[default]
    General,
    Files,
    Viewports,
    InteractionMode,
    ColorManagement,
    Rendering,
    Animation,
    InverseKinematics,
    Gizmos,
    MaxScript,
    Snaps,
    Radiosity,
    Arnold,
}

impl PreferencesTab {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Files => "Files",
            Self::Viewports => "Viewports",
            Self::InteractionMode => "Interaction Mode",
            Self::ColorManagement => "Color Management",
            Self::Rendering => "Rendering",
            Self::Animation => "Animation",
            Self::InverseKinematics => "Inverse Kinematics",
            Self::Gizmos => "Gizmos",
            Self::MaxScript => "MAXScript",
            Self::Snaps => "Snaps",
            Self::Radiosity => "Radiosity",
            Self::Arnold => "Arnold",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::General,
            Self::Files,
            Self::Viewports,
            Self::InteractionMode,
            Self::ColorManagement,
            Self::Rendering,
            Self::Animation,
            Self::InverseKinematics,
            Self::Gizmos,
            Self::MaxScript,
            Self::Snaps,
            Self::Radiosity,
            Self::Arnold,
        ]
    }
}

/// Interaction Mode Preset (3ds Max style vs Maya style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum InteractionModePreset {
    #[default]
    Dcc3dsMax,
    Maya,
    IndustryCompatible,
}

/// Quad Menu Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum QuadMenuKind {
    #[default]
    Modeling,
    Animation,
    Snap,
    Lighting,
    Custom,
    Viewports,
    Windows,
}

/// Quad Menu Quadrant Definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadMenuQuadrant {
    pub title: String,
    pub items: Vec<String>,
}

/// Quad Menu Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuadMenuModel {
    pub kind: QuadMenuKind,
    pub top_left: QuadMenuQuadrant,
    pub top_right: QuadMenuQuadrant,
    pub bottom_left: QuadMenuQuadrant,
    pub bottom_right: QuadMenuQuadrant,
    pub is_visible: bool,
    pub screen_position: [f32; 2],
}

impl Default for QuadMenuModel {
    fn default() -> Self {
        Self {
            kind: QuadMenuKind::Modeling,
            top_left: QuadMenuQuadrant {
                title: "View / Display".to_string(),
                items: vec![
                    "Unhide by Name".to_string(),
                    "Unhide All".to_string(),
                    "Freeze Selected".to_string(),
                ],
            },
            top_right: QuadMenuQuadrant {
                title: "Transform".to_string(),
                items: vec![
                    "Move".to_string(),
                    "Rotate".to_string(),
                    "Scale".to_string(),
                    "Select".to_string(),
                ],
            },
            bottom_left: QuadMenuQuadrant {
                title: "Convert".to_string(),
                items: vec![
                    "Convert to Editable Poly".to_string(),
                    "Convert to Editable Mesh".to_string(),
                ],
            },
            bottom_right: QuadMenuQuadrant {
                title: "Edit Poly Tools".to_string(),
                items: vec![
                    "Extrude".to_string(),
                    "Bevel".to_string(),
                    "Inset".to_string(),
                    "Chamfer".to_string(),
                ],
            },
            is_visible: false,
            screen_position: [0.0, 0.0],
        }
    }
}

/// Shortcut Key Binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutBinding {
    pub key: String,
    pub action_id: String,
    pub description: String,
}

/// Preferences & Shortcuts Catalog Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencesShortcutsModel {
    pub active_tab: PreferencesTab,
    pub interaction_mode: InteractionModePreset,
    pub auto_backup_interval_mins: u32,
    pub undo_levels: u32,
    pub quad_menu: QuadMenuModel,
    pub shortcuts: Vec<ShortcutBinding>,
}

impl Default for PreferencesShortcutsModel {
    fn default() -> Self {
        Self::new()
    }
}

impl PreferencesShortcutsModel {
    #[must_use]
    pub fn new() -> Self {
        let shortcuts = vec![
            ShortcutBinding {
                key: "Ctrl+Z".to_string(),
                action_id: "edit_undo".to_string(),
                description: "Undo Operation".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+Y".to_string(),
                action_id: "edit_redo".to_string(),
                description: "Redo Operation".to_string(),
            },
            ShortcutBinding {
                key: "W".to_string(),
                action_id: "tool_move".to_string(),
                description: "Select and Move".to_string(),
            },
            ShortcutBinding {
                key: "E".to_string(),
                action_id: "tool_rotate".to_string(),
                description: "Select and Rotate".to_string(),
            },
            ShortcutBinding {
                key: "R".to_string(),
                action_id: "tool_scale".to_string(),
                description: "Select and Scale".to_string(),
            },
            ShortcutBinding {
                key: "Q".to_string(),
                action_id: "tool_select".to_string(),
                description: "Select Object".to_string(),
            },
            ShortcutBinding {
                key: "H".to_string(),
                action_id: "select_by_name".to_string(),
                description: "Select by Name".to_string(),
            },
            ShortcutBinding {
                key: "M".to_string(),
                action_id: "mat_compact_editor".to_string(),
                description: "Material Editor".to_string(),
            },
            ShortcutBinding {
                key: "F9".to_string(),
                action_id: "render_frame".to_string(),
                description: "Render Frame".to_string(),
            },
            ShortcutBinding {
                key: "F10".to_string(),
                action_id: "render_setup".to_string(),
                description: "Render Setup".to_string(),
            },
            ShortcutBinding {
                key: "F11".to_string(),
                action_id: "script_listener".to_string(),
                description: "MAXScript Listener".to_string(),
            },
            ShortcutBinding {
                key: "F12".to_string(),
                action_id: "transform_type_in".to_string(),
                description: "Transform Type-In Dialog".to_string(),
            },
            ShortcutBinding {
                key: "Alt+W".to_string(),
                action_id: "viewport_maximize".to_string(),
                description: "Maximize Viewport Toggle".to_string(),
            },
            ShortcutBinding {
                key: "Alt+Q".to_string(),
                action_id: "tools_isolate_selection".to_string(),
                description: "Isolate Selection".to_string(),
            },
            ShortcutBinding {
                key: "F3".to_string(),
                action_id: "shade_selected".to_string(),
                description: "Wireframe / Shaded Toggle".to_string(),
            },
            ShortcutBinding {
                key: "F4".to_string(),
                action_id: "edged_faces".to_string(),
                description: "Edged Faces Toggle".to_string(),
            },
            ShortcutBinding {
                key: "G".to_string(),
                action_id: "grid_toggle".to_string(),
                description: "Grid Visibility Toggle".to_string(),
            },
            ShortcutBinding {
                key: "S".to_string(),
                action_id: "snap_toggle".to_string(),
                description: "Snap Toggle".to_string(),
            },
            ShortcutBinding {
                key: "A".to_string(),
                action_id: "angle_snap_toggle".to_string(),
                description: "Angle Snap Toggle".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+N".to_string(),
                action_id: "file_new".to_string(),
                description: "New Scene".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+O".to_string(),
                action_id: "file_open".to_string(),
                description: "Open Scene".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+S".to_string(),
                action_id: "file_save".to_string(),
                description: "Save Scene".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+Shift+S".to_string(),
                action_id: "file_save_as".to_string(),
                description: "Save Scene As".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+Alt+S".to_string(),
                action_id: "file_save_incremental".to_string(),
                description: "Save Incremental".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+A".to_string(),
                action_id: "edit_select_all".to_string(),
                description: "Select All".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+D".to_string(),
                action_id: "edit_select_none".to_string(),
                description: "Select None".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+I".to_string(),
                action_id: "edit_select_invert".to_string(),
                description: "Select Invert".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+V".to_string(),
                action_id: "edit_clone".to_string(),
                description: "Clone".to_string(),
            },
            ShortcutBinding {
                key: "Del".to_string(),
                action_id: "edit_delete".to_string(),
                description: "Delete".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+H".to_string(),
                action_id: "edit_hold".to_string(),
                description: "Hold Scene".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+Alt+F".to_string(),
                action_id: "edit_fetch".to_string(),
                description: "Fetch Scene".to_string(),
            },
            ShortcutBinding {
                key: "Alt+A".to_string(),
                action_id: "tools_align".to_string(),
                description: "Align Tool".to_string(),
            },
            ShortcutBinding {
                key: "Alt+N".to_string(),
                action_id: "tools_normal_align".to_string(),
                description: "Normal Align Tool".to_string(),
            },
            ShortcutBinding {
                key: "Shift+Q".to_string(),
                action_id: "render_production".to_string(),
                description: "Render Production".to_string(),
            },
            ShortcutBinding {
                key: "Shift+F".to_string(),
                action_id: "views_show_safe_frame".to_string(),
                description: "Safe Frame Toggle".to_string(),
            },
            ShortcutBinding {
                key: "Shift+T".to_string(),
                action_id: "file_asset_tracking".to_string(),
                description: "Asset Tracking Toggle".to_string(),
            },
            ShortcutBinding {
                key: "Ctrl+X".to_string(),
                action_id: "views_expert_mode".to_string(),
                description: "Expert Mode Toggle".to_string(),
            },
        ];
        Self {
            active_tab: PreferencesTab::General,
            interaction_mode: InteractionModePreset::Dcc3dsMax,
            auto_backup_interval_mins: 5,
            undo_levels: 100,
            quad_menu: QuadMenuModel::default(),
            shortcuts,
        }
    }
}
