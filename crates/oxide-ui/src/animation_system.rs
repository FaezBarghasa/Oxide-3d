//! Animation, Timeline, Controllers, Rigging and Track View System.

use serde::{Deserialize, Serialize};

/// Animation Keyframing Modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum KeyframeMode {
    #[default]
    AutoKey,
    SetKey,
}

/// Track View Display Mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TrackViewMode {
    #[default]
    CurveEditor,
    DopeSheet,
}

/// Controller Type Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ControllerKind {
    #[default]
    BezierPosition,
    LinearPosition,
    TcbPosition,
    PositionXyz,
    EulerRotation,
    TcbRotation,
    BezierScale,
    ScaleXyz,
    PathConstraint,
    LookAtConstraint,
    OrientationConstraint,
    PositionConstraint,
    LinkConstraint,
    TransformScript,
}

/// Keyframe Filter Flags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyFilters {
    pub position: bool,
    pub rotation: bool,
    pub scale: bool,
    pub ik_parameters: bool,
    pub object_parameters: bool,
    pub custom_attributes: bool,
    pub modifiers: bool,
    pub materials: bool,
}

impl Default for KeyFilters {
    fn default() -> Self {
        Self {
            position: true,
            rotation: true,
            scale: true,
            ik_parameters: true,
            object_parameters: false,
            custom_attributes: false,
            modifiers: false,
            materials: false,
        }
    }
}

/// Animation Timeline & Rigging State Model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationSystemModel {
    pub key_mode: KeyframeMode,
    pub track_view: TrackViewMode,
    pub current_frame: i32,
    pub start_frame: i32,
    pub end_frame: i32,
    pub fps: f32,
    pub is_playing: bool,
    pub key_filters: KeyFilters,
    pub active_layer: String,
    pub animation_layers: Vec<String>,
}

impl Default for AnimationSystemModel {
    fn default() -> Self {
        Self {
            key_mode: KeyframeMode::AutoKey,
            track_view: TrackViewMode::CurveEditor,
            current_frame: 0,
            start_frame: 0,
            end_frame: 100,
            fps: 30.0,
            is_playing: false,
            key_filters: KeyFilters::default(),
            active_layer: "BaseLayer".to_string(),
            animation_layers: vec!["BaseLayer".to_string(), "AnimLayer_01".to_string()],
        }
    }
}

impl AnimationSystemModel {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Step forward one frame.
    pub fn next_frame(&mut self) {
        if self.current_frame < self.end_frame {
            self.current_frame += 1;
        } else {
            self.current_frame = self.start_frame;
        }
    }

    /// Step backward one frame.
    pub fn prev_frame(&mut self) {
        if self.current_frame > self.start_frame {
            self.current_frame -= 1;
        } else {
            self.current_frame = self.end_frame;
        }
    }
}
