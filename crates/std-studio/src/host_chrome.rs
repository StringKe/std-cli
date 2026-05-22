use crate::{host_chrome_drag, ui, StudioEguiApp};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space},
};
use std_studio::WorkspacePaneId;

const HOST_CONTROL_WIDTH: f32 = Space::XL as f32;
const HOST_CONTROL_HEIGHT: f32 = Space::LG as f32;
const CLOSE_ICON_HALF: f32 = Space::TWO_XS as f32;
const MINIMIZE_ICON_HALF_WIDTH: f32 = (Space::XS as f32) * 0.5;
const MINIMIZE_ICON_Y_OFFSET: f32 = (Space::XS as f32) * 0.5;
const MAXIMIZE_ICON_WIDTH: f32 = Space::SM as f32;
const MAXIMIZE_ICON_HEIGHT: f32 = Space::XS as f32;
const HOST_CHROME_SURFACE_TOKEN: &str = "bg/surface-1";

impl StudioEguiApp {
    pub(crate) fn render_app_chrome(&mut self, ui: &mut egui::Ui) {
        let frame = egui::Frame::new()
            .fill(host_chrome_surface_fill(ui.ctx()))
            .inner_margin(egui::Margin::symmetric(Space::SM, Space::XS));
        frame.show(ui, |ui| {
            host_chrome_drag::install_host_chrome_drag_region(ui);
            ui.horizontal(|ui| {
                self.render_top_identity(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    self.render_host_window_controls(ui);
                    ui.add_space(Space::SM as f32);
                    self.render_top_actions(ui);
                });
            });
        });
    }

    fn render_top_identity(&self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.label(
                egui::RichText::new(&self.app.name)
                    .font(std_egui::tokens::Text::headline())
                    .strong()
                    .color(ui::strong_text(ui.ctx())),
            );
            ui.label(
                egui::RichText::new(self.app.active_pane.label()).color(ui::muted_text(ui.ctx())),
            );
        });
    }

    fn render_top_actions(&mut self, ui: &mut egui::Ui) {
        if ui::quiet_button(ui, i18n::t("studio.chrome.refresh")).clicked() {
            self.app.refresh();
            self.status = i18n::t("studio.status.workspace_refreshed").to_string();
        }
        if ui::quiet_button(ui, i18n::t("studio.chrome.open_current_pane")).clicked() {
            let id = self.open_current_pane_from_host_chrome();
            self.status = format!(
                "{} {}",
                i18n::t("studio.status.workspace_pane_opened"),
                id.value()
            );
        }
        ui.label(
            egui::RichText::new(format!(
                "{} {}",
                self.app.open_workspace_panes().count(),
                i18n::t("studio.chrome.workspace_panes")
            ))
            .color(ui::muted_text(ui.ctx())),
        );
    }

    fn open_current_pane_from_host_chrome(&mut self) -> WorkspacePaneId {
        debug_assert!(!self.app.workspace_policy.allows_native_child_windows());
        debug_assert!(!self.app.workspace_policy.allows_detached_panels());
        let id = self.app.open_workspace_pane(self.app.active_pane);
        self.pending_workspace_focus = Some(id);
        id
    }

    fn render_host_window_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if host_control(
                ui,
                HostControlKind::Close,
                i18n::t("studio.chrome.exit"),
                i18n::t("studio.chrome.exit.tooltip"),
            )
            .clicked()
            {
                self.pending_closeguard = Some(self.app.close_workspace_instance());
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
            if host_control(
                ui,
                HostControlKind::Minimize,
                i18n::t("studio.chrome.hide"),
                i18n::t("studio.chrome.hide.tooltip"),
            )
            .clicked()
            {
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            }
            let maximize_label = if self.host_maximized {
                i18n::t("studio.chrome.fit")
            } else {
                i18n::t("studio.chrome.fill")
            };
            if host_control(
                ui,
                HostControlKind::Maximize,
                maximize_label,
                i18n::t("studio.chrome.size.tooltip"),
            )
            .clicked()
            {
                self.host_maximized = !self.host_maximized;
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Maximized(self.host_maximized));
            }
        });
    }
}

pub(crate) fn host_chrome_surface_contract() -> &'static str {
    "host_chrome=egui-owned,borderless,native-controls=false,surface=bg/surface-1"
}

pub(crate) fn host_chrome_input_contract() -> &'static str {
    host_chrome_drag::host_chrome_drag_contract()
}

pub(crate) fn host_chrome_surface_token() -> &'static str {
    HOST_CHROME_SURFACE_TOKEN
}

