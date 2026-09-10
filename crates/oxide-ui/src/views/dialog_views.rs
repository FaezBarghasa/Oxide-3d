//! Dialog Modals for Material Editor, Render Setup, and Preferences.

use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};

use crate::{OxideApp, OxideUiMessage};

/// Render the Material Editor dialog modal.
pub fn render_material_editor_dialog(app: &OxideApp) -> Element<'_, OxideUiMessage> {
    let slots_col = app.material_editor.sample_slots.iter().take(8).fold(
        column![].spacing(3),
        |col, slot| {
            col.push(
                row![
                    text("■").size(14),
                    text(&slot.name).size(11),
                    text(format!("({:?})", slot.material_type)).size(10),
                ]
                .spacing(4),
            )
        },
    );

    container(
        column![
            row![
                text("Material Editor — Compact (24 Slots)").size(13),
                button(text("✕").size(10)).padding([1, 4]),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text("──────────────────────────────").size(9),
            slots_col,
            text("──────────────────────────────").size(9),
            row![
                button(text("Assign to Selection").size(11)).padding([3, 6]),
                button(text("Get Material").size(11)).padding([3, 6]),
            ]
            .spacing(6),
        ]
        .spacing(6)
        .padding(8)
        .width(Length::Fixed(260.0)),
    )
    .into()
}

/// Render the Render Setup dialog modal.
pub fn render_render_setup_dialog(app: &OxideApp) -> Element<'_, OxideUiMessage> {
    container(
        column![
            row![
                text("Render Setup (F10)").size(13),
                button(text("✕").size(10)).padding([1, 4]),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            text("──────────────────────────────").size(9),
            text(format!("Target Renderer: {}", app.rendering_system.renderer.label())).size(11),
            text(format!("Output Resolution: {:?}", app.rendering_system.resolution)).size(11),
            text(format!("Lock Viewport: {}", app.rendering_system.lock_to_viewport)).size(11),
            text("──────────────────────────────").size(9),
            row![
                button(text("Render (F9)").size(11)).padding([3, 6]),
                button(text("ActiveShade").size(11)).padding([3, 6]),
            ]
            .spacing(6),
        ]
        .spacing(6)
        .padding(8)
        .width(Length::Fixed(260.0)),
    )
    .into()
}
