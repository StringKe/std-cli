use crate::{
    shell_icons,
    shell_nav_model::{studio_nav_sections, StudioNavItem},
    ui,
    workspace_panes::{StudioWorkspaceCommand, WorkspaceCommandQueue},
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_studio::StudioPane;

const NAV_ROW_HEIGHT: f32 = 28.0;
const PANE_ROW_HEIGHT: f32 = 36.0;
const ICON_SIZE: f32 = Space::MD as f32;
const ICON_RAIL_WIDTH: f32 = Space::XL as f32;
const ICON_RAIL_HEIGHT: f32 = NAV_ROW_HEIGHT;
const PANE_CLOSE_SIZE: f32 = Space::LG as f32;
const PANE_CLOSE_CENTER_INSET: f32 = Space::SM as f32 + Space::TWO_XS as f32 / 2.0;
const PANE_STATE_CENTER_X: f32 = Space::XS as f32 + Space::TWO_XS as f32 / 2.0;
const PANE_STATE_RADIUS: f32 = 3.0;
const CLOSE_GLYPH_HALF: f32 = Space::XS as f32 / 2.0;
#[cfg(test)]
const TEST_NAV_WIDTH: f32 = Space::LG as f32 * 10.0;

impl StudioEguiApp {
    pub(crate) fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Space::XS as f32);
        if !self.layout.sidebar_open {
            self.render_icon_rail(ui);
            return;
        }
        self.render_workspace_nav(ui);
        ui.add_space(Space::LG as f32);
        self.render_workspace_pane_manager(ui);
    }

    fn render_icon_rail(&mut self, ui: &mut egui::Ui) {
        for pane in icon_rail_panes() {
            let selected = self.app.active_pane == pane;
            if nav_icon_button(ui, pane, selected)
                .on_hover_text(pane.label())
                .clicked()
            {
                self.app.switch_pane(pane);
            }
        }
    }

    fn render_workspace_nav(&mut self, ui: &mut egui::Ui) {
        for section in studio_nav_sections() {
            ui.vertical(|ui| {
                ui::section_header(ui, i18n::t(section.title_key), i18n::t(section.detail_key));
                for item in section.items {
                    self.render_nav_item(ui, &item);
                }
            });
            ui.add_space(Space::LG as f32);
        }
    }

    fn render_nav_item(&mut self, ui: &mut egui::Ui, item: &StudioNavItem) {
        let selected = self.app.active_pane == item.pane;
        if nav_row(ui, item.pane, item.title, selected).clicked() {
            if item.opens_workspace_pane {
                self.open_workspace_item(item.pane);
            } else {
                self.app.switch_pane(item.pane);
            }
        }
    }

    fn open_workspace_item(&mut self, pane: StudioPane) {
        let id = self.open_workspace_pane_for_nav(pane);
        self.status = format!(
            "{} {}",
            i18n::t("studio.status.workspace_pane_opened"),
            id.value()
        );
    }

    fn open_workspace_pane_for_nav(&mut self, pane: StudioPane) -> std_studio::WorkspacePaneId {
        match pane {
            StudioPane::Workflows => self
                .app
                .open_workflow_builder(self.app.core.config.workflows_dir()),
            StudioPane::Analysis => self
                .app
                .open_analysis_workbench(std::path::PathBuf::from(&self.analysis.path)),
            StudioPane::Plugins => self.app.open_plugin_manager_pane(),
            StudioPane::Memory => self.app.open_memory_browser_pane(),
            StudioPane::History => self.app.open_execution_history_pane(),
            StudioPane::Settings => self.app.open_settings_pane(),
            _ => self.app.open_workspace_pane(pane),
        }
    }

    pub(crate) fn render_workspace_pane_manager(&mut self, ui: &mut egui::Ui) {
        let panes = self
            .app
            .workspace_panes
            .iter()
            .map(|pane| (pane.id, pane.title.clone(), pane.open))
            .collect::<Vec<_>>();
        if panes.is_empty() {
            return;
        }

        ui.vertical(|ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workspace_panes.title"),
                i18n::t("studio.shell.pane_manager.detail"),
            );
            for (id, title, open) in panes {
                let focused = self.app.focused_pane == Some(id);
                let action = pane_manager_row(ui, &title, open, focused);
                if action.focus_requested {
                    push_workspace_command(
                        &self.workspace_commands,
                        StudioWorkspaceCommand::Focus(id),
                    );
                }
                if action.close_requested {
                    push_workspace_command(
                        &self.workspace_commands,
                        StudioWorkspaceCommand::Close(id),
                    );
                }
            }
        });
    }
}

