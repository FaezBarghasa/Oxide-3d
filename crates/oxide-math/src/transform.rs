use nalgebra::Isometry3;
use serde::{Deserialize, Serialize};

/// 3D Rigid body transform representation with conversion bridges.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Transform3 {
    /// Translation [x, y, z].
    pub translation: [f64; 3],
    /// Unit quaternion rotation [x, y, z, w].
    pub rotation: [f64; 4],
}

impl Default for Transform3 {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

impl Transform3 {
    /// Convert to glam Affine3A for GPU rendering.
    #[must_use]
    pub fn to_glam_affine(&self) -> glam::Affine3A {
        let t = glam::Vec3::new(
            self.translation[0] as f32,
            self.translation[1] as f32,
            self.translation[2] as f32,
        );
        let q = glam::Quat::from_xyzw(
            self.rotation[0] as f32,
            self.rotation[1] as f32,
            self.rotation[2] as f32,
            self.rotation[3] as f32,
        );
        glam::Affine3A::from_rotation_translation(q, t)
    }

    /// Convert to nalgebra Isometry3 for physics and kinematics.
    #[must_use]
    pub fn to_nalgebra_isometry(&self) -> Isometry3<f64> {
        let t = nalgebra::Translation3::new(self.translation[0], self.translation[1], self.translation[2]);
        let q = nalgebra::UnitQuaternion::from_quaternion(nalgebra::Quaternion::new(
            self.rotation[3],
            self.rotation[0],
            self.rotation[1],
            self.rotation[2],
        ));
        Isometry3::from_parts(t, q)
    }
}
