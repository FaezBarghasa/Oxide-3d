//! Complete 13-Menu Bar System for DCC / 3D Digital Content Creation in Oxide-3D.

use serde::{Deserialize, Serialize};

/// The 13 main menu categories in the DCC menu bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DccMenuCategory {
    File,
    Edit,
    Tools,
    Group,
    Views,
    Create,
    Modifiers,
    Animation,
    GraphEditors,
    Rendering,
    Customize,
    MaxScript,
    Help,
}

impl DccMenuCategory {
    /// Return human-readable label for menu category.
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::Tools => "Tools",
            Self::Group => "Group",
            Self::Views => "Views",
            Self::Create => "Create",
            Self::Modifiers => "Modifiers",
            Self::Animation => "Animation",
            Self::GraphEditors => "Graph Editors",
            Self::Rendering => "Rendering",
            Self::Customize => "Customize",
            Self::MaxScript => "MAXScript",
            Self::Help => "Help",
        }
    }

    /// All 13 menu categories in order.
    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::File,
            Self::Edit,
            Self::Tools,
            Self::Group,
            Self::Views,
            Self::Create,
            Self::Modifiers,
            Self::Animation,
            Self::GraphEditors,
            Self::Rendering,
            Self::Customize,
            Self::MaxScript,
            Self::Help,
        ]
    }
}

/// Item definition within a DCC Menu or Submenu.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DccMenuItemDef {
    /// Action Identifier.
    pub id: String,
    /// Display Label.
    pub label: String,
    /// Optional shortcut label (e.g. "Ctrl+S", "F10", "Alt+A").
    pub shortcut: Option<String>,
    /// Whether item is a separator.
    pub is_separator: bool,
    /// Nested sub-items if this is a submenu.
    pub children: Vec<Self>,
    /// Whether the menu item is checked/active.
    pub is_checked: bool,
    /// Whether the menu item is enabled.
    pub is_enabled: bool,
}

impl DccMenuItemDef {
    /// Create a standard command item.
    #[must_use]
    pub fn command(
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: shortcut.map(ToString::to_string),
            is_separator: false,
            children: Vec::new(),
            is_checked: false,
            is_enabled: true,
        }
    }

    /// Create a toggleable / checkable item.
    #[must_use]
    pub fn checkable(
        id: impl Into<String>,
        label: impl Into<String>,
        shortcut: Option<&str>,
        checked: bool,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: shortcut.map(ToString::to_string),
            is_separator: false,
            children: Vec::new(),
            is_checked: checked,
            is_enabled: true,
        }
    }

    /// Create a submenu with children items.
    #[must_use]
    pub fn submenu(label: impl Into<String>, children: Vec<Self>) -> Self {
        let lbl = label.into();
        Self {
            id: format!("submenu_{lbl}"),
            label: lbl,
            shortcut: None,
            is_separator: false,
            children,
            is_checked: false,
            is_enabled: true,
        }
    }

    /// Create a visual separator.
    #[must_use]
    pub fn separator() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            shortcut: None,
            is_separator: true,
            children: Vec::new(),
            is_checked: false,
            is_enabled: true,
        }
    }
}

/// The complete 13-Menu Bar model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DccMenuBarModel {
    /// Active open menu category, if any.
    pub active_menu: Option<DccMenuCategory>,
}

impl Default for DccMenuBarModel {
    fn default() -> Self {
        Self::new()
    }
}

impl DccMenuBarModel {
    /// Initialize the DCC Menu Bar model.
    #[must_use]
    pub const fn new() -> Self {
        Self { active_menu: None }
    }

    /// Open or toggle a menu.
    pub fn toggle_menu(&mut self, cat: DccMenuCategory) {
        if self.active_menu == Some(cat) {
            self.active_menu = None;
        } else {
            self.active_menu = Some(cat);
        }
    }

    /// Close active menu dropdown.
    pub fn close(&mut self) {
        self.active_menu = None;
    }

    /// Retrieve the full menu tree for a given category.
    #[must_use]
    pub fn get_menu_items(&self, cat: DccMenuCategory) -> Vec<DccMenuItemDef> {
        match cat {
            DccMenuCategory::File => Self::build_file_menu(),
            DccMenuCategory::Edit => Self::build_edit_menu(),
            DccMenuCategory::Tools => Self::build_tools_menu(),
            DccMenuCategory::Group => Self::build_group_menu(),
            DccMenuCategory::Views => Self::build_views_menu(),
            DccMenuCategory::Create => Self::build_create_menu(),
            DccMenuCategory::Modifiers => Self::build_modifiers_menu(),
            DccMenuCategory::Animation => Self::build_animation_menu(),
            DccMenuCategory::GraphEditors => Self::build_graph_editors_menu(),
            DccMenuCategory::Rendering => Self::build_rendering_menu(),
            DccMenuCategory::Customize => Self::build_customize_menu(),
            DccMenuCategory::MaxScript => Self::build_maxscript_menu(),
            DccMenuCategory::Help => Self::build_help_menu(),
        }
    }

