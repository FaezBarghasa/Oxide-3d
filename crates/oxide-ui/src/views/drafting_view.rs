//! OpenCADStudio-Style 2D Drafting Workspace View Renderer.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Element};
use oxide_ui_widgets::viewport_canvas;

use crate::{OxideApp, OxideUiMessage};

/// Render the OpenCADStudio 2D drafting workspace.
pub fn render_drafting_workspace(app: &OxideApp) -> Element<'_, OxideUiMessage> {
    // Top Drafting Ribbon (Draw, Modify, Layers, Annotation)
    let ribbon = row![
        column![
            text("Draw").size(11),
            row![
                button(text("Line (L)").size(10)).padding([2, 5]),
                button(text("PLine (PL)").size(10)).padding([2, 5]),
                button(text("Circle (C)").size(10)).padding([2, 5]),
                button(text("Arc (A)").size(10)).padding([2, 5]),
                button(text("Rect (REC)").size(10)).padding([2, 5]),
                button(text("Hatch (H)").size(10)).padding([2, 5]),
            ]
            .spacing(3),
        ]
        .spacing(2),
        text("│").size(12),
        column![
            text("Modify").size(11),
            row![
                button(text("Move (M)").size(10)).padding([2, 5]),
                button(text("Copy (CO)").size(10)).padding([2, 5]),
                button(text("Rotate (RO)").size(10)).padding([2, 5]),
                button(text("Trim (TR)").size(10)).padding([2, 5]),
                button(text("Extend (EX)").size(10)).padding([2, 5]),
                button(text("Fillet (F)").size(10)).padding([2, 5]),
                button(text("Offset (O)").size(10)).padding([2, 5]),
            ]
            .spacing(3),
        ]
        .spacing(2),
        text("│").size(12),
        column![
            text("Layers").size(11),
            row![
                text("Layer: 0 [White]").size(10),
                button(text("Layer Properties (LA)").size(10)).padding([2, 5]),
            ]
            .spacing(4),
        ]
        .spacing(2),
        text("│").size(12),
        column![
            text("Annotation").size(11),
            row![
                button(text("Dimension (DIM)").size(10)).padding([2, 5]),
                button(text("MText (T)").size(10)).padding([2, 5]),
                button(text("Leader").size(10)).padding([2, 5]),
            ]
            .spacing(3),
        ]
        .spacing(2),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    // Center 2D Drafting canvas
    let viewport = viewport_canvas(&app.camera, &app.active_mesh, OxideUiMessage::Viewport);

    // Bottom AutoCAD-Style Command Prompt
    let command_line = row![
        text("Command:").size(12),
        text_input(
            "Type a command (LINE, CIRCLE, PL, M, TR, EXT)...",
            &app.command_prompt.input_text
        )
        .size(12)
        .padding(4)
        .on_input(OxideUiMessage::CommandPromptInput)
        .on_submit(OxideUiMessage::CommandPromptSubmit),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    // Drafting status bar (OSNAP, ORTHO, POLAR, MODEL/PAPER)
    let status_bar = row![
        button(text("MODEL").size(10)).padding([2, 4]),
        text("│").size(10),
        button(text("GRID [ON]").size(10)).padding([2, 4]),
        button(text("SNAP [ON]").size(10)).padding([2, 4]),
        button(text("ORTHO [OFF]").size(10)).padding([2, 4]),
        button(text("POLAR [45°]").size(10)).padding([2, 4]),
        button(text("OSNAP [END,MID,CEN,INT]").size(10)).padding([2, 4]),
        button(text("DYN [ON]").size(10)).padding([2, 4]),
        text(format!("│ {}", app.command_prompt.prompt_message)).size(11),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    column![
        container(ribbon).padding(4),
        viewport,
        container(command_line).padding(3),
        container(status_bar).padding(3),
    ]
    .into()
}
