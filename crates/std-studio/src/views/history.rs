use crate::{ui, StudioEguiApp};
use eframe::egui;

impl StudioEguiApp {
    pub(crate) fn render_history(&mut self, ui: &mut egui::Ui) {
        ui::section_header(ui, "History", "workflow traces and audit events");
        ui.columns(2, |columns| {
            columns[0].vertical(|ui| self.render_workflow_traces(ui));
            columns[1].vertical(|ui| self.render_audit_events(ui));
        });
    }

    fn render_workflow_traces(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Workflow Traces", "persisted execution timeline");
            match self.app.recent_workflow_traces(20) {
                Ok(traces) if traces.is_empty() => ui::empty_state(ui, "No workflow traces"),
                Ok(traces) => {
                    egui::ScrollArea::vertical()
                        .max_height(620.0)
                        .show(ui, |ui| {
                            for trace in traces {
                                ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                                    ui.label(egui::RichText::new(trace.summary()).strong());
                                    ui.small(format!(
                                        "workflow_id={} steps={}",
                                        trace.execution.workflow_id,
                                        trace.steps.len()
                                    ));
                                    for step in &trace.steps {
                                        ui.horizontal_wrapped(|ui| {
                                            ui::chip(
                                                ui,
                                                &format!("{:?}", step.status),
                                                ui::panel_alt(ui.ctx()),
                                            );
                                            if let Some(status) = &step.action_status {
                                                ui::chip(
                                                    ui,
                                                    &format!("{status:?}"),
                                                    ui::selected_bg(ui.ctx()),
                                                );
                                            }
                                            ui.label(&step.name);
                                        });
                                        if let Some(message) =
                                            step.message.as_deref().or(step.error.as_deref())
                                        {
                                            ui.small(message);
                                        }
                                    }
                                });
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
            ui::section_header(ui, "Audit Events", "recent core event log");
            if self.app.dashboard.recent_events.is_empty() {
                ui::empty_state(ui, "No audit events");
                return;
            }
            egui::ScrollArea::vertical()
                .max_height(620.0)
                .show(ui, |ui| {
                    for event in self.app.dashboard.recent_events.iter().rev().take(40) {
                        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui::chip(
                                    ui,
                                    &format!("{:?}", event.event_type),
                                    ui::selected_bg(ui.ctx()),
                                );
                                ui::chip(ui, &event.source, ui::panel_alt(ui.ctx()));
                            });
                            ui.small(event.created_at.to_rfc3339());
                            ui.small(event.payload.to_string());
                        });
                    }
                });
        });
    }
}
