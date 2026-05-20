use crate::LauncherState;
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherKeyboardReport {
    pub selected_before: usize,
    pub selected_after_down: usize,
    pub selected_after_up: usize,
    pub trigger_status: Option<ActionExecutionStatus>,
    pub closed_after_escape: bool,
    pub ime_selection_unchanged: bool,
    pub ime_trigger_blocked: bool,
    pub ime_escape_blocked: bool,
    pub focus_after_tab: LauncherFocusSection,
    pub focus_after_shift_tab: LauncherFocusSection,
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
            LauncherKey::Enter if self.action_panel.open => self.trigger_action_panel_selection(),
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
            LauncherKey::TriggerResult(index) => self.trigger_result_by_user(index),
            LauncherKey::Escape if self.action_panel.open => {
                self.close_action_panel();
                self.focus_section = LauncherFocusSection::Results;
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

    pub fn keyboard_smoke(query: &str) -> LauncherKeyboardReport {
        let mut state = Self::new();
        state.controller.show();
        state.update_query(query);
        let selected_before = state.view.selected;
        state.handle_keyboard_input(LauncherKey::ArrowDown, false);
        let selected_after_down = state.view.selected;
        state.handle_keyboard_input(LauncherKey::ArrowUp, false);
        let selected_after_up = state.view.selected;
        let before_ime = state.view.selected;
        state.handle_keyboard_input(LauncherKey::ArrowDown, true);
        let ime_selection_unchanged = state.view.selected == before_ime;
        state.focus_section = LauncherFocusSection::Search;
        state.handle_keyboard_input(LauncherKey::FocusNext, false);
        let focus_after_tab = state.focus_section;
        state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
        let focus_after_shift_tab = state.focus_section;
        let ime_trigger_blocked = state
            .handle_keyboard_input(LauncherKey::Enter, true)
            .is_none()
            && state.view.feedback.is_none();
        state.handle_keyboard_input(LauncherKey::Escape, true);
        let ime_escape_blocked = state.controller.visible;
        let trigger_status = state
            .handle_keyboard_input(LauncherKey::Enter, false)
            .map(|execution| execution.status);
        state.handle_keyboard_input(LauncherKey::Escape, false);
        state.handle_keyboard_input(LauncherKey::Escape, false);
        LauncherKeyboardReport {
            selected_before,
            selected_after_down,
            selected_after_up,
            trigger_status,
            closed_after_escape: !state.controller.visible,
            ime_selection_unchanged,
            ime_trigger_blocked,
            ime_escape_blocked,
            focus_after_tab,
            focus_after_shift_tab,
        }
    }

    fn focus_next_section(&mut self) {
        self.focus_section = match (self.focus_section, self.action_panel.open) {
            (LauncherFocusSection::Search, _) => LauncherFocusSection::Results,
            (LauncherFocusSection::Results, true) => LauncherFocusSection::ActionPanel,
            (LauncherFocusSection::Results, false) => LauncherFocusSection::Search,
            (LauncherFocusSection::ActionPanel, _) => LauncherFocusSection::Search,
        };
    }

    fn focus_previous_section(&mut self) {
        self.focus_section = match (self.focus_section, self.action_panel.open) {
            (LauncherFocusSection::Search, true) => LauncherFocusSection::ActionPanel,
            (LauncherFocusSection::Search, false) => LauncherFocusSection::Results,
            (LauncherFocusSection::Results, _) => LauncherFocusSection::Search,
            (LauncherFocusSection::ActionPanel, _) => LauncherFocusSection::Results,
        };
    }
}

impl LauncherKeyboardReport {
    pub fn pass(&self) -> bool {
        self.selected_after_down > self.selected_before
            && self.selected_after_up == self.selected_before
            && self.trigger_status.is_some()
            && self.closed_after_escape
            && self.ime_selection_unchanged
            && self.ime_trigger_blocked
            && self.ime_escape_blocked
            && self.focus_after_tab == LauncherFocusSection::Results
            && self.focus_after_shift_tab == LauncherFocusSection::Search
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_keyboard_smoke {}\nselected_before={}\nselected_after_down={}\nselected_after_up={}\ntrigger_status={}\nclosed_after_escape={}\nime_selection_unchanged={}\nime_trigger_blocked={}\nime_escape_blocked={}\nfocus_after_tab={:?}\nfocus_after_shift_tab={:?}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.selected_before,
            self.selected_after_down,
            self.selected_after_up,
            self.trigger_status
                .as_ref()
                .map(|status| format!("{status:?}"))
                .unwrap_or_else(|| "none".to_string()),
            self.closed_after_escape,
            self.ime_selection_unchanged,
            self.ime_trigger_blocked,
            self.ime_escape_blocked,
            self.focus_after_tab,
            self.focus_after_shift_tab
        )
    }
}
