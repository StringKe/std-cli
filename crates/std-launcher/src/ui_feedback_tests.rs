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
fn feedback_detail_surface_wraps_two_lines_without_truncating() {
    let source = include_str!("ui_feedback.rs");
    let metrics = include_str!("ui_metrics.rs");

    assert!(source.contains("clamped_feedback_detail(feedback)"));
    assert!(source.contains(".wrap()"));
    assert!(!source.contains(".truncate()"));
    assert!(metrics.contains("feedback_text_height()"));
    assert!(metrics.contains("scale().f32(58.0)"));
    assert!(metrics.contains("scale().f32(36.0)"));
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
fn feedback_actions_expose_a11y_name_action_status_and_enter_hint() {
    let feedback = feedback(ActionExecutionStatus::Failed, "plugin crashed");

    assert_eq!(
        feedback_action_a11y_label(&feedback, LauncherFeedbackAction::Copy),
        format!(
            "{}, feedback action for StdFixtureTerminal, {}, press Enter",
            feedback_action_label(LauncherFeedbackAction::Copy),
            feedback.status_label()
        )
    );
    assert_eq!(
        feedback_action_a11y_label(&feedback, LauncherFeedbackAction::Retry),
        format!(
            "{}, feedback action for StdFixtureTerminal, {}, press Enter",
            feedback_action_label(LauncherFeedbackAction::Retry),
            feedback.status_label()
        )
    );
    assert_eq!(
        feedback_action_a11y_label(&feedback, LauncherFeedbackAction::OpenStudio),
        format!(
            "{}, feedback action for StdFixtureTerminal, {}, press Enter",
            feedback_action_label(LauncherFeedbackAction::OpenStudio),
            feedback.status_label()
        )
    );
}

#[test]
fn feedback_panel_exposes_a11y_state_and_available_actions() {
    let feedback = feedback(ActionExecutionStatus::Failed, "plugin crashed");

    let label = feedback_panel_a11y_label(&feedback);

    assert!(label.contains("Execution feedback"));
    assert!(label.contains(feedback.status_label()));
    assert!(label.contains("StdFixtureTerminal"));
    for action in feedback.actions() {
        assert!(label.contains(feedback_action_label(action)));
    }
}

#[test]
fn feedback_buttons_register_accessibility_widget_info() {
    let source = include_str!("ui_feedback.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap();

    assert!(production_source.contains("feedback_action_a11y_label"));
    assert!(production_source.contains("WidgetType::Button"));
    assert!(production_source.contains("press Enter"));
}

#[test]
fn feedback_panel_registers_accessibility_widget_info() {
    let source = include_str!("ui_feedback.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap();

    assert!(production_source.contains("feedback_panel_a11y_label"));
    assert!(production_source.contains("WidgetType::Label"));
    assert!(production_source.contains("Execution feedback"));
}

#[test]
fn retry_click_uses_launcher_user_execution_path() {
    let source = include_str!("ui_feedback.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap();

    assert!(production_source.contains("state.trigger_selected_by_user();"));
    assert!(!production_source.contains("state.trigger_selected();"));
}

#[test]
fn copy_click_uses_shared_feedback_copy_model() {
    let source = include_str!("ui_feedback.rs");
    let production_source = source.split("#[cfg(test)]").next().unwrap();

    assert!(production_source.contains("state.copy_feedback_to_clipboard_model()"));
    assert!(!production_source.contains("ui.ctx().copy_text(feedback.summary())"));
}
