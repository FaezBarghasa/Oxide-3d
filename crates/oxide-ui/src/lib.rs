//! Oxide-3D Iced Application Shell, Workspace Layouts, and Command Palette.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Length, Task};
use oxide_core::command::OxideCommand;
use oxide_render::{Camera, TriMesh};
use oxide_settings::OxideSettings;
use oxide_ui_widgets::{viewport_canvas, ViewportMessage};

/// Application workspace modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorkspaceMode {
    /// 3D Part & Surface Modeling.
    #[default]
    Model,
    /// Multi-Component Assemblies.
    Assembly,
    /// Procedural Geometry Nodes.
    Nodes,
    /// FEA / CFD / Thermal Simulation.
    Simulation,
    /// Kinematics & Mechanics.
    Mechanism,
    /// CAM & Toolpath Generation.
    Cam,
    /// GD&T & Metrology.
    Inspect,
    /// Product Lifecycle Management & BOM.
    Plm,
}

/// Oxide-3D UI top-level message enum.
#[derive(Debug, Clone)]
pub enum OxideUiMessage {
    /// Mode tab switch.
    SwitchMode(WorkspaceMode),
    /// Viewport camera/selection event.
    Viewport(ViewportMessage),
    /// Dispatch a core domain command.
    Command(OxideCommand),
    /// Open document dialog.
    OpenFileDialog,
    /// Reset camera view.
    ResetCamera,
}

/// Main Oxide-3D Iced Application State.
#[derive(Debug)]
pub struct OxideApp {
    /// Current workspace mode.
    pub mode: WorkspaceMode,
    /// Active settings.
    pub settings: OxideSettings,
    /// Document status banner text.
    pub status_text: String,
    /// Viewport 3D camera.
    pub camera: Camera,
    /// Active demo/scene triangle mesh.
    pub active_mesh: TriMesh,
}

impl Default for OxideApp {
    fn default() -> Self {
        Self {
            mode: WorkspaceMode::Model,
            settings: OxideSettings::default(),
            status_text: "Ready — Oxide-3D Industrial CAD/CAE Platform".to_string(),
            camera: Camera::default(),
            active_mesh: TriMesh::cube(2.0, [0.2, 0.6, 0.95, 0.85]),
        }
    }
}

impl OxideApp {
    /// Initialize application state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Application update logic.
    pub fn update(&mut self, message: OxideUiMessage) -> Task<OxideUiMessage> {
        match message {
            OxideUiMessage::SwitchMode(new_mode) => {
                self.mode = new_mode;
                self.status_text = format!("Switched to {:?} mode", new_mode);
            }
            OxideUiMessage::Viewport(vp_msg) => match vp_msg {
                ViewportMessage::Orbit { dx, dy } => {
                    self.camera.orbit(dx, dy);
                }
                ViewportMessage::Pan { dx, dy } => {
                    self.camera.pan(dx, dy);
                }
                ViewportMessage::Zoom { delta } => {
                    self.camera.zoom(delta);
                }
                ViewportMessage::Pick { screen_pos } => {
                    self.status_text = format!(
                        "Picked screen point [{:.1}, {:.1}]",
                        screen_pos[0], screen_pos[1]
                    );
                }
            },
            OxideUiMessage::ResetCamera => {
                self.camera = Camera::default();
                self.status_text = "Camera view reset".to_string();
            }
            OxideUiMessage::Command(cmd) => {
                tracing::info!(?cmd, "UI Command received");
            }
            OxideUiMessage::OpenFileDialog => {
                self.status_text = "Opening file dialog...".to_string();
            }
        }
        Task::none()
    }

    /// Application view construction.
    pub fn view(&self) -> Element<'_, OxideUiMessage> {
        let modes = [
            (WorkspaceMode::Model, "Model (CAD)"),
            (WorkspaceMode::Assembly, "Assembly"),
            (WorkspaceMode::Nodes, "Nodes"),
            (WorkspaceMode::Simulation, "Simulation"),
            (WorkspaceMode::Mechanism, "Mechanism"),
            (WorkspaceMode::Cam, "CAM"),
            (WorkspaceMode::Inspect, "Inspect"),
            (WorkspaceMode::Plm, "PLM"),
        ];

        let mode_buttons = modes.iter().fold(
            row![text("OXIDE-3D").size(18)].spacing(8).align_y(Alignment::Center),
            |acc, (mode, label)| {
                let is_active = self.mode == *mode;
                let btn = button(text(*label).size(13))
                    .padding([4, 10])
                    .on_press(OxideUiMessage::SwitchMode(*mode));
                acc.push(btn)
            },
        );

        let header = container(
            row![
                mode_buttons,
                row![
                    button(text("Reset View").size(13))
                        .padding([4, 10])
                        .on_press(OxideUiMessage::ResetCamera),
                    button(text("Open").size(13))
                        .padding([4, 10])
                        .on_press(OxideUiMessage::OpenFileDialog),
                ]
                .spacing(8)
            ]
            .width(Length::Fill)
            .align_y(Alignment::Center),
        )
        .padding(6);

        // Sidebar tools
        let sidebar = container(
            column![
                text("Toolbox").size(14),
                button(text("Select").size(12)).width(Length::Fill),
                button(text("Extrude").size(12)).width(Length::Fill),
                button(text("Revolve").size(12)).width(Length::Fill),
                button(text("Fillet").size(12)).width(Length::Fill),
                button(text("Boolean CSG").size(12)).width(Length::Fill),
                button(text("Meshing / FEA").size(12)).width(Length::Fill),
            ]
            .spacing(8)
            .padding(10)
            .width(Length::Fixed(160.0)),
        );

        // Interactive 3D Viewport canvas
        let viewport = viewport_canvas(&self.camera, &self.active_mesh, OxideUiMessage::Viewport);

        // Tree / Properties right panel
        let right_panel = container(
            column![
                text("Feature Tree").size(14),
                text("• Cube Solid (Demo)").size(12),
                text("• Mesh Triangles: 12").size(12),
                text("• Vertices: 24").size(12),
                text("Camera XYZ:").size(12),
                text(format!(
                    "[{:.1}, {:.1}, {:.1}]",
                    self.camera.eye.x, self.camera.eye.y, self.camera.eye.z
                ))
                .size(11),
            ]
            .spacing(8)
            .padding(10)
            .width(Length::Fixed(200.0)),
        );

        let center_area = row![sidebar, viewport, right_panel]
            .width(Length::Fill)
            .height(Length::Fill);

        let footer = container(
            row![
                text(&self.status_text).size(12),
                text(" | Orbit: Left Click + Drag | Pan: Middle/Right Click | Zoom: Scroll")
                    .size(12),
            ]
            .padding(4),
        );

        column![header, center_area, footer].into()
    }
}
