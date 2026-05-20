use super::*;

#[test]
fn workflow_step_add_persists_and_run_uses_step() {
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

    run_cli([
        "std",
        "workflow",
        "new",
        "Release Notes",
        "--description",
        "Draft release notes",
    ])
    .unwrap();
    let added = run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "release-notes",
        "Collect commits",
        "--json",
        "{\"command\":\"git log --oneline\"}",
    ])
    .unwrap();
    let checked = run_cli(["std", "workflow", "check", "release-notes"]).unwrap();
    let executed = run_cli(["std", "run", "release-notes"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(added.contains("Collect commits"));
    assert!(checked.contains("\"step_name\": \"Collect commits\""));
    assert!(executed.contains("\"step_name\": \"Collect commits\""));
    assert!(executed.contains("git log --oneline"));
}

#[test]
fn workflow_step_update_move_remove_round_trip() {
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

    run_cli([
        "std",
        "workflow",
        "new",
        "Release Notes",
        "--description",
        "Draft release notes",
    ])
    .unwrap();
    run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "release-notes",
        "Collect commits",
    ])
    .unwrap();
    run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "release-notes",
        "Draft notes",
    ])
    .unwrap();
    let updated = run_cli([
        "std",
        "workflow",
        "step",
        "update",
        "release-notes",
        "0",
        "--name",
        "Collect merged commits",
        "--json",
        "{\"command\":\"git log --merges\"}",
    ])
    .unwrap();
    let moved = run_cli(["std", "workflow", "step", "move", "release-notes", "1", "0"]).unwrap();
    let removed = run_cli(["std", "workflow", "step", "remove", "release-notes", "1"]).unwrap();
    let checked = run_cli(["std", "workflow", "check", "release-notes"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(updated.contains("Collect merged commits"));
    assert!(moved.contains("Draft notes"));
    assert!(removed.contains("Collect merged commits"));
    assert!(checked.contains("\"step_name\": \"Draft notes\""));
    assert!(!checked.contains("Collect merged commits"));
}

#[test]
fn workflow_step_add_supports_condition_type() {
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

    run_cli([
        "std",
        "workflow",
        "new",
        "Guarded Release",
        "--description",
        "Run only after context is ready",
    ])
    .unwrap();
    run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "guarded-release",
        "Collect context",
        "--json",
        "{\"status\":\"ready\"}",
    ])
    .unwrap();
    let added = run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "guarded-release",
        "Require ready",
        "--type",
        "condition",
        "--json",
        "{\"operator\":\"equals\",\"left\":{\"previous\":true,\"path\":\"/output/parameters/status\"},\"right\":\"ready\"}",
    ])
    .unwrap();
    let checked = run_cli(["std", "workflow", "check", "guarded-release"]).unwrap();
    let executed = run_cli(["std", "run", "guarded-release"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(added.contains("\"step_type\": \"Condition\""));
    assert!(checked.contains("condition validated: equals"));
    assert!(executed.contains("\"operator\": \"equals\""));
    assert!(executed.contains("\"matched\": true"));
}

#[test]
fn workflow_step_add_supports_loop_type() {
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

    run_cli([
        "std",
        "workflow",
        "new",
        "Repeat Context",
        "--description",
        "Collect context more than once",
    ])
    .unwrap();
    let added = run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "repeat-context",
        "Repeat collect",
        "--type",
        "loop",
        "--json",
        "{\"count\":2,\"steps\":[{\"id\":\"00000000-0000-0000-0000-000000000001\",\"name\":\"Collect\",\"action_id\":null,\"step_type\":\"Action\",\"parameters\":{\"kind\":\"context\"}}]}",
    ])
    .unwrap();
    let checked = run_cli(["std", "workflow", "check", "repeat-context"]).unwrap();
    let executed = run_cli(["std", "run", "repeat-context"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(added.contains("\"step_type\": \"Loop\""));
    assert!(checked.contains("loop validated: count=2, steps=1"));
    assert!(executed.contains("\"step_type\": \"loop\""));
    assert!(executed.contains("\"iterations\""));
}

#[test]
fn workflow_step_add_supports_ai_subtask_type() {
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

    run_cli([
        "std",
        "workflow",
        "new",
        "Plan Terminal",
        "--description",
        "Plan terminal automation",
    ])
    .unwrap();
    let added = run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "plan-terminal",
        "Plan terminal subtask",
        "--type",
        "ai_subtask",
        "--json",
        "{\"goal\":\"terminal\"}",
    ])
    .unwrap();
    let checked = run_cli(["std", "workflow", "check", "plan-terminal"]).unwrap();
    let executed = run_cli(["std", "run", "plan-terminal"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(added.contains("\"step_type\": \"AiSubtask\""));
    assert!(checked.contains("ai subtask validated: terminal"));
    assert!(executed.contains("\"step_type\": \"ai_subtask\""));
    assert!(executed.contains("\"goal\": \"terminal\""));
}

#[test]
fn workflow_step_add_supports_user_interaction_type() {
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

    run_cli([
        "std",
        "workflow",
        "new",
        "Confirm Release",
        "--description",
        "Capture release confirmation",
    ])
    .unwrap();
    let added = run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "confirm-release",
        "Confirm release",
        "--type",
        "user_interaction",
        "--json",
        "{\"prompt\":\"Continue release?\",\"choices\":[\"yes\",\"no\"],\"response\":\"yes\",\"required\":true}",
    ])
    .unwrap();
    let checked = run_cli(["std", "workflow", "check", "confirm-release"]).unwrap();
    let executed = run_cli(["std", "run", "confirm-release"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(added.contains("\"step_type\": \"UserInteraction\""));
    assert!(checked.contains("user interaction validated: Continue release?"));
    assert!(executed.contains("\"step_type\": \"user_interaction\""));
    assert!(executed.contains("\"response\": \"yes\""));
}
