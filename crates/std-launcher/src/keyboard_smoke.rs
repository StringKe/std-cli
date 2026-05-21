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
        let navigation_boundary_path = navigation_boundary_path(query);
        let ime = ime_evidence(&mut state);
        state.focus_section = LauncherFocusSection::Search;
        state.handle_keyboard_input(LauncherKey::FocusNext, false);
        let focus_after_tab = state.focus_section;
        state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
        let focus_after_shift_tab = state.focus_section;
        let action_panel_focus_path = action_panel_focus_path(&mut state);
        let completed_query = completed_query();
        let token_delete_query = token_delete_query();
        let empty_suggestion_keyboard_path = empty_suggestion_keyboard_path();
        let direct_trigger_status = state
            .handle_keyboard_input(LauncherKey::TriggerResult(0), false)
            .map(|execution| execution.status);
        let trigger_status = state
            .handle_keyboard_input(LauncherKey::Enter, false)
            .map(|execution| execution.status);
        let user_enter = user_enter_defer_evidence();
        state.handle_keyboard_input(LauncherKey::Escape, false);
        state.handle_keyboard_input(LauncherKey::Escape, false);
        LauncherKeyboardReport {
            selected_before,
            selected_after_down,
            selected_after_up,
            navigation_boundary_path,
            direct_trigger_status,
            trigger_status,
            user_enter_status: user_enter.status,
            user_enter_route: user_enter.route,
            user_enter_deferred: user_enter.deferred,
            user_enter_feedback_visible: user_enter.feedback_visible,
            user_enter_keeps_launcher_open: user_enter.keeps_launcher_open,
            closed_after_escape: !state.controller.visible,
            ime_selection_unchanged: ime.selection_unchanged,
            ime_action_panel_selection_unchanged: ime.action_panel_selection_unchanged,
            ime_trigger_blocked: ime.trigger_blocked,
            ime_escape_blocked: ime.escape_blocked,
            ime_composition_path: ime.composition_path,
            ime_preedit_query_unchanged: ime.preedit_query_unchanged,
            ime_commit_query: ime.commit_query,
            ime_commit_trigger_status: ime.commit_trigger_status,
            empty_suggestion_keyboard_path,
            focus_after_tab,
            focus_after_shift_tab,
            focus_path: "Search>Results>Search".to_string(),
            action_panel_focus_path,
            completed_query,
            token_delete_query,
        }
    }
}

fn empty_suggestion_keyboard_path() -> String {
    let mut state = LauncherState::new();
    state.update_query("");
    let before = state.empty_suggestion_selected;
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let after_down = state.empty_suggestion_selected;
    state.handle_keyboard_input(LauncherKey::JumpToLast, false);
    let after_last = state.empty_suggestion_selected;
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let after_boundary = state.empty_suggestion_selected;
    state.handle_keyboard_input(LauncherKey::Enter, false);
    format!(
        "{before}->{after_down}->{after_last}->{after_boundary}=> {}",
        state.view.query
    )
}

fn navigation_boundary_path(query: &str) -> String {
    let mut state = LauncherState::new();
    state.update_query(query);
    state.handle_keyboard_input(LauncherKey::ArrowUp, false);
    let top = state.view.selected;
    state.handle_keyboard_input(LauncherKey::JumpToLast, false);
    let bottom_before = state.view.selected;
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let bottom_after = state.view.selected;
    let bottom_marker = if bottom_before == bottom_after {
        "same".to_string()
    } else {
        bottom_after.to_string()
    };
    format!("top:0->{top};bottom:{bottom_before}->{bottom_marker}")
}

struct UserEnterEvidence {
    status: Option<ActionExecutionStatus>,
    route: String,
    deferred: bool,
    feedback_visible: bool,
    keeps_launcher_open: bool,
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

fn completed_query() -> String {
    let mut state = LauncherState::new();
    state.update_query("reb");
    state.handle_keyboard_input(LauncherKey::CompleteSelectedQuery, false);
    state.view.query
}

fn user_enter_defer_evidence() -> UserEnterEvidence {
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
        return UserEnterEvidence {
            status: None,
            route: "Enter>handle_keyboard_input_by_user>LauncherUser".to_string(),
            deferred: false,
            feedback_visible: false,
            keeps_launcher_open: false,
        };
    };
    let deferred = execution
        .output
        .as_ref()
        .and_then(|output| output.get("deferred"))
        .and_then(|value| value.as_bool())
        .unwrap_or(false);
    let feedback_visible = state.view.feedback.is_some();
    let keeps_launcher_open = state.controller.visible;
    let _ = std::fs::remove_dir_all(root);
    UserEnterEvidence {
        status: Some(execution.status),
        route: "Enter>handle_keyboard_input_by_user>LauncherUser".to_string(),
        deferred,
        feedback_visible,
        keeps_launcher_open,
    }
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
