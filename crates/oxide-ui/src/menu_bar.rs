//! SolidWorks-style Menu Bar definitions and action dispatching.

use serde::{Deserialize, Serialize};

/// Top-level Menu Bar category tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuCategory {
    /// File operations (New, Open, Save, Export, Print, Pack & Go).
    File,
    /// Edit operations (Undo, Redo, Cut, Copy, Paste, Rebuild, Suppress).
    Edit,
    /// View configuration (Display styles, Orientation, Visibility, Heads-Up).
    View,
    /// Geometry insertion (Features, Surfaces, Sheet Metal, Weldments, Molds).
    Insert,
    /// Evaluation and inspection (Measure, Mass, SimulationXpress, Diagnostics).
    Tools,
    /// Window arrangement (Tile, Cascade, Switch).
    Window,
    /// Documentation, tutorials, API help.
    Help,
}

/// Menu item descriptor with shortcut and action ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MenuItemDef {
    /// Display label.
    pub label: String,
    /// Optional keyboard shortcut text (e.g. "Ctrl+N").
    pub shortcut: Option<String>,
    /// Unique action identifier.
    pub action_id: &'static str,
    /// Whether this item is enabled in current context.
    pub enabled: bool,
}

impl MenuItemDef {
    /// Create a new menu item.
    #[must_use]
    pub fn new(label: impl Into<String>, action_id: &'static str, shortcut: Option<&str>) -> Self {
        Self {
            label: label.into(),
            shortcut: shortcut.map(str::to_string),
            action_id,
            enabled: true,
        }
    }
}

/// Menu Bar manager aggregating all standard SolidWorks menus.
#[derive(Debug, Clone, Default)]
pub struct MenuBarModel {
    /// Currently open / active flyout category.
    pub active_flyout: Option<MenuCategory>,
    /// Pinned state (shows both toolbar and menu bar simultaneously).
    pub is_pinned: bool,
}