fn host_chrome_surface_fill(ctx: &egui::Context) -> egui::Color32 {
    Color::bg_surface_1(ctx)
}

#[derive(Debug, Clone, Copy)]
enum HostControlKind {
    Close,
    Minimize,
    Maximize,
}

fn host_control(
    ui: &mut egui::Ui,
    kind: HostControlKind,
    label: &str,
    tooltip: &str,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(HOST_CONTROL_WIDTH, HOST_CONTROL_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if ui.is_rect_visible(rect) {
        paint_host_control(ui, rect, kind, response.hovered());
    }
    response.on_hover_text(tooltip)
}

fn paint_host_control(ui: &egui::Ui, rect: egui::Rect, kind: HostControlKind, hovered: bool) {
    let ctx = ui.ctx();
    let fill = if hovered {
        Color::bg_surface_2(ctx)
    } else {
        Color::bg_surface_1(ctx)
    };
    let stroke = egui::Stroke::new(1.0, Color::stroke_divider(ctx));
    ui.painter().rect(
        rect,
        egui::CornerRadius::same(Radius::SM),
        fill,
        stroke,
        egui::StrokeKind::Inside,
    );
    let icon_stroke = egui::Stroke::new(1.5, ui::muted_text(ctx));
    match kind {
        HostControlKind::Close => paint_close_icon(ui, rect, icon_stroke),
        HostControlKind::Minimize => paint_minimize_icon(ui, rect, icon_stroke),
        HostControlKind::Maximize => paint_maximize_icon(ui, rect, icon_stroke),
    }
}

fn paint_close_icon(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let center = rect.center();
    ui.painter().line_segment(
        [
            egui::pos2(center.x - CLOSE_ICON_HALF, center.y - CLOSE_ICON_HALF),
            egui::pos2(center.x + CLOSE_ICON_HALF, center.y + CLOSE_ICON_HALF),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x + CLOSE_ICON_HALF, center.y - CLOSE_ICON_HALF),
            egui::pos2(center.x - CLOSE_ICON_HALF, center.y + CLOSE_ICON_HALF),
        ],
        stroke,
    );
}

fn paint_minimize_icon(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let center = rect.center();
    ui.painter().line_segment(
        [
            egui::pos2(
                center.x - MINIMIZE_ICON_HALF_WIDTH,
                center.y + MINIMIZE_ICON_Y_OFFSET,
            ),
            egui::pos2(
                center.x + MINIMIZE_ICON_HALF_WIDTH,
                center.y + MINIMIZE_ICON_Y_OFFSET,
            ),
        ],
        stroke,
    );
}

fn paint_maximize_icon(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let size = egui::vec2(MAXIMIZE_ICON_WIDTH, MAXIMIZE_ICON_HEIGHT);
    let icon_rect = egui::Rect::from_center_size(rect.center(), size);
    ui.painter().rect_stroke(
        icon_rect,
        egui::CornerRadius::same(Radius::SM),
        stroke,
        egui::StrokeKind::Inside,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_studio::StudioPane;

    #[test]
    fn host_chrome_open_current_pane_uses_internal_workspace_pane() {
        let mut app = StudioEguiApp::default();
        app.app.switch_pane(StudioPane::Plugins);

        let id = app.open_current_pane_from_host_chrome();

        assert_eq!(app.app.focused_pane, Some(id));
        assert_eq!(app.pending_workspace_focus, Some(id));
        assert_eq!(app.app.open_workspace_panes().count(), 1);
        assert!(!app.app.workspace_policy.allows_native_child_windows());
        assert!(!app.app.workspace_policy.allows_detached_panels());
    }

    #[test]
    fn host_chrome_open_current_pane_deduplicates_same_workspace() {
        let mut app = StudioEguiApp::default();
        app.app.switch_pane(StudioPane::Plugins);

        let first = app.open_current_pane_from_host_chrome();
        let second = app.open_current_pane_from_host_chrome();

        assert_eq!(first, second);
        assert_eq!(app.app.open_workspace_panes().count(), 1);
    }

    #[test]
    fn host_chrome_drag_region_does_not_cover_controls() {
        let source = include_str!("host_chrome.rs");

        assert!(source.contains("install_host_chrome_drag_region"));
        assert!(!source.contains("ui.id().with(\"host_drag\")"));
        assert_eq!(
            host_chrome_input_contract(),
            "drag_region=background-only,left-identity-area;controls_reserved=true"
        );
    }
}
