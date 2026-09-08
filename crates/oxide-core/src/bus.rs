use std::sync::{Arc, Mutex};
use crate::command::OxideCommand;
use crate::error::CoreResult;
use crate::event::{EventCallback, OxideEvent};

/// Central command dispatcher and event bus for Oxide-3D.
#[derive(Default)]
pub struct CommandBus {
    subscribers: Arc<Mutex<Vec<EventCallback>>>,
}

impl std::fmt::Debug for CommandBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CommandBus").finish_non_exhaustive()
    }
}

impl CommandBus {
    /// Create a new command bus.
    #[must_use]
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribe to all emitted events on the bus.
    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&OxideEvent) + Send + Sync + 'static,
    {
        if let Ok(mut subs) = self.subscribers.lock() {
            subs.push(Box::new(callback));
        }
    }

    /// Emit an event to all subscribers.
    pub fn publish(&self, event: &OxideEvent) {
        if let Ok(subs) = self.subscribers.lock() {
            for sub in subs.iter() {
                sub(event);
            }
        }
    }

    /// Execute a command through the bus.
    pub fn dispatch(&self, command: OxideCommand) -> CoreResult<()> {
        tracing::debug!(?command, "Dispatching command");
        // Handlers can hook into this pipeline
        Ok(())
    }
}
