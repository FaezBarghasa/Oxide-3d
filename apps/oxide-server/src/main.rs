//! Oxide-3D Collaboration Relay & Remote Compute Server.

use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = oxide_telemetry::init();
    tracing::info!("Starting Oxide-3D Collaboration Server");

    let app = Router::new().route("/health", get(|| async { "OK" }));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
