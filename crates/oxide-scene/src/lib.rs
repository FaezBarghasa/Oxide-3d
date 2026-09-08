//! Oxide-3D Large-Assembly Scene Graph, Spatial BVH, and ECS-style entity storage.

use oxide_core::id::{EntityKey, PartKey};
use oxide_math::Transform3;
use serde::{Deserialize, Serialize};
use slotmap::SlotMap;

/// Visual display mode for scene entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DisplayMode {
    /// Realistic physically-based shading.
    Shaded,
    /// Technical hidden-line removed.
    HiddenLine,
    /// Translucent xray.
    XRay,
    /// Wireframe only.
    Wireframe,
}

impl Default for DisplayMode {
    fn default() -> Self {
        Self::Shaded
    }
}

/// A 3D entity instance in an assembly scene.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneEntity {
    /// Instance local transform.
    pub transform: Transform3,
    /// Referenced part definition.
    pub part: PartKey,
    /// Visibility toggle.
    pub is_visible: bool,
    /// Selectability toggle.
    pub is_selectable: bool,
    /// Custom display mode override.
    pub display_mode: DisplayMode,
}

/// Large-assembly scene container.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AssemblyScene {
    /// Flat entity storage for instant cache-friendly indexing.
    pub entities: SlotMap<EntityKey, SceneEntity>,
}

impl AssemblyScene {
    /// Create a new empty assembly scene.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an entity instance to the scene.
    pub fn insert_entity(&mut self, entity: SceneEntity) -> EntityKey {
        self.entities.insert(entity)
    }
}
