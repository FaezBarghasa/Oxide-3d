use serde::{Deserialize, Serialize};
use slotmap::new_key_type;
use uuid::Uuid;

new_key_type! {
    /// Stable slotmap identifier for entities in a scene or document.
    pub struct EntityKey;
    /// Stable identifier for parts within an assembly.
    pub struct PartKey;
    /// Stable identifier for B-Rep faces.
    pub struct FaceKey;
    /// Stable identifier for B-Rep edges.
    pub struct EdgeKey;
    /// Stable identifier for B-Rep vertices.
    pub struct VertexKey;
    /// Stable identifier for B-Rep wires.
    pub struct WireKey;
    /// Stable identifier for B-Rep shells.
    pub struct ShellKey;
    /// Stable identifier for B-Rep solids.
    pub struct SolidKey;
}

/// A universally unique operation identifier based on UUIDv7 for time-ordered event streams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OperationId(pub Uuid);

impl OperationId {
    /// Creates a new time-ordered UUIDv7 operation ID.
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for OperationId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for OperationId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
