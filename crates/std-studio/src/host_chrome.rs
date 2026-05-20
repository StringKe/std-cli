use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

impl StudioEguiApp {
    pub(crate) fn render_app_chrome(&mut self, ui: &mut egui::Ui) {
        let frame = egui::Frame::new()
            .fill(std_egui::tokens::Color::bg_surface_1(ui.ctx()))
            .inner_margin(egui::Margin::symmetric(Space::SM, Space::XS));
        frame.show(ui, |ui| {
            let drag_rect = ui.max_rect();
            let drag_response = ui.interact(
                drag_rect,
                ui.id().with("host_drag"),
                egui::Sense::click_and_drag(),
            );
            if drag_response.drag_started() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }
            ui.horizontal(|ui| {
                self.render_top_identity(ui);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    host_window_controls(ui, &mut self.host_maximized);
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
            self.status = "refreshed workspace state".to_string();
        }
        if ui::quiet_button(ui, i18n::t("studio.chrome.open_current_pane")).clicked() {
            let id = self.app.open_workspace_pane(self.app.active_pane);
            self.status = format!("opened workspace pane {}", id.value());
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
}

fn host_window_controls(ui: &mut egui::Ui, host_maximized: &mut bool) {
    ui.horizontal(|ui| {
        if host_control(
            ui,
            i18n::t("studio.chrome.exit"),
            i18n::t("studio.chrome.exit.tooltip"),
        )
        .clicked()
        {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if host_control(
            ui,
            i18n::t("studio.chrome.hide"),
            i18n::t("studio.chrome.hide.tooltip"),
        )
        .clicked()
        {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        let maximize_label = if *host_maximized {
            i18n::t("studio.chrome.fit")
        } else {
            i18n::t("studio.chrome.fill")
        };
        if host_control(ui, maximize_label, i18n::t("studio.chrome.size.tooltip")).clicked() {
            *host_maximized = !*host_maximized;
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Maximized(*host_maximized));
        }
    });
}

fn host_control(ui: &mut egui::Ui, label: &str, tooltip: &str) -> egui::Response {
    ui.add(
        egui::Button::new(
            egui::RichText::new(label)
                .font(std_egui::tokens::Text::caption())
                .color(ui::muted_text(ui.ctx())),
        )
        .min_size(egui::vec2(40.0, 24.0))
        .fill(egui::Color32::TRANSPARENT)
        .stroke(egui::Stroke::new(
            1.0,
            std_egui::tokens::Color::stroke_divider(ui.ctx()),
        ))
        .corner_radius(egui::CornerRadius::same(4)),
    )
    .on_hover_text(tooltip)
}
