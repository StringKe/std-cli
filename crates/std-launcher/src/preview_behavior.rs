use std_egui::LauncherLoadingState;
use std_launcher::LauncherState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviewStateBehavior {
    search_indicator: &'static str,
    loading_progress: &'static str,
    empty_progress: &'static str,
    input: &'static str,
    action_bar: &'static str,
}

impl PreviewStateBehavior {
    pub(crate) fn for_state(state: &LauncherState, state_name: &str) -> Self {
        Self {
            search_indicator: search_indicator_for_state(state),
            loading_progress: loading_progress_for_state(state),
            empty_progress: empty_progress_for_state(state),
            input: input_contract_for_state(state),
            action_bar: action_bar_contract_for_state(state_name),
        }
    }

    pub(crate) fn passes(&self, state_name: &str) -> bool {
        match state_name {
            "searching" => {
                self.search_indicator == "search"
                    && self.loading_progress == "2px-accent-indeterminate"
                    && self.empty_progress == "not-rendered"
                    && self.input == "editable"
            }
            "loading" => {
                self.search_indicator == "spinner"
                    && self.loading_progress == "not-rendered"
                    && self.empty_progress == "visible"
                    && self.input == "editable"
            }
            "executing" => {
                self.search_indicator == "executing"
                    && self.input == "locked"
                    && self.action_bar == "cancel-and-background-hints"
            }
            "defer" | "error" => self.action_bar == "feedback-actions",
            "action-panel" => self.action_bar == "action-panel-open",
            _ => self.search_indicator == "search" && self.input == "editable",
        }
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "state_behavior=search_indicator:{},loading_progress:{},empty_progress:{},input:{},action_bar:{}",
            self.search_indicator,
            self.loading_progress,
            self.empty_progress,
            self.input,
            self.action_bar
        )
    }
}

fn search_indicator_for_state(state: &LauncherState) -> &'static str {
    match state.view.phase {
        std_egui::LauncherPhase::Searching
            if state.view.loading == LauncherLoadingState::SlowEmptyResults =>
        {
            "spinner"
        }
        std_egui::LauncherPhase::Searching => "search",
        std_egui::LauncherPhase::Executing => "executing",
        _ => "search",
    }
}

fn loading_progress_for_state(state: &LauncherState) -> &'static str {
    if state.view.loading == LauncherLoadingState::UpdatingResults {
        "2px-accent-indeterminate"
    } else {
        "not-rendered"
    }
}

fn empty_progress_for_state(state: &LauncherState) -> &'static str {
    if state.view.loading == LauncherLoadingState::SlowEmptyResults && state.view.results.is_empty()
    {
        "visible"
    } else {
        "not-rendered"
    }
}

fn input_contract_for_state(state: &LauncherState) -> &'static str {
    if state.view.phase == std_egui::LauncherPhase::Executing {
        "locked"
    } else {
        "editable"
    }
}

fn action_bar_contract_for_state(state_name: &str) -> &'static str {
    match state_name {
        "collapsed" => "not-rendered",
        "executing" => "cancel-and-background-hints",
        "defer" | "error" => "feedback-actions",
        "action-panel" => "action-panel-open",
        _ => "primary-action-and-actions-hint",
    }
}
