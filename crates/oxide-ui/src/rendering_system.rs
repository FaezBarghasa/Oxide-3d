//! DCC Rendering Engine, Render Setup, Environment & Effects, and Frame Window System.

use serde::{Deserialize, Serialize};

/// Production Render Engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ProductionRenderer {
    #[default]
    Arnold,
    Scanline,
    MentalRay,
    VRay,
    Corona,
    Redshift,
    OxidePathTracer,
}

impl ProductionRenderer {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::Arnold => "Arnold",
            Self::Scanline => "Scanline Renderer",
            Self::MentalRay => "NVIDIA mental ray",
            Self::VRay => "V-Ray",
            Self::Corona => "Corona Renderer",
            Self::Redshift => "Redshift",
            Self::OxidePathTracer => "Oxide GPU Path Tracer",
        }
    }
}

/// Time Output Option.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TimeOutputMode {
    #[default]
    SingleFrame,
    ActiveTimeSegment,
    Range(i32, i32),
    Frames(Option<[i32; 2]>),
}

/// Render Output Resolution Preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum OutputResolutionPreset {
    Custom(u32, u32),
    #[default]
    Hd1080p,
    Uhd4k,
    Square1024,
    InstagramPortrait,
}

impl OutputResolutionPreset {
    #[must_use]
    pub const fn dimensions(&self) -> (u32, u32) {
        match self {
            Self::Custom(w, h) => (*w, *h),
            Self::Hd1080p => (1920, 1080),
            Self::Uhd4k => (3840, 2160),
            Self::Square1024 => (1024, 1024),
            Self::InstagramPortrait => (1080, 1350),
        }
    }
}

/// Environment & Effects Settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentEffectsConfig {
    pub background_color: [f32; 3],
    pub ambient_color: [f32; 3],
    pub tint_color: [f32; 3],
    pub global_light_level: f32,
    pub atmosphere_effects: Vec<String>,
    pub post_render_effects: Vec<String>,
    pub exposure_control: String,
}

impl Default for EnvironmentEffectsConfig {
    fn default() -> Self {
        Self {
            background_color: [0.05, 0.05, 0.07],
            ambient_color: [0.0, 0.0, 0.0],
            tint_color: [1.0, 1.0, 1.0],
            global_light_level: 1.0,
            atmosphere_effects: vec!["Volume Fog".to_string()],
            post_render_effects: vec!["Film Grain".to_string(), "Lens Effects".to_string()],
            exposure_control: "Physical Camera Exposure".to_string(),
        }
    }
}

/// Complete Rendering System State Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingSystemModel {
    pub renderer: ProductionRenderer,
    pub time_output: TimeOutputMode,
    pub resolution: OutputResolutionPreset,
    pub lock_to_viewport: bool,
    pub target_viewport: String,
    pub render_elements: Vec<String>,
    pub environment: EnvironmentEffectsConfig,
    pub color_management_ocio: bool,
}

impl Default for RenderingSystemModel {
    fn default() -> Self {
        Self {
            renderer: ProductionRenderer::Arnold,
            time_output: TimeOutputMode::SingleFrame,
            resolution: OutputResolutionPreset::Hd1080p,
            lock_to_viewport: true,
            target_viewport: "Perspective".to_string(),
            render_elements: vec![
                "Beauty".to_string(),
                "Alpha".to_string(),
                "Z-Depth".to_string(),
                "Diffuse".to_string(),
                "Specular".to_string(),
                "Shadows".to_string(),
            ],
            environment: EnvironmentEffectsConfig::default(),
            color_management_ocio: true,
        }
    }
}

impl RenderingSystemModel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