fn icon_rail_panes() -> Vec<StudioPane> {
    studio_nav_sections()
        .into_iter()
        .flat_map(|section| section.items.into_iter().map(|item| item.pane))
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
struct PaneRowAction {
    focus_requested: bool,
    close_requested: bool,
}

fn nav_row(ui: &mut egui::Ui, pane: StudioPane, title: &str, selected: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), NAV_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), title)
    });
    if ui.is_rect_visible(rect) {
        paint_nav_bg(ui, rect, response.hovered(), selected);
        let icon_rect = nav_row_icon_rect(rect);
        shell_icons::paint_pane_icon(ui, icon_rect, pane, selected);
        ui.painter().text(
            egui::pos2(icon_rect.right() + Space::XS as f32, rect.center().y),
            egui::Align2::LEFT_CENTER,
            title,
            Text::body(),
            ui::strong_text(ui.ctx()),
        );
    }
    response
}

fn nav_icon_button(ui: &mut egui::Ui, pane: StudioPane, selected: bool) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ICON_RAIL_WIDTH, ICON_RAIL_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), pane.label())
    });
    if ui.is_rect_visible(rect) {
        paint_nav_bg(ui, rect, response.hovered(), selected);
        let icon_rect =
            egui::Rect::from_center_size(rect.center(), egui::vec2(ICON_SIZE, ICON_SIZE));
        shell_icons::paint_pane_icon(ui, icon_rect, pane, selected);
    }
    response
}

fn pane_manager_row(ui: &mut egui::Ui, title: &str, open: bool, focused: bool) -> PaneRowAction {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANE_ROW_HEIGHT),
        egui::Sense::click(),
    );
    let close_rect = pane_close_rect(rect);
    let close_id = ui.id().with(("pane_close", title));
    let close_response = ui.interact(close_rect, close_id, egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), title)
    });
    close_response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            format!("{} {}", i18n::t("studio.shell.close_pane"), title),
        )
    });

    if ui.is_rect_visible(rect) {
        paint_nav_bg(ui, rect, response.hovered(), focused);
        paint_pane_state(ui, rect, open, focused);
        paint_pane_title(ui, rect, close_rect, title, open);
        paint_close_control(ui, close_rect, close_response.hovered());
    }

    PaneRowAction {
        focus_requested: response.clicked() && !close_response.clicked(),
        close_requested: close_response.clicked(),
    }
}

fn paint_pane_state(ui: &egui::Ui, rect: egui::Rect, open: bool, focused: bool) {
    let color = if focused {
        Color::accent_base(ui.ctx())
    } else if open {
        Color::fg_secondary(ui.ctx())
    } else {
        Color::fg_tertiary(ui.ctx())
    };
    ui.painter()
        .circle_filled(pane_state_center(rect), PANE_STATE_RADIUS, color);
}

fn paint_pane_title(
    ui: &egui::Ui,
    rect: egui::Rect,
    close_rect: egui::Rect,
    title: &str,
    open: bool,
) {
    let color = if open {
        ui::strong_text(ui.ctx())
    } else {
        Color::fg_tertiary(ui.ctx())
    };
    let text_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + Space::LG as f32, rect.top()),
        egui::pos2(close_rect.left() - Space::XS as f32, rect.bottom()),
    );
    ui.painter().text(
        text_rect.left_center(),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        color,
    );
}

fn paint_close_control(ui: &egui::Ui, rect: egui::Rect, hovered: bool) {
    let fill = if hovered {
        Color::bg_surface_2(ui.ctx())
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    let stroke = egui::Stroke::new(1.5, Color::fg_secondary(ui.ctx()));
    let center = rect.center();
    let half = CLOSE_GLYPH_HALF;
    ui.painter().line_segment(
        [
            egui::pos2(center.x - half, center.y - half),
            egui::pos2(center.x + half, center.y + half),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x + half, center.y - half),
            egui::pos2(center.x - half, center.y + half),
        ],
        stroke,
    );
}

