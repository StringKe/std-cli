use super::*;
use std::{
    os::unix::process::ExitStatusExt,
    process::ExitStatus,
    sync::{Arc, Mutex},
};
use std_core::{StdConfig, StdCore};
use std_types::{Action, ActionExecutionStatus, ActionType, RegistryEntry};

#[test]
fn mod_number_triggers_matching_result() {
    let temp = tempfile::tempdir().unwrap();
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded_commands = Arc::clone(&commands);
    let core = StdCore::with_config_and_command_runner(
        StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        },
        move |program, args| {
            recorded_commands
                .lock()
                .unwrap()
                .push((program.to_string(), args.to_vec()));
            Ok(std::process::Output {
                status: ExitStatus::from_raw(0),
                stdout: Vec::new(),
                stderr: Vec::new(),
            })
        },
    );
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("");
    let second_action = state.view.results[1].action.name.clone();
    let execution = state
        .handle_keyboard_input_by_user(LauncherKey::TriggerResult(1), false)
        .unwrap();

    assert_eq!(state.view.selected, 1);
    assert_eq!(execution.action_name, second_action);
    assert!(
        commands.lock().unwrap().len() <= 1,
        "test runner must record commands instead of launching external apps"
    );
}

#[test]
fn mod_number_uses_safe_defer_path_without_user_external_opt_in() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.register_action(fixture_app_action(temp.path()))
        .unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("StdNeverLaunchFixture");
    let fixture_index = state
        .view
        .results
        .iter()
        .position(|result| result.action.name == "Open App: StdNeverLaunchFixture")
        .unwrap();
    let execution = state
        .handle_keyboard_input(LauncherKey::TriggerResult(fixture_index), false)
        .unwrap();

    assert_eq!(state.view.selected, fixture_index);
    assert_eq!(execution.action_name, "Open App: StdNeverLaunchFixture");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.deferred),
        Some(true)
    );
}

#[test]
fn mouse_double_click_result_uses_user_safe_launcher_path_in_tests() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.register_action(fixture_app_action(temp.path()))
        .unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("StdNeverLaunchFixture");
    let fixture_index = state
        .view
        .results
        .iter()
        .position(|result| result.action.name == "Open App: StdNeverLaunchFixture")
        .unwrap();
    let execution = state.trigger_result_by_user(fixture_index).unwrap();

    assert_eq!(execution.action_name, "Open App: StdNeverLaunchFixture");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        execution
            .output
            .as_ref()
            .unwrap()
            .get("deferred")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
}

fn fixture_app_action(root: &std::path::Path) -> RegistryEntry {
    let app = root.join("StdNeverLaunchFixture.app");
    RegistryEntry::from_action(
        Action::new(
            "Open App: StdNeverLaunchFixture",
            format!("Launch fixture app at {}", app.display()),
            "When testing external runner deferral",
            ActionType::AppLaunch,
        ),
        vec!["app".to_string(), "fixture".to_string()],
    )
    .with_metadata("path", app.display().to_string())
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