impl MenuBarModel {
    /// Create a new MenuBarModel.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return items for a given menu category.
    #[must_use]
    pub fn get_items(&self, category: MenuCategory) -> Vec<MenuItemDef> {
        match category {
            MenuCategory::File => vec![
                MenuItemDef::new("New...", "file.new", Some("Ctrl+N")),
                MenuItemDef::new("Open...", "file.open", Some("Ctrl+O")),
                MenuItemDef::new("Browse Recent Documents", "file.recent", Some("R")),
                MenuItemDef::new("Close", "file.close", Some("Ctrl+W")),
                MenuItemDef::new("Save", "file.save", Some("Ctrl+S")),
                MenuItemDef::new("Save As...", "file.save_as", None),
                MenuItemDef::new("Save All", "file.save_all", None),
                MenuItemDef::new("Pack and Go...", "file.pack_and_go", None),
                MenuItemDef::new("Print...", "file.print", Some("Ctrl+P")),
                MenuItemDef::new("Print Preview", "file.print_preview", None),
                MenuItemDef::new("Publish eDrawings File", "file.publish_edrawings", None),
                MenuItemDef::new("Find References...", "file.find_references", None),
                MenuItemDef::new("Reload", "file.reload", None),
                MenuItemDef::new("Properties", "file.properties", None),
                MenuItemDef::new("Exit", "file.exit", Some("Alt+F4")),
            ],
            MenuCategory::Edit => vec![
                MenuItemDef::new("Undo", "edit.undo", Some("Ctrl+Z")),
                MenuItemDef::new("Redo", "edit.redo", Some("Ctrl+Y")),
                MenuItemDef::new("Cut", "edit.cut", Some("Ctrl+X")),
                MenuItemDef::new("Copy", "edit.copy", Some("Ctrl+C")),
                MenuItemDef::new("Paste", "edit.paste", Some("Ctrl+V")),
                MenuItemDef::new("Delete", "edit.delete", Some("Del")),
                MenuItemDef::new("Delete Body", "edit.delete_body", None),
                MenuItemDef::new("Suppress", "edit.suppress", None),
                MenuItemDef::new("Unsuppress", "edit.unsuppress", None),
                MenuItemDef::new("Select All", "edit.select_all", Some("Ctrl+A")),
                MenuItemDef::new("Select Other", "edit.select_other", None),
                MenuItemDef::new("Rebuild", "edit.rebuild", Some("Ctrl+B")),
                MenuItemDef::new("Rebuild All", "edit.rebuild_all", Some("Ctrl+Q")),
                MenuItemDef::new("Rollback", "edit.rollback", None),
                MenuItemDef::new("Find / Replace", "edit.find_replace", Some("Ctrl+F")),
                MenuItemDef::new("Keyboard Shortcuts...", "edit.shortcuts", None),
                MenuItemDef::new("Mouse Gestures...", "edit.gestures", None),
            ],
            MenuCategory::View => vec![
                MenuItemDef::new("Zoom to Fit", "view.zoom_fit", Some("F")),
                MenuItemDef::new("Zoom to Area", "view.zoom_area", None),
                MenuItemDef::new("Zoom Out", "view.zoom_out", None),
                MenuItemDef::new("Pan", "view.pan", None),
                MenuItemDef::new("Rotate View", "view.rotate", None),
                MenuItemDef::new("View Orientation...", "view.orientation", Some("Spacebar")),
                MenuItemDef::new("Front View", "view.front", Some("Ctrl+1")),
                MenuItemDef::new("Back View", "view.back", Some("Ctrl+2")),
                MenuItemDef::new("Left View", "view.left", Some("Ctrl+3")),
                MenuItemDef::new("Right View", "view.right", Some("Ctrl+4")),
                MenuItemDef::new("Top View", "view.top", Some("Ctrl+5")),
                MenuItemDef::new("Bottom View", "view.bottom", Some("Ctrl+6")),
                MenuItemDef::new("Isometric View", "view.isometric", Some("Ctrl+7")),
                MenuItemDef::new("Normal To", "view.normal_to", Some("Ctrl+8")),
                MenuItemDef::new("Section View", "view.section", None),
                MenuItemDef::new(
                    "Display Style: Shaded with Edges",
                    "view.display_shaded_edges",
                    None,
                ),
                MenuItemDef::new("Display Style: Wireframe", "view.display_wireframe", None),
                MenuItemDef::new("Hide/Show Items: Planes", "view.toggle_planes", None),
                MenuItemDef::new("Hide/Show Items: Axes", "view.toggle_axes", None),
                MenuItemDef::new("Hide/Show Items: Origin", "view.toggle_origin", None),
                MenuItemDef::new("Hide/Show Items: Sketches", "view.toggle_sketches", None),
                MenuItemDef::new("Perspective View", "view.toggle_perspective", None),
                MenuItemDef::new("Full Screen", "view.fullscreen", Some("F11")),
            ],
            MenuCategory::Insert => vec![
                MenuItemDef::new("Boss/Base Extrude...", "insert.extrude", None),
                MenuItemDef::new("Boss/Base Revolve...", "insert.revolve", None),
                MenuItemDef::new("Boss/Base Sweep...", "insert.sweep", None),
                MenuItemDef::new("Boss/Base Loft...", "insert.loft", None),
                MenuItemDef::new("Boss/Base Boundary...", "insert.boundary", None),
                MenuItemDef::new("Cut Extrude...", "insert.cut_extrude", None),
                MenuItemDef::new("Cut Revolve...", "insert.cut_revolve", None),
                MenuItemDef::new("Cut Sweep...", "insert.cut_sweep", None),
                MenuItemDef::new("Cut Loft...", "insert.cut_loft", None),
                MenuItemDef::new("Hole Wizard...", "insert.hole_wizard", None),
                MenuItemDef::new("Fillet...", "insert.fillet", None),
                MenuItemDef::new("Chamfer...", "insert.chamfer", None),
                MenuItemDef::new("Draft...", "insert.draft", None),
                MenuItemDef::new("Shell...", "insert.shell", None),
                MenuItemDef::new("Rib...", "insert.rib", None),
                MenuItemDef::new("Wrap...", "insert.wrap", None),
                MenuItemDef::new("Dome...", "insert.dome", None),
                MenuItemDef::new("Mirror...", "insert.mirror", None),
                MenuItemDef::new("Linear Pattern...", "insert.linear_pattern", None),
                MenuItemDef::new("Circular Pattern...", "insert.circular_pattern", None),
                MenuItemDef::new("Reference Geometry: Plane", "insert.ref_plane", None),
                MenuItemDef::new("Reference Geometry: Axis", "insert.ref_axis", None),
                MenuItemDef::new(
                    "Reference Geometry: Coordinate System",
                    "insert.ref_csys",
                    None,
                ),
            ],
            MenuCategory::Tools => vec![
                MenuItemDef::new("Measure", "tools.measure", None),
                MenuItemDef::new("Mass Properties", "tools.mass_props", None),
                MenuItemDef::new("Section Properties", "tools.section_props", None),
                MenuItemDef::new("Interference Detection", "tools.interference", None),
                MenuItemDef::new("Check Geometry...", "tools.check", None),
                MenuItemDef::new("Import Diagnostics", "tools.import_diag", None),
                MenuItemDef::new("Equations...", "tools.equations", None),
                MenuItemDef::new("Sensors...", "tools.sensors", None),
                MenuItemDef::new("Design Study...", "tools.design_study", None),
                MenuItemDef::new("SimulationXpress Analysis Wizard", "tools.sim_xpress", None),
                MenuItemDef::new("DFMXpress Analysis Wizard", "tools.dfm_xpress", None),
                MenuItemDef::new("FloXpress Analysis Wizard", "tools.flo_xpress", None),
                MenuItemDef::new("Costing...", "tools.costing", None),
                MenuItemDef::new("Sustainability...", "tools.sustainability", None),
                MenuItemDef::new("Macros: Record / Stop", "tools.macro_record", None),
                MenuItemDef::new("Macros: Run Script...", "tools.macro_run", None),
                MenuItemDef::new("Add-Ins...", "tools.add_ins", None),
                MenuItemDef::new("Customize...", "tools.customize", None),
                MenuItemDef::new("Options...", "tools.options", None),
            ],
            MenuCategory::Window => vec![
                MenuItemDef::new("New Window", "window.new_window", None),
                MenuItemDef::new("Cascade", "window.cascade", None),
                MenuItemDef::new("Tile Horizontally", "window.tile_horizontal", None),
                MenuItemDef::new("Tile Vertically", "window.tile_vertical", None),
                MenuItemDef::new("Close All", "window.close_all", None),
            ],
            MenuCategory::Help => vec![
                MenuItemDef::new("Oxide-3D Help Topics", "help.topics", Some("F1")),
                MenuItemDef::new("Tutorials & Step-by-Step Guides", "help.tutorials", None),
                MenuItemDef::new("What's New in Oxide-3D", "help.whats_new", None),
                MenuItemDef::new("API & Scripting Help", "help.api_help", None),
                MenuItemDef::new("Hardware & GPU Diagnostics", "help.hardware", None),
                MenuItemDef::new("Check for Updates...", "help.updates", None),
                MenuItemDef::new("About Oxide-3D", "help.about", None),
            ],
        }
    }
}
