//! Oxide-3D Iced Application Shell, Workspace Layouts, and Command Palette.

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};
use oxide_core::command::OxideCommand;
use oxide_settings::OxideSettings;

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
    /// Dispatch a core domain command.
    Command(OxideCommand),
    /// Open document dialog.
    OpenFileDialog,
}

/// Main Oxide-3D Iced Application State.
pub struct OxideApp {
    /// Current workspace mode.
    pub mode: WorkspaceMode,
    /// Active settings.
    pub settings: OxideSettings,
    /// Document status banner text.
    pub status_text: String,
}

impl Default for OxideApp {
    fn default() -> Self {
        Self {
            mode: WorkspaceMode::Model,
            settings: OxideSettings::default(),
            status_text: "Ready — Oxide-3D Industrial CAD/CAE Platform".to_string(),
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
        let header = row![
            text("OXIDE-3D").size(20),
            text(format!(" | Mode: {:?}", self.mode)).size(16),
        ]
        .spacing(12)
        .padding(8);

        let content = container(
            column![
                text(format!("Active Workspace: {:?}", self.mode)).size(24),
                text("Interactive 3D Viewport / Node Editor Area").size(16),
            ]
            .spacing(16)
            .padding(20),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let footer = row![text(&self.status_text).size(13)].padding(6);

        column![header, content, footer].into()
    }
}
