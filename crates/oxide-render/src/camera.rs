use glam::{Mat4, Vec3};

/// Viewport 3D Orbit/Pan/Zoom Camera.
#[derive(Debug, Clone, Copy)]
pub struct Camera {
    /// Eye position in world space.
    pub eye: Vec3,
    /// Target center point in world space.
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

    /// Orbit camera around the target point.
    pub fn orbit(&mut self, dx: f32, dy: f32) {
        let mut offset = self.eye - self.target;
        let radius = offset.length();
        if radius < 1e-4 {
            return;
        }

        // Spherical coordinates
        let mut theta = offset.z.atan2(offset.x);
        let mut phi = (offset.y / radius).clamp(-1.0, 1.0).acos();

        theta -= dx * 0.01;
        phi = (phi - dy * 0.01).clamp(0.01, std::f32::consts::PI - 0.01);

        offset.x = radius * phi.sin() * theta.cos();
        offset.y = radius * phi.cos();
        offset.z = radius * phi.sin() * theta.sin();

        self.eye = self.target + offset;
    }

    /// Pan camera relative to view plane.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let forward = (self.target - self.eye).normalize_or_zero();
        let right = forward.cross(self.up).normalize_or_zero();
        let cam_up = right.cross(forward).normalize_or_zero();

        let pan_speed = (self.eye - self.target).length() * 0.002;
        let delta = -right * dx * pan_speed + cam_up * dy * pan_speed;

        self.eye += delta;
        self.target += delta;
    }

    /// Zoom camera toward/away from target point.
    pub fn zoom(&mut self, delta: f32) {
        let mut offset = self.eye - self.target;
        let dist = offset.length();
        let zoom_factor = (1.0 - delta * 0.0015).clamp(0.05, 5.0);
        let new_dist = (dist * zoom_factor).clamp(0.1, 10_000.0);

        if dist > 1e-4 {
            offset = offset.normalize() * new_dist;
            self.eye = self.target + offset;
        }
    }
}
