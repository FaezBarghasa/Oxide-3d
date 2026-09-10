//! Event-Sourced Document and Temporal Operation Log for Oxide-3D.
//!
//! Provides an immutable, append-only operation log with millisecond-accurate UUIDv7 keys.
//! Supports infinite undo/redo, rollback bar sliding (CAD feature rollback / DCC modifier evaluation),
//! branching revisions, and state snapshots.

use crate::error::{CoreError, CoreResult};
use crate::id::{EntityKey, OperationId};
use serde::{Deserialize, Serialize};

/// Status of an operation in the event log.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationStatus {
    /// Active and applied to the current scene evaluation.
    Applied,
    /// Rolled back via undo or rollback bar.
    Suppressed,
    /// Failed evaluation with error recorded.
    Failed,
}

/// Generic atomic operational delta applied to the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationPayload {
    /// Create or instantiate a new entity.
    CreateEntity {
        /// Type tag of entity.
        entity_type: String,
        /// Initial serialized state.
        data: serde_json::Value,
    },
    /// Modify properties or parameters of an existing entity.
    UpdateEntity {
        /// Target entity.
        target: EntityKey,
        /// Parameter key.
        parameter: String,
        /// Previous value for undo verification.
        old_value: serde_json::Value,
        /// New value.
        new_value: serde_json::Value,
    },
    /// Delete or destroy an entity.
    DeleteEntity {
        /// Target entity.
        target: EntityKey,
        /// Serialized snapshot of entity for undo restoration.
        snapshot: serde_json::Value,
    },
    /// Parametric CAD feature (Extrude, Revolve, Fillet, Pattern).
    CadFeature {
        /// Feature name/type.
        feature_type: String,
        /// Input parameters (depth, draft, profiles).
        params: serde_json::Value,
    },
    /// DCC Modifier (Subdivision, Mirror, Bevel, Geometry Nodes).
    DccModifier {
        /// Target entity.
        target: EntityKey,
        /// Modifier name/type.
        modifier_type: String,
        /// Configuration settings.
        settings: serde_json::Value,
    },
    /// Animation keyframe or curve track update.
    AnimationKey {
        /// Target entity.
        target: EntityKey,
        /// Property path (e.g. `transform.position.x`).
        property_path: String,
        /// Frame number or timestamp.
        time: f64,
        /// Value at key.
        value: serde_json::Value,
    },
    /// Custom extension or script-generated operation.
    Custom {
        /// Identifier of operation type.
        op_tag: String,
        /// Payload.
        data: serde_json::Value,
    },
}

/// Immutable historical entry in the document's event stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationRecord {
    /// Unique time-ordered ID.
    pub id: OperationId,
    /// Human-readable label for history UI and feature tree.
    pub label: String,
    /// Operation payload.
    pub payload: OperationPayload,
    /// Current status.
    pub status: OperationStatus,
    /// Timestamp (UTC milliseconds since Unix epoch).
    pub timestamp_ms: u64,
}

impl OperationRecord {
    /// Create a new applied operation record with generated UUIDv7.
    #[must_use]
    pub fn new(label: impl Into<String>, payload: OperationPayload) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        Self {
            id: OperationId::new(),
            label: label.into(),
            payload,
            status: OperationStatus::Applied,
            timestamp_ms: now,
        }
    }
}

/// Event-Sourced Document Operation Log.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventLog {
    /// Ordered stream of operations.
    entries: Vec<OperationRecord>,
    /// Active evaluation cursor (index in `entries`).
    /// Operations with index `< cursor` are active; `>= cursor` are rolled back/suppressed.
    cursor: usize,
}

impl EventLog {
    /// Creates an empty event log.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            cursor: 0,
        }
    }

    /// Appends and applies a new operation to the log, truncating any rolled-back redo history.
    pub fn append(&mut self, label: impl Into<String>, payload: OperationPayload) -> OperationId {
        if self.cursor < self.entries.len() {
            self.entries.truncate(self.cursor);
        }

        let record = OperationRecord::new(label, payload);
        let id = record.id;
        self.entries.push(record);
        self.cursor = self.entries.len();
        id
    }

    /// Number of total operations recorded.
    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the log is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Current evaluation cursor position.
    #[must_use]
    pub fn cursor(&self) -> usize {
        self.cursor
    }

    /// Slice of all operations.
    #[must_use]
    pub fn entries(&self) -> &[OperationRecord] {
        &self.entries
    }

    /// Slice of currently active operations (up to cursor).
    #[must_use]
    pub fn active_entries(&self) -> &[OperationRecord] {
        &self.entries[..self.cursor]
    }

    /// Undo one operation by stepping the evaluation cursor back.
    pub fn undo(&mut self) -> CoreResult<Option<&OperationRecord>> {
        if self.cursor == 0 {
            return Ok(None);
        }
        self.cursor -= 1;
        self.entries[self.cursor].status = OperationStatus::Suppressed;
        Ok(Some(&self.entries[self.cursor]))
    }

    /// Redo one operation by advancing the evaluation cursor.
    pub fn redo(&mut self) -> CoreResult<Option<&OperationRecord>> {
        if self.cursor >= self.entries.len() {
            return Ok(None);
        }
        self.entries[self.cursor].status = OperationStatus::Applied;
        let record = &self.entries[self.cursor];
        self.cursor += 1;
        Ok(Some(record))
    }

    /// Move the rollback bar to a specific historical operation index $k \in [0, N]$.
    pub fn set_cursor(&mut self, new_cursor: usize) -> CoreResult<()> {
        if new_cursor > self.entries.len() {
            return Err(CoreError::InvalidOperation(format!(
                "Rollback cursor index {} out of range (max {})",
                new_cursor,
                self.entries.len()
            )));
        }

        for (i, entry) in self.entries.iter_mut().enumerate() {
            if i < new_cursor {
                entry.status = OperationStatus::Applied;
            } else {
                entry.status = OperationStatus::Suppressed;
            }
        }

        self.cursor = new_cursor;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_log_append_undo_redo() {
        let mut log = EventLog::new();
        assert!(log.is_empty());

        let id1 = log.append(
            "Create Cube",
            OperationPayload::CreateEntity {
                entity_type: "Cube".to_string(),
                data: serde_json::json!({"size": 10.0}),
            },
        );
        let id2 = log.append(
            "Extrude Face",
            OperationPayload::CadFeature {
                feature_type: "Extrude".to_string(),
                params: serde_json::json!({"distance": 25.0}),
            },
        );

        assert_eq!(log.len(), 2);
        assert_eq!(log.cursor(), 2);
        assert_eq!(log.active_entries().len(), 2);

        // Undo Extrude
        let undone = log.undo().unwrap();
        assert!(undone.is_some());
        assert_eq!(undone.unwrap().id, id2);
        assert_eq!(log.cursor(), 1);
        assert_eq!(log.active_entries().len(), 1);

        // Undo Cube
        let undone_cube = log.undo().unwrap();
        assert!(undone_cube.is_some());
        assert_eq!(undone_cube.unwrap().id, id1);
        assert_eq!(log.cursor(), 0);
        assert!(log.active_entries().is_empty());

        // Redo Cube
        let redone = log.redo().unwrap();
        assert!(redone.is_some());
        assert_eq!(redone.unwrap().id, id1);
        assert_eq!(log.cursor(), 1);

        // Rollback Bar setting
        log.set_cursor(2).unwrap();
        assert_eq!(log.cursor(), 2);
        assert_eq!(log.active_entries().len(), 2);
    }
}
