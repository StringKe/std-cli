use super::*;
use std_orchestration::Workflow;

#[test]
fn plan_command_outputs_registered_action_plan() {
    let output = run_cli(["std", "plan", "terminal"]).unwrap();

    assert!(output.contains("\"goal\": \"terminal\""));
    assert!(output.contains("Open Terminal"));
    assert!(output.contains("action_id"));
    assert!(output.contains("\"evidence\""));
    assert!(output.contains("action: Open Terminal"));
}

#[test]
fn plan_command_outputs_workflow_draft() {
    let output = run_cli(["std", "plan", "terminal", "--workflow"]).unwrap();
    let workflow: Workflow = serde_json::from_str(&output).unwrap();

    assert_eq!(workflow.name, "terminal");
    assert_eq!(workflow.steps[0].name, "Open Terminal");
    assert!(workflow.steps[0].action_id.is_some());
}

#[test]
fn plan_command_saves_workflow_draft_for_later_execution() {
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

    let saved = run_cli(["std", "plan", "terminal", "--save"]).unwrap();
    let searched = run_cli(["std", "search", "Run Workflow: terminal"]).unwrap();
    let run = run_cli(["std", "run", "terminal"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(saved.contains("planned workflow saved"));
    assert!(saved.contains("workflow.json"));
    assert!(searched.contains("Run Workflow: terminal"));
    assert!(run.contains("\"status\": \"Completed\""));
    assert!(run.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(run.contains("\"deferred\": true"));
}