fn paint_nav_bg(ui: &egui::Ui, rect: egui::Rect, hovered: bool, selected: bool) {
    let fill = if selected {
        ui::selected_bg(ui.ctx())
    } else if hovered {
        Color::bg_surface_2(ui.ctx())
    } else {
        egui::Color32::TRANSPARENT
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::MD), fill);
}

fn nav_row_icon_rect(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_center_size(
        egui::pos2(
            rect.left() + Space::SM as f32 + ICON_SIZE / 2.0,
            rect.center().y,
        ),
        egui::vec2(ICON_SIZE, ICON_SIZE),
    )
}

fn pane_close_rect(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_center_size(
        egui::pos2(rect.right() - PANE_CLOSE_CENTER_INSET, rect.center().y),
        egui::vec2(PANE_CLOSE_SIZE, PANE_CLOSE_SIZE),
    )
}

fn pane_state_center(rect: egui::Rect) -> egui::Pos2 {
    egui::pos2(rect.left() + PANE_STATE_CENTER_X, rect.center().y)
}

fn push_workspace_command(commands: &WorkspaceCommandQueue, command: StudioWorkspaceCommand) {
    if let Ok(mut queue) = commands.lock() {
        queue.push(command);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_rows_match_documented_studio_metrics() {
        assert_eq!(NAV_ROW_HEIGHT, 28.0);
        assert_eq!(PANE_ROW_HEIGHT, 36.0);
        assert_eq!(ICON_RAIL_WIDTH, Space::XL as f32);
        assert_eq!(PANE_CLOSE_SIZE, Space::LG as f32);
    }

    #[test]
    fn nav_row_icon_rect_uses_centered_token_size() {
        let rect =
            egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(TEST_NAV_WIDTH, NAV_ROW_HEIGHT));
        let icon = nav_row_icon_rect(rect);

        assert_eq!(icon.width(), ICON_SIZE);
        assert_eq!(icon.height(), ICON_SIZE);
        assert_eq!(icon.center().y, rect.center().y);
        assert_eq!(icon.left(), Space::SM as f32);
    }

    #[test]
    fn pane_manager_controls_have_stable_token_geometry() {
        let rect = egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(TEST_NAV_WIDTH, PANE_ROW_HEIGHT),
        );
        let close = pane_close_rect(rect);
        let state = pane_state_center(rect);

        assert_eq!(close.width(), Space::LG as f32);
        assert_eq!(close.height(), Space::LG as f32);
        assert_eq!(close.center().y, rect.center().y);
        assert_eq!(state.y, rect.center().y);
        assert_eq!(state.x, PANE_STATE_CENTER_X);
        assert_eq!(CLOSE_GLYPH_HALF, Space::XS as f32 / 2.0);
    }

    #[test]
    fn pane_manager_routes_focus_and_close_through_workspace_command_queue() {
        let source = include_str!("shell_navigation.rs");
        let production_source = source.split("mod tests").next().unwrap();

        assert!(production_source.contains("StudioWorkspaceCommand::Focus(id)"));
        assert!(production_source.contains("StudioWorkspaceCommand::Close(id)"));
        assert!(production_source.contains("push_workspace_command("));
        assert!(production_source.contains("&self.workspace_commands"));
        assert!(!production_source.contains("self.app.focus_workspace_pane(id)"));
        assert!(!production_source.contains("self.app.close_workspace_pane(id)"));
    }

    #[test]
    fn sidebar_uses_docs_22_navigation_sections_without_legacy_open_group() {
        let source = include_str!("shell_navigation.rs");
        let production_source = source.split("mod tests").next().unwrap();

        assert!(production_source.contains("self.render_workspace_nav(ui);"));
        assert!(!production_source.contains("self.render_open_nav(ui);"));
        assert!(production_source.contains("studio_nav_sections()"));
        assert!(production_source.contains("self.open_workspace_item(item.pane);"));
    }
}
