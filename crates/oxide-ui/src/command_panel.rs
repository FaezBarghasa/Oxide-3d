//! 6-Tab Command Panel Model for Oxide-3D DCC System.

use serde::{Deserialize, Serialize};

/// The 6 main Command Panel tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CommandPanelTab {
    #[default]
    Create,
    Modify,
    Hierarchy,
    Motion,
    Display,
    Utilities,
}

impl CommandPanelTab {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Create => "Create",
            Self::Modify => "Modify",
            Self::Hierarchy => "Hierarchy",
            Self::Motion => "Motion",
            Self::Display => "Display",
            Self::Utilities => "Utilities",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Self] {
        &[
            Self::Create,
            Self::Modify,
            Self::Hierarchy,
            Self::Motion,
            Self::Display,
            Self::Utilities,
        ]
    }
}

/// Create Panel Object Categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CreateCategory {
    #[default]
    Geometry,
    Shapes,
    Lights,
    Cameras,
    Helpers,
    SpaceWarps,
    Systems,
}

/// Geometry Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GeometrySubcategory {
    #[default]
    StandardPrimitives,
    ExtendedPrimitives,
    CompoundObjects,
    ParticleSystems,
    PatchGrids,
    NurbsSurfaces,
    Dynamics,
    AecExtended,
    Stairs,
    Doors,
    Windows,
}

/// Shapes Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShapesSubcategory {
    #[default]
    Splines,
    ExtendedSplines,
    NurbsCurves,
}

/// Lights Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LightsSubcategory {
    #[default]
    Standard,
    Photometric,
    Arnold,
}

/// Cameras Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CamerasSubcategory {
    #[default]
    Standard,
    Arnold,
}

/// Helpers Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum HelpersSubcategory {
    #[default]
    Standard,
    AtmosphericApparatus,
    CameraMatch,
    Manipulators,
    ParticleFlow,
    Vrml97,
}

/// Space Warps Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SpaceWarpsSubcategory {
    #[default]
    Forces,
    Deflectors,
    GeometricDeformable,
    ModifierBased,
    Reactor,
}

/// Systems Subcategories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SystemsSubcategory {
    #[default]
    Standard,
    Biped,
    Cat,
}

/// Create Panel State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePanelModel {
    pub category: CreateCategory,
    pub geometry_sub: GeometrySubcategory,
    pub shapes_sub: ShapesSubcategory,
    pub lights_sub: LightsSubcategory,
    pub cameras_sub: CamerasSubcategory,
    pub helpers_sub: HelpersSubcategory,
    pub space_warps_sub: SpaceWarpsSubcategory,
    pub systems_sub: SystemsSubcategory,
    pub active_primitive: Option<String>,
}

impl Default for CreatePanelModel {
    fn default() -> Self {
        Self {
            category: CreateCategory::Geometry,
            geometry_sub: GeometrySubcategory::StandardPrimitives,
            shapes_sub: ShapesSubcategory::Splines,
            lights_sub: LightsSubcategory::Standard,
            cameras_sub: CamerasSubcategory::Standard,
            helpers_sub: HelpersSubcategory::Standard,
            space_warps_sub: SpaceWarpsSubcategory::Forces,
            systems_sub: SystemsSubcategory::Standard,
            active_primitive: Some("Box".to_string()),
        }
    }
}

/// Applied Modifier Stack Item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifierStackItem {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub is_open: bool,
}

/// Modify Panel State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyPanelModel {
    pub selected_modifier: Option<String>,
    pub stack: Vec<ModifierStackItem>,
    pub show_end_result: bool,
    pub pin_stack: bool,
    pub make_unique_enabled: bool,
}

impl Default for ModifyPanelModel {
    fn default() -> Self {
        Self {
            selected_modifier: Some("Edit Poly".to_string()),
            stack: vec![
                ModifierStackItem {
                    id: "mod_turbosmooth".to_string(),
                    name: "TurboSmooth".to_string(),
                    enabled: true,
                    is_open: true,
                },
                ModifierStackItem {
                    id: "mod_edit_poly".to_string(),
                    name: "Edit Poly".to_string(),
                    enabled: true,
                    is_open: false,
                },
                ModifierStackItem {
                    id: "base_box".to_string(),
                    name: "Box".to_string(),
                    enabled: true,
                    is_open: false,
                },
            ],
            show_end_result: true,
            pin_stack: false,
            make_unique_enabled: false,
        }
    }
}

/// Hierarchy Panel State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyPanelModel {
    pub affect_pivot_only: bool,
    pub affect_object_only: bool,
    pub affect_hierarchy_only: bool,
    pub ik_active: bool,
    pub display_links: bool,
}

impl Default for HierarchyPanelModel {
    fn default() -> Self {
        Self {
            affect_pivot_only: false,
            affect_object_only: false,
            affect_hierarchy_only: false,
            ik_active: false,
            display_links: false,
        }
    }
}

/// Motion Panel State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionPanelModel {
    pub prs_active: bool,
    pub trajectories_active: bool,
    pub show_keys: bool,
}

impl Default for MotionPanelModel {
    fn default() -> Self {
        Self {
            prs_active: true,
            trajectories_active: false,
            show_keys: true,
        }
    }
}

/// Display Panel State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayPanelModel {
    pub display_as_box: bool,
    pub backface_cull: bool,
    pub edged_faces: bool,
    pub vertex_ticks: bool,
    pub trajectory: bool,
    pub see_through: bool,
    pub hide_geometry: bool,
    pub hide_shapes: bool,
    pub hide_lights: bool,
    pub hide_cameras: bool,
    pub hide_helpers: bool,
    pub hide_space_warps: bool,
}

impl Default for DisplayPanelModel {
    fn default() -> Self {
        Self {
            display_as_box: false,
            backface_cull: false,
            edged_faces: true,
            vertex_ticks: false,
            trajectory: false,
            see_through: false,
            hide_geometry: false,
            hide_shapes: false,
            hide_lights: false,
            hide_cameras: false,
            hide_helpers: false,
            hide_space_warps: false,
        }
    }
}

/// Utilities Panel State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilitiesPanelModel {
    pub active_utility: Option<String>,
    pub available_utilities: Vec<String>,
}

impl Default for UtilitiesPanelModel {
    fn default() -> Self {
        Self {
            active_utility: Some("MAXScript".to_string()),
            available_utilities: vec![
                "Asset Browser".to_string(),
                "Camera Tracker".to_string(),
                "Collapse".to_string(),
                "Color Clipboard".to_string(),
                "Measure".to_string(),
                "Motion Capture".to_string(),
                "Reset XForm".to_string(),
                "MAXScript".to_string(),
                "Reactor".to_string(),
                "Particle View".to_string(),
                "Perspective Match".to_string(),
                "Channel Info".to_string(),
                "Clean MultiMaterial".to_string(),
                "Light Lister".to_string(),
                "Layer Manager".to_string(),
                "Scene Explorer".to_string(),
                "Material Explorer".to_string(),
                "Skin Tools".to_string(),
                "Radiosity".to_string(),
            ],
        }
    }
}

/// Complete Command Panel Model.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommandPanelModel {
    pub active_tab: CommandPanelTab,
    pub create: CreatePanelModel,
    pub modify: ModifyPanelModel,
    pub hierarchy: HierarchyPanelModel,
    pub motion: MotionPanelModel,
    pub display: DisplayPanelModel,
    pub utilities: UtilitiesPanelModel,
}

impl CommandPanelModel {
    /// Create a new command panel state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Select an active tab.
    pub fn set_tab(&mut self, tab: CommandPanelTab) {
        self.active_tab = tab;
    }
}
