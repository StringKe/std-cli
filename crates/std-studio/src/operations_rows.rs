use crate::views::{
    row_metrics,
    row_paint::{self, RowSurface, ThreeTextRows},
};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_studio::{
    operations_completion::CompletionAuditRow, OpsEvidence, OpsGate, OpsStatus, OpsStep,
};

pub(crate) fn summary_rail(ui: &mut egui::Ui, evidence: &OpsEvidence) {
    egui::Frame::new()
        .fill(Color::bg_surface_1(ui.ctx()))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(ui.ctx())))
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::same(Space::XS))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for gate in operations_gates(evidence) {
                    summary_chip(ui, gate);
                }
            });
        });
}

fn summary_chip(ui: &mut egui::Ui, gate: &OpsGate) {
    let text = format!("{} {}", gate.title, gate.status.label());
    let fill = match gate.status {
        OpsStatus::Pass => Color::status_success(ui.ctx()),
        OpsStatus::Missing => Color::status_warning(ui.ctx()),
        OpsStatus::Manual => Color::bg_surface_2(ui.ctx()),
    };
    let response = egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(&text)
                    .font(Text::caption())
                    .color(Color::fg_primary(ui.ctx())),
            );
        })
        .response;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text.clone())
    });
}

pub(crate) fn gate_row(ui: &mut egui::Ui, label: &str, value: &str, detail: &str) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::OPS_GATE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    let a11y_label = gate_row_a11y_label(label, value, detail);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), a11y_label.clone())
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_caption_body_caption(
            ui,
            rect,
            label,
            value,
            detail,
            ThreeTextRows {
                top_y: row_metrics::OPS_LABEL_Y,
                body_y: row_metrics::OPS_VALUE_Y,
                bottom_y: row_metrics::OPS_DETAIL_Y,
            },
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn completion_audit_rows(ui: &mut egui::Ui, rows: &[CompletionAuditRow]) {
    for row in rows {
        gate_row(ui, row.area, row.status.label(), &row.evidence);
    }
}

pub(crate) fn step_row(ui: &mut egui::Ui, step: &OpsStep) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::OPS_STEP_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    let detail = format!("{} -> {}", step.name, step.result);
    let a11y_label = gate_row_a11y_label("Step", &step.command, &detail);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), a11y_label.clone())
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Raised);
        row_paint::paint_inset_caption_body_caption(
            ui,
            rect,
            step.name,
            &step.command,
            &step.result,
            ThreeTextRows {
                top_y: row_metrics::OPS_STEP_NAME_Y,
                body_y: row_metrics::OPS_STEP_COMMAND_Y,
                bottom_y: row_metrics::OPS_STEP_RESULT_Y,
            },
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn gate_row_a11y_label(label: &str, value: &str, detail: &str) -> String {
    format!("{label}: {value}; {detail}")
}

pub(crate) fn operations_gate_visual_contract() -> &'static str {
    "gate=title|status-icon|status-text|command|step-name|step-command|step-result|runbook|evidence|result|artifact|output|record-evidence"
}

pub(crate) fn operations_summary_rail_contract(evidence: &OpsEvidence) -> String {
    let gates = operations_gates(evidence);
    format!(
        "summary_rail=gates:{};statuses:{};next={};surface=token-inline-rail;a11y=gate-label-status",
        gates.iter().map(|gate| gate.title).collect::<Vec<_>>().join("|"),
        gates
            .iter()
            .map(|gate| gate.status.label())
            .collect::<Vec<_>>()
            .join("|"),
        next_manual_gate(&gates).unwrap_or("complete")
    )
}

pub(crate) fn completion_audit_visual_contract() -> &'static str {
    "completion=area|status|evidence|manual_gates"
}

pub(crate) fn operations_gate_a11y_contract() -> &'static str {
    "a11y=row-label-includes-label-value-detail,status-chip-includes-icon-text-result"
}

fn operations_gates(evidence: &OpsEvidence) -> [&OpsGate; 7] {
    [
        &evidence.qa,
        &evidence.doctor,
        &evidence.release,
        &evidence.install,
        &evidence.plugin,
        &evidence.index,
        &evidence.runtime,
    ]
}

fn next_manual_gate<'a>(gates: &'a [&OpsGate; 7]) -> Option<&'a str> {
    gates
        .iter()
        .find(|gate| gate.status != OpsStatus::Pass)
        .map(|gate| gate.title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_row_a11y_label_exposes_value_and_detail() {
        let label = gate_row_a11y_label("Command", "mise run quality", "docs/14");

        assert_eq!(label, "Command: mise run quality; docs/14");
    }

    #[test]
    fn operations_contract_requires_status_icon_text_and_result() {
        assert!(operations_gate_visual_contract().contains("status-icon"));
        assert!(operations_gate_visual_contract().contains("status-text"));
        assert!(operations_gate_visual_contract().contains("step-name"));
        assert!(operations_gate_visual_contract().contains("step-command"));
        assert!(operations_gate_visual_contract().contains("step-result"));
        assert!(operations_gate_a11y_contract().contains("status-chip-includes-icon-text-result"));
    }

    #[test]
    fn operations_summary_rail_contract_exposes_gate_statuses_and_next_manual_gate() {
        let evidence = OpsEvidence::load();
        let contract = operations_summary_rail_contract(&evidence);

        assert!(
            contract.contains("summary_rail=gates:QA|Doctor|Release|Install|Plugin|Index|Runtime")
        );
        assert!(contract.contains("surface=token-inline-rail"));
        assert!(contract.contains("a11y=gate-label-status"));
        assert!(contract.contains("next="));
    }
}
