use crate::LauncherState;
use std_egui::{LauncherFeedbackAction, LauncherResultMode};
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherKey {
    ArrowDown,
    ArrowUp,
    JumpToFirst,
    JumpToLast,
    FocusNext,
    FocusPrevious,
    Enter,
    Escape,
    ActionPanel,
    DeletePreviousToken,
    TriggerResult(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherFocusSection {
    Search,
    Results,
    ActionPanel,
    Feedback,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherKeyboardReport {
    pub selected_before: usize,
    pub selected_after_down: usize,
    pub selected_after_up: usize,
    pub direct_trigger_status: Option<ActionExecutionStatus>,
    pub trigger_status: Option<ActionExecutionStatus>,
    pub user_enter_status: Option<ActionExecutionStatus>,
    pub user_enter_deferred: bool,
    pub closed_after_escape: bool,
    pub ime_selection_unchanged: bool,
    pub ime_action_panel_selection_unchanged: bool,
    pub ime_trigger_blocked: bool,
    pub ime_escape_blocked: bool,
    pub ime_composition_path: String,
    pub ime_preedit_query_unchanged: bool,
    pub ime_commit_query: String,
    pub ime_commit_trigger_status: Option<ActionExecutionStatus>,
    pub focus_after_tab: LauncherFocusSection,
    pub focus_after_shift_tab: LauncherFocusSection,
    pub focus_path: String,
    pub action_panel_focus_path: String,
    pub token_delete_query: String,
}

impl LauncherState {
    pub fn handle_keyboard_input(
        &mut self,
        key: LauncherKey,
        ime_composing: bool,
    ) -> Option<std_types::ActionExecution> {
        self.handle_keyboard_input_with_external_runner(key, ime_composing, false)
    }

    pub fn handle_keyboard_input_by_user(
        &mut self,
        key: LauncherKey,
        ime_composing: bool,
    ) -> Option<std_types::ActionExecution> {
        self.handle_keyboard_input_with_external_runner(key, ime_composing, true)
    }

    pub fn handle_ime_preedit(&mut self, _preedit: &str) {
        // IME preedit is candidate text, not committed query text.
    }

    pub fn handle_ime_commit(
        &mut self,
        committed: impl Into<String>,
    ) -> Option<std_types::ActionPreview> {
        self.update_query(committed)
    }

    fn handle_keyboard_input_with_external_runner(
        &mut self,
        key: LauncherKey,
        ime_composing: bool,
        allow_external_runner: bool,
    ) -> Option<std_types::ActionExecution> {
        if ime_composing {
            return None;
        }
        match key {
            LauncherKey::ArrowDown if self.focus_section == LauncherFocusSection::Feedback => {
                self.view.move_feedback_action(1);
                None
            }
            LauncherKey::ArrowUp if self.focus_section == LauncherFocusSection::Feedback => {
                self.view.move_feedback_action(-1);
                None
            }
            LauncherKey::JumpToFirst if self.focus_section == LauncherFocusSection::Feedback => {
                self.view.selected_feedback_action = 0;
                None
            }
            LauncherKey::JumpToLast if self.focus_section == LauncherFocusSection::Feedback => {
                let last = self.view.feedback_actions().len().saturating_sub(1);
                self.view.selected_feedback_action = last;
                None
            }
            LauncherKey::ArrowDown if self.action_panel.open => {
                self.move_action_panel_selection(1);
                None
            }
            LauncherKey::ArrowUp if self.action_panel.open => {
                self.move_action_panel_selection(-1);
                None
            }
            LauncherKey::JumpToFirst if self.action_panel.open => {
                self.jump_action_panel_selection(true);
                None
            }
            LauncherKey::JumpToLast if self.action_panel.open => {
                self.jump_action_panel_selection(false);
                None
            }
            LauncherKey::ArrowDown => {
                self.move_selection(1);
                None
            }
            LauncherKey::ArrowUp => {
                self.move_selection(-1);
                None
            }
            LauncherKey::JumpToFirst => {
                self.jump_selection(true);
                None
            }
            LauncherKey::JumpToLast => {
                self.jump_selection(false);
                None
            }
            LauncherKey::FocusNext => {
                self.focus_next_section();
                None
            }
            LauncherKey::FocusPrevious => {
                self.focus_previous_section();
                None
            }
            LauncherKey::Enter if self.focus_section == LauncherFocusSection::Feedback => {
                self.trigger_feedback_action()
            }
            LauncherKey::Enter if self.action_panel.open => self.trigger_action_panel_selection(),
            LauncherKey::Enter if self.view.result_mode == LauncherResultMode::NaturalLanguage => {
                self.trigger_selected()
            }
            LauncherKey::Enter if self.view.results.is_empty() => {
                self.trigger_no_match_fallback();
                None
            }
            LauncherKey::Enter if allow_external_runner => self.trigger_selected_by_user(),
            LauncherKey::Enter => self.trigger_selected(),
            LauncherKey::ActionPanel => {
                self.open_action_panel();
                self.focus_section = LauncherFocusSection::ActionPanel;
                None
            }
            LauncherKey::DeletePreviousToken => {
                self.view.delete_previous_query_token(&self.core);
                None
            }
            LauncherKey::TriggerResult(index) if allow_external_runner => {
                self.trigger_result_by_user(index)
            }
            LauncherKey::TriggerResult(index) => self.trigger_result(index),
            LauncherKey::Escape if self.action_panel.open => {
                self.close_action_panel();
                self.focus_section = LauncherFocusSection::Results;
                None
            }
            LauncherKey::Escape if self.focus_section == LauncherFocusSection::Feedback => {
                self.focus_section = LauncherFocusSection::Search;
                None
            }
            LauncherKey::Escape if !self.view.query.is_empty() => {
                self.update_query("");
                None
            }
            LauncherKey::Escape => {
                self.hide();
                None
            }
        }
    }

    pub fn trigger_feedback_action(&mut self) -> Option<std_types::ActionExecution> {
        match self.view.selected_feedback_action()? {
            LauncherFeedbackAction::Copy => Some(self.complete_feedback_copy()),
            LauncherFeedbackAction::Retry => self.trigger_selected(),
            LauncherFeedbackAction::OpenStudio => {
                self.open_studio_execution_history_from_feedback();
                None
            }
        }
    }

    fn complete_feedback_copy(&mut self) -> std_types::ActionExecution {
        let feedback_summary = self
            .view
            .feedback
            .as_ref()
            .map(std_egui::LauncherFeedback::summary)
            .unwrap_or_else(|| "Launcher feedback".to_string());
        let execution = std_types::ActionExecution {
            action_id: Default::default(),
            action_name: "Copy Feedback".to_string(),
            status: ActionExecutionStatus::Completed,
            message: feedback_summary.clone(),
            output: Some(serde_json::json!({ "copied": feedback_summary })),
            created_at: chrono::Utc::now(),
        };
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        execution
    }

    fn focus_next_section(&mut self) {
        self.focus_section = match (
            self.focus_section,
            self.action_panel.open,
            self.view.feedback.is_some(),
        ) {
            (LauncherFocusSection::Search, _, _) => LauncherFocusSection::Results,
            (LauncherFocusSection::Results, true, _) => LauncherFocusSection::ActionPanel,
            (LauncherFocusSection::Results, false, true) => LauncherFocusSection::Feedback,
            (LauncherFocusSection::Results, false, false) => LauncherFocusSection::Search,
            (LauncherFocusSection::ActionPanel, _, true) => LauncherFocusSection::Feedback,
            (LauncherFocusSection::ActionPanel, _, false) => LauncherFocusSection::Search,
            (LauncherFocusSection::Feedback, _, _) => LauncherFocusSection::Search,
        };
    }

    fn focus_previous_section(&mut self) {
        self.focus_section = match (
            self.focus_section,
            self.action_panel.open,
            self.view.feedback.is_some(),
        ) {
            (LauncherFocusSection::Search, _, true) => LauncherFocusSection::Feedback,
            (LauncherFocusSection::Search, true, false) => LauncherFocusSection::ActionPanel,
            (LauncherFocusSection::Search, false, false) => LauncherFocusSection::Results,
            (LauncherFocusSection::Results, _, _) => LauncherFocusSection::Search,
            (LauncherFocusSection::ActionPanel, _, _) => LauncherFocusSection::Results,
            (LauncherFocusSection::Feedback, true, _) => LauncherFocusSection::ActionPanel,
            (LauncherFocusSection::Feedback, false, _) => LauncherFocusSection::Results,
        };
    }
}

impl LauncherKeyboardReport {
    pub fn pass(&self) -> bool {
        self.selected_after_down > self.selected_before
            && self.selected_after_up == self.selected_before
            && self.direct_trigger_status.is_some()
            && self.trigger_status.is_some()
            && self.user_enter_status == Some(ActionExecutionStatus::NeedsExternalRunner)
            && self.user_enter_deferred
            && self.closed_after_escape
            && self.ime_selection_unchanged
            && self.ime_action_panel_selection_unchanged
            && self.ime_trigger_blocked
            && self.ime_escape_blocked
            && self.ime_preedit_query_unchanged
            && self.ime_commit_query == "rebuild index"
            && self.ime_composition_path == "zh-preedit(index)>blocked>commit(rebuild index)>enter"
            && self.ime_commit_trigger_status.is_some()
            && self.focus_after_tab == LauncherFocusSection::Results
            && self.focus_after_shift_tab == LauncherFocusSection::Search
            && self.focus_path == "Search>Results>Search"
            && self.action_panel_focus_path == "ActionPanel>Search"
            && self.token_delete_query == "open terminal"
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_keyboard_smoke {}\nselected_before={}\nselected_after_down={}\nselected_after_up={}\ndirect_trigger_status={}\ntrigger_status={}\nuser_enter_status={}\nuser_enter_deferred={}\nclosed_after_escape={}\nime_selection_unchanged={}\nime_action_panel_selection_unchanged={}\nime_trigger_blocked={}\nime_escape_blocked={}\nime_composition_path={}\nime_preedit_query_unchanged={}\nime_commit_query={}\nime_commit_trigger_status={}\nfocus_after_tab={:?}\nfocus_after_shift_tab={:?}\nfocus_path={}\naction_panel_focus_path={}\ntoken_delete_query={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.selected_before,
            self.selected_after_down,
            self.selected_after_up,
            self.direct_trigger_status
                .as_ref()
                .map(|status| format!("{status:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.trigger_status
                .as_ref()
                .map(|status| format!("{status:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.user_enter_status
                .as_ref()
                .map(|status| format!("{status:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.user_enter_deferred,
            self.closed_after_escape,
            self.ime_selection_unchanged,
            self.ime_action_panel_selection_unchanged,
            self.ime_trigger_blocked,
            self.ime_escape_blocked,
            self.ime_composition_path,
            self.ime_preedit_query_unchanged,
            self.ime_commit_query,
            self.ime_commit_trigger_status
                .as_ref()
                .map(|status| format!("{status:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.focus_after_tab,
            self.focus_after_shift_tab,
            self.focus_path,
            self.action_panel_focus_path,
            self.token_delete_query
        )
    }
}
