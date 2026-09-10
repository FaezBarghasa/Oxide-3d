//! Scene graph object types, empty types, lights, cameras, and transform hierarchy.

use oxide_math::Transform3;
use serde::{Deserialize, Serialize};

/// Type of scene object entity in 3D Object Mode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ObjectData {
    /// Polygonal Mesh object.
    Mesh {
        /// Mesh resource name or ID.
        name: String,
    },
    /// Parametric 3D curve object.
    Curve {
        /// Curve type name (Bézier, NURBS, Path).
        curve_type: String,
    },
    /// Surface sheet object.
    Surface {
        /// Surface geometry type.
        surface_type: String,
    },
    /// Implicit Metaball object.
    Metaball {
        /// Element type (Ball, Capsule, Plane, Ellipsoid, Cube).
        element_type: String,
        /// Field stiffness radius.
        stiffness: f64,
    },
    /// 3D Text geometry object.
    Text {
        /// Display text content.
        body: String,
        /// Font size.
        size: f64,
    },
    /// VDB Volume grid object.
    Volume {
        /// File path or grid reference.
        vdb_path: String,
    },
    /// Point Cloud attribute dataset.
    PointCloud {
        /// Point count.
        count: usize,
    },
    /// Non-renderable reference Empty.
    Empty {
        /// Display visualization style (Plain Axes, Arrows, Single Arrow, Circle, Cube, Sphere, Cone, Image).
        display_type: EmptyDisplayType,
        /// Display scale size.
        size: f64,
    },
    /// Scene Light Source.
    Light {
        /// Light type (Point, Sun, Spot, Area).
        kind: LightKind,
        /// Luminous power in Watts or Lumens.
        power_watts: f64,
        /// Light emission color [r, g, b].
        color: [f32; 3],
    },
    /// Viewport / Render Camera.
    Camera {
        /// Focal length in mm (e.g. 50.0).
        focal_length_mm: f64,
        /// Sensor width in mm (e.g. 36.0 for full-frame).
        sensor_width_mm: f64,
        /// Field of view in degrees.
        fov_deg: f64,
    },
}

/// Empty object visualization styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EmptyDisplayType {
    /// 3-axis crosshair.
    #[default]
    PlainAxes,
    /// 3 orthogonal arrows (+X, +Y, +Z).
    Arrows,
    /// Single directional pointing arrow (+Z).
    SingleArrow,
    /// Wireframe Circle.
    Circle,
    /// Wireframe Cube bounding box.
    Cube,
    /// Wireframe Sphere.
    Sphere,
    /// Wireframe Cone.
    Cone,
    /// Image reference plane (Background blueprint).
    Image,
}

/// Scene illumination light categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum LightKind {
    /// Omnidirectional point light.
    #[default]
    Point,
    /// Directional sunlight with parallel rays.
    Sun,
    /// Conical spotlight with inner/outer cutoff.
    Spot,
    /// Rectangular/disc area light with soft specular reflections.
    Area,
}

/// Object mode transform, relations, and viewport display properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectProperties {
    /// Object human-readable name.
    pub name: String,
    /// Object local 3D transform.
    pub transform: Transform3,
    /// Delta offset transform applied on top.
    pub delta_transform: Transform3,
    /// Parent object ID if nested in hierarchy.
    pub parent_id: Option<String>,
    /// Show in 3D Viewports.
    pub show_in_viewport: bool,
    /// Show in final render passes.
    pub show_in_render: bool,
    /// Display wireframe bounds in viewport.
    pub show_bounds: bool,
    /// Display name tag in 3D space.
    pub show_name_tag: bool,
    /// Display local coordinate axes.
    pub show_axis: bool,
    /// Render in front of other objects (X-ray overlay).
    pub in_front: bool,
    /// Specific geometry payload.
    pub data: ObjectData,
}

impl ObjectProperties {
    /// Create a new generic Object.
    #[must_use]
    pub fn new(name: impl Into<String>, data: ObjectData) -> Self {
        Self {
            name: name.into(),
            transform: Transform3::default(),
            delta_transform: Transform3::default(),
            parent_id: None,
            show_in_viewport: true,
            show_in_render: true,
            show_bounds: false,
            show_name_tag: true,
            show_axis: false,
            in_front: false,
            data,
        }
    }
}
