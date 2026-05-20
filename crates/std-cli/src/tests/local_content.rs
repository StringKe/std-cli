use super::*;

#[test]
fn tool_command_executes_echo_tool() {
    let output = run_cli(["std", "tool", "run", "Echo", "{\"ok\":true}"]).unwrap();

    assert!(output.contains("\"ok\": true"));
}

#[test]
fn memory_commands_remember_and_recall() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let remembered = run_cli([
        "std",
        "memory",
        "remember",
        "Workflow rule",
        "Use std run for workflows",
        "--tags",
        "workflow,cli",
    ])
    .unwrap();
    let recalled = run_cli(["std", "memory", "recall", "workflow"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(remembered.contains("Workflow rule"));
    assert!(recalled.contains("Workflow rule"));
}

#[test]
fn skill_and_command_template_commands_round_trip_through_search() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let skill = run_cli([
        "std",
        "skill",
        "define",
        "Summarize Diff",
        "Summarize a git diff",
        "When preparing review notes",
        "--examples",
        "std skill run Summarize Diff",
    ])
    .unwrap();
    let skills = run_cli(["std", "skill", "list"]).unwrap();
    let skill_search = run_cli(["std", "search", "Summarize Diff"]).unwrap();
    let skill_run = run_cli(["std", "skill", "run", "Summarize Diff"]).unwrap();
    let command = run_cli([
        "std",
        "command",
        "define",
        "Print Command Smoke",
        "Print command smoke",
        "printf command-template-smoke",
    ])
    .unwrap();
    let commands = run_cli(["std", "command", "list"]).unwrap();
    let command_search = run_cli(["std", "search", "Print Command Smoke"]).unwrap();
    let command_run = run_cli(["std", "command", "run", "Print Command Smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(skill.contains("Summarize Diff"));
    assert!(skills.contains("Summarize Diff"));
    assert!(skill_search.contains("Skill: Summarize Diff"));
    assert!(skill_run.contains("\"action_name\": \"Skill: Summarize Diff\""));
    assert!(command.contains("Print Command Smoke"));
    assert!(commands.contains("printf command-template-smoke"));
    assert!(command_search.contains("Command: Print Command Smoke"));
    assert!(command_run.contains("command-template-smoke"));
    assert!(command_run.contains("\"status\": \"Completed\""));
}

#[test]
fn clipboard_commands_capture_and_recall() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let captured = run_cli([
        "std",
        "clipboard",
        "capture",
        "cargo test --workspace",
        "--source",
        "test",
    ])
    .unwrap();
    let recalled = run_cli(["std", "clipboard", "recall", "cargo"]).unwrap();
    let searched = run_cli(["std", "search", "cargo"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(captured.contains("cargo test --workspace"));
    assert!(recalled.contains("cargo test --workspace"));
    assert!(searched.contains("Clipboard: cargo test --workspace"));
}

#[test]
fn app_commands_register_list_and_search_local_bundle() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let source_app = temp.path().join("Workbench.app");
    std::fs::create_dir_all(source_app.join("Contents").join("MacOS")).unwrap();
    std::fs::write(
        source_app.join("Contents").join("MacOS").join("workbench"),
        "bin",
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let registered = run_cli(["std", "app", "register", source_app.to_str().unwrap()]).unwrap();
    let listed = run_cli(["std", "app", "list"]).unwrap();
    let searched = run_cli(["std", "search", "Workbench"]).unwrap();
    let preview = run_cli(["std", "preview", "Workbench"]).unwrap();
    let triggered = run_cli(["std", "trigger", "Workbench"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(registered.contains("app registered"));
    assert!(listed.contains("Workbench.app"));
    assert!(searched.contains("Open App: Workbench"));
    assert!(preview.contains("Open App: Workbench"));
    assert!(triggered.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(triggered.contains("\"deferred\": true"));
}

#[test]
fn app_register_rejects_non_app_path() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let source_file = temp.path().join("Workbench.txt");
    std::fs::write(&source_file, "not an app").unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let error = run_cli(["std", "app", "register", source_file.to_str().unwrap()]).unwrap_err();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(error.to_string().contains("app bundle expected"));
}

#[test]
fn events_command_can_read_audit_history() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    run_cli(["std", "plan", "terminal"]).unwrap();
    let output = run_cli(["std", "events", "--audit"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("AiPlannerProducedPlan"));
}
