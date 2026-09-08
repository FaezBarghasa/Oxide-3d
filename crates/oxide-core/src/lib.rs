//! Oxide-3D Core domain primitives, IDs, events, commands, and error handling.

pub mod bus;
pub mod command;
pub mod error;
pub mod event;
pub mod id;
pub mod units;

pub use bus::CommandBus;
pub use command::OxideCommand;
pub use error::{CoreError, CoreResult};
pub use event::{EventCallback, OxideEvent};
pub use id::{EdgeKey, EntityKey, FaceKey, OperationId, PartKey, ShellKey, SolidKey, VertexKey, WireKey};
