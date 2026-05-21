use crate::LauncherState;
use std_core::{StdConfig, StdCore};
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
        let focus_path = "Search>Results>Search".to_string();
        state.handle_keyboard_input(LauncherKey::ActionPanel, false);
        let action_panel_selected_before = state.action_panel.selected;
        state.handle_keyboard_input(LauncherKey::ArrowDown, true);
        let ime_action_panel_selection_unchanged =
            state.action_panel.selected == action_panel_selected_before;
        state.handle_keyboard_input(LauncherKey::FocusNext, false);
        let action_panel_focus_path = format!(
            "{:?}>{:?}",
            LauncherFocusSection::ActionPanel,
            state.focus_section
        );
        state.handle_keyboard_input(LauncherKey::Escape, false);
        let mut token_state = Self::new();
        token_state.update_query("open terminal now");
        token_state.handle_keyboard_input(LauncherKey::DeletePreviousToken, false);
        let token_delete_query = token_state.view.query;
        let ime_trigger_blocked = state
            .handle_keyboard_input(LauncherKey::Enter, true)
            .is_none()
            && state.view.feedback.is_none();
        state.handle_keyboard_input(LauncherKey::Escape, true);
        let ime_escape_blocked = state.controller.visible;
        let mut ime_commit_state = Self::new();
        ime_commit_state.update_query("index");
        let query_before_preedit = ime_commit_state.view.query.clone();
        ime_commit_state.handle_ime_preedit("zhong");
        let ime_preedit_query_unchanged = ime_commit_state.view.query == query_before_preedit;
        ime_commit_state.handle_ime_commit("rebuild index");
        let ime_commit_query = ime_commit_state.view.query.clone();
        let ime_composition_path =
            format!("zh-preedit({query_before_preedit})>blocked>commit({ime_commit_query})>enter");
        let ime_commit_trigger_status = ime_commit_state
            .handle_keyboard_input(LauncherKey::Enter, false)
            .map(|execution| execution.status);
        let direct_trigger_status = state
            .handle_keyboard_input(LauncherKey::TriggerResult(0), false)
            .map(|execution| execution.status);
        let trigger_status = state
            .handle_keyboard_input(LauncherKey::Enter, false)
            .map(|execution| execution.status);
        let (user_enter_status, user_enter_deferred) = user_enter_defer_evidence();
        state.handle_keyboard_input(LauncherKey::Escape, false);
        state.handle_keyboard_input(LauncherKey::Escape, false);
        LauncherKeyboardReport {
            selected_before,
            selected_after_down,
            selected_after_up,
            direct_trigger_status,
            trigger_status,
            user_enter_status,
            user_enter_deferred,
            closed_after_escape: !state.controller.visible,
            ime_selection_unchanged,
            ime_action_panel_selection_unchanged,
            ime_trigger_blocked,
            ime_escape_blocked,
            ime_composition_path,
            ime_preedit_query_unchanged,
            ime_commit_query,
            ime_commit_trigger_status,
            focus_after_tab,
            focus_after_shift_tab,
            focus_path,
            action_panel_focus_path,
            token_delete_query,
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

fn user_enter_defer_evidence() -> (Option<ActionExecutionStatus>, bool) {
    let root = std::env::temp_dir().join(format!(
        "std-launcher-keyboard-smoke-{}",
        std::process::id()
    ));
    let config = StdConfig {
        data_dir: root.join("data"),
        ..StdConfig::default()
    };
    write_keyboard_smoke_app(&config);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);
    state.controller.show();
    state.update_query("Keyboard Smoke App");
    let Some(execution) = state.handle_keyboard_input_by_user(LauncherKey::Enter, false) else {
        let _ = std::fs::remove_dir_all(&root);
        return (None, false);
    };
    let deferred = execution
        .output
        .as_ref()
        .and_then(|output| output.get("deferred"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let _ = std::fs::remove_dir_all(root);
    (Some(execution.status), deferred)
}

fn write_keyboard_smoke_app(config: &StdConfig) {
    let app = config.apps_dir().join("KeyboardSmokeApp.app");
    let contents = app.join("Contents");
    let _ = std::fs::create_dir_all(&contents);
    let _ = std::fs::write(
        contents.join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Keyboard Smoke App</string>
<key>CFBundleName</key><string>KeyboardSmokeApp</string>
</dict></plist>"#,
    );
}
