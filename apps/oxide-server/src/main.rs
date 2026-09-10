//! Oxide-3D Web & Collaboration Server.

mod handlers;
mod web_ui;

use axum::Router;
use axum::response::Html;
use axum::routing::{get, post};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = oxide_telemetry::init();
    tracing::info!("Starting Oxide-3D Engineering Platform Server");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Serve Web Client at root
        .route("/", get(|| async { Html(web_ui::get_web_app_html()) }))
        // REST API
        .route("/api/health", get(handlers::health_handler))
        .route("/api/settings", get(handlers::get_settings_handler).post(handlers::save_settings_handler))
        .route("/api/geometry/primitive", post(handlers::create_primitive_handler))
        .route("/api/cam/toolpath", post(handlers::generate_cam_handler))
        .route("/api/plm/sample-bom", get(handlers::get_plm_bom_handler))
        .route("/api/command", post(handlers::execute_command_handler))
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Oxide-3D Server listening at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
