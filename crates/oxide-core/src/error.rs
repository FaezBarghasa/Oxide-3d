use thiserror::Error;

/// Standard result type for core operations.
pub type CoreResult<T> = Result<T, CoreError>;

/// Core domain error types.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Referenced entity was not found in slotmap storage.
    #[error("Entity not found: {0}")]
    EntityNotFound(String),

    /// Invalid command execution or parameters.
    #[error("Invalid command execution: {0}")]
    InvalidCommand(String),

    /// Invalid operation in event log or history.
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    /// Unit parsing or conversion mismatch.
    #[error("Unit conversion error: {0}")]
    UnitError(String),

    /// Serialization or deserialization error.
    #[error("Serialization failure: {0}")]
    SerializationError(String),

    /// General I/O or persistent storage failure.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
