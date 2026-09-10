//! DCC Material Editor, Shader Models, and Procedural Maps System.

use serde::{Deserialize, Serialize};

/// Material Editor Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum MaterialEditorMode {
    #[default]
    Compact,
    Slate,
}

/// Supported Material Types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DccMaterialType {
    #[default]
    Standard,
    ArnoldStandardSurface,
    PhysicalMaterial,
    Raytrace,
    Architectural,
    MatteShadow,
    InkNPaint,
    Shell,
    XRefMaterial,
    MultiSubObject,
    Blend,
    Composite,
    DoubleSided,
    Morpher,
    Shellac,
    TopBottom,
    DirectXShader,
}

/// Shader / BRDF Illumination Models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ShaderIlluminationModel {
    #[default]
    Blinn,
    Phong,
    Anisotropic,
    Metal,
    MultiLayer,
    OrenNayarBlinn,
    Strauss,
    Translucent,
}

/// 2D Map Types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Map2DType {
    Bitmap,
    Checker,
    Combustion,
    Gradient,
    GradientRamp,
    Swirl,
    Tiles,
    CameraMapPerPixel,
    NormalBump,
    Substance,
    VectorDisplacement,
}

/// 3D Map Types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Map3DType {
    Cellular,
    Dent,
    Falloff,
    Marble,
    Noise,
    ParticleAge,
    ParticleMBlur,
    PerlinMarble,
    Planet,
    Smoke,
    Speckle,
    Splat,
    Stucco,
    Waves,
    Wood,
}

/// Compositing Map Types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MapCompositorType {
    Composite,
    Mask,
    Mix,
    RgbMultiply,
}

/// Sample Slot Definition for Compact Material Editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompactSampleSlot {
    pub id: usize,
    pub name: String,
    pub material_type: DccMaterialType,
    pub shader_model: ShaderIlluminationModel,
    pub diffuse_color: [f32; 4],
    pub specular_level: f32,
    pub glossiness: f32,
    pub self_illumination: f32,
    pub opacity: f32,
}

impl CompactSampleSlot {
    #[must_use]
    pub fn new(id: usize, name: impl Into<String>, diffuse: [f32; 4]) -> Self {
        Self {
            id,
            name: name.into(),
            material_type: DccMaterialType::Standard,
            shader_model: ShaderIlluminationModel::Blinn,
            diffuse_color: diffuse,
            specular_level: 40.0,
            glossiness: 25.0,
            self_illumination: 0.0,
            opacity: 1.0,
        }
    }
}

/// Material Editor Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialEditorModel {
    pub mode: MaterialEditorMode,
    pub active_slot_index: usize,
    pub sample_slots: Vec<CompactSampleSlot>,
    pub show_background: bool,
    pub show_backlight: bool,
    pub sample_uv_tiling: f32,
}

impl Default for MaterialEditorModel {
    fn default() -> Self {
        Self::new()
    }
}

impl MaterialEditorModel {
    #[must_use]
    pub fn new() -> Self {
        let mut slots = Vec::with_capacity(24);
        for i in 0..24 {
            let hue = (i as f32) / 24.0;
            let r = (hue * 6.0).sin().abs();
            let g = (hue * 6.0 + 2.0).sin().abs();
            let b = (hue * 6.0 + 4.0).sin().abs();
            slots.push(CompactSampleSlot::new(
                i,
                format!("Material #{i}"),
                [r, g, b, 1.0],
            ));
        }
        Self {
            mode: MaterialEditorMode::Compact,
            active_slot_index: 0,
            sample_slots: slots,
            show_background: true,
            show_backlight: true,
            sample_uv_tiling: 1.0,
        }
    }
}
