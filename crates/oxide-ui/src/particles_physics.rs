//! Particles, Space Warps, MassFX, and Reactor Physics Simulation Models.

use serde::{Deserialize, Serialize};

/// Particle Systems Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ParticleSystemType {
    #[default]
    ParticleFlow,
    Spray,
    Snow,
    SuperSpray,
    Blizzard,
    PArray,
    PCloud,
}

/// Space Warp Forces & Deflectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SpaceWarpType {
    #[default]
    Gravity,
    Wind,
    Drag,
    Vortex,
    PathFollow,
    PBomb,
    Displace,
    Noise,
    Push,
    Motor,
    POmniFlect,
    SDeflector,
    UDeflector,
    UOmniFlect,
    FfdBox,
    FfdCyl,
    Wave,
    Ripple,
}

/// MassFX Rigid Body Physics Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MassFxBodyType {
    Dynamic,
    Kinematic,
    #[default]
    Static,
}

/// MassFX Mesh Collider Shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MassFxColliderShape {
    Sphere,
    Box,
    Capsule,
    #[default]
    ConvexHull,
    ConcaveMesh,
    Custom,
}

/// MassFX World Parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassFxWorldParams {
    pub gravity_z: f32,
    pub substeps: u32,
    pub solver_iterations: u32,
    pub sleep_threshold_energy: f32,
    pub ground_plane_enabled: bool,
}

impl Default for MassFxWorldParams {
    fn default() -> Self {
        Self {
            gravity_z: -9.81,
            substeps: 4,
            solver_iterations: 8,
            sleep_threshold_energy: 0.005,
            ground_plane_enabled: true,
        }
    }
}

/// MassFX Simulation State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MassFxState {
    pub is_simulating: bool,
    pub current_sim_frame: u32,
    pub world: MassFxWorldParams,
    pub show_visualizer: bool,
}

impl Default for MassFxState {
    fn default() -> Self {
        Self {
            is_simulating: false,
            current_sim_frame: 0,
            world: MassFxWorldParams::default(),
            show_visualizer: true,
        }
    }
}

/// Legacy Reactor Physics State.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactorPhysicsState {
    pub enabled: bool,
    pub col_tolerance: f32,
    pub friction: f32,
    pub air_resistance: f32,
}

impl Default for ReactorPhysicsState {
    fn default() -> Self {
        Self {
            enabled: false,
            col_tolerance: 0.01,
            friction: 0.3,
            air_resistance: 0.05,
        }
    }
}

/// Complete Particles and Physics Model.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParticlesPhysicsModel {
    pub active_particle_system: ParticleSystemType,
    pub space_warps: Vec<SpaceWarpType>,
    pub massfx: MassFxState,
    pub reactor: ReactorPhysicsState,
}

impl ParticlesPhysicsModel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
