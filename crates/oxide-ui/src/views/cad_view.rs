//! SolidWorks-Style CAD Workspace View Renderer.

use iced::widget::{button, column, container, row, text};
use iced::{Element, Length};
use oxide_ui_widgets::viewport_canvas;

use crate::{OxideApp, OxideUiMessage};

/// Render the SolidWorks-style Parametric CAD workspace.
pub fn render_cad_workspace(app: &OxideApp) -> Element<'_, OxideUiMessage> {
    // 1. CommandManager Tab Bar
    let available_tabs = app.command_manager.get_available_tabs();
    let command_tabs = available_tabs.iter().fold(row![].spacing(3), |acc, tab| {
        let is_active = app.command_manager.active_tab == *tab;
        let lbl = if is_active {
            format!("▶ {}", tab.label())
        } else {
            tab.label().to_string()
        };
        let btn = button(text(lbl).size(11)).padding([3, 7]);
        acc.push(btn)
    });

    // 1.1 Active Tab Tools
    let active_tools = app
        .command_manager
        .get_tools(app.command_manager.active_tab);
    let tool_buttons = active_tools
        .into_iter()
        .take(8)
        .fold(row![].spacing(4), |acc, tool| {
            let btn =
                button(text(tool.name).size(11))
                    .padding([3, 6])
                    .on_press(match tool.action_id {
                        "cmd.extrude" => OxideUiMessage::ToolExtrude,
                        "cmd.revolve" => OxideUiMessage::ToolRevolve,
                        "cmd.fillet" => OxideUiMessage::ToolFillet,
                        _ => OxideUiMessage::ToolSelect,
                    });
            acc.push(btn)
        });

    let command_manager_container =
        container(column![command_tabs, tool_buttons].spacing(4)).padding(4);

    // 2. Center 3D Viewport canvas
    let viewport = viewport_canvas(&app.camera, &app.active_mesh, OxideUiMessage::Viewport);

    // 3. Left-Hand FeatureManager Design Tree & PropertyManager
    let feature_tree_col = app
        .feature_tree
        .nodes
        .iter()
        .fold(column![].spacing(3), |col, node| {
            col.push(
                row![
                    text(if node.is_selected { "▶" } else { "•" }).size(11),
                    text(&node.label).size(11),
                ]
                .spacing(4),
            )
        });

    let feature_manager_panel = container(
        column![
            text("FeatureManager Tree").size(12),
            text("──────────────────────").size(9),
            feature_tree_col,
            text("──────────────────────").size(9),
            text("PropertyManager").size(12),
            text(format!("Tool: {}", app.active_tool_name)).size(11),
        ]
        .spacing(4)
        .padding(6)
        .width(Length::Fixed(200.0)),
    );

    let center_area = row![feature_manager_panel, viewport]
        .width(Length::Fill)
        .height(Length::Fill);

    column![command_manager_container, center_area,].into()
}
