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
    egui::Frame::new()
        .fill(feedback_fill(&ctx, &feedback))
        .stroke(egui::Stroke::new(1.0, feedback_stroke(&ctx, &feedback)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::sm(), Space::xs()))
        .show(ui, |ui| render_contents(ui, state, &feedback));
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
                    if feedback_button(ui, i18n::t("launcher.feedback.copy"), selected).clicked() {
                        if let Some(execution) = state.copy_feedback_to_clipboard_model() {
                            ui.ctx().copy_text(execution.message);
                        }
                    }
                }
                LauncherFeedbackAction::Retry => {
                    if feedback_button(ui, i18n::t("launcher.feedback.retry"), selected).clicked() {
                        state.trigger_selected_by_user();
                    }
                }
                LauncherFeedbackAction::OpenStudio => {
                    if feedback_button(ui, i18n::t("launcher.feedback.open_studio"), selected)
                        .clicked()
                    {
                        state.open_studio_execution_history_from_feedback();
                    }
                }
            }
        }
    });
}

fn feedback_button(ui: &mut egui::Ui, label: &str, selected: bool) -> egui::Response {
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
mod tests {
    use super::*;
    use chrono::Utc;
    use std_types::{ActionExecution, ActionId};

    fn feedback(status: ActionExecutionStatus, message: &str) -> LauncherFeedback {
        LauncherFeedback::from_execution(&ActionExecution {
            action_id: ActionId::default(),
            action_name: "StdFixtureTerminal".to_string(),
            status,
            message: message.to_string(),
            output: None,
            created_at: Utc::now(),
        })
    }

    #[test]
    fn failed_feedback_exposes_copy_retry_and_studio_actions() {
        let feedback = feedback(ActionExecutionStatus::Failed, "plugin crashed");

        assert_eq!(feedback_kind(&feedback), FeedbackKind::Failed);
        assert_eq!(
            feedback.actions(),
            vec![
                LauncherFeedbackAction::Copy,
                LauncherFeedbackAction::Retry,
                LauncherFeedbackAction::OpenStudio
            ]
        );
    }

    #[test]
    fn open_studio_action_creates_history_intent_without_launching() {
        let core = std_core::StdCore::with_config(std_core::StdConfig::default());
        let mut state = LauncherState::with_core(core);
        let feedback = feedback(ActionExecutionStatus::Failed, "plugin crashed");
        state.view.feedback = Some(feedback);

        let intent = state.open_studio_execution_history_from_feedback();

        assert_eq!(intent.command, "studio-pane://history");
        assert_eq!(
            intent.target,
            std_launcher::StudioLaunchTarget::ExecutionHistory
        );
        assert_eq!(intent.source_action, "StdFixtureTerminal");
        assert_eq!(state.studio_intent, Some(intent));
    }

    #[test]
    fn deferred_feedback_exposes_copy_and_retry_only() {
        let feedback = feedback(
            ActionExecutionStatus::NeedsExternalRunner,
            "external runner",
        );

        assert_eq!(feedback_kind(&feedback), FeedbackKind::Deferred);
        assert_eq!(
            feedback.actions(),
            vec![LauncherFeedbackAction::Copy, LauncherFeedbackAction::Retry]
        );
    }

    #[test]
    fn feedback_detail_is_limited_to_two_lines() {
        let feedback = feedback(ActionExecutionStatus::Failed, "one\ntwo\nthree");

        assert_eq!(clamped_feedback_detail(&feedback), "one two");
    }

    #[test]
    fn feedback_surface_hides_performance_metrics_from_user_copy() {
        let source = include_str!("ui_feedback.rs");
        let old_metric_label = ["{}ms", " search"].join("");

        assert!(!source.contains(&old_metric_label));
    }

    #[test]
    fn feedback_surface_stacks_text_above_actions() {
        let source = include_str!("ui_feedback.rs");
        let render_contents = source
            .split("fn render_contents")
            .nth(1)
            .and_then(|body| body.split("fn render_text").next())
            .unwrap();

        assert!(render_contents.contains("render_text(ui, &ctx, feedback);"));
        assert!(render_contents.contains("render_actions(ui, state, feedback);"));
        assert!(source.contains("ui.horizontal_wrapped"));
        assert!(!render_contents.contains("right_to_left"));
    }

    #[test]
    fn selected_feedback_action_shows_enter_keycap() {
        let source = include_str!("ui_feedback.rs");

        assert!(source.contains("keycap(ui, &input::enter().label())"));
        assert!(source.contains("return response.on_hover_text(input::enter().label())"));
    }

    #[test]
    fn feedback_status_uses_icon_and_text_not_color_only() {
        let source = include_str!("ui_feedback.rs");

        assert!(source.contains("fn render_status_icon"));
        assert!(source.contains("feedback_icon_label"));
        assert!(source.contains("launcher.feedback.icon.completed"));
        assert!(source.contains("launcher.feedback.icon.deferred"));
        assert!(source.contains("launcher.feedback.icon.failed"));
        assert!(source.contains("render_status_icon(ui, ctx, feedback);"));
    }

    #[test]
    fn retry_click_uses_launcher_user_execution_path() {
        let source = include_str!("ui_feedback.rs");

        assert!(source.contains("state.trigger_selected_by_user();"));
        assert!(!source.contains("state.trigger_selected();"));
    }

    #[test]
    fn copy_click_uses_shared_feedback_copy_model() {
        let source = include_str!("ui_feedback.rs");

        assert!(source.contains("state.copy_feedback_to_clipboard_model()"));
        assert!(!source.contains("ui.ctx().copy_text(feedback.summary())"));
    }
}
