//! Oxide-3D Telemetry, structured logs, and metrics.

use thiserror::Error;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Telemetry initialization errors.
#[derive(Debug, Error)]
pub enum TelemetryError {
    /// Failure initializing tracing subscriber.
    #[error("Failed to initialize tracing subscriber: {0}")]
    InitError(String),
}

/// Initialize tracing with environment filtering and console output.
pub fn init() -> Result<(), TelemetryError> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,oxide=debug"));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .try_init()
        .map_err(|e| TelemetryError::InitError(e.to_string()))?;

    tracing::info!("Oxide-3D Telemetry initialized");
    Ok(())
}
