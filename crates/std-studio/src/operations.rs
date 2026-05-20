use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::{OpsEvidence, OpsGate, OpsStatus};

const QUALITY_TOOLS: [&str; 5] = ["rustfmt", "clippy", "dylint", "cargo-deny", "cargo-machete"];

impl StudioEguiApp {
    pub(crate) fn render_operations(&mut self, ui: &mut egui::Ui) {
        let evidence = OpsEvidence::load();
        ui::section_header(
            ui,
            i18n::t("studio.operations.title"),
            i18n::t("studio.operations.detail"),
        );
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| {
                self.render_evidence_gate(ui, &evidence.qa);
                ui.add_space(Space::SM as f32);
                self.render_evidence_gate(ui, &evidence.doctor);
            });
            columns[1].vertical(|ui| {
                self.render_evidence_gate(ui, &evidence.release);
                ui.add_space(Space::SM as f32);
                self.render_evidence_gate(ui, &evidence.install);
            });
            columns[2].vertical(|ui| {
                self.render_evidence_gate(ui, &evidence.runtime);
                ui.add_space(Space::SM as f32);
                self.render_completion_gate(ui);
            });
        });
    }

    fn render_evidence_gate(&mut self, ui: &mut egui::Ui, gate: &OpsGate) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, gate.title, gate.status.label());
            if gate.title == "QA" {
                ui.horizontal_wrapped(|ui| {
                    for tool in QUALITY_TOOLS {
                        ui::chip(ui, tool, ui::panel_alt(ui.ctx()));
                    }
                });
            } else {
                ui::chip(
                    ui,
                    gate.status.label(),
                    gate_status_fill(ui.ctx(), gate.status),
                );
            }
            gate_row(
                ui,
                i18n::t("studio.operations.command"),
                &gate.command,
                &gate.detail,
            );
            gate_row(
                ui,
                i18n::t("studio.operations.evidence"),
                &gate.evidence,
                i18n::t("studio.operations.current_workspace"),
            );
            if ui::quiet_button(ui, i18n::t("studio.operations.record_evidence")).clicked() {
                self.status = format!(
                    "{} evidence {}",
                    gate.title.to_ascii_lowercase(),
                    gate.status.label()
                );
            }
        });
    }

    fn render_completion_gate(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.operations.completion.title"),
                i18n::t("studio.operations.completion.detail"),
            );
            for item in [
                "Core", "Launcher", "Studio", "Terminal", "Plugin", "Index", "Workflow", "Release",
                "Install", "Quality",
            ] {
                ui::chip(ui, item, ui::warn_bg(ui.ctx()));
            }
            ui.add_space(Space::XS as f32);
            ui.label(i18n::t("studio.operations.completion.note"));
        });
    }
}

fn gate_row(ui: &mut egui::Ui, label: &str, value: &str, detail: &str) {
    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
        ui.label(egui::RichText::new(label).strong());
        ui.label(value);
        ui.small(detail);
    });
}

fn gate_status_fill(ctx: &egui::Context, status: OpsStatus) -> egui::Color32 {
    match status {
        OpsStatus::Pass => ui::ok_bg(ctx),
        OpsStatus::Missing => ui::warn_bg(ctx),
        OpsStatus::Manual => ui::panel_alt(ctx),
    }
}
