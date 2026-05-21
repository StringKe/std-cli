use crate::{LauncherFocusSection, LauncherKey, LauncherKeyboardReport, LauncherState};
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

impl LauncherState {
    pub fn keyboard_smoke(query: &str) -> LauncherKeyboardReport {
        let mut state = Self::new();
        state.controller.show();
        state.update_query(query);
        let selected_before = state.view.selected;
        state.handle_keyboard_input(LauncherKey::ArrowDown, false);
        let selected_after_down = state.view.selected;
        state.handle_keyboard_input(LauncherKey::ArrowUp, false);
        let selected_after_up = state.view.selected;
        let ime = ime_evidence(&mut state);
        state.focus_section = LauncherFocusSection::Search;
        state.handle_keyboard_input(LauncherKey::FocusNext, false);
        let focus_after_tab = state.focus_section;
        state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
        let focus_after_shift_tab = state.focus_section;
        let action_panel_focus_path = action_panel_focus_path(&mut state);
        let token_delete_query = token_delete_query();
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
            ime_selection_unchanged: ime.selection_unchanged,
            ime_action_panel_selection_unchanged: ime.action_panel_selection_unchanged,
            ime_trigger_blocked: ime.trigger_blocked,
            ime_escape_blocked: ime.escape_blocked,
            ime_composition_path: ime.composition_path,
            ime_preedit_query_unchanged: ime.preedit_query_unchanged,
            ime_commit_query: ime.commit_query,
            ime_commit_trigger_status: ime.commit_trigger_status,
            focus_after_tab,
            focus_after_shift_tab,
            focus_path: "Search>Results>Search".to_string(),
            action_panel_focus_path,
            token_delete_query,
        }
    }
}

struct ImeEvidence {
    selection_unchanged: bool,
    action_panel_selection_unchanged: bool,
    trigger_blocked: bool,
    escape_blocked: bool,
    composition_path: String,
    preedit_query_unchanged: bool,
    commit_query: String,
    commit_trigger_status: Option<ActionExecutionStatus>,
}

fn ime_evidence(state: &mut LauncherState) -> ImeEvidence {
    let before_ime = state.view.selected;
    state.handle_keyboard_input(LauncherKey::ArrowDown, true);
    let selection_unchanged = state.view.selected == before_ime;
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let action_panel_selected_before = state.action_panel.selected;
    state.handle_keyboard_input(LauncherKey::ArrowDown, true);
    let action_panel_selection_unchanged =
        state.action_panel.selected == action_panel_selected_before;
    state.handle_keyboard_input(LauncherKey::Escape, false);
    let trigger_blocked = state
        .handle_keyboard_input(LauncherKey::Enter, true)
        .is_none()
        && state.view.feedback.is_none();
    state.handle_keyboard_input(LauncherKey::Escape, true);
    let escape_blocked = state.controller.visible;
    let mut commit_state = LauncherState::new();
    commit_state.update_query("index");
    let query_before_preedit = commit_state.view.query.clone();
    commit_state.handle_ime_preedit("zhong");
    let preedit_query_unchanged = commit_state.view.query == query_before_preedit;
    commit_state.handle_ime_commit("rebuild index");
    let commit_query = commit_state.view.query.clone();
    let composition_path =
        format!("zh-preedit({query_before_preedit})>blocked>commit({commit_query})>enter");
    let commit_trigger_status = commit_state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .map(|execution| execution.status);
    ImeEvidence {
        selection_unchanged,
        action_panel_selection_unchanged,
        trigger_blocked,
        escape_blocked,
        composition_path,
        preedit_query_unchanged,
        commit_query,
        commit_trigger_status,
    }
}

fn action_panel_focus_path(state: &mut LauncherState) -> String {
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    state.handle_keyboard_input(LauncherKey::FocusNext, false);
    let path = format!(
        "{:?}>{:?}",
        LauncherFocusSection::ActionPanel,
        state.focus_section
    );
    state.handle_keyboard_input(LauncherKey::Escape, false);
    path
}

fn token_delete_query() -> String {
    let mut state = LauncherState::new();
    state.update_query("open terminal now");
    state.handle_keyboard_input(LauncherKey::DeletePreviousToken, false);
    state.view.query
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
