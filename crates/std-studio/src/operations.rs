use crate::{operations_rows, ui, workspace_policy_evidence, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::{OpsEvidence, OpsGate, OpsStatus};

const QUALITY_TOOLS: [&str; 5] = ["rustfmt", "clippy", "dylint", "cargo-deny", "cargo-machete"];
const OPERATIONS_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_operations(&mut self, ui: &mut egui::Ui) {
        let evidence = OpsEvidence::load();
        ui::section_header(
            ui,
            i18n::t("studio.operations.title"),
            i18n::t("studio.operations.detail"),
        );
        self.render_operations_workspace(ui, &evidence);
    }

    fn render_operations_workspace(&mut self, ui: &mut egui::Ui, evidence: &OpsEvidence) {
        let available_width = ui.available_width();
        if available_width < 920.0 {
            self.render_operations_column(ui, &[&evidence.qa, &evidence.doctor]);
            ui.add_space(OPERATIONS_PANEL_GAP);
            self.render_operations_column(ui, &[&evidence.release, &evidence.install]);
            ui.add_space(OPERATIONS_PANEL_GAP);
            self.render_evidence_gate(ui, &evidence.runtime);
            ui.add_space(OPERATIONS_PANEL_GAP);
            self.render_completion_gate(ui);
            return;
        }
        let column_width = (available_width - OPERATIONS_PANEL_GAP * 2.0) / 3.0;
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_operations_column(ui, &[&evidence.qa, &evidence.doctor]),
            );
            ui.add_space(OPERATIONS_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_operations_column(ui, &[&evidence.release, &evidence.install]),
            );
            ui.add_space(OPERATIONS_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    self.render_evidence_gate(ui, &evidence.runtime);
                    ui.add_space(OPERATIONS_PANEL_GAP);
                    workspace_policy_evidence::render(ui, self.app.workspace_policy);
                    ui.add_space(OPERATIONS_PANEL_GAP);
                    self.render_completion_gate(ui);
                },
            );
        });
    }

    fn render_operations_column(&mut self, ui: &mut egui::Ui, gates: &[&OpsGate]) {
        for (index, gate) in gates.iter().enumerate() {
            self.render_evidence_gate(ui, gate);
            if index + 1 < gates.len() {
                ui.add_space(OPERATIONS_PANEL_GAP);
            }
        }
    }

    fn render_evidence_gate(&mut self, ui: &mut egui::Ui, gate: &OpsGate) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, gate.title, gate.status.label());
            render_gate_status(ui, gate);
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.command"),
                &gate.command,
                &gate.detail,
            );
            for step in &gate.steps {
                operations_rows::gate_row(
                    ui,
                    i18n::t("studio.operations.step"),
                    &step.command,
                    &step.result,
                );
            }
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.runbook"),
                &gate.runbook,
                i18n::t("studio.operations.current_workspace"),
            );
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.evidence"),
                &gate.evidence,
                i18n::t("studio.operations.current_workspace"),
            );
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.result"),
                &gate.result,
                gate.status.label(),
            );
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.artifact"),
                &gate.artifact,
                i18n::t("studio.operations.current_workspace"),
            );
            operations_rows::gate_row(
                ui,
                i18n::t("studio.operations.output"),
                &gate.output,
                gate.status.label(),
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
            operations_rows::completion_chip_bar(
                ui,
                &[
                    "Core", "Launcher", "Studio", "Terminal", "Plugin", "Index", "Workflow",
                    "Release", "Install", "Quality",
                ],
            );
            ui.add_space(Space::XS as f32);
            ui.label(i18n::t("studio.operations.completion.note"));
        });
    }
}

fn render_gate_status(ui: &mut egui::Ui, gate: &OpsGate) {
    let icon = gate_status_icon(gate.status);
    let label = format!("{icon} {}", gate.status.label());
    let response = ui::chip(ui, &label, gate_status_fill(ui.ctx(), gate.status));
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            gate_status_a11y_label(gate),
        )
    });
    if gate.title == "QA" {
        ui.horizontal_wrapped(|ui| {
            for tool in QUALITY_TOOLS {
                ui::chip(ui, tool, ui::panel_alt(ui.ctx()));
            }
        });
    }
}

fn gate_status_icon(status: OpsStatus) -> &'static str {
    match status {
        OpsStatus::Pass => "PASS",
        OpsStatus::Missing => "MISSING",
        OpsStatus::Manual => "MANUAL",
    }
}

fn gate_status_a11y_label(gate: &OpsGate) -> String {
    format!(
        "{} gate status, {}, {}",
        gate.title,
        gate.status.label(),
        gate.result
    )
}

fn gate_status_fill(ctx: &egui::Context, status: OpsStatus) -> egui::Color32 {
    match status {
        OpsStatus::Pass => ui::ok_bg(ctx),
        OpsStatus::Missing => ui::warn_bg(ctx),
        OpsStatus::Manual => ui::panel_alt(ctx),
    }
}
