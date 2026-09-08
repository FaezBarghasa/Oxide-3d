//! Oxide-3D Native .oxd File Format Container and Persistence Engine.

use std::path::Path;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

/// Persistence engine errors.
#[derive(Debug, Error)]
pub enum PersistError {
    /// Format container error.
    #[error("Container format error: {0}")]
    Format(String),

    /// Compression failure.
    #[error("Compression error: {0}")]
    Compression(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Manifest stored in root of `.oxd` package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxdManifest {
    /// Schema format version.
    pub format_version: u32,
    /// Unique document ID.
    pub document_id: Uuid,
    /// Human readable project name.
    pub title: String,
    /// Timestamp created.
    pub created_at: String,
}

impl Default for OxdManifest {
    fn default() -> Self {
        Self {
            format_version: 1,
            document_id: Uuid::now_v7(),
            title: "Untitled Project".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
        }
    }
}

/// Save an `.oxd` manifest to a file path.
pub fn save_manifest<P: AsRef<Path>>(path: P, manifest: &OxdManifest) -> Result<(), PersistError> {
    let json = serde_json::to_vec_pretty(manifest)
        .map_err(|e| PersistError::Format(e.to_string()))?;
    std::fs::write(path, json)?;
    Ok(())
}
