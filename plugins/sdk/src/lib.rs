//! Oxide-3D Plugin Authoring SDK for WebAssembly Components.

use serde::{Deserialize, Serialize};

/// Custom plugin metadata descriptor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    /// Plugin display name.
    pub name: String,
    /// SemVer string.
    pub version: String,
    /// Plugin author.
    pub author: String,
}
