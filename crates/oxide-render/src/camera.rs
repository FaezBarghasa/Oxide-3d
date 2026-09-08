use glam::{Mat4, Vec3};

/// Viewport 3D Orbit/Pan/Zoom Camera.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Eye position.
    pub eye: Vec3,
    /// Target center point.
    pub target: Vec3,
    /// Up direction.
    pub up: Vec3,
    /// Aspect ratio (width / height).
    pub aspect: f32,
    /// Vertical FOV in radians.
    pub fov_y: f32,
    /// Near clip plane.
    pub z_near: f32,
    /// Far clip plane.
    pub z_far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: Vec3::new(5.0, 5.0, 5.0),
            target: Vec3::ZERO,
            up: Vec3::Y,
            aspect: 16.0 / 9.0,
            fov_y: 45.0f32.to_radians(),
            z_near: 0.1,
            z_far: 1000.0,
        }
    }
}

impl Camera {
    /// Compute combined View-Projection matrix.
    #[must_use]
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(self.eye, self.target, self.up);
        let proj = Mat4::perspective_rh(self.fov_y, self.aspect, self.z_near, self.z_far);
        proj * view
    }
}
