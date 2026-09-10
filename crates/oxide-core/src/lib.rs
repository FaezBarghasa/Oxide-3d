//! Oxide-3D Core domain primitives, IDs, events, commands, and error handling.

/// Async command bus and handler registry.
pub mod bus;
/// Industrial CAD system commands and transactions.
pub mod command;
/// Core error types and domain result abstractions.
pub mod error;
/// Pub-sub system event dispatching.
pub mod event;
/// Typed entity key identifiers and arena slot keys.
pub mod id;
/// Physical engineering units and dimension conversions.
pub mod units;

pub use bus::CommandBus;
pub use command::OxideCommand;
pub use error::{CoreError, CoreResult};
pub use event::{EventCallback, OxideEvent};
pub use id::{EdgeKey, EntityKey, FaceKey, OperationId, PartKey, ShellKey, SolidKey, VertexKey, WireKey};
