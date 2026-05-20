use super::*;
use std_core::{StdConfig, StdCore};

#[test]
fn mod_number_triggers_matching_result() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("");
    let second_action = state.view.results[1].action.name.clone();
    let execution = state
        .handle_keyboard_input_by_user(LauncherKey::TriggerResult(1), false)
        .unwrap();

    assert_eq!(state.view.selected, 1);
    assert_eq!(execution.action_name, second_action);
}

#[test]
fn mod_number_ignores_out_of_range_result() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("index");
    let selected_before = state.view.selected;
    let execution = state.handle_keyboard_input(LauncherKey::TriggerResult(9), false);

    assert!(execution.is_none());
    assert_eq!(state.view.selected, selected_before);
}
