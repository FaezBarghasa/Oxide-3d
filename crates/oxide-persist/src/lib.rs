//! Oxide-3D Native .oxd File Format Container and Persistence Engine.

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use thiserror::Error;
use uuid::Uuid;

/// Persistence engine errors.
#[derive(Debug, Error)]
pub enum PersistError {
    /// Format container error.
    #[error("Container format error: {0}")]
    Format(String),

    /// Serialization/deserialization failure.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Compression/decompression failure.
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

/// Generic complete Oxide document data package for persistence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxdDocument<T> {
    /// Document manifest.
    pub manifest: OxdManifest,
    /// Payload containing scene graph, features, and/or topology data.
    pub payload: T,
}

/// Save an `.oxd` document to a compressed binary file.
pub fn save_document<P: AsRef<Path>, T: Serialize>(path: P, doc: &OxdDocument<T>) -> Result<(), PersistError> {
    let raw_bytes = rmp_serde::to_vec(doc)
        .map_err(|e| PersistError::Serialization(e.to_string()))?;

    let compressed = zstd::encode_all(&raw_bytes[..], 3)
        .map_err(|e| PersistError::Compression(e.to_string()))?;

    let mut file = File::create(path)?;
    file.write_all(&compressed)?;
    Ok(())
}

/// Load an `.oxd` document from a compressed binary file.
pub fn load_document<P: AsRef<Path>, T: for<'de> Deserialize<'de>>(path: P) -> Result<OxdDocument<T>, PersistError> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let decompressed = zstd::decode_all(&buffer[..])
        .map_err(|e| PersistError::Compression(e.to_string()))?;

    let doc: OxdDocument<T> = rmp_serde::from_slice(&decompressed)
        .map_err(|e| PersistError::Serialization(e.to_string()))?;

    Ok(doc)
}

/// Save an `.oxd` manifest to a JSON file path.
pub fn save_manifest<P: AsRef<Path>>(path: P, manifest: &OxdManifest) -> Result<(), PersistError> {
    let json =
        serde_json::to_vec_pretty(manifest).map_err(|e| PersistError::Format(e.to_string()))?;
    std::fs::write(path, json)?;
    Ok(())
}
