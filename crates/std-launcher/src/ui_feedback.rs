use crate::{ui_metrics, ui_parts::quiet_button};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
    LauncherFeedback,
};
use std_launcher::{LauncherPerformanceReport, LauncherState};
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeedbackKind {
    Completed,
    Failed,
    Deferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FeedbackAction {
    Copy,
    Retry,
    OpenStudio,
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
    ui.horizontal(|ui| {
        render_text(ui, &ctx, feedback);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            render_actions(ui, state, feedback);
            render_performance(ui, &state.performance_report());
        });
    });
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
    for action in feedback_actions(feedback).into_iter().rev() {
        match action {
            FeedbackAction::Copy => {
                if quiet_button(ui, i18n::t("launcher.feedback.copy")).clicked() {
                    ui.ctx().copy_text(feedback.summary());
                }
            }
            FeedbackAction::Retry => {
                if quiet_button(ui, i18n::t("launcher.feedback.retry")).clicked() {
                    state.trigger_selected();
                }
            }
            FeedbackAction::OpenStudio => {
                if quiet_button(ui, i18n::t("launcher.feedback.open_studio")).clicked() {
                    state.open_studio_execution_history_from_feedback();
                }
            }
        }
    }
}

fn render_performance(ui: &mut egui::Ui, report: &LauncherPerformanceReport) {
    let ctx = ui.ctx().clone();
    let text = format!(
        "{}ms search  {}ms preview  {}ms action",
        report.last_search_ms, report.last_preview_ms, report.last_trigger_ms
    );
    ui.label(
        egui::RichText::new(text)
            .font(Text::code())
            .color(Color::fg_secondary(&ctx)),
    );
}

fn feedback_kind(feedback: &LauncherFeedback) -> FeedbackKind {
    match feedback.status {
        ActionExecutionStatus::Completed => FeedbackKind::Completed,
        ActionExecutionStatus::Failed => FeedbackKind::Failed,
        ActionExecutionStatus::NeedsExternalRunner => FeedbackKind::Deferred,
    }
}

fn feedback_actions(feedback: &LauncherFeedback) -> Vec<FeedbackAction> {
    match feedback_kind(feedback) {
        FeedbackKind::Completed => vec![FeedbackAction::Copy],
        FeedbackKind::Deferred => vec![FeedbackAction::Copy, FeedbackAction::Retry],
        FeedbackKind::Failed => vec![
            FeedbackAction::Copy,
            FeedbackAction::Retry,
            FeedbackAction::OpenStudio,
        ],
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
            action_name: "Open Terminal".to_string(),
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
            feedback_actions(&feedback),
            vec![
                FeedbackAction::Copy,
                FeedbackAction::Retry,
                FeedbackAction::OpenStudio
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

        assert_eq!(intent.command, "std-studio --open history");
        assert_eq!(
            intent.target,
            std_launcher::StudioLaunchTarget::ExecutionHistory
        );
        assert_eq!(intent.source_action, "Open Terminal");
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
            feedback_actions(&feedback),
            vec![FeedbackAction::Copy, FeedbackAction::Retry]
        );
    }

    #[test]
    fn feedback_detail_is_limited_to_two_lines() {
        let feedback = feedback(ActionExecutionStatus::Failed, "one\ntwo\nthree");

        assert_eq!(clamped_feedback_detail(&feedback), "one two");
    }
}
