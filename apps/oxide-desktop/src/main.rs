//! Oxide-3D Desktop GUI Application Entry Point.

use oxide_ui::OxideApp;

fn main() -> iced::Result {
    let _ = oxide_telemetry::init();
    tracing::info!("Starting Oxide-3D Desktop UI");

    iced::application(
        OxideApp::new,
        OxideApp::update,
        OxideApp::view,
    )
    .title("Oxide-3D Engineering Platform")
    .run()
}
