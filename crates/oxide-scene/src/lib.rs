//! Oxide-3D Large-Assembly Scene Graph, Spatial BVH, and ECS-style entity storage.

pub mod object;

pub use object::{EmptyDisplayType, LightKind, ObjectData, ObjectProperties};
pub use oxide_core::id::{EntityKey, PartKey};
pub use oxide_math::Transform3;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembly_scene_and_object_properties() {
        let mut scene = AssemblyScene::new();
        let entity = SceneEntity {
            transform: Transform3::default(),
            part: PartKey::default(),
            is_visible: true,
            is_selectable: true,
            display_mode: DisplayMode::Shaded,
        };
        let key = scene.insert_entity(entity);
        assert!(scene.entities.contains_key(key));

        let obj = ObjectProperties::new(
            "Main Camera",
            ObjectData::Camera {
                focal_length_mm: 50.0,
                sensor_width_mm: 36.0,
                fov_deg: 39.6,
            },
        );
        assert_eq!(obj.name, "Main Camera");
        assert!(obj.show_in_viewport);

        let empty = ObjectProperties::new(
            "Origin Axes",
            ObjectData::Empty {
                display_type: EmptyDisplayType::PlainAxes,
                size: 1.0,
            },
        );
        assert_eq!(empty.name, "Origin Axes");
    }
}
