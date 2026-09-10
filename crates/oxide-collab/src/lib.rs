//! Oxide-3D Multi-User Collaboration, CRDT Operation Log Sync, and Git Integration.

use automerge::AutoCommit;
use automerge::sync::{Message as SyncMessage, State as SyncState, SyncDoc};
use serde::{Deserialize, Serialize};

/// Collaboration sync message transferred over WebSockets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollabMessage {
    /// Peer joined session.
    UserJoined {
        /// Username of the joining peer.
        username: String,
    },
    /// Peer cursor / selection update.
    UserPresence {
        /// Username.
        username: String,
        /// 3D position of the user's cursor.
        cursor: [f64; 3],
    },
    /// Compressed CRDT binary sync payload.
    SyncChanges {
        /// Serialized Automerge sync message.
        payload: Vec<u8>,
    },
}

/// Collaborative CRDT document session manager.
pub struct CollabSession {
    /// Automerge CRDT state container.
    pub doc: AutoCommit,
    /// Peer sync state.
    pub sync_state: SyncState,
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
            sync_state: SyncState::new(),
        }
    }
}

impl CollabSession {
    /// Create a new collaboration session.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate an incremental sync message for a peer.
    pub fn generate_sync_message(&mut self) -> Option<Vec<u8>> {
        self.doc
            .sync()
            .generate_sync_message(&mut self.sync_state)
            .map(|msg| msg.encode())
    }

    /// Receive and apply an incremental sync message from a peer.
    pub fn receive_sync_message(&mut self, payload: &[u8]) -> Result<(), String> {
        let msg = SyncMessage::decode(payload)
            .map_err(|e| format!("Failed to decode sync message: {e}"))?;
        self.doc
            .sync()
            .receive_sync_message(&mut self.sync_state, msg)
            .map_err(|e| format!("Failed to apply sync message: {e}"))?;
        Ok(())
    }

    /// Export the full document state as bytes.
    pub fn save_snapshot(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// Load a full document snapshot.
    pub fn load_snapshot(data: &[u8]) -> Result<Self, String> {
        let doc = AutoCommit::load(data).map_err(|e| format!("Failed to load snapshot: {e}"))?;
        Ok(Self {
            doc,
            sync_state: SyncState::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use automerge::ReadDoc;
    use automerge::transaction::Transactable;

    #[test]
    fn test_crdt_sync_between_peers() {
        let mut peer1 = CollabSession::new();
        let mut peer2 = CollabSession::new();

        // Peer 1 writes a feature property
        peer1
            .doc
            .put(automerge::ROOT, "part_name", "Robotic End Effector")
            .unwrap();

        // Round 1: Peer 2 initiates sync by sending its state to Peer 1
        if let Some(msg_init) = peer2.generate_sync_message() {
            peer1.receive_sync_message(&msg_init).unwrap();
        }

        // Round 2: Peer 1 responds with missing changes
        if let Some(msg_changes) = peer1.generate_sync_message() {
            peer2.receive_sync_message(&msg_changes).unwrap();
        }

        // Validate peer 2 now has the updated property
        let val = peer2
            .doc
            .get(automerge::ROOT, "part_name")
            .unwrap()
            .expect("Property should exist");

        assert_eq!(val.0.to_str().unwrap(), "Robotic End Effector");
    }
}
