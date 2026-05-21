use crate::LauncherState;
use std_egui::{LauncherFeedbackAction, LauncherPhase, LauncherResultMode};
use std_types::{ActionExecution, ActionExecutionStatus};

pub fn launcher_execution_hides_window(execution: &ActionExecution) -> bool {
    execution.status == ActionExecutionStatus::Completed
}

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
    CompleteSelectedQuery,
    DeletePreviousToken,
    TypeActionPanelQuery(char),
    TriggerResult(usize),
    MoveExecutingToBackground,
    CancelExecuting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherFocusSection {
    Search,
    Results,
    ActionPanel,
    Feedback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LauncherFocusSource {
    Keyboard,
    Pointer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherKeyboardReport {
    pub selected_before: usize,
    pub selected_after_down: usize,
    pub selected_after_up: usize,
    pub navigation_boundary_path: String,
    pub direct_trigger_status: Option<ActionExecutionStatus>,
    pub trigger_status: Option<ActionExecutionStatus>,
    pub user_enter_status: Option<ActionExecutionStatus>,
    pub user_enter_route: String,
    pub user_enter_deferred: bool,
    pub user_enter_feedback_visible: bool,
    pub user_enter_keeps_launcher_open: bool,
    pub closed_after_escape: bool,
    pub ime_selection_unchanged: bool,
    pub ime_action_panel_selection_unchanged: bool,
    pub ime_trigger_blocked: bool,
    pub ime_escape_blocked: bool,
    pub ime_composition_path: String,
    pub ime_preedit_query_unchanged: bool,
    pub ime_commit_query: String,
    pub ime_commit_trigger_status: Option<ActionExecutionStatus>,
    pub empty_suggestion_keyboard_path: String,
    pub focus_after_tab: LauncherFocusSection,
    pub focus_after_shift_tab: LauncherFocusSection,
    pub focus_path: String,
    pub action_panel_focus_path: String,
    pub completed_query: String,
    pub token_delete_query: String,
    pub enter_window: LauncherEnterWindowReport,
    pub model_contract: &'static str,
    pub real_interaction_contract: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherEnterWindowReport {
    pub completed_status: Option<ActionExecutionStatus>,
    pub completed_hide_requested: bool,
    pub completed_window_commands: String,
    pub deferred_status: Option<ActionExecutionStatus>,
    pub deferred_hide_requested: bool,
    pub deferred_window_commands: String,
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
        self.focus_source = LauncherFocusSource::Keyboard;
        match key {
            LauncherKey::ArrowDown
            | LauncherKey::ArrowUp
            | LauncherKey::JumpToFirst
            | LauncherKey::JumpToLast => self.handle_navigation_key(key),
            LauncherKey::FocusNext => {
                self.focus_next_section();
                None
            }
            LauncherKey::FocusPrevious => {
                self.focus_previous_section();
                None
            }
            LauncherKey::Enter => self.handle_enter_key(allow_external_runner),
            LauncherKey::ActionPanel => {
                self.open_action_panel();
                self.focus_section = LauncherFocusSection::ActionPanel;
                None
            }
            LauncherKey::CompleteSelectedQuery => {
                self.complete_query_from_selection();
                None
            }
            LauncherKey::DeletePreviousToken => {
                self.view.delete_previous_query_token(&self.core);
                None
            }
            LauncherKey::TypeActionPanelQuery(ch) => {
                self.type_action_panel_query(ch);
                None
            }
            LauncherKey::MoveExecutingToBackground => {
                self.move_executing_to_background();
                None
            }
            LauncherKey::CancelExecuting => {
                self.cancel_executing();
                None
            }
            LauncherKey::TriggerResult(index) if allow_external_runner => {
                self.trigger_result_by_user(index)
            }
            LauncherKey::TriggerResult(index) => self.trigger_result(index),
            LauncherKey::Escape => self.handle_escape_key(),
        }
    }

    fn handle_navigation_key(&mut self, key: LauncherKey) -> Option<std_types::ActionExecution> {
        let (first, delta) = match key {
            LauncherKey::ArrowDown => (None, 1),
            LauncherKey::ArrowUp => (None, -1),
            LauncherKey::JumpToFirst => (Some(true), 0),
            LauncherKey::JumpToLast => (Some(false), 0),
            _ => return None,
        };
        if self.focus_section == LauncherFocusSection::Feedback {
            self.navigate_feedback(first, delta);
        } else if self.action_panel.open {
            self.navigate_action_panel(first, delta);
        } else if self.empty_query_suggestions_visible() {
            self.navigate_empty_suggestion(first, delta);
        } else if let Some(first) = first {
            self.jump_selection(first);
        } else {
            self.move_selection(delta);
        }
        None
    }

    fn navigate_feedback(&mut self, first: Option<bool>, delta: isize) {
        if let Some(first) = first {
            self.view.selected_feedback_action = if first {
                0
            } else {
                self.view.feedback_actions().len().saturating_sub(1)
            };
        } else {
            self.view.move_feedback_action(delta);
        }
    }

    fn navigate_action_panel(&mut self, first: Option<bool>, delta: isize) {
        if let Some(first) = first {
            self.jump_action_panel_selection(first);
        } else {
            self.move_action_panel_selection(delta);
        }
    }

    fn navigate_empty_suggestion(&mut self, first: Option<bool>, delta: isize) {
        if let Some(first) = first {
            self.jump_empty_suggestion(first);
        } else {
            self.move_empty_suggestion(delta);
        }
    }

    fn handle_enter_key(
        &mut self,
        allow_external_runner: bool,
    ) -> Option<std_types::ActionExecution> {
        if self.view.phase == LauncherPhase::Executing {
            self.move_executing_to_background();
            return None;
        }
        if self.focus_section == LauncherFocusSection::Feedback {
            return self.trigger_feedback_action();
        }
        if self.action_panel.open {
            return if allow_external_runner {
                self.trigger_action_panel_selection_by_user()
            } else {
                self.trigger_action_panel_selection()
            };
        }
        if self.view.result_mode == LauncherResultMode::NaturalLanguage {
            return self.trigger_selected();
        }
        if self.empty_query_suggestions_visible() {
            self.apply_empty_suggestion();
            return None;
        }
        if self.view.results.is_empty() {
            self.trigger_no_match_fallback();
            return None;
        }
        if allow_external_runner {
            self.trigger_selected_by_user()
        } else {
            self.trigger_selected()
        }
    }

    fn handle_escape_key(&mut self) -> Option<std_types::ActionExecution> {
        if self.action_panel.open {
            self.close_action_panel();
            self.focus_section = LauncherFocusSection::Search;
        } else if self.focus_section == LauncherFocusSection::Feedback {
            self.focus_section = LauncherFocusSection::Search;
        } else if !self.view.query.is_empty() {
            self.update_query("");
        } else {
            self.hide();
        }
        None
    }

    pub fn move_executing_to_background(&mut self) {
        if self.view.phase != LauncherPhase::Executing {
            return;
        }
        self.hide();
    }

    pub fn cancel_executing(&mut self) {
        if self.view.phase != LauncherPhase::Executing {
            return;
        }
        self.view.phase = if self.view.results.is_empty() {
            if self.view.query.trim().is_empty() {
                std_egui::LauncherPhase::Empty
            } else {
                std_egui::LauncherPhase::NoMatches
            }
        } else {
            std_egui::LauncherPhase::WithResults
        };
        self.focus_section = LauncherFocusSection::Search;
        self.focus_source = LauncherFocusSource::Keyboard;
    }

    pub fn trigger_feedback_action(&mut self) -> Option<std_types::ActionExecution> {
        match self.view.selected_feedback_action()? {
            LauncherFeedbackAction::Copy => self.copy_feedback_to_clipboard_model(),
            LauncherFeedbackAction::Retry => self.trigger_selected_by_user(),
            LauncherFeedbackAction::OpenStudio => {
                self.open_studio_execution_history_from_feedback();
                None
            }
        }
    }

    pub fn copy_feedback_to_clipboard_model(&mut self) -> Option<std_types::ActionExecution> {
        let execution = self.complete_feedback_copy()?;
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        Some(execution)
    }

    fn complete_feedback_copy(&self) -> Option<std_types::ActionExecution> {
        let feedback_summary = self
            .view
            .feedback
            .as_ref()
            .map(std_egui::LauncherFeedback::summary)?;
        Some(std_types::ActionExecution {
            action_id: Default::default(),
            action_name: "Copy Feedback".to_string(),
            status: ActionExecutionStatus::Completed,
            message: feedback_summary.clone(),
            output: Some(serde_json::json!({ "copied": feedback_summary })),
            created_at: chrono::Utc::now(),
        })
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
            && self
                .navigation_boundary_path
                .starts_with("top:0->0;bottom:")
            && self.navigation_boundary_path.ends_with("->same")
            && self.direct_trigger_status.is_some()
            && self.trigger_status.is_some()
            && self.user_enter_status == Some(ActionExecutionStatus::NeedsExternalRunner)
            && self.user_enter_route == "Enter>handle_keyboard_input_by_user>LauncherUser"
            && self.user_enter_deferred
            && self.user_enter_feedback_visible
            && self.user_enter_keeps_launcher_open
            && self.closed_after_escape
            && self.ime_selection_unchanged
            && self.ime_action_panel_selection_unchanged
            && self.ime_trigger_blocked
            && self.ime_escape_blocked
            && self.ime_preedit_query_unchanged
            && self.ime_commit_query == "rebuild index"
            && self.ime_composition_path == "zh-preedit(index)>blocked>commit(rebuild index)>enter"
            && self.ime_commit_trigger_status.is_some()
            && self.empty_suggestion_keyboard_path == "0->1->2->2=> > studio"
            && self.focus_after_tab == LauncherFocusSection::Results
            && self.focus_after_shift_tab == LauncherFocusSection::Search
            && self.focus_path == "Search>Results>Search"
            && self.action_panel_focus_path == "ActionPanel>Search"
            && self.completed_query == "rebuild index"
            && self.token_delete_query == "open terminal"
            && self.enter_window.pass()
            && self.model_contract
                == "model=keyboard-navigation,ime-guard,user-enter-defer,no-desktop-events"
            && self.real_interaction_contract
                == "real-focus-enter-toggle=requires-STD_ALLOW_BACKGROUND_UI_AUTOMATION"
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_keyboard_smoke {}\nselected_before={}\nselected_after_down={}\nselected_after_up={}\nnavigation_boundary_path={}\ndirect_trigger_status={}\ntrigger_status={}\nuser_enter_status={}\nuser_enter_route={}\nuser_enter_deferred={}\nuser_enter_feedback_visible={}\nuser_enter_keeps_launcher_open={}\nclosed_after_escape={}\nime_selection_unchanged={}\nime_action_panel_selection_unchanged={}\nime_trigger_blocked={}\nime_escape_blocked={}\nime_composition_path={}\nime_preedit_query_unchanged={}\nime_commit_query={}\nime_commit_trigger_status={}\nempty_suggestion_keyboard_path={}\nfocus_after_tab={:?}\nfocus_after_shift_tab={:?}\nfocus_path={}\naction_panel_focus_path={}\ncompleted_query={}\ntoken_delete_query={}\nenter_window={}\nmodel_contract={}\nreal_interaction_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.selected_before,
            self.selected_after_down,
            self.selected_after_up,
            self.navigation_boundary_path,
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
            self.user_enter_route,
            self.user_enter_deferred,
            self.user_enter_feedback_visible,
            self.user_enter_keeps_launcher_open,
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
            self.empty_suggestion_keyboard_path,
            self.focus_after_tab,
            self.focus_after_shift_tab,
            self.focus_path,
            self.action_panel_focus_path,
            self.completed_query,
            self.token_delete_query,
            self.enter_window.summary(),
            self.model_contract,
            self.real_interaction_contract
        )
    }
}

impl LauncherEnterWindowReport {
    pub fn pass(&self) -> bool {
        self.completed_status == Some(ActionExecutionStatus::Completed)
            && self.completed_hide_requested
            && self.completed_window_commands == "Visible(false)"
            && self.deferred_status == Some(ActionExecutionStatus::NeedsExternalRunner)
            && !self.deferred_hide_requested
            && self.deferred_window_commands == "none"
    }

    pub fn summary(&self) -> String {
        format!(
            "completed_status={};completed_hide={};completed_commands={};deferred_status={};deferred_hide={};deferred_commands={}",
            status_label(self.completed_status.as_ref()),
            self.completed_hide_requested,
            self.completed_window_commands,
            status_label(self.deferred_status.as_ref()),
            self.deferred_hide_requested,
            self.deferred_window_commands
        )
    }
}

fn status_label(status: Option<&ActionExecutionStatus>) -> String {
    status
        .map(|status| format!("{status:?}"))
        .unwrap_or_else(|| "none".to_string())
}
