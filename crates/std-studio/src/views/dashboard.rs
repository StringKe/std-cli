use crate::{ui, StudioEguiApp};
use eframe::egui;

impl StudioEguiApp {
    pub(crate) fn render_dashboard(&mut self, ui: &mut egui::Ui) {
        ui.label(
            egui::RichText::new("Dashboard")
                .size(24.0)
                .strong()
                .color(ui::strong_text(ui.ctx())),
        );
        ui.label(
            egui::RichText::new("Operational overview for the local automation layer")
                .color(ui::muted_text(ui.ctx())),
        );
        ui.add_space(8.0);
        ui.columns(3, |columns| {
            ui::metric(
                &mut columns[0],
                "Actions",
                self.app.dashboard.action_count,
                "searchable units",
            );
            ui::metric(
                &mut columns[1],
                "Memory",
                self.app.dashboard.memory_count,
                "local notes",
            );
            ui::metric(
                &mut columns[2],
                "Audit Events",
                self.app.dashboard.audit_event_count,
                "event trail",
            );
        });

        ui.add_space(12.0);
        ui.columns(2, |columns| {
            let left_ctx = columns[0].ctx().clone();
            ui::surface_frame(&left_ctx).show(&mut columns[0], |ui| {
                ui::section_header(ui, "Planner Draft", "AI local plan");
                ui.label(
                    egui::RichText::new(format!(
                        "Goal: {}",
                        self.app.dashboard.suggested_plan.goal
                    ))
                    .color(ui::strong_text(ui.ctx())),
                );
                for (index, step) in self.app.dashboard.suggested_plan.steps.iter().enumerate() {
                    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui::chip(ui, &format!("{}", index + 1), ui::selected_bg(ui.ctx()));
                            ui.label(
                                egui::RichText::new(&step.action_name)
                                    .strong()
                                    .color(ui::strong_text(ui.ctx())),
                            );
                        });
                        ui.label(egui::RichText::new(&step.reason).color(ui::muted_text(ui.ctx())));
                    });
                    ui.add_space(6.0);
                }
            });
            let right_ctx = columns[1].ctx().clone();
            ui::surface_frame(&right_ctx).show(&mut columns[1], |ui| {
                ui::section_header(ui, "Recent Memory", "shared core");
                if self.app.dashboard.recent_memories.is_empty() {
                    ui::empty_state(ui, "No memory records yet");
                } else {
                    for memory in &self.app.dashboard.recent_memories {
                        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(&memory.title)
                                    .strong()
                                    .color(ui::strong_text(ui.ctx())),
                            );
                            ui.label(
                                egui::RichText::new(format!(
                                    "scope={} tags={}",
                                    memory.scope,
                                    memory.tags.join(",")
                                ))
                                .color(ui::muted_text(ui.ctx())),
                            );
                            ui.label(
                                egui::RichText::new(&memory.body).color(ui::strong_text(ui.ctx())),
                            );
                        });
                        ui.add_space(6.0);
                    }
                }
            });
        });

        ui.add_space(12.0);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Next Gates", "completion audit");
            ui.horizontal_wrapped(|ui| {
                ui::chip(ui, "Launcher PASS evidence exists", ui::ok_bg(ui.ctx()));
                ui::chip(ui, "Studio real UI audit ongoing", ui::warn_bg(ui.ctx()));
                ui::chip(ui, "Quality rust ecosystem only", ui::ok_bg(ui.ctx()));
            });
        });
    }
}
