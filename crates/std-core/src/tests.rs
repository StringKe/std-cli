use super::*;
use chrono::Utc;
use std::fs;
use std_types::{Action, ActionExecutionStatus, ActionType, RegistryEntry, StdEventType};

mod apps;
mod execution;
mod indexing;

fn make_test_action(name: &str) -> Action {
    let mut action = Action::new(
        name,
        "Test action",
        "For testing",
        ActionType::Custom("test".into()),
    );
    action.created_at = Utc::now();
    action.updated_at = Utc::now();
    action
}

#[test]
fn registry_register_and_search_works() {
    let mut reg = ActionRegistry::new();
    let action = make_test_action("Open VS Code");
    let entry = RegistryEntry {
        action,
        tags: vec!["editor".to_string()],
        metadata: Default::default(),
    };

    reg.register(entry.clone()).unwrap();
    assert_eq!(reg.len(), 1);

    let results = reg.search("vs code", 10);
    assert_eq!(results.len(), 1);
    assert!(results[0].action.name.contains("VS Code"));
}

#[test]
fn registry_search_supports_launcher_fuzzy_abbreviations() {
    let mut reg = ActionRegistry::new();
    reg.register(RegistryEntry {
        action: make_test_action("StdFixtureTerminal"),
        tags: vec!["terminal".to_string(), "shell".to_string()],
        metadata: Default::default(),
    })
    .unwrap();
    reg.register(RegistryEntry {
        action: make_test_action("Rebuild Index"),
        tags: vec!["index".to_string()],
        metadata: Default::default(),
    })
    .unwrap();

    let results = reg.search("std term", 10);
    let missed = reg.search("zzzz", 10);

    assert_eq!(results[0].action.name, "StdFixtureTerminal");
    assert!(results[0]
        .matched_fields
        .contains(&"name:fuzzy".to_string()));
    assert!(missed.is_empty());
}

#[test]
fn duplicate_action_is_rejected() {
    let mut reg = ActionRegistry::new();
    let action = make_test_action("Duplicate");
    let entry = RegistryEntry {
        action,
        tags: vec![],
        metadata: Default::default(),
    };

    reg.register(entry.clone()).unwrap();
    assert!(reg.register(entry).is_err());
}

#[test]
fn core_registers_actions_and_events() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let action = make_test_action("Search Memory");

    core.register_action(RegistryEntry::from_action(
        action,
        vec!["memory".to_string()],
    ))
    .unwrap();

    let results = core.search("memory", 10).unwrap();
    assert_eq!(results.len(), 1);

    let events = core.events().unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, StdEventType::RegistryChanged);

    let audit_events = core.read_audit_events().unwrap();
    assert_eq!(audit_events.len(), 1);
}

#[test]
fn core_executes_builtin_tool() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();

    let output = core
        .execute_tool("Echo", serde_json::json!({"value": 1}))
        .unwrap();

    assert_eq!(output, serde_json::json!({"value": 1}));
}

#[test]
fn desktop_automation_is_disabled_in_unit_tests() {
    assert!(!desktop_automation_allowed());
}

#[test]
fn cargo_test_binary_is_forced_into_std_test_mode() {
    assert!(std_test_mode_enabled());
}

#[test]
fn build_env_is_present_but_not_the_runtime_test_mode_gate() {
    assert_eq!(option_env!("STD_TEST_MODE"), Some("1"));
    assert_eq!(option_env!("STD_ALLOW_DESKTOP_AUTOMATION"), Some("0"));
    assert_eq!(option_env!("STD_ALLOW_UI_PREVIEW"), Some("0"));
}

#[test]
fn core_executes_clipboard_action_locally() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.capture_clipboard("cargo test --workspace", "test")
        .unwrap();

    let result = core.search("cargo test", 1).unwrap().remove(0);
    let preview = core.preview_action(result.action.id).unwrap();
    let execution = core.execute_action(result.action.id).unwrap();

    assert_eq!(preview.primary_command, "cargo test --workspace");
    assert_eq!(execution.status, ActionExecutionStatus::Completed);
    assert!(execution.message.contains("cargo test"));
}

