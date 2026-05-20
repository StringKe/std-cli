use crate::{ui, StudioEguiApp};
use eframe::egui;

impl StudioEguiApp {
    pub(crate) fn render_app_chrome(&mut self, ui: &mut egui::Ui) {
        let frame = egui::Frame::new()
            .fill(std_egui::tokens::Color::bg_surface_1(ui.ctx()))
            .inner_margin(egui::Margin::symmetric(14, 8));
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
                    ui.add_space(12.0);
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
        if ui::quiet_button(ui, "Refresh").clicked() {
            self.app.refresh();
            self.status = "refreshed workspace state".to_string();
        }
        if ui::quiet_button(ui, "Open Current Pane").clicked() {
            let id = self.app.open_workspace_pane(self.app.active_pane);
            self.status = format!("opened workspace pane {}", id.value());
        }
        ui.label(
            egui::RichText::new(format!(
                "{} workspace panes",
                self.app.open_workspace_panes().count()
            ))
            .color(ui::muted_text(ui.ctx())),
        );
    }
}

fn host_window_controls(ui: &mut egui::Ui, host_maximized: &mut bool) {
    ui.horizontal(|ui| {
        if host_control(ui, "Exit", "Close Studio").clicked() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
        if host_control(ui, "Hide", "Minimize Studio").clicked() {
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
        }
        let maximize_label = if *host_maximized { "Fit" } else { "Fill" };
        if host_control(ui, maximize_label, "Toggle Studio size").clicked() {
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
