use crate::{LauncherFocusSection, LauncherState};
use std_egui::{a11y::AccessibilityContext, input, LauncherFeedback, LauncherFeedbackAction};
use std_types::{ActionExecution, ActionExecutionStatus, ActionId};

pub(crate) struct FeedbackSemantics {
    pub(crate) defer_label: String,
    pub(crate) defer_actions: String,
    pub(crate) failed_label: String,
    pub(crate) running_label: String,
    pub(crate) completion_label: String,
    pub(crate) error_actions: String,
    pub(crate) open_studio_target: String,
    pub(crate) open_studio_command: String,
    pub(crate) keyboard_path: String,
    pub(crate) contract: String,
    pub(crate) a11y_contract: String,
}

pub(crate) fn feedback_semantics() -> FeedbackSemantics {
    let defer_feedback = LauncherFeedback::from_execution(&deferred_execution());
    let failed_feedback = LauncherFeedback::from_execution(&failed_execution());
    let studio = open_studio_feedback_target(&failed_feedback);
    let keyboard = keyboard_feedback_path(failed_feedback.clone());
    let a11y = AccessibilityContext::from_env();
    FeedbackSemantics {
        defer_label: feedback_label(&defer_feedback),
        defer_actions: action_path(&defer_feedback.actions()).replace('>', ","),
        failed_label: feedback_label(&failed_feedback),
        running_label: a11y.launcher_running_label(&failed_feedback.action_name),
        completion_label: a11y.launcher_completed_label(&format!(
            "{} {}",
            defer_feedback.title, defer_feedback.detail
        )),
        error_actions: action_path(&failed_feedback.actions())
            .replace('>', ",")
            .replace("OpenStudio", "Open Studio"),
        open_studio_target: studio.0,
        open_studio_command: studio.1,
        keyboard_path: keyboard,
        contract: format!(
            "defer={},error={},keyboard=copy>retry>open-studio",
            action_path(&defer_feedback.actions()),
            action_path(&failed_feedback.actions())
        ),
        a11y_contract: "panel=status>target>actions,actions=action>target>status>enter".to_string(),
    }
}

fn open_studio_feedback_target(feedback: &LauncherFeedback) -> (String, String) {
    let mut state = LauncherState::new();
    state.view.feedback = Some(feedback.clone());
    let studio_intent = state.open_studio_execution_history_from_feedback();
    (format!("{:?}", studio_intent.target), studio_intent.command)
}

fn keyboard_feedback_path(feedback: LauncherFeedback) -> String {
    let mut state = LauncherState::new();
    state.view.feedback = Some(feedback);
    state.focus_section = LauncherFocusSection::Feedback;
    state.handle_keyboard_input(crate::keyboard::LauncherKey::ArrowDown, false);
    let retry = selected_feedback_action(&state);
    state.handle_keyboard_input(crate::keyboard::LauncherKey::ArrowDown, false);
    let open_studio = selected_feedback_action(&state);
    let _ = state.handle_keyboard_input(crate::keyboard::LauncherKey::Enter, false);
    format!(
        "Feedback>{}:{retry}>{}:{open_studio}>{}:{}",
        input::arrow_down().label(),
        input::arrow_down().label(),
        input::enter().label(),
        state
            .studio_intent
            .as_ref()
            .map(|intent| intent.command.as_str())
            .unwrap_or("none")
    )
}

fn selected_feedback_action(state: &LauncherState) -> String {
    state
        .view
        .selected_feedback_action()
        .map(|action| format!("{action:?}"))
        .unwrap_or_else(|| "none".to_string())
}

fn action_path(actions: &[LauncherFeedbackAction]) -> String {
    actions
        .iter()
        .map(|action| match action {
            LauncherFeedbackAction::Copy => "Copy",
            LauncherFeedbackAction::Retry => "Retry",
            LauncherFeedbackAction::OpenStudio => "OpenStudio",
        })
        .collect::<Vec<_>>()
        .join(">")
}

fn deferred_execution() -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "StdFixtureTerminal".to_string(),
        status: ActionExecutionStatus::NeedsExternalRunner,
        message: "std-fixture-terminal".to_string(),
        output: Some(serde_json::json!({
            "deferred": true,
            "reason": "external runner action requires explicit user trigger",
        })),
        created_at: chrono::Utc::now(),
    }
}

fn failed_execution() -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "Plugin Crash".to_string(),
        status: ActionExecutionStatus::Failed,
        message: "plugin crashed while rendering launcher feedback".to_string(),
        output: None,
        created_at: chrono::Utc::now(),
    }
}

fn feedback_label(feedback: &LauncherFeedback) -> String {
    format!(
        "{}: {} {}",
        feedback.title, feedback.action_name, feedback.detail
    )
}

#[cfg(test)]
#[path = "semantics_feedback_tests.rs"]
mod semantics_feedback_tests;
