use serde::{Deserialize, Serialize};
use crate::id::{EntityKey, OperationId};

/// Domain events emitted during CAD modeling, simulation, and document lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OxideEvent {
    /// Document was initialized or opened.
    DocumentLoaded {
        /// Name or path of document.
        name: String,
    },
    /// An operation was appended to the event log.
    OperationApplied {
        /// Operation UUID.
        id: OperationId,
        /// Description.
        description: String,
    },
    /// An entity was created or modified.
    EntityUpdated {
        /// Target entity.
        key: EntityKey,
    },
    /// Viewport selection was updated.
    SelectionChanged {
        /// Currently selected entities.
        selected: Vec<EntityKey>,
    },
}

/// Dynamic callback for event listeners.
pub type EventCallback = Box<dyn Fn(&OxideEvent) + Send + Sync + 'static>;
