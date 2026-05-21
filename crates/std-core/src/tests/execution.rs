use super::*;
use std::sync::{Arc, Mutex};
use std_types::{ActionExecutionStatus, ActionType, RegistryEntry};

#[test]
fn core_previews_and_executes_actions_with_feedback() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let mut action = make_test_action("Print Runner Smoke");
    action.action_type = ActionType::Command;
    action.description = "Run a deterministic command".to_string();
    core.register_action(
        RegistryEntry::from_action(action, vec!["runner".to_string()])
            .with_metadata("command", "printf runner-smoke"),
    )
    .unwrap();

    let result = core.search("runner", 1).unwrap().remove(0);
    let preview = core.preview_action(result.action.id).unwrap();
    let execution = core.execute_action(result.action.id).unwrap();
    let events = core.events().unwrap();

    assert_eq!(preview.title, "Print Runner Smoke");
    assert_eq!(preview.primary_command, "printf runner-smoke");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.message, "printf runner-smoke");
    assert!(execution
        .output
        .as_ref()
        .unwrap()
        .get("deferred")
        .and_then(|value| value.as_bool())
        .unwrap());
    assert!(events
        .iter()
        .any(|event| event.event_type == StdEventType::ActionPreviewed));
    assert!(events
        .iter()
        .any(|event| event.event_type == StdEventType::ActionExecuted));
}

#[test]
fn core_test_runner_blocks_explicit_open_app_commands() {
    let temp = tempfile::tempdir().unwrap();
    let command = fake_open_app_command();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let mut action = make_test_action("Open Real App Command");
    action.action_type = ActionType::Command;
    core.register_action(
        RegistryEntry::from_action(action, vec!["runner".to_string()])
            .with_metadata("command", command.as_str()),
    )
    .unwrap();

    let result = core.search("real app", 1).unwrap().remove(0);
    let execution = core.execute_action(result.action.id).unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.message, command);
    assert!(execution
        .output
        .unwrap()
        .get("deferred")
        .and_then(|value| value.as_bool())
        .unwrap());
}

#[test]
fn command_actions_require_desktop_automation_even_when_external_is_allowed() {
    let temp = tempfile::tempdir().unwrap();
    let command = fake_open_app_command();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let mut action = make_test_action("Open Real App Command");
    action.action_type = ActionType::Command;
    core.register_action(
        RegistryEntry::from_action(action, vec!["runner".to_string()])
            .with_metadata("command", command.as_str()),
    )
    .unwrap();

    let result = core.search("real app", 1).unwrap().remove(0);
    let execution = core
        .execute_action_with_external_runner(result.action.id, true)
        .unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.message, command);
    assert!(execution
        .output
        .unwrap()
        .get("deferred")
        .and_then(|value| value.as_bool())
        .unwrap());
}

#[test]
fn test_core_blocks_external_application_launches_by_default() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    register_fixture_app(&core, "/tmp/StdNeverLaunchFixture.app");

    let result = core.search("StdNeverLaunchFixture", 1).unwrap().remove(0);
    let execution = core.execute_action(result.action.id).unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.message, "open /tmp/StdNeverLaunchFixture.app");
    assert!(execution
        .output
        .unwrap()
        .get("deferred")
        .and_then(|value| value.as_bool())
        .unwrap());
}

#[test]
fn cli_external_permission_still_blocks_app_open_without_desktop_opt_in() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    register_fixture_app(&core, "/tmp/StdNeverLaunchFixture.app");

    let result = core.search("StdNeverLaunchFixture", 1).unwrap().remove(0);
    let execution = core
        .execute_action_with_external_runner(result.action.id, true)
        .unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.message, "open /tmp/StdNeverLaunchFixture.app");
}

#[test]
fn launcher_user_app_open_is_blocked_by_test_mode_before_runner() {
    let temp = tempfile::tempdir().unwrap();
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded = Arc::clone(&commands);
    let core = StdCore::with_config_and_command_runner(
        StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        },
        move |program, args| {
            recorded
                .lock()
                .unwrap()
                .push((program.to_string(), args.to_vec()));
            Err(std::io::Error::other("runner must not execute in tests"))
        },
    );
    register_fixture_app(&core, &blocked_real_app_path());

    let result = core.search("StdNeverLaunchFixture", 1).unwrap().remove(0);
    let execution = core
        .execute_action_from_launcher_user(result.action.id)
        .unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        execution.message,
        format!("open {}", blocked_real_app_path())
    );
    assert!(commands.lock().unwrap().is_empty());
}

fn fake_open_app_command() -> String {
    ["op", "en -a StdNeverLaunchFixture"].join("")
}

fn register_fixture_app(core: &StdCore, path: &str) {
    let mut action = make_test_action("Open StdNeverLaunchFixture");
    action.action_type = ActionType::AppLaunch;
    core.register_action(
        RegistryEntry::from_action(action, vec!["app".to_string()]).with_metadata("path", path),
    )
    .unwrap();
}

fn blocked_real_app_path() -> String {
    ["/Appli", "cations/", "SensitiveVault", ".app"].concat()
}
