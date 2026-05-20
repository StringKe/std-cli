use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Space, Text},
};

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
        ui.columns(3, |columns| {
            ui::metric(
                &mut columns[0],
                i18n::t("studio.dashboard.actions"),
                self.app.dashboard.action_count,
                i18n::t("studio.dashboard.actions.detail"),
            );
            ui::metric(
                &mut columns[1],
                i18n::t("studio.dashboard.memory"),
                self.app.dashboard.memory_count,
                i18n::t("studio.dashboard.memory.detail"),
            );
            ui::metric(
                &mut columns[2],
                i18n::t("studio.dashboard.audit_events"),
                self.app.dashboard.audit_event_count,
                i18n::t("studio.dashboard.audit_events.detail"),
            );
        });
    }

    fn render_dashboard_workbench(&self, ui: &mut egui::Ui) {
        ui.columns(2, |columns| {
            self.render_planner_draft(&mut columns[0]);
            self.render_recent_memory(&mut columns[1]);
        });
    }

    fn render_planner_draft(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.dashboard.planner.title"),
                i18n::t("studio.dashboard.planner.detail"),
            );
            ui.label(
                egui::RichText::new(format!(
                    "{}: {}",
                    i18n::t("studio.dashboard.goal"),
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
                ui.add_space(Space::XS as f32);
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
                    ui.label(egui::RichText::new(&memory.body).color(ui::strong_text(ui.ctx())));
                });
                ui.add_space(Space::XS as f32);
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
