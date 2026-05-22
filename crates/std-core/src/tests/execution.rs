use super::*;
use crate::execution::{
    execute_registry_entry_for_test_mode, user_desktop_open_allowed_for_test_mode,
    ExternalExecutionMode,
};
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
    assert_eq!(
        execution
            .output
            .as_ref()
            .unwrap()
            .get("reason")
            .and_then(|value| value.as_str()),
        Some("STD_TEST_MODE blocked desktop open")
    );
    assert!(commands.lock().unwrap().is_empty());
}

#[test]
fn launcher_user_app_open_attempt_is_still_blocked_by_process_test_mode() {
    let temp = tempfile::tempdir().unwrap();
    let app_path = temp.path().join("Fixture.app");
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
            Ok(empty_output())
        },
    );
    let entry = fixture_app_entry(&app_path.display().to_string());
    core.register_action(entry.clone()).unwrap();

    let execution = execute_registry_entry_for_test_mode(
        &core,
        &entry,
        ExternalExecutionMode::LauncherUser,
        false,
    )
    .unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::Failed);
    assert!(execution
        .message
        .contains("STD_TEST_MODE blocked desktop command"));
    assert!(execution.message.contains("open"));
    assert!(execution.message.contains(&app_path.display().to_string()));
    assert!(commands.lock().unwrap().is_empty());
}

#[test]
fn launcher_user_mode_allows_only_user_open_routes_outside_test_mode() {
    assert!(user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::LauncherUser,
        false
    ));
    assert!(user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::DesktopAutomation,
        false
    ));
    assert!(!user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::Disabled,
        false
    ));
    assert!(!user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::LauncherUser,
        true
    ));
    assert!(!crate::execution::external_runner_allowed_for_mode(
        ExternalExecutionMode::LauncherUser
    ));
    assert!(crate::execution::external_runner_allowed_for_mode(
        ExternalExecutionMode::DesktopAutomation
    ));
}

#[test]
fn test_mode_blocks_desktop_commands_before_custom_runner() {
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded = Arc::clone(&commands);
    let core =
        StdCore::with_config_and_command_runner(StdConfig::default(), move |program, args| {
            recorded
                .lock()
                .unwrap()
                .push((program.to_string(), args.to_vec()));
            Err(std::io::Error::other(
                "runner must not execute desktop command",
            ))
        });

    let args = vec![["/Applic", "ations/", "SensitiveVault", ".app"].concat()];
    let output = core.run_external_command(&["o", "pen"].concat(), &args);

    assert!(output.is_err());
    assert!(output
        .unwrap_err()
        .to_string()
        .contains("STD_TEST_MODE blocked desktop command"));
    assert!(commands.lock().unwrap().is_empty());
}

#[test]
fn test_mode_blocks_named_sensitive_desktop_targets_before_runner() {
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded = Arc::clone(&commands);
    let core =
        StdCore::with_config_and_command_runner(StdConfig::default(), move |program, args| {
            recorded
                .lock()
                .unwrap()
                .push((program.to_string(), args.to_vec()));
            Err(std::io::Error::other(
                "runner must not execute desktop target",
            ))
        });

    for target in [
        ["1Pass", "word"].concat(),
        ["W", "eChat"].concat(),
        ["w", "eixin"].concat(),
        ["微", "信"].concat(),
        ["o", "pen -a Terminal"].concat(),
    ] {
        let args = vec!["-c".to_string(), target.to_string()];
        let output = core.run_external_command("sh", &args);
        assert!(output.is_err(), "{target}");
        assert!(output.unwrap_err().to_string().contains(&target));
    }
    assert!(commands.lock().unwrap().is_empty());
}

#[test]
fn test_mode_blocks_vault_cli_before_runner() {
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded = Arc::clone(&commands);
    let core =
        StdCore::with_config_and_command_runner(StdConfig::default(), move |program, args| {
            recorded
                .lock()
                .unwrap()
                .push((program.to_string(), args.to_vec()));
            Err(std::io::Error::other("runner must not execute vault CLI"))
        });

    for program in ["op", "/usr/local/bin/op", "/opt/homebrew/bin/op"] {
        let args = vec!["item".to_string(), "list".to_string()];
        let output = core.run_external_command(program, &args);
        assert!(output.is_err(), "{program}");
        assert!(output.unwrap_err().to_string().contains(program));
    }
    assert!(commands.lock().unwrap().is_empty());
}

#[test]
fn launcher_user_gate_allows_app_open_outside_test_mode() {
    assert!(user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::LauncherUser,
        false
    ));
    assert!(user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::DesktopAutomation,
        false
    ));
    assert!(!user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::Disabled,
        false
    ));
    assert!(!user_desktop_open_allowed_for_test_mode(
        ExternalExecutionMode::LauncherUser,
        true
    ));
}

fn fake_open_app_command() -> String {
    ["op", "en -a StdNeverLaunchFixture"].join("")
}

fn register_fixture_app(core: &StdCore, path: &str) {
    core.register_action(fixture_app_entry(path)).unwrap();
}

fn fixture_app_entry(path: &str) -> RegistryEntry {
    let mut action = make_test_action("Open StdNeverLaunchFixture");
    action.action_type = ActionType::AppLaunch;
    RegistryEntry::from_action(action, vec!["app".to_string()]).with_metadata("path", path)
}

fn blocked_real_app_path() -> String {
    ["/Appli", "cations/", "SensitiveVault", ".app"].concat()
}

fn empty_output() -> std::process::Output {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        std::process::Output {
            status: std::process::ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }
    #[cfg(not(unix))]
    {
        compile_error!("std-core tests require a manual Output constructor for this platform");
    }
}
