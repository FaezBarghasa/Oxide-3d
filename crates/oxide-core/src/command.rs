use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::id::{EdgeKey, EntityKey};

/// Enumeration of top-level user actions and domain commands in Oxide-3D.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OxideCommand {
    /// Undo the most recent operation.
    Undo,
    /// Redo the next operation in history.
    Redo,
    /// Initialize a new blank CAD/CAE document.
    NewDocument,
    /// Open a document from a local filesystem path.
    OpenDocument {
        /// File path to load.
        path: PathBuf,
    },
    /// Save the active document.
    SaveDocument {
        /// Optional path to save to; if None, saves in-place.
        path: Option<PathBuf>,
    },
    /// Extrude a 2D profile into a 3D solid.
    CreateExtrude {
        /// Sketch or face profile entity.
        profile: EntityKey,
        /// Extrusion distance in default document units.
        distance: f64,
    },
    /// Create a round fillet on selected edges.
    CreateFillet {
        /// Edge keys to fillet.
        edges: Vec<EdgeKey>,
        /// Fillet radius.
        radius: f64,
    },
    /// Run an external or embedded Python automation script.
    RunPythonScript {
        /// Path to the Python script.
        path: PathBuf,
    },
    /// Export active solid geometry to STEP AP242.
    ExportStep {
        /// Target output path.
        path: PathBuf,
    },
}
