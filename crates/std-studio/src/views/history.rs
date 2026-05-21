use crate::{
    ui,
    views::{history_rows, history_timeline},
    StudioEguiApp,
};
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
        history_rows::filter_bar(ui, &mut self.history_filter);
        ui.add_space(Space::XS as f32);
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
                Ok(traces) if filtered_traces(&traces, &self.history_filter).is_empty() => {
                    ui::empty_state(ui, i18n::t("studio.history.traces.empty"))
                }
                Ok(traces) => {
                    let filtered = filtered_traces(&traces, &self.history_filter);
                    egui::ScrollArea::vertical()
                        .max_height(620.0)
                        .show(ui, |ui| {
                            for (index, trace) in filtered.into_iter().enumerate() {
                                history_rows::trace_row(ui, &trace);
                                if index == 0 {
                                    history_timeline::render(ui, &trace);
                                    ui.add_space(HISTORY_PANEL_GAP);
                                }
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

#[cfg(test)]
pub(crate) fn history_layout_contract() -> &'static str {
    "history=filter-bar>traces+timeline+events;filters=time,status-input,workflow-input;trace-columns=time,workflow,status,duration,source;timeline=step,status,started,finished,payload"
}

fn filtered_traces(
    traces: &[std_orchestration::WorkflowExecutionTrace],
    filter: &str,
) -> Vec<std_orchestration::WorkflowExecutionTrace> {
    let filter = filter.trim().to_lowercase();
    if filter.is_empty() {
        return traces.to_vec();
    }
    traces
        .iter()
        .filter(|trace| {
            trace
                .execution
                .workflow_name
                .to_lowercase()
                .contains(&filter)
                || format!("{:?}", trace.execution.status)
                    .to_lowercase()
                    .contains(&filter)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn history_filter_matches_workflow_name_or_status() {
        let temp = tempfile::tempdir().unwrap();
        let core = std_core::StdCore::with_config(std_core::StdConfig {
            data_dir: temp.path().join("data"),
            ..std_core::StdConfig::default()
        });
        let mut studio = std_studio::StudioApp::with_core(core);
        studio.plan_workflow("terminal").unwrap();
        let path = studio.save_planned_workflow().unwrap();
        studio.run_workflow_path(&path).unwrap();
        let traces = studio.recent_workflow_traces(10).unwrap();

        assert_eq!(filtered_traces(&traces, "terminal").len(), 1);
        assert_eq!(filtered_traces(&traces, "completed").len(), 1);
        assert!(filtered_traces(&traces, "missing").is_empty());
    }

    #[test]
    fn history_layout_contract_includes_expanded_timeline() {
        let contract = history_layout_contract();

        assert!(contract.contains("traces+timeline+events"));
        assert!(contract.contains("started,finished,payload"));
    }
}
