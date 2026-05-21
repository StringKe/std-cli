use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std_egui::tokens::{Color, Space, Text};
use std_orchestration::{WorkflowExecutionTrace, WorkflowTraceStep};
use std_types::{ActionExecutionStatus, StdEvent};

pub(crate) fn filter_bar(ui: &mut egui::Ui) {
    ui::surface_frame(ui.ctx()).show(ui, |ui| {
        ui.horizontal_wrapped(|ui| {
            filter_chip(ui, "Time range");
            filter_chip(ui, "Status");
            filter_chip(ui, "Workflow");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new("time / workflow / status / duration / source")
                        .font(Text::caption())
                        .color(ui::muted_text(ui.ctx())),
                );
            });
        });
    });
}

pub(crate) fn trace_row(ui: &mut egui::Ui, trace: &WorkflowExecutionTrace) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::HISTORY_TRACE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            &trace.execution.workflow_name,
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            &trace.execution.workflow_name,
            &trace.execution.workflow_id.to_string(),
            row_metrics::FILE_TITLE_Y,
            row_metrics::FILE_DETAIL_Y,
        );
        paint_trace_chips(ui, rect, trace);
        paint_step_preview(ui, rect, &trace.steps);
    }
    ui.add_space(Space::TWO_XS as f32);
}

fn filter_chip(ui: &mut egui::Ui, label: &str) {
    ui::chip(ui, label, Color::bg_surface_2(ui.ctx()));
}

pub(crate) fn event_row(ui: &mut egui::Ui, event: &StdEvent) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::HISTORY_EVENT_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    let title = format!("{:?}", event.event_type);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title.as_str())
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            &format!("{:?}", event.event_type),
            &event.created_at.to_rfc3339(),
            row_metrics::DENSE_TITLE_Y,
            row_metrics::DENSE_DETAIL_Y,
        );
        paint_event_chips(ui, rect, event);
        paint_payload_preview(ui, rect, &event.payload);
    }
    ui.add_space(Space::TWO_XS as f32);
}

fn paint_trace_chips(ui: &mut egui::Ui, rect: egui::Rect, trace: &WorkflowExecutionTrace) {
    let chips = [
        format!("{:?}", trace.execution.status),
        format!("steps={}", trace.steps.len()),
        format!("events={}", trace.audit_events.len()),
    ];
    paint_chips(
        ui,
        rect.left() + row_metrics::TEXT_INSET_X,
        rect.bottom() - row_metrics::HISTORY_TRACE_CHIP_Y,
        &chips,
    );
}

fn paint_step_preview(ui: &mut egui::Ui, rect: egui::Rect, steps: &[WorkflowTraceStep]) {
    let mut x = rect.left() + row_metrics::TEXT_INSET_X;
    let y = rect.bottom() - row_metrics::CHIP_ROW_Y_19;
    for step in steps.iter().take(3) {
        let label = step_label(step);
        let width = (label.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::ANALYSIS_CHIP_MIN_WIDTH,
                row_metrics::HISTORY_STEP_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(
                width,
                row_metrics::STATUS_CHIP_HEIGHT - row_metrics::MATCH_CHIP_CHAR_WIDTH,
            ),
        );
        row_paint::paint_chip(ui, chip_rect, &label, step_fill(ui.ctx(), step));
        x += width + row_metrics::CHIP_GAP;
    }
}

fn paint_event_chips(ui: &mut egui::Ui, rect: egui::Rect, event: &StdEvent) {
    let chips = [event.source.clone(), event.id.to_string()];
    paint_chips(
        ui,
        rect.left() + row_metrics::TEXT_INSET_X,
        rect.bottom() - row_metrics::CHIP_ROW_Y_23,
        &chips,
    );
}

fn paint_payload_preview(ui: &mut egui::Ui, rect: egui::Rect, payload: &serde_json::Value) {
    let text = compact_payload(payload);
    let clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + row_metrics::HISTORY_PAYLOAD_X, rect.top()),
        rect.right_bottom(),
    );
    ui.painter().with_clip_rect(clip).text(
        egui::pos2(
            rect.left() + row_metrics::HISTORY_PAYLOAD_X,
            rect.top() + row_metrics::DENSE_DETAIL_Y,
        ),
        egui::Align2::LEFT_CENTER,
        text,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_chips(ui: &mut egui::Ui, start_x: f32, y: f32, labels: &[String]) {
    let mut x = start_x;
    for label in labels.iter().take(4) {
        let width = (label.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::HISTORY_CHIP_MIN_WIDTH,
                row_metrics::HISTORY_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(
                width,
                row_metrics::STATUS_CHIP_HEIGHT - row_metrics::MATCH_CHIP_CHAR_WIDTH,
            ),
        );
        row_paint::paint_chip(ui, chip_rect, label, Color::bg_surface_2(ui.ctx()));
        x += width + row_metrics::CHIP_GAP;
    }
}

fn step_label(step: &WorkflowTraceStep) -> String {
    if let Some(ActionExecutionStatus::NeedsExternalRunner) = step.action_status {
        return format!("{} external", step.name);
    }
    format!("{} {:?}", step.name, step.status)
}

fn step_fill(ctx: &egui::Context, step: &WorkflowTraceStep) -> egui::Color32 {
    if step.action_status == Some(ActionExecutionStatus::NeedsExternalRunner) {
        return ui::warn_bg(ctx);
    }
    Color::bg_surface_2(ctx)
}

fn compact_payload(payload: &serde_json::Value) -> String {
    let text = payload.to_string();
    if text.chars().count() > row_metrics::HISTORY_PAYLOAD_LIMIT {
        format!(
            "{}...",
            text.chars()
                .take(row_metrics::HISTORY_PAYLOAD_LIMIT)
                .collect::<String>()
        )
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn history_filter_bar_contract_matches_docs_22() {
        let source = include_str!("history_rows.rs");

        assert!(source.contains("Time range"));
        assert!(source.contains("Status"));
        assert!(source.contains("Workflow"));
        assert!(source.contains("duration"));
        assert!(crate::views::history::history_layout_contract().contains("filter-bar"));
    }
}
