use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std_egui::{i18n, tokens::Text};
use std_orchestration::{StepResult, WorkflowExecutionTrace};

pub(crate) fn render(ui: &mut egui::Ui, trace: &WorkflowExecutionTrace) {
    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.history.timeline.title"),
            i18n::t("studio.history.timeline.detail"),
        );
        for (index, step) in trace.execution.results.iter().enumerate() {
            timeline_step(ui, index + 1, step);
        }
    });
}

pub(crate) fn history_timeline_contract(trace: &WorkflowExecutionTrace) -> String {
    let has_payload = trace
        .execution
        .results
        .iter()
        .any(|step| !step.output.is_null());
    let has_error = trace.steps.iter().any(|step| step.error.is_some());
    format!(
        "timeline=expanded;steps={};columns=step,status,started,finished,payload;payload={};error_expand={}",
        trace.execution.results.len(),
        has_payload,
        has_error
    )
}

fn timeline_step(ui: &mut egui::Ui, index: usize, step: &StepResult) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::HISTORY_TIMELINE_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            timeline_accessibility_label(index, step),
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Raised);
        paint_step(ui, rect, index, step);
    }
    ui.add_space(row_metrics::CHIP_GAP);
}

fn paint_step(ui: &mut egui::Ui, rect: egui::Rect, index: usize, step: &StepResult) {
    let title = format!("{index}. {}", step.step_name);
    let detail = format!(
        "status={:?} started={} finished={}",
        step.status,
        step.started_at.to_rfc3339(),
        step.finished_at.to_rfc3339()
    );
    row_paint::paint_inset_title_detail(
        ui,
        rect,
        &title,
        &detail,
        row_metrics::COMPACT_TITLE_Y,
        row_metrics::COMPACT_DETAIL_Y,
    );
    paint_payload(ui, rect, step);
}

fn paint_payload(ui: &mut egui::Ui, rect: egui::Rect, step: &StepResult) {
    let text = compact_payload(&step.output);
    let clip = egui::Rect::from_min_max(
        egui::pos2(
            rect.left() + row_metrics::HISTORY_TIMELINE_PAYLOAD_X,
            rect.top(),
        ),
        rect.right_bottom(),
    );
    ui.painter().with_clip_rect(clip).text(
        egui::pos2(
            rect.left() + row_metrics::HISTORY_TIMELINE_PAYLOAD_X,
            rect.top() + row_metrics::COMPACT_DETAIL_Y,
        ),
        egui::Align2::LEFT_CENTER,
        text,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn timeline_accessibility_label(index: usize, step: &StepResult) -> String {
    format!(
        "step {index} {} {:?} started {} finished {} payload {}",
        step.step_name,
        step.status,
        step.started_at.to_rfc3339(),
        step.finished_at.to_rfc3339(),
        compact_payload(&step.output)
    )
}

fn compact_payload(payload: &serde_json::Value) -> String {
    let text = payload.to_string();
    if text.chars().count() > row_metrics::HISTORY_TIMELINE_PAYLOAD_LIMIT {
        format!(
            "{}...",
            text.chars()
                .take(row_metrics::HISTORY_TIMELINE_PAYLOAD_LIMIT)
                .collect::<String>()
        )
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_payload_keeps_timeline_row_bounded() {
        let payload = serde_json::json!({"message": "abcdefghijklmnopqrstuvwxyz"});
        let compact = compact_payload(&payload);

        assert!(compact.len() <= payload.to_string().len());
        assert!(compact.contains("message"));
    }
}
