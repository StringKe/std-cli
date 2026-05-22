use crate::preview::apply_preview_scenario;
use std_egui::{i18n, input, LauncherPhase, LauncherResultMode};
use std_launcher::{suggested_workflow_rows, LauncherState};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherAffordanceSummary {
    pub suggested: usize,
    pub ask_ai: bool,
    pub feedback_actions: String,
    pub feedback_action_shortcuts: String,
    pub action_panel_actions: String,
}

impl LauncherAffordanceSummary {
    pub(crate) fn for_scenario(scenario: &str) -> Self {
        let mut state = LauncherState::new();
        apply_preview_scenario(&mut state, scenario);
        Self {
            suggested: suggested_count(&state),
            ask_ai: ask_ai_available(&state),
            feedback_actions: feedback_actions(&state),
            feedback_action_shortcuts: feedback_action_shortcuts(&state),
            action_panel_actions: action_panel_actions(&state),
        }
    }

    pub(crate) fn passes(&self, scenario: &str) -> bool {
        match scenario {
            "empty" => self.suggested >= 3,
            "no-results" => self.ask_ai,
            "defer" => {
                self.feedback_actions == "Copy,Retry"
                    && self.feedback_action_shortcuts
                        == format!("Copy:{enter},Retry:{enter}", enter = input::enter().label())
            }
            "error" => {
                self.feedback_actions == "Copy,Retry,OpenStudio"
                    && self.feedback_action_shortcuts
                        == format!(
                            "Copy:{enter},Retry:{enter},OpenStudio:{enter}",
                            enter = input::enter().label()
                        )
            }
            "action-panel" => {
                self.action_panel_actions
                    == [
                        i18n::t("launcher.action.review_first"),
                        i18n::t("launcher.action.defer"),
                        i18n::t("launcher.action.open_in_studio"),
                        i18n::t("launcher.action.copy_command"),
                    ]
                    .join(",")
            }
            _ => true,
        }
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "suggested={},ask_ai={},feedback_actions={},feedback_action_shortcuts={},action_panel_actions={}",
            self.suggested,
            self.ask_ai,
            self.feedback_actions,
            self.feedback_action_shortcuts,
            self.action_panel_actions
        )
    }
}

fn suggested_count(state: &LauncherState) -> usize {
    if state.view.phase == LauncherPhase::Empty
        && state.view.result_mode == LauncherResultMode::SuggestedWorkflows
    {
        suggested_workflow_rows().len()
    } else {
        0
    }
}

fn ask_ai_available(state: &LauncherState) -> bool {
    state.view.phase == LauncherPhase::NoMatches
        && state
            .no_match_fallback_query()
            .as_deref()
            .is_some_and(|query| query.starts_with("? "))
}

fn feedback_actions(state: &LauncherState) -> String {
    state
        .view
        .feedback
        .as_ref()
        .map(|feedback| {
            feedback
                .actions()
                .into_iter()
                .map(|action| format!("{action:?}"))
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_else(|| "none".to_string())
}

fn feedback_action_shortcuts(state: &LauncherState) -> String {
    state
        .view
        .feedback
        .as_ref()
        .map(|feedback| {
            feedback
                .actions()
                .into_iter()
                .map(|action| format!("{action:?}:{}", input::enter().label()))
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_else(|| "none".to_string())
}

fn action_panel_actions(state: &LauncherState) -> String {
    if !state.action_panel.open {
        return "none".to_string();
    }
    state
        .action_panel
        .visible_items()
        .into_iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_types::ActionExecutionStatus;

    #[test]
    fn launcher_preview_affordances_cover_docs_21_states() {
        let empty = LauncherAffordanceSummary::for_scenario("empty");
        let no_results = LauncherAffordanceSummary::for_scenario("no-results");
        let defer = LauncherAffordanceSummary::for_scenario("defer");
        let error = LauncherAffordanceSummary::for_scenario("error");
        let action_panel = LauncherAffordanceSummary::for_scenario("action-panel");

        assert_eq!(empty.suggested, 3);
        assert!(no_results.ask_ai);
        assert_eq!(defer.feedback_actions, "Copy,Retry");
        assert_eq!(
            defer.feedback_action_shortcuts,
            feedback_shortcuts(&["Copy", "Retry"])
        );
        assert_eq!(error.feedback_actions, "Copy,Retry,OpenStudio");
        assert_eq!(
            error.feedback_action_shortcuts,
            feedback_shortcuts(&["Copy", "Retry", "OpenStudio"])
        );
        assert_eq!(
            action_panel.action_panel_actions,
            [
                i18n::t("launcher.action.review_first"),
                i18n::t("launcher.action.defer"),
                i18n::t("launcher.action.open_in_studio"),
                i18n::t("launcher.action.copy_command"),
            ]
            .join(",")
        );
        for scenario in ["empty", "no-results", "defer", "error", "action-panel"] {
            assert!(LauncherAffordanceSummary::for_scenario(scenario).passes(scenario));
        }
    }

    #[test]
    fn deferred_affordance_tracks_external_runner_status() {
        let mut state = LauncherState::new();
        apply_preview_scenario(&mut state, "defer");

        assert_eq!(
            state.view.feedback.as_ref().unwrap().status,
            ActionExecutionStatus::NeedsExternalRunner
        );
        assert_eq!(
            LauncherAffordanceSummary::for_scenario("defer").feedback_actions,
            "Copy,Retry"
        );
        assert_eq!(
            LauncherAffordanceSummary::for_scenario("defer").feedback_action_shortcuts,
            feedback_shortcuts(&["Copy", "Retry"])
        );
    }

    fn feedback_shortcuts(actions: &[&str]) -> String {
        let enter = input::enter().label();
        actions
            .iter()
            .map(|action| format!("{action}:{enter}"))
            .collect::<Vec<_>>()
            .join(",")
    }
}
