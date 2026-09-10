//! Oxide-3D Custom Iced Widgets (Viewport Canvas, Tree View, Node Graph Canvas).

pub mod viewport;

pub use viewport::{ViewportMessage, ViewportState, ViewportWidget, viewport_canvas};

use serde::{Deserialize, Serialize};

/// Message emitted by the Parametric Feature Tree widget.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeatureTreeMessage {
    /// Feature node selected in tree.
    SelectFeature {
        /// Selected feature id.
        id: usize,
    },
    /// Toggle feature suppression.
    ToggleSuppress {
        /// Toggled feature id.
        id: usize,
    },
}