    // 1.1 File Menu
    fn build_file_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("file_new", "New", Some("Ctrl+N")),
            DccMenuItemDef::command("file_reset", "Reset", None),
            DccMenuItemDef::command("file_open", "Open...", Some("Ctrl+O")),
            DccMenuItemDef::submenu(
                "Open Recent",
                vec![
                    DccMenuItemDef::command("file_recent_1", "Scene_01.max", None),
                    DccMenuItemDef::command("file_recent_2", "Mechanical_Part.max", None),
                    DccMenuItemDef::command("file_recent_3", "Character_Rig.max", None),
                ],
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("file_save", "Save", Some("Ctrl+S")),
            DccMenuItemDef::command("file_save_as", "Save As...", Some("Ctrl+Shift+S")),
            DccMenuItemDef::command("file_save_copy_as", "Save Copy As...", None),
            DccMenuItemDef::command("file_save_selected", "Save Selected...", None),
            DccMenuItemDef::command(
                "file_save_incremental",
                "Save Incremental",
                Some("Ctrl+Alt+S"),
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("file_archive", "Archive...", None),
            DccMenuItemDef::command("file_summary_info", "Summary Info...", None),
            DccMenuItemDef::command("file_properties", "File Properties...", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("file_import", "Import...", None),
            DccMenuItemDef::command("file_merge", "Merge...", None),
            DccMenuItemDef::command("file_replace", "Replace...", None),
            DccMenuItemDef::command("file_export", "Export...", None),
            DccMenuItemDef::command("file_export_selected", "Export Selected...", None),
            DccMenuItemDef::command("file_export_scene_as", "Export Scene As...", None),
            DccMenuItemDef::submenu(
                "Send To",
                vec![
                    DccMenuItemDef::command("send_to_my_computer", "Send To My Computer", None),
                    DccMenuItemDef::command("send_to_network", "Send To Network Folder", None),
                    DccMenuItemDef::command("send_to_backburner", "Send To Backburner", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Reference",
                vec![
                    DccMenuItemDef::command("ref_xref_objects", "XRef Objects...", None),
                    DccMenuItemDef::command("ref_xref_scene", "XRef Scene...", None),
                    DccMenuItemDef::command("ref_files", "Files...", None),
                    DccMenuItemDef::command("ref_batch_load", "Batch File Load...", None),
                    DccMenuItemDef::command("ref_object", "Object...", None),
                    DccMenuItemDef::command("ref_scene", "Scene...", None),
                    DccMenuItemDef::command("ref_proxies", "Proxies...", None),
                    DccMenuItemDef::command("ref_containers", "Containers...", None),
                ],
            ),
            DccMenuItemDef::command("file_manage_links", "Manage Links...", None),
            DccMenuItemDef::command("file_asset_tracking", "Asset Tracking...", Some("Shift+T")),
            DccMenuItemDef::command("file_project_folder", "Project Folder...", None),
            DccMenuItemDef::command("file_view_image", "View Image File...", None),
            DccMenuItemDef::command("file_batch_render", "Batch Render...", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("file_exit", "Exit", None),
        ]
    }

    // 1.2 Edit Menu
    fn build_edit_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("edit_undo", "Undo", Some("Ctrl+Z")),
            DccMenuItemDef::command("edit_redo", "Redo", Some("Ctrl+Y")),
            DccMenuItemDef::command("edit_history", "Undo/Redo History...", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("edit_hold", "Hold", Some("Ctrl+H")),
            DccMenuItemDef::command("edit_fetch", "Fetch", Some("Ctrl+Alt+F")),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("edit_delete", "Delete", Some("Del")),
            DccMenuItemDef::command("edit_clone", "Clone", Some("Ctrl+V")),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("edit_select_all", "Select All", Some("Ctrl+A")),
            DccMenuItemDef::command("edit_select_none", "Select None", Some("Ctrl+D")),
            DccMenuItemDef::command("edit_select_invert", "Select Invert", Some("Ctrl+I")),
            DccMenuItemDef::submenu(
                "Select By",
                vec![
                    DccMenuItemDef::command("select_by_color", "Color...", None),
                    DccMenuItemDef::command("select_by_name", "Name...", None),
                    DccMenuItemDef::command("select_by_layer", "Layer...", None),
                    DccMenuItemDef::command("select_by_material", "Material...", None),
                    DccMenuItemDef::command("select_by_wire_color", "Wireframe Color...", None),
                ],
            ),
            DccMenuItemDef::command("edit_select_from_scene", "Select From Scene...", Some("H")),
            DccMenuItemDef::submenu(
                "Selection Region",
                vec![
                    DccMenuItemDef::checkable("region_rect", "Rectangular Region", None, true),
                    DccMenuItemDef::checkable("region_circ", "Circular Region", None, false),
                    DccMenuItemDef::checkable("region_fence", "Fence Region", None, false),
                    DccMenuItemDef::checkable("region_lasso", "Lasso Region", None, false),
                    DccMenuItemDef::checkable("region_paint", "Paint Region", None, false),
                ],
            ),
            DccMenuItemDef::command(
                "edit_manage_selection_sets",
                "Manage Selection Sets...",
                None,
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("edit_object_properties", "Object Properties...", None),
        ]
    }

    // 1.3 Tools Menu
    fn build_tools_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("tools_transform_toolbox", "Transform Toolbox...", None),
            DccMenuItemDef::command("tools_align", "Align", Some("Alt+A")),
            DccMenuItemDef::command("tools_mirror", "Mirror...", None),
            DccMenuItemDef::command("tools_array", "Array...", None),
            DccMenuItemDef::command("tools_snapshot", "Snapshot...", None),
            DccMenuItemDef::command("tools_spacing_tool", "Spacing Tool...", None),
            DccMenuItemDef::command("tools_clone_and_align", "Clone and Align...", None),
            DccMenuItemDef::command("tools_normal_align", "Normal Align...", Some("Alt+N")),
            DccMenuItemDef::command("tools_align_camera", "Align Camera...", None),
            DccMenuItemDef::command("tools_align_to_view", "Align to View...", None),
            DccMenuItemDef::command("tools_place_highlight", "Place Highlight", Some("Ctrl+H")),
            DccMenuItemDef::command(
                "tools_isolate_selection",
                "Isolate Selection",
                Some("Alt+Q"),
            ),
            DccMenuItemDef::submenu(
                "Hide",
                vec![
                    DccMenuItemDef::command("hide_selected", "Hide Selected", None),
                    DccMenuItemDef::command("hide_unselected", "Hide Unselected", None),
                    DccMenuItemDef::command("hide_by_name", "Hide by Name...", None),
                    DccMenuItemDef::command("hide_by_hit", "Hide by Hit", None),
                    DccMenuItemDef::command("unhide_all", "Unhide All", None),
                    DccMenuItemDef::command("unhide_by_name", "Unhide by Name...", None),
                    DccMenuItemDef::command("hide_frozen_objects", "Hide Frozen Objects", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Freeze",
                vec![
                    DccMenuItemDef::command("freeze_selected", "Freeze Selected", None),
                    DccMenuItemDef::command("freeze_unselected", "Freeze Unselected", None),
                    DccMenuItemDef::command("freeze_by_name", "Freeze by Name...", None),
                    DccMenuItemDef::command("freeze_by_hit", "Freeze by Hit", None),
                    DccMenuItemDef::command("unfreeze_all", "Unfreeze All", None),
                    DccMenuItemDef::command("unfreeze_by_name", "Unfreeze by Name...", None),
                    DccMenuItemDef::command("unfreeze_by_hit", "Unfreeze by Hit", None),
                ],
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("tools_display_floater", "Display Floater...", None),
            DccMenuItemDef::command("tools_selection_floater", "Selection Floater...", None),
            DccMenuItemDef::command("tools_layer_manager", "Layer Manager...", None),
            DccMenuItemDef::command("tools_manage_layers", "Manage Layers...", None),
            DccMenuItemDef::command("tools_scene_explorer", "Scene Explorer", None),
            DccMenuItemDef::command("tools_material_explorer", "Material Explorer", None),
            DccMenuItemDef::command("tools_light_lister", "Light Lister...", None),
            DccMenuItemDef::command("tools_channel_info", "Channel Info...", None),
            DccMenuItemDef::command("tools_measure_distance", "Measure Distance...", None),
            DccMenuItemDef::command("tools_measure", "Measure...", None),
            DccMenuItemDef::command("tools_reset_xform", "Reset XForm", None),
            DccMenuItemDef::command("tools_clean_multimaterial", "Clean MultiMaterial...", None),
            DccMenuItemDef::command("tools_color_clipboard", "Color Clipboard...", None),
            DccMenuItemDef::command("tools_motion_capture", "Motion Capture...", None),
            DccMenuItemDef::command("tools_perspective_match", "Perspective Match...", None),
            DccMenuItemDef::command("tools_camera_tracker", "Camera Tracker...", None),
            DccMenuItemDef::command("tools_grab_viewport", "Grab Viewport...", None),
            DccMenuItemDef::command(
                "tools_grab_viewport_selected",
                "Grab Viewport (Selected)...",
                None,
            ),
        ]
    }

    // 1.4 Group Menu
    fn build_group_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("group_group", "Group...", None),
            DccMenuItemDef::command("group_ungroup", "Ungroup", None),
            DccMenuItemDef::command("group_open", "Open", None),
            DccMenuItemDef::command("group_close", "Close", None),
            DccMenuItemDef::command("group_attach", "Attach", None),
            DccMenuItemDef::command("group_detach", "Detach", None),
            DccMenuItemDef::command("group_explode", "Explode", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::submenu(
                "Assembly",
                vec![
                    DccMenuItemDef::command("assembly_assemble", "Assemble...", None),
                    DccMenuItemDef::command("assembly_disassemble", "Disassemble...", None),
                    DccMenuItemDef::command("assembly_open", "Open...", None),
                    DccMenuItemDef::command("assembly_close", "Close...", None),
                    DccMenuItemDef::command("assembly_explode", "Explode...", None),
                    DccMenuItemDef::command("assembly_connect", "Connect...", None),
                    DccMenuItemDef::command("assembly_disconnect", "Disconnect...", None),
                    DccMenuItemDef::command("assembly_insert", "Insert...", None),
                    DccMenuItemDef::command("assembly_extract", "Extract...", None),
                    DccMenuItemDef::command("assembly_reset_transform", "Reset Transform...", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Collect",
                vec![
                    DccMenuItemDef::command("collect_collect", "Collect...", None),
                    DccMenuItemDef::command("collect_remove", "Remove from Collection", None),
                ],
            ),
        ]
    }

    // 1.5 Views Menu
    fn build_views_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("views_undo_view", "Undo View Change", Some("Shift+Z")),
            DccMenuItemDef::command("views_redo_view", "Redo View Change", Some("Shift+Y")),
            DccMenuItemDef::command("views_save_active_view", "Save Active View...", None),
            DccMenuItemDef::command("views_restore_active_view", "Restore Active View...", None),
            DccMenuItemDef::command("views_viewport_config", "Viewport Configuration...", None),
            DccMenuItemDef::submenu(
                "Grids",
                vec![
                    DccMenuItemDef::checkable("grid_show_home", "Show Home Grid", Some("G"), true),
                    DccMenuItemDef::command("grid_activate_home", "Activate Home Grid", None),
                    DccMenuItemDef::command("grid_activate_object", "Activate Grid Object", None),
                    DccMenuItemDef::command("grid_align_to_view", "Align to View...", None),
                    DccMenuItemDef::command("grid_create_default", "Create Default Grids", None),
                    DccMenuItemDef::command(
                        "grid_snap_settings",
                        "Grid and Snap Settings...",
                        None,
                    ),
                ],
            ),
            DccMenuItemDef::command(
                "views_viewport_background",
                "Viewport Background...",
                Some("Alt+B"),
            ),
            DccMenuItemDef::checkable("views_show_gizmo", "Show Transform Gizmo", Some("X"), true),
            DccMenuItemDef::checkable("views_show_ghosting", "Show Ghosting", None, false),
            DccMenuItemDef::checkable("views_show_key_times", "Show Key Times", None, true),
            DccMenuItemDef::checkable("views_shade_selected", "Shade Selected", Some("F3"), false),
            DccMenuItemDef::checkable(
                "views_show_edged_faces",
                "Show Edged Faces",
                Some("F4"),
                true,
            ),
            DccMenuItemDef::checkable(
                "views_show_safe_frame",
                "Show Safe Frame",
                Some("Shift+F"),
                false,
            ),
            DccMenuItemDef::checkable("views_show_statistics", "Show Statistics", None, true),
            DccMenuItemDef::submenu(
                "Show Materials in Viewport As",
                vec![
                    DccMenuItemDef::checkable(
                        "mat_realistic_maps",
                        "Realistic Materials with Maps",
                        None,
                        true,
                    ),
                    DccMenuItemDef::checkable(
                        "mat_shaded_maps",
                        "Shaded Materials with Maps",
                        None,
                        false,
                    ),
                    DccMenuItemDef::checkable("mat_realistic", "Realistic Materials", None, false),
                    DccMenuItemDef::checkable("mat_shaded", "Shaded Materials", None, false),
                    DccMenuItemDef::checkable("mat_consistent", "Consistent Colors", None, false),
                    DccMenuItemDef::checkable("mat_clay", "Clay", None, false),
                    DccMenuItemDef::checkable("mat_xray", "X-Ray", None, false),
                    DccMenuItemDef::checkable("mat_wireframe", "Wireframe", None, false),
                    DccMenuItemDef::checkable("mat_bbox", "Bounding Box", None, false),
                ],
            ),
            DccMenuItemDef::submenu(
                "xView",
                vec![
                    DccMenuItemDef::command(
                        "xview_overlapping_faces",
                        "Show Overlapping Faces",
                        None,
                    ),
                    DccMenuItemDef::command(
                        "xview_isolated_vertices",
                        "Show Isolated Vertices",
                        None,
                    ),
                    DccMenuItemDef::command("xview_open_edges", "Show Open Edges", None),
                    DccMenuItemDef::command(
                        "xview_multi_edged_faces",
                        "Show Multi-Edged Faces",
                        None,
                    ),
                    DccMenuItemDef::command("xview_unused_uvs", "Show Unused UVs", None),
                    DccMenuItemDef::command("xview_vertex_normals", "Show Vertex Normals", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "ViewCube",
                vec![
                    DccMenuItemDef::checkable("vc_show", "Show the ViewCube", None, true),
                    DccMenuItemDef::checkable("vc_compass", "Show the Compass", None, true),
                    DccMenuItemDef::command("vc_configure", "Configure...", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "SteeringWheels",
                vec![
                    DccMenuItemDef::checkable("sw_show", "Show SteeringWheels", None, false),
                    DccMenuItemDef::command("sw_configure", "Configure...", None),
                ],
            ),
            DccMenuItemDef::checkable(
                "views_progressive_display",
                "Progressive Display",
                None,
                true,
            ),
            DccMenuItemDef::command("views_expert_mode", "Expert Mode", Some("Ctrl+X")),
        ]
    }

    // 1.6 Create Menu
    fn build_create_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::submenu(
                "Standard Primitives",
                vec![
                    DccMenuItemDef::command("create_box", "Box", None),
                    DccMenuItemDef::command("create_sphere", "Sphere", None),
                    DccMenuItemDef::command("create_cylinder", "Cylinder", None),
                    DccMenuItemDef::command("create_torus", "Torus", None),
                    DccMenuItemDef::command("create_teapot", "Teapot", None),
                    DccMenuItemDef::command("create_cone", "Cone", None),
                    DccMenuItemDef::command("create_geosphere", "GeoSphere", None),
                    DccMenuItemDef::command("create_tube", "Tube", None),
                    DccMenuItemDef::command("create_pyramid", "Pyramid", None),
                    DccMenuItemDef::command("create_plane", "Plane", None),
                    DccMenuItemDef::submenu(
                        "Extended Primitives",
                        vec![
                            DccMenuItemDef::command("create_hedra", "Hedra", None),
                            DccMenuItemDef::command("create_torus_knot", "Torus Knot", None),
                            DccMenuItemDef::command("create_chamferbox", "ChamferBox", None),
                            DccMenuItemDef::command("create_chamfercyl", "ChamferCyl", None),
                            DccMenuItemDef::command("create_oiltank", "OilTank", None),
                            DccMenuItemDef::command("create_capsule", "Capsule", None),
                            DccMenuItemDef::command("create_spindle", "Spindle", None),
                            DccMenuItemDef::command("create_l_ext", "L-Ext", None),
                            DccMenuItemDef::command("create_gengon", "Gengon", None),
                            DccMenuItemDef::command("create_c_ext", "C-Ext", None),
                            DccMenuItemDef::command("create_prism", "Prism", None),
                            DccMenuItemDef::command("create_hose", "Hose", None),
                        ],
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "AEC Objects",
                vec![
                    DccMenuItemDef::command("create_foliage", "Foliage", None),
                    DccMenuItemDef::command("create_railing", "Railing", None),
                    DccMenuItemDef::command("create_wall", "Wall", None),
                    DccMenuItemDef::submenu(
                        "Stairs",
                        vec![
                            DccMenuItemDef::command("stairs_ltype", "LType", None),
                            DccMenuItemDef::command("stairs_spiral", "Spiral", None),
                            DccMenuItemDef::command("stairs_straight", "Straight", None),
                            DccMenuItemDef::command("stairs_utype", "UType", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Doors",
                        vec![
                            DccMenuItemDef::command("doors_pivot", "Pivot", None),
                            DccMenuItemDef::command("doors_sliding", "Sliding", None),
                            DccMenuItemDef::command("doors_bifold", "BiFold", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Windows",
                        vec![
                            DccMenuItemDef::command("win_awning", "Awning", None),
                            DccMenuItemDef::command("win_casement", "Casement", None),
                            DccMenuItemDef::command("win_fixed", "Fixed", None),
                            DccMenuItemDef::command("win_pivoted", "Pivoted", None),
                            DccMenuItemDef::command("win_projected", "Projected", None),
                            DccMenuItemDef::command("win_sliding", "Sliding", None),
                        ],
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "Compound",
                vec![
                    DccMenuItemDef::command("compound_morph", "Morph", None),
                    DccMenuItemDef::command("compound_scatter", "Scatter", None),
                    DccMenuItemDef::command("compound_conform", "Conform", None),
                    DccMenuItemDef::command("compound_connect", "Connect", None),
                    DccMenuItemDef::command("compound_shapemerge", "ShapeMerge", None),
                    DccMenuItemDef::command("compound_boolean", "Boolean", None),
                    DccMenuItemDef::command("compound_terrain", "Terrain", None),
                    DccMenuItemDef::command("compound_loft", "Loft", None),
                    DccMenuItemDef::command("compound_mesher", "Mesher", None),
                    DccMenuItemDef::command("compound_proboolean", "ProBoolean", None),
                    DccMenuItemDef::command("compound_procutter", "ProCutter", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Particles",
                vec![
                    DccMenuItemDef::command("particles_pflow", "Particle Flow", None),
                    DccMenuItemDef::command("particles_spray", "Spray", None),
                    DccMenuItemDef::command("particles_snow", "Snow", None),
                    DccMenuItemDef::command("particles_superspray", "Super Spray", None),
                    DccMenuItemDef::command("particles_blizzard", "Blizzard", None),
                    DccMenuItemDef::command("particles_parray", "PArray", None),
                    DccMenuItemDef::command("particles_pcloud", "PCloud", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Patch Grids",
                vec![
                    DccMenuItemDef::command("patch_quad", "Quad Patch", None),
                    DccMenuItemDef::command("patch_tri", "Tri Patch", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "NURBS",
                vec![
                    DccMenuItemDef::command("nurbs_pointsurf", "Point Surf", None),
                    DccMenuItemDef::command("nurbs_cvsurf", "CV Surf", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Dynamics",
                vec![
                    DccMenuItemDef::command("dynamics_reactor", "Reactor", None),
                    DccMenuItemDef::command("dynamics_massfx", "MassFX", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Shapes",
                vec![
                    DccMenuItemDef::submenu(
                        "Splines",
                        vec![
                            DccMenuItemDef::command("spline_line", "Line", None),
                            DccMenuItemDef::command("spline_rect", "Rectangle", None),
                            DccMenuItemDef::command("spline_circle", "Circle", None),
                            DccMenuItemDef::command("spline_ellipse", "Ellipse", None),
                            DccMenuItemDef::command("spline_arc", "Arc", None),
                            DccMenuItemDef::command("spline_donut", "Donut", None),
                            DccMenuItemDef::command("spline_ngon", "NGon", None),
                            DccMenuItemDef::command("spline_star", "Star", None),
                            DccMenuItemDef::command("spline_text", "Text", None),
                            DccMenuItemDef::command("spline_helix", "Helix", None),
                            DccMenuItemDef::command("spline_section", "Section", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Extended Splines",
                        vec![
                            DccMenuItemDef::command("ext_spline_wrect", "WRectangle", None),
                            DccMenuItemDef::command("ext_spline_channel", "Channel", None),
                            DccMenuItemDef::command("ext_spline_angle", "Angle", None),
                            DccMenuItemDef::command("ext_spline_tee", "Tee", None),
                            DccMenuItemDef::command("ext_spline_wflange", "Wide Flange", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "NURBS Curves",
                        vec![
                            DccMenuItemDef::command("nurbs_point_curve", "Point Curve", None),
                            DccMenuItemDef::command("nurbs_cv_curve", "CV Curve", None),
                        ],
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "Lights",
                vec![
                    DccMenuItemDef::submenu(
                        "Standard",
                        vec![
                            DccMenuItemDef::command("light_omni", "Omni", None),
                            DccMenuItemDef::command("light_target_spot", "Target Spot", None),
                            DccMenuItemDef::command("light_free_spot", "Free Spot", None),
                            DccMenuItemDef::command("light_target_direct", "Target Direct", None),
                            DccMenuItemDef::command("light_free_direct", "Free Direct", None),
                            DccMenuItemDef::command("light_skylight", "Skylight", None),
                            DccMenuItemDef::command("light_mr_area_omni", "mr Area Omni", None),
                            DccMenuItemDef::command("light_mr_area_spot", "mr Area Spot", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Photometric",
                        vec![
                            DccMenuItemDef::command("light_target_photo", "Target Light", None),
                            DccMenuItemDef::command("light_free_photo", "Free Light", None),
                            DccMenuItemDef::command("light_mr_sky_portal", "mr Sky Portal", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Arnold",
                        vec![DccMenuItemDef::command(
                            "light_arnold",
                            "Arnold Light",
                            None,
                        )],
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "Cameras",
                vec![
                    DccMenuItemDef::command("cam_free", "Free Camera", None),
                    DccMenuItemDef::command("cam_target", "Target Camera", None),
                    DccMenuItemDef::command("cam_physical", "Physical Camera", None),
                    DccMenuItemDef::command("cam_arnold", "Arnold Camera", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Helpers",
                vec![
                    DccMenuItemDef::submenu(
                        "Standard",
                        vec![
                            DccMenuItemDef::command("helper_dummy", "Dummy", None),
                            DccMenuItemDef::command("helper_grid", "Grid", None),
                            DccMenuItemDef::command("helper_point", "Point", None),
                            DccMenuItemDef::command("helper_tape", "Tape", None),
                            DccMenuItemDef::command("helper_protractor", "Protractor", None),
                            DccMenuItemDef::command("helper_compass", "Compass", None),
                            DccMenuItemDef::command("helper_box_gizmo", "Box Gizmo", None),
                            DccMenuItemDef::command("helper_cyl_gizmo", "Cylinder Gizmo", None),
                            DccMenuItemDef::command("helper_sphere_gizmo", "Sphere Gizmo", None),
                            DccMenuItemDef::command("helper_cam_point", "Camera Point", None),
                            DccMenuItemDef::command("helper_crowd", "Crowd", None),
                            DccMenuItemDef::command("helper_delegate", "Delegate", None),
                            DccMenuItemDef::command("helper_expose_tm", "Expose Transform", None),
                            DccMenuItemDef::command("helper_manipulator", "Manipulator", None),
                            DccMenuItemDef::command("helper_slider", "Slider", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Atmospheric Apparatus",
                        vec![
                            DccMenuItemDef::command("helper_boxgizmo_atm", "BoxGizmo", None),
                            DccMenuItemDef::command("helper_cylgizmo_atm", "CylGizmo", None),
                            DccMenuItemDef::command("helper_spheregizmo_atm", "SphereGizmo", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Camera Match",
                        vec![DccMenuItemDef::command(
                            "helper_campoint_match",
                            "CamPoint",
                            None,
                        )],
                    ),
                    DccMenuItemDef::submenu(
                        "Particle Flow",
                        vec![DccMenuItemDef::command(
                            "helper_pf_source",
                            "PF Source",
                            None,
                        )],
                    ),
                    DccMenuItemDef::submenu(
                        "VRML97",
                        vec![
                            DccMenuItemDef::command("vrml_anchor", "Anchor", None),
                            DccMenuItemDef::command("vrml_touch_sensor", "TouchSensor", None),
                            DccMenuItemDef::command("vrml_sound", "Sound", None),
                            DccMenuItemDef::command("vrml_fog", "Fog", None),
                            DccMenuItemDef::command("vrml_background", "Background", None),
                        ],
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "Space Warps",
                vec![
                    DccMenuItemDef::submenu(
                        "Forces",
                        vec![
                            DccMenuItemDef::command("warp_gravity", "Gravity", None),
                            DccMenuItemDef::command("warp_wind", "Wind", None),
                            DccMenuItemDef::command("warp_drag", "Drag", None),
                            DccMenuItemDef::command("warp_vortex", "Vortex", None),
                            DccMenuItemDef::command("warp_path_follow", "Path Follow", None),
                            DccMenuItemDef::command("warp_pbomb", "PBomb", None),
                            DccMenuItemDef::command("warp_displace", "Displace", None),
                            DccMenuItemDef::command("warp_noise", "Noise", None),
                            DccMenuItemDef::command("warp_push", "Push", None),
                            DccMenuItemDef::command("warp_motor", "Motor", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Deflectors",
                        vec![
                            DccMenuItemDef::command("deflect_parray", "PArray", None),
                            DccMenuItemDef::command("deflect_pomniflect", "POmniFlect", None),
                            DccMenuItemDef::command("deflect_sdeflector", "SDeflector", None),
                            DccMenuItemDef::command("deflect_udeflector", "UDeflector", None),
                            DccMenuItemDef::command("deflect_uomniflect", "UOmniFlect", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Geometric/Deformable",
                        vec![
                            DccMenuItemDef::command("warp_ffd_box", "FFD (Box)", None),
                            DccMenuItemDef::command("warp_ffd_cyl", "FFD (Cyl)", None),
                            DccMenuItemDef::command("warp_wave", "Wave", None),
                            DccMenuItemDef::command("warp_ripple", "Ripple", None),
                            DccMenuItemDef::command("warp_bend", "Bend", None),
                            DccMenuItemDef::command("warp_twist", "Twist", None),
                            DccMenuItemDef::command("warp_stretch", "Stretch", None),
                            DccMenuItemDef::command("warp_spherify", "Spherify", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Modifier-Based",
                        vec![
                            DccMenuItemDef::command("warp_mod_noise", "Noise", None),
                            DccMenuItemDef::command("warp_mod_displace", "Displace", None),
                        ],
                    ),
                    DccMenuItemDef::submenu(
                        "Reactor",
                        vec![
                            DccMenuItemDef::command("warp_reactor_water", "Water", None),
                            DccMenuItemDef::command("warp_reactor_wind", "Wind", None),
                        ],
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "Systems",
                vec![
                    DccMenuItemDef::command("system_sunlight", "Sunlight", None),
                    DccMenuItemDef::command("system_daylight", "Daylight", None),
                    DccMenuItemDef::command("system_biped", "Biped", None),
                    DccMenuItemDef::command("system_ring_array", "Ring Array", None),
                    DccMenuItemDef::command("system_bone_tools", "Bone Tools", None),
                    DccMenuItemDef::command("system_crowd", "Crowd", None),
                    DccMenuItemDef::command("system_cat_rig", "CAT Rig", None),
                ],
            ),
        ]
    }

    // 1.7 Modifiers Menu
    fn build_modifiers_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::submenu(
                "Mesh Editing",
                vec![
                    DccMenuItemDef::command("mod_edit_mesh", "Edit Mesh", None),
                    DccMenuItemDef::command("mod_editable_poly", "Editable Poly", None),
                    DccMenuItemDef::command("mod_edit_poly", "Edit Poly", None),
                    DccMenuItemDef::command("mod_vertex_weld", "Vertex Weld", None),
                    DccMenuItemDef::command("mod_vertex_paint", "Vertex Paint", None),
                    DccMenuItemDef::command("mod_projection", "Projection", None),
                    DccMenuItemDef::command("mod_subdivision", "Subdivision", None),
                    DccMenuItemDef::command("mod_turbosmooth", "TurboSmooth", None),
                    DccMenuItemDef::command("mod_meshsmooth", "MeshSmooth", None),
                    DccMenuItemDef::command("mod_hsds", "HSDS", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Patch/Spline Editing",
                vec![
                    DccMenuItemDef::command("mod_edit_patch", "Edit Patch", None),
                    DccMenuItemDef::command("mod_edit_spline", "Edit Spline", None),
                    DccMenuItemDef::command("mod_crosssection", "CrossSection", None),
                    DccMenuItemDef::command("mod_surface", "Surface", None),
                    DccMenuItemDef::command("mod_sweep", "Sweep", None),
                    DccMenuItemDef::command("mod_lathe", "Lathe", None),
                    DccMenuItemDef::command("mod_extrude", "Extrude", None),
                    DccMenuItemDef::command("mod_bevel", "Bevel", None),
                    DccMenuItemDef::command("mod_bevel_profile", "Bevel Profile", None),
                    DccMenuItemDef::command("mod_fillet_chamfer", "Fillet/Chamfer", None),
                    DccMenuItemDef::command("mod_trim_extend", "Trim/Extend", None),
                    DccMenuItemDef::command("mod_delete_patch", "Delete Patch", None),
                    DccMenuItemDef::command("mod_delete_spline", "Delete Spline", None),
                    DccMenuItemDef::command("mod_normalize_spline", "Normalize Spline", None),
                    DccMenuItemDef::command("mod_material_id", "Material ID", None),
                    DccMenuItemDef::command("mod_spline_ik", "Spline IK", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Conversion",
                vec![
                    DccMenuItemDef::command("mod_turn_to_mesh", "Turn to Mesh", None),
                    DccMenuItemDef::command("mod_turn_to_poly", "Turn to Poly", None),
                    DccMenuItemDef::command("mod_turn_to_patch", "Turn to Patch", None),
                    DccMenuItemDef::command("mod_turn_to_nurbs", "Turn to NURBS", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Animation Modifiers",
                vec![
                    DccMenuItemDef::command("mod_skin", "Skin", None),
                    DccMenuItemDef::command("mod_skin_wrap", "Skin Wrap", None),
                    DccMenuItemDef::command("mod_skin_morph", "Skin Morph", None),
                    DccMenuItemDef::command("mod_morpher", "Morpher", None),
                    DccMenuItemDef::command("mod_flex", "Flex", None),
                    DccMenuItemDef::command("mod_melt", "Melt", None),
                    DccMenuItemDef::command("mod_linked_xform", "Linked XForm", None),
                    DccMenuItemDef::command("mod_patchdeform", "PatchDeform", None),
                    DccMenuItemDef::command("mod_pathdeform", "PathDeform", None),
                    DccMenuItemDef::command("mod_surfdeform", "SurfDeform", None),
                    DccMenuItemDef::command("mod_splineik", "SplineIK", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Cloth",
                vec![
                    DccMenuItemDef::command("mod_cloth", "Cloth", None),
                    DccMenuItemDef::command("mod_garment_maker", "Garment Maker", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Hair and Fur",
                vec![
                    DccMenuItemDef::command("mod_hair_wsm", "Hair and Fur (WSM)", None),
                    DccMenuItemDef::command("mod_hair_os", "Hair and Fur (Object-Space)", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Free Form Deformers",
                vec![
                    DccMenuItemDef::command("mod_ffd_box", "FFD (Box)", None),
                    DccMenuItemDef::command("mod_ffd_cyl", "FFD (Cyl)", None),
                    DccMenuItemDef::command("mod_ffd_select", "FFD (Select)", None),
                    DccMenuItemDef::command("mod_ffd_box_wsm", "FFD (Box) (WSM)", None),
                    DccMenuItemDef::command("mod_ffd_cyl_wsm", "FFD (Cyl) (WSM)", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Parametric Deformers",
                vec![
                    DccMenuItemDef::command("mod_bend", "Bend", None),
                    DccMenuItemDef::command("mod_taper", "Taper", None),
                    DccMenuItemDef::command("mod_twist", "Twist", None),
                    DccMenuItemDef::command("mod_stretch", "Stretch", None),
                    DccMenuItemDef::command("mod_spherify", "Spherify", None),
                    DccMenuItemDef::command("mod_noise", "Noise", None),
                    DccMenuItemDef::command("mod_wave", "Wave", None),
                    DccMenuItemDef::command("mod_ripple", "Ripple", None),
                    DccMenuItemDef::command("mod_shell", "Shell", None),
                    DccMenuItemDef::command("mod_affect_region", "Affect Region", None),
                    DccMenuItemDef::command("mod_displace", "Displace", None),
                    DccMenuItemDef::command("mod_xform", "XForm", None),
                    DccMenuItemDef::command("mod_lattice", "Lattice", None),
                    DccMenuItemDef::command("mod_mirror", "Mirror", None),
                    DccMenuItemDef::command("mod_push", "Push", None),
                    DccMenuItemDef::command("mod_relax", "Relax", None),
                    DccMenuItemDef::command("mod_slice", "Slice", None),
                    DccMenuItemDef::command("mod_squeeze", "Squeeze", None),
                    DccMenuItemDef::command("mod_substitute", "Substitute", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Surface",
                vec![
                    DccMenuItemDef::command("mod_uvw_map", "UVW Map", None),
                    DccMenuItemDef::command("mod_unwrap_uvw", "Unwrap UVW", None),
                    DccMenuItemDef::command("mod_uvw_xform", "UVW Xform", None),
                    DccMenuItemDef::command("mod_camera_map", "Camera Map", None),
                    DccMenuItemDef::command("mod_normal_map", "Normal Map", None),
                    DccMenuItemDef::command("mod_mapscaler", "MapScaler", None),
                    DccMenuItemDef::command("mod_surface_mapper", "Surface Mapper", None),
                    DccMenuItemDef::command("mod_material_by_element", "MaterialByElement", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Cache Tools",
                vec![
                    DccMenuItemDef::command("mod_point_cache", "Point Cache", None),
                    DccMenuItemDef::command("mod_cache_file", "Cache File", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Arnold",
                vec![
                    DccMenuItemDef::command("mod_arnold_props", "Arnold Properties", None),
                    DccMenuItemDef::command("mod_arnold_disp", "Arnold Displacement", None),
                    DccMenuItemDef::command("mod_arnold_params", "Arnold Parameters", None),
                    DccMenuItemDef::command("mod_arnold_proc", "Arnold Procedural", None),
                    DccMenuItemDef::command("mod_arnold_vol", "Arnold Volume", None),
                ],
            ),
        ]
    }

    // 1.8 Animation Menu
    fn build_animation_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::submenu(
                "IK Solvers",
                vec![
                    DccMenuItemDef::command("ik_hi_solver", "HI Solver", None),
                    DccMenuItemDef::command("ik_hd_solver", "HD Solver", None),
                    DccMenuItemDef::command("ik_limb_solver", "IK Limb Solver", None),
                    DccMenuItemDef::command("ik_spline_solver", "Spline IK Solver", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Constraints",
                vec![
                    DccMenuItemDef::command("constraint_attachment", "Attachment Constraint", None),
                    DccMenuItemDef::command("constraint_surface", "Surface Constraint", None),
                    DccMenuItemDef::command("constraint_path", "Path Constraint", None),
                    DccMenuItemDef::command("constraint_position", "Position Constraint", None),
                    DccMenuItemDef::command("constraint_link", "Link Constraint", None),
                    DccMenuItemDef::command("constraint_lookat", "LookAt Constraint", None),
                    DccMenuItemDef::command(
                        "constraint_orientation",
                        "Orientation Constraint",
                        None,
                    ),
                ],
            ),
            DccMenuItemDef::submenu(
                "Transform Controllers",
                vec![
                    DccMenuItemDef::command("ctrl_link_transform", "Link Transform", None),
                    DccMenuItemDef::command("ctrl_prs", "Position/Rotation/Scale", None),
                    DccMenuItemDef::command("ctrl_transform_script", "Transform Script", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Position Controllers",
                vec![
                    DccMenuItemDef::command("pos_ctrl_bezier", "Bezier Position", None),
                    DccMenuItemDef::command("pos_ctrl_linear", "Linear Position", None),
                    DccMenuItemDef::command("pos_ctrl_tcb", "TCB Position", None),
                    DccMenuItemDef::command("pos_ctrl_xyz", "Position XYZ", None),
                    DccMenuItemDef::command("pos_ctrl_noise", "Noise Position", None),
                    DccMenuItemDef::command("pos_ctrl_list", "Position List", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Rotation Controllers",
                vec![
                    DccMenuItemDef::command("rot_ctrl_bezier", "Bezier Rotation", None),
                    DccMenuItemDef::command("rot_ctrl_linear", "Linear Rotation", None),
                    DccMenuItemDef::command("rot_ctrl_euler", "Euler XYZ", None),
                    DccMenuItemDef::command("rot_ctrl_tcb", "TCB Rotation", None),
                    DccMenuItemDef::command("rot_ctrl_noise", "Noise Rotation", None),
                    DccMenuItemDef::command("rot_ctrl_list", "Rotation List", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Scale Controllers",
                vec![
                    DccMenuItemDef::command("scale_ctrl_bezier", "Bezier Scale", None),
                    DccMenuItemDef::command("scale_ctrl_linear", "Linear Scale", None),
                    DccMenuItemDef::command("scale_ctrl_tcb", "TCB Scale", None),
                    DccMenuItemDef::command("scale_ctrl_xyz", "Scale XYZ", None),
                    DccMenuItemDef::command("scale_ctrl_list", "Scale List", None),
                ],
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("anim_param_editor", "Parameter Editor...", None),
            DccMenuItemDef::command("anim_wire_params", "Wire Parameters...", None),
            DccMenuItemDef::command("anim_reaction_manager", "Reaction Manager...", None),
            DccMenuItemDef::command("anim_layers", "Animation Layers...", None),
            DccMenuItemDef::command("anim_bone_tools", "Bone Tools...", None),
            DccMenuItemDef::submenu(
                "Skin",
                vec![
                    DccMenuItemDef::command("skin_bind", "Bind Skin", None),
                    DccMenuItemDef::command("skin_unbind", "Unbind Skin", None),
                    DccMenuItemDef::command("skin_paint_weights", "Paint Skin Weights", None),
                    DccMenuItemDef::command("skin_edit_envelopes", "Edit Envelopes", None),
                ],
            ),
            DccMenuItemDef::command("anim_morpher", "Morpher...", None),
            DccMenuItemDef::submenu(
                "CAT",
                vec![
                    DccMenuItemDef::command("cat_rig", "CAT Rig", None),
                    DccMenuItemDef::command("cat_motion", "CATMotion", None),
                    DccMenuItemDef::command("cat_layers", "CAT Layers", None),
                    DccMenuItemDef::command("cat_muscles", "CAT Muscles", None),
                ],
            ),
            DccMenuItemDef::submenu(
                "Biped",
                vec![
                    DccMenuItemDef::command("biped_create", "Create Biped", None),
                    DccMenuItemDef::command("biped_params", "Biped Parameters", None),
                    DccMenuItemDef::command("biped_motion_mixer", "Motion Mixer", None),
                    DccMenuItemDef::command("biped_motion_flow", "Motion Flow", None),
                    DccMenuItemDef::command("biped_crowd", "Crowd", None),
                ],
            ),
            DccMenuItemDef::command("anim_crowd", "Crowd...", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("anim_make_preview", "Make Preview...", None),
            DccMenuItemDef::command("anim_view_preview", "View Preview", None),
            DccMenuItemDef::command("anim_rename_preview", "Rename Preview...", None),
        ]
    }

    // 1.9 Graph Editors Menu
    fn build_graph_editors_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("graph_curve_editor", "Track View - Curve Editor...", None),
            DccMenuItemDef::command("graph_dope_sheet", "Track View - Dope Sheet...", None),
            DccMenuItemDef::command("graph_new_track_view", "New Track View...", None),
            DccMenuItemDef::command("graph_delete_track_view", "Delete Track View", None),
            DccMenuItemDef::command("graph_saved_track_views", "Saved Track Views", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("graph_new_schematic", "New Schematic View...", None),
            DccMenuItemDef::command("graph_delete_schematic", "Delete Schematic View", None),
            DccMenuItemDef::command("graph_saved_schematics", "Saved Schematic Views", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("graph_particle_view", "Particle View...", None),
        ]
    }

    // 1.10 Rendering Menu
    fn build_rendering_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("render_setup", "Render Setup...", Some("F10")),
            DccMenuItemDef::command("render_frame", "Render Frame", Some("F9")),
            DccMenuItemDef::command("render_frame_window", "Rendered Frame Window...", None),
            DccMenuItemDef::command("render_production", "Render Production", Some("Shift+Q")),
            DccMenuItemDef::command("render_iterative", "Render Iterative", None),
            DccMenuItemDef::command("render_activeshade", "Render ActiveShade", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("render_environment", "Environment...", Some("8")),
            DccMenuItemDef::command("render_effects", "Effects...", None),
            DccMenuItemDef::command("render_video_post", "Video Post...", None),
            DccMenuItemDef::command("render_exposure_control", "Exposure Control...", None),
            DccMenuItemDef::submenu(
                "Lighting Analysis",
                vec![
                    DccMenuItemDef::command(
                        "light_analysis_assistant",
                        "Lighting Analysis Assistant",
                        None,
                    ),
                    DccMenuItemDef::command("light_meter", "Light Meter", None),
                    DccMenuItemDef::command("light_analysis_data", "Lighting Analysis Data", None),
                ],
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::submenu(
                "Material Editor",
                vec![
                    DccMenuItemDef::command(
                        "mat_compact_editor",
                        "Compact Material Editor...",
                        Some("M"),
                    ),
                    DccMenuItemDef::command("mat_slate_editor", "Slate Material Editor...", None),
                ],
            ),
            DccMenuItemDef::command("render_mat_browser", "Material/Map Browser...", None),
            DccMenuItemDef::command("render_to_texture", "Render to Texture...", Some("0")),
            DccMenuItemDef::command("render_batch", "Batch Render...", None),
            DccMenuItemDef::command("render_network", "Network Render...", None),
            DccMenuItemDef::command("render_message", "Render Message...", None),
            DccMenuItemDef::command(
                "render_raytrace_settings",
                "Raytrace Global Settings...",
                None,
            ),
            DccMenuItemDef::command("render_radiosity", "Radiosity...", None),
            DccMenuItemDef::command("render_presets", "Render Presets...", None),
            DccMenuItemDef::command("render_shortcuts_toolbar", "Render Shortcuts Toolbar", None),
            DccMenuItemDef::command("render_color_management", "Color Management...", None),
            DccMenuItemDef::command("render_ocio_config", "OCIO Configuration...", None),
        ]
    }

    // 1.11 Customize Menu
    fn build_customize_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::submenu(
                "Customize User Interface...",
                vec![
                    DccMenuItemDef::command("cui_keyboard", "Keyboard", None),
                    DccMenuItemDef::command("cui_toolbars", "Toolbars", None),
                    DccMenuItemDef::command("cui_quads", "Quads", None),
                    DccMenuItemDef::command("cui_menus", "Menus", None),
                    DccMenuItemDef::command("cui_colors", "Colors", None),
                ],
            ),
            DccMenuItemDef::command("cust_load_scheme", "Load Custom UI Scheme...", None),
            DccMenuItemDef::command("cust_save_scheme", "Save Custom UI Scheme...", None),
            DccMenuItemDef::command("cust_revert_layout", "Revert to Startup UI Layout", None),
            DccMenuItemDef::checkable("cust_lock_layout", "Lock UI Layout", None, false),
            DccMenuItemDef::command(
                "cust_defaults_switcher",
                "Custom UI and Defaults Switcher...",
                None,
            ),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("cust_preferences", "Preferences...", None),
            DccMenuItemDef::command("cust_units_setup", "Units Setup...", None),
            DccMenuItemDef::command("cust_grid_snap", "Grid and Snap Settings...", None),
            DccMenuItemDef::command("cust_viewport_config", "Viewport Configuration...", None),
            DccMenuItemDef::command("cust_plugin_manager", "Plug-in Manager...", None),
            DccMenuItemDef::submenu(
                "Display UI",
                vec![
                    DccMenuItemDef::checkable("show_main_toolbar", "Show Main Toolbar", None, true),
                    DccMenuItemDef::checkable("show_tab_panel", "Show Tab Panel", None, true),
                    DccMenuItemDef::checkable(
                        "show_command_panel",
                        "Show Command Panel",
                        None,
                        true,
                    ),
                    DccMenuItemDef::checkable("show_track_bar", "Show Track Bar", None, true),
                    DccMenuItemDef::checkable("show_status_bar", "Show Status Bar", None, true),
                    DccMenuItemDef::checkable(
                        "show_scripting_toolbar",
                        "Show Scripting Toolbar",
                        None,
                        false,
                    ),
                    DccMenuItemDef::checkable(
                        "show_layers_toolbar",
                        "Show Layers Toolbar",
                        None,
                        false,
                    ),
                    DccMenuItemDef::checkable(
                        "show_massfx_toolbar",
                        "Show MassFX Toolbar",
                        None,
                        false,
                    ),
                ],
            ),
        ]
    }

    // 1.12 MAXScript Menu
    fn build_maxscript_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("script_new", "New Script", None),
            DccMenuItemDef::command("script_open", "Open Script...", None),
            DccMenuItemDef::command("script_run", "Run Script...", None),
            DccMenuItemDef::command("script_listener", "MAXScript Listener...", Some("F11")),
            DccMenuItemDef::command("script_editor", "MAXScript Editor...", None),
            DccMenuItemDef::command("script_debugger", "Debugger...", None),
            DccMenuItemDef::command("script_macro_recorder", "Macro Recorder...", None),
            DccMenuItemDef::command("script_reference", "MAXScript Reference", None),
            DccMenuItemDef::command("script_visual_editor", "Visual MAXScript Editor...", None),
        ]
    }

    // 1.13 Help Menu
    fn build_help_menu() -> Vec<DccMenuItemDef> {
        vec![
            DccMenuItemDef::command("help_welcome", "Welcome Screen", None),
            DccMenuItemDef::command("help_learning", "Learning Resources", None),
            DccMenuItemDef::command("help_tutorials", "Tutorials", None),
            DccMenuItemDef::command("help_user_reference", "User Reference", Some("F1")),
            DccMenuItemDef::command("help_script_ref", "MAXScript Reference", None),
            DccMenuItemDef::command("help_additional", "Additional Help", None),
            DccMenuItemDef::command("help_web", "3ds Max on the Web", None),
            DccMenuItemDef::separator(),
            DccMenuItemDef::command("help_about", "About Oxide-3D DCC", None),
        ]
    }
}
