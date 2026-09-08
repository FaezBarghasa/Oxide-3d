//! Oxide-3D Desktop GUI Application Entry Point.

use oxide_ui::OxideApp;

fn main() -> iced::Result {
    let _ = oxide_telemetry::init();
    tracing::info!("Starting Oxide-3D Desktop UI");

    iced::application(
        "Oxide-3D Engineering Platform",
        OxideApp::update,
        OxideApp::view,
    )
    .run()
}
