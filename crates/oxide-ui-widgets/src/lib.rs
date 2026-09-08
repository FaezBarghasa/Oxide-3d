//! Oxide-3D Custom Iced Widgets (Viewport Canvas, Tree View, Node Graph Canvas).

use serde::{Deserialize, Serialize};

/// Message emitted by the interactive 3D Viewport widget.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ViewportMessage {
    /// Orbit camera drag [dx, dy].
    Orbit { dx: f32, dy: f32 },
    /// Pan camera drag [dx, dy].
    Pan { dx: f32, dy: f32 },
    /// Zoom camera delta.
    Zoom { delta: f32 },
    /// Selection click at screen coords [x, y].
    Pick { screen_pos: [f32; 2] },
}

/// Message emitted by the Parametric Feature Tree widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureTreeMessage {
    /// Feature node selected in tree.
    SelectFeature { id: usize },
    /// Toggle feature suppression.
    ToggleSuppress { id: usize },
}
