use crate::{ui_metrics, ui_parts::quiet_button};
use eframe::egui;
use std_egui::{
    i18n,
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
            ui.label(
                egui::RichText::new(feedback_title(feedback))
                    .font(Text::body())
                    .color(Color::fg_primary(ctx))
                    .strong(),
            );
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

fn render_actions(ui: &mut egui::Ui, state: &mut LauncherState, feedback: &LauncherFeedback) {
    let actions = feedback.actions();
    ui.horizontal_wrapped(|ui| {
        for (index, action) in actions.into_iter().enumerate() {
            let selected = state.focus_section == std_launcher::LauncherFocusSection::Feedback
                && state.view.selected_feedback_action == index;
            match action {
                LauncherFeedbackAction::Copy => {
                    if feedback_button(ui, i18n::t("launcher.feedback.copy"), selected).clicked() {
                        ui.ctx().copy_text(feedback.summary());
                    }
                }
                LauncherFeedbackAction::Retry => {
                    if feedback_button(ui, i18n::t("launcher.feedback.retry"), selected).clicked() {
                        state.trigger_selected();
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
    if selected {
        let ctx = ui.ctx().clone();
        return ui.add(
            egui::Button::new(egui::RichText::new(label).color(Color::fg_primary(&ctx)))
                .fill(Color::accent_weak(&ctx))
                .stroke(egui::Stroke::new(1.0, Color::accent_base(&ctx)))
                .corner_radius(egui::CornerRadius::same(Radius::sm())),
        );
    }
    quiet_button(ui, label)
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
        FeedbackKind::Completed => {
            crate::ui_parts::weak_status_fill(ctx, Color::status_success(ctx))
        }
        FeedbackKind::Failed => crate::ui_parts::weak_status_fill(ctx, Color::status_danger(ctx)),
        FeedbackKind::Deferred => {
            crate::ui_parts::weak_status_fill(ctx, Color::status_warning(ctx))
        }
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
}
