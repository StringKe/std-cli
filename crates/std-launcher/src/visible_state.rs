use std_egui::{LauncherLoadingState, LauncherPhase, LauncherResultMode};
use std_types::{ActionExecution, ActionExecutionStatus, ActionId};

use crate::LauncherState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherVisibleState {
    pub phase: &'static str,
    pub query: String,
    pub elements: Vec<&'static str>,
    pub keyboard_path: &'static str,
    pub feedback_status: &'static str,
    pub feedback_actions: Vec<&'static str>,
}

impl LauncherVisibleState {
    pub fn from_state(state: &LauncherState) -> Self {
        let mut elements = vec!["search-input", "focus-ring"];
        if !state.ime_preedit.as_deref().unwrap_or("").is_empty() {
            elements.push("ime-chip");
        }
        match state.view.result_mode {
            LauncherResultMode::NaturalLanguage => elements.push("nl-suggestion"),
            LauncherResultMode::NoMatches => elements.push("no-results"),
            LauncherResultMode::SuggestedWorkflows if state.view.results.is_empty() => {
                elements.push("suggested-workflows")
            }
            _ if !state.view.results.is_empty() => {
                elements.extend(["grouped-results", "selected-row", "action-bar"])
            }
            _ => elements.push("empty-state"),
        }
        if state.view.loading != LauncherLoadingState::Idle {
            elements.push("loading-indicator");
        }
        if state.action_panel.open {
            elements.push("action-panel");
        }
        if state.view.feedback.is_some() {
            elements.push("inline-feedback");
        }
        Self {
            phase: phase_name(state.view.phase),
            query: state.view.query.clone(),
            elements,
            keyboard_path: keyboard_path(state),
            feedback_status: feedback_status(state),
            feedback_actions: feedback_actions(state),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "phase={};query={};elements={};keyboard={};feedback={};feedback_actions={}",
            self.phase,
            self.query,
            self.elements.join("|"),
            self.keyboard_path,
            self.feedback_status,
            self.feedback_actions.join("|")
        )
    }

    pub fn pass_docs21_delivery(&self) -> bool {
        self.elements.contains(&"search-input")
            && self.elements.contains(&"focus-ring")
            && self.keyboard_path != "none"
    }
}

pub fn launcher_visible_state_summary(state: &LauncherState) -> String {
    LauncherVisibleState::from_state(state).summary()
}

pub fn launcher_feedback_visible_state_summary(status: ActionExecutionStatus) -> String {
    let mut state = LauncherState::new();
    state.view.feedback = Some(std_egui::LauncherFeedback::from_execution(
        &feedback_execution(status),
    ));
    state.view.phase = LauncherPhase::Feedback;
    LauncherVisibleState::from_state(&state).summary()
}

fn phase_name(phase: LauncherPhase) -> &'static str {
    match phase {
        LauncherPhase::Empty => "empty",
        LauncherPhase::Searching => "searching",
        LauncherPhase::WithResults => "with-results",
        LauncherPhase::NoMatches => "no-results",
        LauncherPhase::Executing => "executing",
        LauncherPhase::Feedback => "feedback",
    }
}

fn keyboard_path(state: &LauncherState) -> &'static str {
    if state.action_panel.open {
        return "mod-k>arrows>enter";
    }
    if state.view.feedback.is_some() {
        return "tab-feedback>enter";
    }
    if state.view.result_mode == LauncherResultMode::NoMatches {
        return "enter-ask-ai";
    }
    if state.view.results.is_empty() {
        return "slash-question-down";
    }
    "arrows>enter>mod-k"
}

fn feedback_status(state: &LauncherState) -> &'static str {
    state
        .view
        .feedback
        .as_ref()
        .map(|feedback| match feedback.status {
            ActionExecutionStatus::Completed => "completed",
            ActionExecutionStatus::NeedsExternalRunner => "deferred",
            ActionExecutionStatus::Failed => "failed",
        })
        .unwrap_or("none")
}

fn feedback_actions(state: &LauncherState) -> Vec<&'static str> {
    state
        .view
        .feedback_actions()
        .into_iter()
        .map(|action| match action {
            std_egui::LauncherFeedbackAction::Copy => "copy",
            std_egui::LauncherFeedbackAction::Retry => "retry",
            std_egui::LauncherFeedbackAction::OpenStudio => "open-studio",
        })
        .collect()
}

fn feedback_execution(status: ActionExecutionStatus) -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: "Fixture Feedback".to_string(),
        status,
        message: "fixture feedback".to_string(),
        output: None,
        created_at: chrono::Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_state_reports_results_contract() {
        let mut state = LauncherState::new();
        state.update_query("index");
        let summary = launcher_visible_state_summary(&state);

        assert!(summary.contains("phase=with-results"));
        assert!(summary.contains("grouped-results|selected-row|action-bar"));
        assert!(summary.contains("keyboard=arrows>enter>mod-k"));
    }

    #[test]
    fn visible_state_reports_no_results_and_ime_contracts() {
        let mut state = LauncherState::new();
        state.update_query("zzzz-no-launcher-match");
        let no_results = LauncherVisibleState::from_state(&state);

        assert!(no_results.pass_docs21_delivery());
        assert!(no_results
            .summary()
            .contains("elements=search-input|focus-ring|no-results"));
        assert!(no_results.summary().contains("keyboard=enter-ask-ai"));

        state.handle_ime_preedit("zhong");
        let ime = launcher_visible_state_summary(&state);
        assert!(ime.contains("ime-chip"));
    }

    #[test]
    fn visible_state_reports_defer_and_error_feedback_actions() {
        let defer =
            launcher_feedback_visible_state_summary(ActionExecutionStatus::NeedsExternalRunner);
        let error = launcher_feedback_visible_state_summary(ActionExecutionStatus::Failed);

        assert!(defer.contains("phase=feedback"));
        assert!(defer.contains("inline-feedback"));
        assert!(defer.contains("feedback=deferred"));
        assert!(defer.contains("feedback_actions=copy|retry"));
        assert!(error.contains("feedback=failed"));
        assert!(error.contains("feedback_actions=copy|retry|open-studio"));
    }
}
