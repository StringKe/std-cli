use crate::{ui_metrics, ui_parts::keycap};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Radius, Space, Text},
    LauncherFeedback, LauncherFeedbackAction,
};
use std_launcher::LauncherState;
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeedbackKind {
    Completed,
    Failed,
    Deferred,
}

pub(crate) fn render(ui: &mut egui::Ui, state: &mut LauncherState) {
    let Some(feedback) = state.view.feedback.clone() else {
        return;
    };
    let ctx = ui.ctx().clone();
    let response = egui::Frame::new()
        .fill(feedback_fill(&ctx, &feedback))
        .stroke(egui::Stroke::new(1.0, feedback_stroke(&ctx, &feedback)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::sm(), Space::xs()))
        .show(ui, |ui| render_contents(ui, state, &feedback))
        .response;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            feedback_panel_a11y_label(&feedback),
        )
    });
}

fn render_contents(ui: &mut egui::Ui, state: &mut LauncherState, feedback: &LauncherFeedback) {
    let ctx = ui.ctx().clone();
    render_text(ui, &ctx, feedback);
    ui.add_space(Space::xs() as f32);
    render_actions(ui, state, feedback);
}

fn render_text(ui: &mut egui::Ui, ctx: &egui::Context, feedback: &LauncherFeedback) {
    let width = ui.available_width().min(ui_metrics::scale().f32(360.0));
    ui.allocate_ui_with_layout(
        egui::vec2(width, ui_metrics::feedback_text_height()),
        egui::Layout::top_down(egui::Align::Min),
        |ui| {
            ui.horizontal(|ui| {
                render_status_icon(ui, ctx, feedback);
                ui.label(
                    egui::RichText::new(feedback_title(feedback))
                        .font(Text::body())
                        .color(Color::fg_primary(ctx))
                        .strong(),
                );
            });
            ui.add_sized(
                [width, ui_metrics::feedback_detail_height()],
                egui::Label::new(
                    egui::RichText::new(clamped_feedback_detail(feedback))
                        .font(Text::footnote())
                        .color(Color::fg_secondary(ctx)),
                )
                .truncate(),
            );
        },
    );
}

fn render_status_icon(ui: &mut egui::Ui, ctx: &egui::Context, feedback: &LauncherFeedback) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(Space::md() as f32, Space::md() as f32),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            feedback_icon_label(feedback),
        )
    });
    let stroke = egui::Stroke::new(1.5, feedback_stroke(ctx, feedback));
    let center = rect.center();
    match feedback_kind(feedback) {
        FeedbackKind::Completed => {
            ui.painter().line_segment(
                [
                    egui::pos2(center.x - 4.0, center.y),
                    egui::pos2(center.x, center.y + 4.0),
                ],
                stroke,
            );
            ui.painter().line_segment(
                [
                    egui::pos2(center.x, center.y + 4.0),
                    egui::pos2(center.x + 8.0, center.y - 4.0),
                ],
                stroke,
            );
        }
        FeedbackKind::Failed => {
            ui.painter()
                .circle_stroke(center, Space::xs() as f32, stroke);
            ui.painter().line_segment(
                [
                    egui::pos2(center.x - 4.0, center.y - 4.0),
                    egui::pos2(center.x + 4.0, center.y + 4.0),
                ],
                stroke,
            );
            ui.painter().line_segment(
                [
                    egui::pos2(center.x + 4.0, center.y - 4.0),
                    egui::pos2(center.x - 4.0, center.y + 4.0),
                ],
                stroke,
            );
        }
        FeedbackKind::Deferred => {
            ui.painter()
                .circle_stroke(center, Space::xs() as f32, stroke);
            ui.painter().line_segment(
                [
                    egui::pos2(center.x, center.y - 4.0),
                    egui::pos2(center.x, center.y),
                ],
                stroke,
            );
            ui.painter()
                .circle_filled(egui::pos2(center.x, center.y + 4.0), 1.5, stroke.color);
        }
    }
}

