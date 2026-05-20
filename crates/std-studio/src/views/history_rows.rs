use crate::ui;
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_orchestration::{WorkflowExecutionTrace, WorkflowTraceStep};
use std_types::{ActionExecutionStatus, StdEvent};

const TRACE_ROW_HEIGHT: f32 = 92.0;
const EVENT_ROW_HEIGHT: f32 = 66.0;

pub(crate) fn trace_row(ui: &mut egui::Ui, trace: &WorkflowExecutionTrace) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), TRACE_ROW_HEIGHT),
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
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &trace.execution.workflow_name,
            &trace.execution.workflow_id.to_string(),
            19.0,
            39.0,
        );
        paint_trace_chips(ui, rect, trace);
        paint_step_preview(ui, rect, &trace.steps);
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn event_row(ui: &mut egui::Ui, event: &StdEvent) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), EVENT_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    let title = format!("{:?}", event.event_type);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title.as_str())
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &format!("{:?}", event.event_type),
            &event.created_at.to_rfc3339(),
            18.0,
            38.0,
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
        rect.left() + Space::SM as f32,
        rect.bottom() - 43.0,
        &chips,
    );
}

fn paint_step_preview(ui: &mut egui::Ui, rect: egui::Rect, steps: &[WorkflowTraceStep]) {
    let mut x = rect.left() + Space::SM as f32;
    let y = rect.bottom() - 19.0;
    for step in steps.iter().take(3) {
        let label = step_label(step);
        let width = (label.len() as f32 * 7.0 + 18.0).clamp(52.0, 150.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 15.0));
        paint_chip(ui, chip_rect, &label, step_fill(ui.ctx(), step));
        x += width + Space::TWO_XS as f32;
    }
}

fn paint_event_chips(ui: &mut egui::Ui, rect: egui::Rect, event: &StdEvent) {
    let chips = [event.source.clone(), event.id.to_string()];
    paint_chips(
        ui,
        rect.left() + Space::SM as f32,
        rect.bottom() - 23.0,
        &chips,
    );
}

fn paint_payload_preview(ui: &mut egui::Ui, rect: egui::Rect, payload: &serde_json::Value) {
    let text = compact_payload(payload);
    let clip = egui::Rect::from_min_max(
        egui::pos2(rect.left() + 210.0, rect.top()),
        rect.right_bottom(),
    );
    ui.painter().with_clip_rect(clip).text(
        egui::pos2(rect.left() + 210.0, rect.top() + 38.0),
        egui::Align2::LEFT_CENTER,
        text,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_chips(ui: &mut egui::Ui, start_x: f32, y: f32, labels: &[String]) {
    let mut x = start_x;
    for label in labels.iter().take(4) {
        let width = (label.len() as f32 * 7.0 + 18.0).clamp(48.0, 138.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 15.0));
        paint_chip(ui, chip_rect, label, Color::bg_surface_2(ui.ctx()));
        x += width + Space::TWO_XS as f32;
    }
}

fn paint_chip(ui: &mut egui::Ui, rect: egui::Rect, label: &str, fill: egui::Color32) {
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
}

fn paint_row_frame(ui: &mut egui::Ui, rect: egui::Rect, hovered: bool) {
    let fill = if hovered {
        Color::bg_surface_3(ui.ctx())
    } else {
        Color::bg_surface_1(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(Radius::SM),
        egui::Stroke::new(1.0, Color::stroke_divider(ui.ctx())),
        egui::StrokeKind::Inside,
    );
}

fn paint_title_detail(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    y1: f32,
    y2: f32,
) {
    let clip = rect.shrink2(egui::vec2(Space::SM as f32, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    let x = rect.left() + Space::SM as f32;
    painter.text(
        egui::pos2(x, rect.top() + y1),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(x, rect.top() + y2),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
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
    if text.chars().count() > 72 {
        format!("{}...", text.chars().take(72).collect::<String>())
    } else {
        text
    }
}
