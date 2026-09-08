//! Oxide-3D Multi-User Collaboration, CRDT Operation Log Sync, and Git Integration.

use automerge::AutoCommit;
use serde::{Deserialize, Serialize};

/// Collaboration sync message transferred over WebSockets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollabMessage {
    /// Peer joined session.
    UserJoined { username: String },
    /// Peer cursor / selection update.
    UserPresence { username: String, cursor: [f64; 3] },
    /// Compressed CRDT binary sync payload.
    SyncChanges { payload: Vec<u8> },
}

/// Collaborative CRDT document session manager.
pub struct CollabSession {
    /// Automerge CRDT state container.
    pub doc: AutoCommit,
}

impl std::fmt::Debug for CollabSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CollabSession").finish_non_exhaustive()
    }
}

impl Default for CollabSession {
    fn default() -> Self {
        Self {
            doc: AutoCommit::new(),
        }
    }
}

impl CollabSession {
    /// Create a new collaboration session.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}
