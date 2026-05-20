use crate::{shell_icons, ui, StudioEguiApp};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};
use std_studio::StudioPane;

const NAV_ROW_HEIGHT: f32 = 28.0;
const OPEN_ROW_HEIGHT: f32 = 44.0;
const PANE_ROW_HEIGHT: f32 = 36.0;

impl StudioEguiApp {
    pub(crate) fn render_navigation(&mut self, ui: &mut egui::Ui) {
        ui.add_space(Space::XS as f32);
        if !self.layout.sidebar_open {
            self.render_icon_rail(ui);
            return;
        }
        self.render_workspace_nav(ui);
        ui.add_space(Space::LG as f32);
        self.render_open_nav(ui);
        ui.add_space(Space::LG as f32);
        self.render_workspace_pane_manager(ui);
    }

    fn render_icon_rail(&mut self, ui: &mut egui::Ui) {
        for pane in StudioPane::all() {
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
        ui.vertical(|ui| {
            ui::section_header(
                ui,
                i18n::t("studio.shell.workspace.title"),
                i18n::t("studio.shell.workspace.detail"),
            );
            for pane in StudioPane::all() {
                let selected = self.app.active_pane == pane;
                if nav_row(ui, pane, pane.label(), selected).clicked() {
                    self.app.switch_pane(pane);
                }
            }
        });
    }

    fn render_open_nav(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui::section_header(
                ui,
                i18n::t("studio.shell.open.title"),
                i18n::t("studio.shell.open.detail"),
            );
            self.open_row(
                ui,
                i18n::t("studio.shell.open.workflow.title"),
                i18n::t("studio.shell.open.workflow.detail"),
                StudioPane::Workflows,
            );
            self.open_row(
                ui,
                i18n::t("studio.shell.open.analysis.title"),
                i18n::t("studio.shell.open.analysis.detail"),
                StudioPane::Analysis,
            );
            self.open_row(
                ui,
                i18n::t("studio.plugins.title"),
                i18n::t("studio.shell.open.plugins.detail"),
                StudioPane::Plugins,
            );
            self.open_row(
                ui,
                i18n::t("studio.memory.title"),
                i18n::t("studio.shell.open.memory.detail"),
                StudioPane::Memory,
            );
            self.open_row(
                ui,
                i18n::t("studio.shell.open.history.title"),
                i18n::t("studio.shell.open.history.detail"),
                StudioPane::History,
            );
        });
    }

    fn open_row(&mut self, ui: &mut egui::Ui, title: &str, detail: &str, pane: StudioPane) {
        let response = open_nav_row(ui, pane, title, detail);
        if response.clicked() {
            let id = match pane {
                StudioPane::Workflows => self
                    .app
                    .open_workflow_builder(self.app.core.config.workflows_dir()),
                StudioPane::Analysis => self
                    .app
                    .open_analysis_workbench(std::path::PathBuf::from(&self.analysis_path)),
                StudioPane::Plugins => self.app.open_plugin_manager_pane(),
                StudioPane::Memory => self.app.open_memory_browser_pane(),
                StudioPane::History => self.app.open_execution_history_pane(),
                StudioPane::Settings => self.app.open_settings_pane(),
                _ => self.app.open_workspace_pane(pane),
            };
            self.status = format!(
                "{} {}",
                i18n::t("studio.status.workspace_pane_opened"),
                id.value()
            );
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
                i18n::t("studio.windows.title"),
                i18n::t("studio.shell.pane_manager.detail"),
            );
            for (id, title, open) in panes {
                let focused = self.app.focused_pane == Some(id);
                let action = pane_manager_row(ui, &title, open, focused);
                if action.focus_requested {
                    self.app.focus_workspace_pane(id);
                }
                if action.close_requested {
                    self.app.close_workspace_pane(id);
                }
            }
        });
    }
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
        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + Space::SM as f32, rect.center().y - 8.0),
            egui::vec2(16.0, 16.0),
        );
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
    let (rect, response) = ui.allocate_exact_size(egui::vec2(32.0, 28.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), pane.label())
    });
    if ui.is_rect_visible(rect) {
        paint_nav_bg(ui, rect, response.hovered(), selected);
        let icon_rect = egui::Rect::from_center_size(rect.center(), egui::vec2(16.0, 16.0));
        shell_icons::paint_pane_icon(ui, icon_rect, pane, selected);
    }
    response
}

fn open_nav_row(ui: &mut egui::Ui, pane: StudioPane, title: &str, detail: &str) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), OPEN_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), title)
    });
    if ui.is_rect_visible(rect) {
        paint_nav_bg(ui, rect, response.hovered(), false);
        let icon_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + Space::SM as f32,
                rect.top() + Space::SM as f32,
            ),
            egui::vec2(16.0, 16.0),
        );
        shell_icons::paint_pane_icon(ui, icon_rect, pane, false);
        let text_x = icon_rect.right() + Space::XS as f32;
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 13.0),
            egui::Align2::LEFT_CENTER,
            title,
            Text::body(),
            ui::strong_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 29.0),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
    }
    response
}

fn pane_manager_row(ui: &mut egui::Ui, title: &str, open: bool, focused: bool) -> PaneRowAction {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PANE_ROW_HEIGHT),
        egui::Sense::click(),
    );
    let close_rect = egui::Rect::from_center_size(
        egui::pos2(rect.right() - 14.0, rect.center().y),
        egui::vec2(24.0, 24.0),
    );
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
        .circle_filled(egui::pos2(rect.left() + 10.0, rect.center().y), 3.0, color);
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
    let half = 4.0;
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
