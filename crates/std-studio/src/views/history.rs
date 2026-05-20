use crate::{ui, views::history_rows, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

const HISTORY_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_history(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.history.title"),
            i18n::t("studio.history.detail"),
        );
        self.render_history_workspace(ui);
    }

    fn render_history_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 760.0 {
            self.render_workflow_traces(ui);
            ui.add_space(HISTORY_PANEL_GAP);
            self.render_audit_events(ui);
            return;
        }
        let column_width = (available_width - HISTORY_PANEL_GAP) / 2.0;
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_workflow_traces(ui),
            );
            ui.add_space(HISTORY_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_audit_events(ui),
            );
        });
    }

    fn render_workflow_traces(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.history.traces.title"),
                i18n::t("studio.history.traces.detail"),
            );
            match self.app.recent_workflow_traces(20) {
                Ok(traces) if traces.is_empty() => {
                    ui::empty_state(ui, i18n::t("studio.history.traces.empty"))
                }
                Ok(traces) => {
                    egui::ScrollArea::vertical()
                        .max_height(620.0)
                        .show(ui, |ui| {
                            for trace in traces {
                                history_rows::trace_row(ui, &trace);
                            }
                        });
                }
                Err(error) => {
                    ui.label(error.to_string());
                }
            }
        });
    }

    fn render_audit_events(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.history.events.title"),
                i18n::t("studio.history.events.detail"),
            );
            if self.app.dashboard.recent_events.is_empty() {
                ui::empty_state(ui, i18n::t("studio.history.events.empty"));
                return;
            }
            egui::ScrollArea::vertical()
                .max_height(620.0)
                .show(ui, |ui| {
                    for event in self.app.dashboard.recent_events.iter().rev().take(40) {
                        history_rows::event_row(ui, event);
                    }
                });
        });
    }
}
