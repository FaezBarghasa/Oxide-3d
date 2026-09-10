//! DCC 3ds Max-Style Visual Workspace View Renderer.

use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};
use oxide_ui_widgets::viewport_canvas;

use crate::{
    CommandPanelTab, DccMenuCategory, DccMenuItemDef, OxideApp, OxideUiMessage, RibbonTab,
};

/// Render the complete DCC Workspace view.
pub fn render_dcc_workspace(app: &OxideApp) -> Element<'_, OxideUiMessage> {
    // 1. Top 13-Menu Bar
    let menu_categories = DccMenuCategory::all();
    let menu_bar_row = menu_categories.iter().fold(
        row![text("OXIDE DCC").size(15)].spacing(6).align_y(Alignment::Center),
        |acc, cat| {
            let is_open = app.dcc_menu.active_menu == Some(*cat);
            let label_text = if is_open {
                format!("▼ {}", cat.label())
            } else {
                cat.label().to_string()
            };
            let btn = button(text(label_text).size(12))
                .padding([3, 7])
                .on_press(OxideUiMessage::ToggleDccMenu(*cat));
            acc.push(btn)
        },
    );

    // 1.1 Dropdown items container if a menu is open
    let menu_dropdown: Option<Element<'_, OxideUiMessage>> = app.dcc_menu.active_menu.map(|cat| {
        let items = app.dcc_menu.get_menu_items(cat);
        let items_col = items.into_iter().take(15).fold(column![].spacing(2), |col, item: DccMenuItemDef| {
            if item.is_separator {
                col.push(text("──────────").size(9))
            } else {
                let lbl = if let Some(sc) = item.shortcut {
                    format!("{} ({})", item.label, sc)
                } else {
                    item.label
                };
                col.push(button(text(lbl).size(11)).padding([2, 6]).width(Length::Fill))
            }
        });
        container(scrollable(items_col).height(Length::Fixed(180.0)))
            .padding(4)
            .into()
    });

    // 2. DCC Main Toolbar (Transforms, Snaps, Selection, Named Sets, Explorers)
    let toolbar_row = row![
        text("Filter:").size(11),
        button(text(app.dcc_toolbar.selection_filter.label()).size(11)).padding([2, 5]),
        text("│").size(10),
        button(text("Select (Q)").size(11)).padding([2, 5]).on_press(OxideUiMessage::ToolSelect),
        button(text("Move (W)").size(11)).padding([2, 5]),
        button(text("Rotate (E)").size(11)).padding([2, 5]),
        button(text("Scale (R)").size(11)).padding([2, 5]),
        text("│").size(10),
        text(format!("Coord: {}", app.dcc_toolbar.coord_system.label())).size(11),
        text("│").size(10),
        button(text(if app.dcc_toolbar.snaps.grid_snap { "Grid [ON]" } else { "Grid [OFF]" }).size(11)).padding([2, 5]),
        button(text(if app.dcc_toolbar.snaps.angle_snap { "Angle [ON]" } else { "Angle [OFF]" }).size(11)).padding([2, 5]),
        text("│").size(10),
        button(text("Mat Editor (M)").size(11)).padding([2, 5]),
        button(text("Render Setup (F10)").size(11)).padding([2, 5]),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    // 3. Graphite Modeling Ribbon
    let ribbon_tabs = RibbonTab::all();
    let ribbon_tab_buttons = ribbon_tabs.iter().fold(row![].spacing(4), |acc, tab| {
        let is_active = app.graphite_ribbon.active_tab == *tab;
        let lbl = if is_active {
            format!("• {}", tab.label())
        } else {
            tab.label().to_string()
        };
        let btn = button(text(lbl).size(11))
            .padding([2, 6])
            .on_press(OxideUiMessage::SelectRibbonTab(*tab));
        acc.push(btn)
    });

    let ribbon_subobjects = row![
        text("Sub-Object:").size(11),
        button(text("Vertex (1)").size(10)).padding([2, 4]),
        button(text("Edge (2)").size(10)).padding([2, 4]),
        button(text("Border (3)").size(10)).padding([2, 4]),
        button(text("Polygon (4)").size(10)).padding([2, 4]),
        button(text("Element (5)").size(10)).padding([2, 4]),
    ]
    .spacing(4)
    .align_y(Alignment::Center);

    let ribbon_container = container(
        column![ribbon_tab_buttons, ribbon_subobjects].spacing(3)
    ).padding(3);

    // 4. Center 3D Viewport canvas
    let viewport = viewport_canvas(&app.camera, &app.active_mesh, OxideUiMessage::Viewport);

    // 5. Right-Hand 6-Tab Command Panel
    let command_tabs = CommandPanelTab::all();
    let command_tab_bar = command_tabs.iter().fold(row![].spacing(2), |acc, tab| {
        let is_active = app.command_panel.active_tab == *tab;
        let lbl = if is_active {
            format!("▶{}", tab.label())
        } else {
            tab.label().to_string()
        };
        let btn = button(text(lbl).size(10))
            .padding([3, 4])
            .on_press(OxideUiMessage::SelectCommandPanelTab(*tab));
        acc.push(btn)
    });

    let panel_content = match app.command_panel.active_tab {
        CommandPanelTab::Create => column![
            text("Standard Primitives").size(12),
            row![
                button(text("Box").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Box".to_string())),
                button(text("Sphere").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Sphere".to_string())),
            ].spacing(4),
            row![
                button(text("Cylinder").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Cylinder".to_string())),
                button(text("Torus").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Torus".to_string())),
            ].spacing(4),
            row![
                button(text("Cone").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Cone".to_string())),
                button(text("Plane").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Plane".to_string())),
            ].spacing(4),
            row![
                button(text("Teapot").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Teapot".to_string())),
                button(text("Grid").size(11)).padding([3, 6]).on_press(OxideUiMessage::CreateDccPrimitive("Grid".to_string())),
            ].spacing(4),
        ].spacing(6),
        CommandPanelTab::Modify => {
            let stack_col = app.command_panel.modify.stack.iter().fold(column![].spacing(3), |col, item| {
                col.push(row![
                    text(if item.enabled { "👁" } else { "⊘" }).size(11),
                    text(&item.name).size(11),
                ].spacing(4))
            });
            column![
                text("Modifier Stack").size(12),
                stack_col,
                text("──────────────────").size(9),
                button(text("TurboSmooth").size(11)).width(Length::Fill),
                button(text("Edit Poly").size(11)).width(Length::Fill),
            ].spacing(4)
        },
        _ => column![
            text(format!("{} Panel", app.command_panel.active_tab.label())).size(12),
            text("Parameters & Rollouts active").size(11),
        ].spacing(4),
    };

    let command_panel = container(
        column![
            command_tab_bar,
            text("─────────────────────").size(9),
            panel_content,
        ]
        .spacing(6)
        .padding(6)
        .width(Length::Fixed(200.0)),
    );

    let center_view = row![viewport, command_panel].width(Length::Fill).height(Length::Fill);

    // 6. Bottom Animation Timeline Bar & Navigation HUD
    let timeline_bar = row![
        button(text(if app.animation_system.is_playing { "Pause ⏸" } else { "Play ▶" }).size(11))
            .padding([3, 8])
            .on_press(OxideUiMessage::TimelinePlayToggle),
        button(text("◀ Prev").size(11)).padding([3, 6]),
        button(text("Next ▶").size(11)).padding([3, 6]),
        text(format!("Frame: {} / {}", app.animation_system.current_frame, app.animation_system.end_frame)).size(11),
        text("│ Shading: Realistic (F3/F4) │").size(11),
        button(text(if app.dcc_viewport.is_maximized { "Restore View" } else { "Maximize (Alt+W)" }).size(11))
            .padding([3, 6])
            .on_press(OxideUiMessage::ToggleMaximizeViewport),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let mut main_col = column![
        container(menu_bar_row).padding(4),
    ];

    if let Some(dropdown) = menu_dropdown {
        main_col = main_col.push(dropdown);
    }

    main_col = main_col
        .push(container(toolbar_row).padding(3))
        .push(ribbon_container)
        .push(center_view)
        .push(container(timeline_bar).padding(4));

    main_col.into()
}
