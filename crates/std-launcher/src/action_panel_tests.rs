use super::*;
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

#[test]
fn action_panel_opens_for_selected_result_and_closes_on_search_change() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    let opened = state.open_action_panel();
    let first_item = state
        .action_panel
        .selected_item()
        .unwrap()
        .title()
        .to_string();
    state.update_query("index");

    assert!(opened);
    assert!(state.action_panel.action_name.contains("Terminal"));
    assert_eq!(first_item, "Run");
    assert!(!state.action_panel.open);
}

#[test]
fn action_panel_keyboard_path_defers_external_runner_by_default() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(state.action_panel.selected, 1);
    assert_eq!(state.action_panel.selected_item().unwrap().title(), "Defer");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
}

#[test]
fn action_panel_ime_blocks_open_and_trigger() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("index");
    let open = state.handle_keyboard_input(LauncherKey::ActionPanel, true);
    state.open_action_panel();
    let trigger = state.handle_keyboard_input(LauncherKey::Enter, true);

    assert!(open.is_none());
    assert!(state.action_panel.open);
    assert!(trigger.is_none());
    assert!(state.view.feedback.is_none());
}

#[test]
fn action_panel_filters_actions_and_resets_selection() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.open_action_panel();
    state.move_action_panel_selection(1);
    state.update_action_panel_query("copy");

    let visible_titles = state
        .action_panel
        .visible_items()
        .into_iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>();

    assert_eq!(state.action_panel.selected, 0);
    assert_eq!(visible_titles, vec!["Copy command"]);
    assert_eq!(
        state.action_panel.selected_item().unwrap().title(),
        "Copy command"
    );
}

#[test]
fn mod_backspace_deletes_previous_query_token() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("  open   terminal now ");
    state.handle_keyboard_input(LauncherKey::DeletePreviousToken, false);

    assert_eq!(state.view.query, "open terminal");
}
