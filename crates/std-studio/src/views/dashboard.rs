use crate::{ui, views::dashboard_rows, StudioEguiApp};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Space, Text},
};

const DASHBOARD_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        self.render_dashboard_header(ui);
        ui.add_space(Space::XS as f32);
        self.render_dashboard_metrics(ui);
        ui.add_space(Space::SM as f32);
        self.render_dashboard_workbench(ui);
        ui.add_space(Space::SM as f32);
        self.render_dashboard_gates(ui);
    }

    fn render_dashboard_header(&self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new(i18n::t("studio.dashboard.title"))
                .font(Text::display())
                .strong()
                .color(ui::strong_text(ui.ctx())),
        );
        ui.label(
            egui::RichText::new(i18n::t("studio.dashboard.detail")).color(ui::muted_text(ui.ctx())),
        );
    }

    fn render_dashboard_metrics(&self, ui: &mut egui::Ui) {
        let metrics = [
            (
                i18n::t("studio.dashboard.actions"),
                self.app.dashboard.action_count,
                i18n::t("studio.dashboard.actions.detail"),
            ),
            (
                i18n::t("studio.dashboard.memory"),
                self.app.dashboard.memory_count,
                i18n::t("studio.dashboard.memory.detail"),
            ),
            (
                i18n::t("studio.dashboard.audit_events"),
                self.app.dashboard.audit_event_count,
                i18n::t("studio.dashboard.audit_events.detail"),
            ),
        ];
        let available_width = ui.available_width();
        if available_width < 760.0 {
            for (title, value, detail) in metrics {
                dashboard_rows::metric_tile(ui, title, value, detail);
                ui.add_space(DASHBOARD_PANEL_GAP);
            }
            return;
        }
        let tile_width = (available_width - DASHBOARD_PANEL_GAP * 2.0) / 3.0;
        ui.horizontal_top(|ui| {
            for (index, (title, value, detail)) in metrics.into_iter().enumerate() {
                ui.allocate_ui_with_layout(
                    egui::vec2(tile_width, 0.0),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| dashboard_rows::metric_tile(ui, title, value, detail),
                );
                if index < 2 {
                    ui.add_space(DASHBOARD_PANEL_GAP);
                }
            }
        });
    }

    fn render_dashboard_workbench(&self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 860.0 {
            self.render_planner_draft(ui);
            ui.add_space(DASHBOARD_PANEL_GAP);
            self.render_recent_memory(ui);
            return;
        }
        let left_width = ((available_width - DASHBOARD_PANEL_GAP) * 0.55).max(320.0);
        let right_width = (available_width - left_width - DASHBOARD_PANEL_GAP).max(280.0);
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(left_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_planner_draft(ui),
            );
            ui.add_space(DASHBOARD_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(right_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_recent_memory(ui),
            );
        });
    }

    fn render_planner_draft(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.dashboard.planner.title"),
                i18n::t("studio.dashboard.planner.detail"),
            );
            dashboard_rows::plan_goal_row(
                ui,
                i18n::t("studio.dashboard.goal"),
                &self.app.dashboard.suggested_plan.goal,
            );
            for (index, step) in self.app.dashboard.suggested_plan.steps.iter().enumerate() {
                dashboard_rows::plan_step_row(ui, index, step);
            }
        });
    }

    fn render_recent_memory(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.dashboard.recent_memory.title"),
                i18n::t("studio.dashboard.recent_memory.detail"),
            );
            if self.app.dashboard.recent_memories.is_empty() {
                ui::empty_state(ui, i18n::t("studio.dashboard.recent_memory.empty"));
                return;
            }
            for memory in &self.app.dashboard.recent_memories {
                dashboard_rows::memory_row(ui, memory);
            }
        });
    }

    fn render_dashboard_gates(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.dashboard.next_gates.title"),
                i18n::t("studio.dashboard.next_gates.detail"),
            );
            ui.horizontal_wrapped(|ui| {
                ui::chip(
                    ui,
                    i18n::t("studio.dashboard.gate.launcher"),
                    ui::ok_bg(ui.ctx()),
                );
                ui::chip(
                    ui,
                    i18n::t("studio.dashboard.gate.studio"),
                    ui::warn_bg(ui.ctx()),
                );
                ui::chip(
                    ui,
                    i18n::t("studio.dashboard.gate.quality"),
                    ui::ok_bg(ui.ctx()),
                );
            });
        });
    }
}