fn render_actions(ui: &mut egui::Ui, state: &mut LauncherState, feedback: &LauncherFeedback) {
    let actions = feedback.actions();
    ui.horizontal_wrapped(|ui| {
        for (index, action) in actions.into_iter().enumerate() {
            let selected = state.focus_section == std_launcher::LauncherFocusSection::Feedback
                && state.view.selected_feedback_action == index;
            match action {
                LauncherFeedbackAction::Copy => {
                    if feedback_button(
                        ui,
                        i18n::t("launcher.feedback.copy"),
                        feedback_action_a11y_label(feedback, LauncherFeedbackAction::Copy),
                        selected,
                    )
                    .clicked()
                    {
                        if let Some(execution) = state.copy_feedback_to_clipboard_model() {
                            ui.ctx().copy_text(execution.message);
                        }
                    }
                }
                LauncherFeedbackAction::Retry => {
                    if feedback_button(
                        ui,
                        i18n::t("launcher.feedback.retry"),
                        feedback_action_a11y_label(feedback, LauncherFeedbackAction::Retry),
                        selected,
                    )
                    .clicked()
                    {
                        state.trigger_selected_by_user();
                    }
                }
                LauncherFeedbackAction::OpenStudio => {
                    if feedback_button(
                        ui,
                        i18n::t("launcher.feedback.open_studio"),
                        feedback_action_a11y_label(feedback, LauncherFeedbackAction::OpenStudio),
                        selected,
                    )
                    .clicked()
                    {
                        state.open_studio_execution_history_from_feedback();
                    }
                }
            }
        }
    });
}

fn feedback_button(
    ui: &mut egui::Ui,
    label: &str,
    a11y_label: String,
    selected: bool,
) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_0(&ctx)
    };
    let stroke = if selected {
        Color::accent_base(&ctx)
    } else {
        Color::stroke_divider(&ctx)
    };
    let frame = egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()));

    let response = frame
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .font(Text::caption())
                        .color(Color::fg_primary(&ctx)),
                );
                if selected {
                    keycap(ui, &input::enter().label());
                }
            });
        })
        .response
        .interact(egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            a11y_label.clone(),
        )
    });
    if selected {
        return response.on_hover_text(input::enter().label());
    }
    response
}

fn feedback_kind(feedback: &LauncherFeedback) -> FeedbackKind {
    match feedback.status {
        ActionExecutionStatus::Completed => FeedbackKind::Completed,
        ActionExecutionStatus::Failed => FeedbackKind::Failed,
        ActionExecutionStatus::NeedsExternalRunner => FeedbackKind::Deferred,
    }
}

fn feedback_title(feedback: &LauncherFeedback) -> String {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => feedback.title.clone(),
        FeedbackKind::Failed => format!("{}: {}", feedback.title, feedback.action_name),
        FeedbackKind::Deferred => format!("{}: {}", feedback.title, feedback.action_name),
    }
}

fn feedback_icon_label(feedback: &LauncherFeedback) -> &'static str {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => i18n::t("launcher.feedback.icon.completed"),
        FeedbackKind::Failed => i18n::t("launcher.feedback.icon.failed"),
        FeedbackKind::Deferred => i18n::t("launcher.feedback.icon.deferred"),
    }
}

fn feedback_action_a11y_label(
    feedback: &LauncherFeedback,
    action: LauncherFeedbackAction,
) -> String {
    let label = match action {
        LauncherFeedbackAction::Copy => i18n::t("launcher.feedback.copy"),
        LauncherFeedbackAction::Retry => i18n::t("launcher.feedback.retry"),
        LauncherFeedbackAction::OpenStudio => i18n::t("launcher.feedback.open_studio"),
    };
    format!(
        "{label}, feedback action for {}, {}, press Enter",
        feedback.action_name,
        feedback.status_label()
    )
}

fn feedback_panel_a11y_label(feedback: &LauncherFeedback) -> String {
    let actions = feedback
        .actions()
        .into_iter()
        .map(feedback_action_label)
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "Execution feedback, {}, action {}, available actions {}",
        feedback.status_label(),
        feedback.action_name,
        actions
    )
}

fn feedback_action_label(action: LauncherFeedbackAction) -> &'static str {
    match action {
        LauncherFeedbackAction::Copy => i18n::t("launcher.feedback.copy"),
        LauncherFeedbackAction::Retry => i18n::t("launcher.feedback.retry"),
        LauncherFeedbackAction::OpenStudio => i18n::t("launcher.feedback.open_studio"),
    }
}

fn clamped_feedback_detail(feedback: &LauncherFeedback) -> String {
    feedback
        .detail
        .lines()
        .take(2)
        .collect::<Vec<_>>()
        .join(" ")
}

fn feedback_fill(ctx: &egui::Context, feedback: &LauncherFeedback) -> egui::Color32 {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => Color::status_weak(ctx, Color::status_success(ctx)),
        FeedbackKind::Failed => Color::status_weak(ctx, Color::status_danger(ctx)),
        FeedbackKind::Deferred => Color::status_weak(ctx, Color::status_warning(ctx)),
    }
}

fn feedback_stroke(ctx: &egui::Context, feedback: &LauncherFeedback) -> egui::Color32 {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => Color::status_success(ctx),
        FeedbackKind::Failed => Color::status_danger(ctx),
        FeedbackKind::Deferred => Color::status_warning(ctx),
    }
}

#[cfg(test)]
#[path = "ui_feedback_tests.rs"]
mod ui_feedback_tests;