#[test]
fn core_remembers_and_recalls_memory() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });

    core.remember(
        "project",
        "Workflow storage",
        "Workflow definitions live under workflows",
        vec!["workflow".to_string()],
    )
    .unwrap();
    let memories = core.recall("workflow", 10).unwrap();

    assert_eq!(memories.len(), 1);
    assert_eq!(memories[0].title, "Workflow storage");
}

#[test]
fn core_registers_memory_and_clipboard_in_search() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });

    core.remember(
        "project",
        "Workflow storage",
        "Workflow definitions live under workflows",
        vec!["workflow".to_string()],
    )
    .unwrap();
    core.capture_clipboard("cargo test --workspace", "test")
        .unwrap();
    core.register_local_content_actions().unwrap();

    let memory_results = core.search("Workflow storage", 10).unwrap();
    let clipboard_results = core.search("cargo test", 10).unwrap();

    assert!(memory_results
        .iter()
        .any(|result| result.action.name.contains("Memory: Workflow storage")));
    assert!(clipboard_results
        .iter()
        .any(|result| result.action.name.contains("Clipboard: cargo test")));
}

#[test]
fn core_defines_skills_and_command_templates_as_actions() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();

    let skill = core
        .define_skill(
            "Summarize Diff",
            "Summarize a git diff",
            "When preparing review notes",
            vec!["std skill run Summarize Diff".to_string()],
        )
        .unwrap();
    let command = core
        .define_command(
            "Print Skill Smoke",
            "Print command smoke",
            "printf skill-command-smoke",
            vec![],
        )
        .unwrap();
    let skill_preview = core
        .preview_action(core.search("Summarize Diff", 1).unwrap()[0].action.id)
        .unwrap();
    let command_result = core.search("Print Skill Smoke", 1).unwrap().remove(0);
    let command_execution = core.execute_action(command_result.action.id).unwrap();

    assert_eq!(skill.name, "Summarize Diff");
    assert_eq!(command.template, "printf skill-command-smoke");
    assert_eq!(skill_preview.action_type, ActionType::Skill);
    assert_eq!(
        command_execution.status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert!(command_execution.message.contains("skill-command-smoke"));
    assert_eq!(core.list_skills().unwrap().len(), 1);
    assert_eq!(core.list_commands().unwrap().len(), 1);
}

#[test]
fn core_blocks_shell_plugin_tool_in_test_mode() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let plugin_dir = config.plugins_dir().join("smoke");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "smoke",
            "description": "Smoke plugin",
            "permissions": ["shell"],
            "actions": [{
                "name": "Plugin Smoke",
                "description": "Run plugin smoke",
                "when_to_use": "When validating plugin action discovery",
                "kind": "shell",
                "command": "printf plugin-smoke",
                "tags": ["plugin-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    let core = StdCore::with_config(config);
    core.seed_builtin_actions().unwrap();

    let result = core.search("plugin-smoke", 1).unwrap().remove(0);
    let execution = core.execute_action(result.action.id).unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::Failed);
    assert!(execution
        .message
        .contains("STD_TEST_MODE blocked shell plugin command"));
}

#[test]
fn core_blocks_failing_shell_plugin_before_process_spawn_in_test_mode() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let plugin_dir = config.plugins_dir().join("fail");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "fail",
            "description": "Fail plugin",
            "permissions": ["shell"],
            "actions": [{
                "name": "Plugin Fail",
                "description": "Run failing plugin",
                "when_to_use": "When validating plugin failure handling",
                "kind": "shell",
                "command": "printf plugin-fail >&2; exit 7",
                "tags": ["plugin-fail"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    let core = StdCore::with_config(config);
    core.seed_builtin_actions().unwrap();

    let result = core.search("plugin-fail", 1).unwrap().remove(0);
    let execution = core.execute_action(result.action.id).unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::Failed);
    assert!(execution
        .message
        .contains("STD_TEST_MODE blocked shell plugin command"));
}
