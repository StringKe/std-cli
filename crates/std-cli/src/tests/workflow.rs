use super::*;
use chrono::Utc;
use std_orchestration::{StepType, Workflow, WorkflowStep};
use uuid::Uuid;

#[test]
fn preview_and_trigger_commands_use_action_dispatch_feedback() {
    let preview = run_cli(["std", "preview", "index"]).unwrap();
    let execution = run_cli(["std", "trigger", "index"]).unwrap();

    assert!(preview.contains("\"title\": \"Rebuild Index\""));
    assert!(preview.contains("\"primary_command\": \"std index rebuild .\""));
    assert!(execution.contains("\"action_name\": \"Rebuild Index\""));
    assert!(execution.contains("\"status\": \"Completed\""));
}

#[test]
fn trigger_command_defers_external_runner_actions_by_default() {
    let output = run_cli(["std", "trigger", "terminal"]).unwrap();

    assert!(output.contains("\"action_name\": \"Open Terminal\""));
    assert!(output.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(output.contains("\"deferred\": true"));
}

#[test]
fn run_command_executes_builtin_workflow() {
    let output = run_cli(["std", "run", "smoke"]).unwrap();

    assert!(output.contains("\"status\": \"Completed\""));
    assert!(output.contains("\"step_name\": \"echo\""));
}

#[test]
fn workflow_check_previews_builtin_workflow() {
    let output = run_cli(["std", "workflow", "check", "smoke"]).unwrap();

    assert!(output.contains("\"workflow_name\": \"smoke\""));
    assert!(output.contains("\"status\": \"Completed\""));
    assert!(output.contains("\"message\": \"action resolved\""));
}

#[test]
fn workflow_new_list_check_and_search_round_trip() {
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

    let created = run_cli([
        "std",
        "workflow",
        "new",
        "Deploy Preview",
        "--description",
        "Create preview deployment",
    ])
    .unwrap();
    let listed = run_cli(["std", "workflow", "list"]).unwrap();
    let checked = run_cli(["std", "workflow", "check", "deploy-preview"]).unwrap();
    let searched = run_cli(["std", "search", "deploy-preview"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(created.contains("workflow created"));
    assert!(created.contains("deploy-preview/workflow.md"));
    assert!(listed.contains("deploy-preview/workflow.md"));
    assert!(checked.contains("\"workflow_name\": \"Deploy Preview\""));
    assert!(searched.contains("Run Workflow: Deploy Preview"));
}

#[test]
fn workflow_run_persists_execution_history() {
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

    run_cli(["std", "run", "smoke"]).unwrap();
    let history = run_cli(["std", "workflow", "history", "--limit", "5"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(history.contains("\"status\": \"Completed\""));
    assert!(history.contains("\"step_name\": \"echo\""));
}

#[test]
fn workflow_trace_reports_steps_and_audit_events() {
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

    run_cli(["std", "run", "smoke"]).unwrap();
    let trace = run_cli(["std", "workflow", "trace", "--limit", "5"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(trace.contains("\"workflow_name\": \"smoke\""));
    assert!(trace.contains("\"steps\""));
    assert!(trace.contains("\"audit_events\""));
    assert!(trace.contains("WorkflowStarted"));
    assert!(trace.contains("WorkflowCompleted"));
}

#[test]
fn trigger_command_executes_saved_workflow_action() {
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
        "Daily Smoke",
        "--description",
        "Run daily smoke",
    ])
    .unwrap();
    run_cli([
        "std",
        "workflow",
        "step",
        "add",
        "daily-smoke",
        "Collect context",
        "--json",
        "{\"kind\":\"context\"}",
    ])
    .unwrap();
    let triggered = run_cli(["std", "trigger", "workflow"]).unwrap();
    let history = run_cli(["std", "workflow", "history", "--limit", "5"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(triggered.contains("\"action_name\": \"Run Workflow: Daily Smoke\""));
    assert!(triggered.contains("\"status\": \"Completed\""));
    assert!(triggered.contains("Collect context"));
    assert!(history.contains("Collect context"));
}

#[test]
fn run_command_executes_json_workflow_file() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("workflow.json");
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: "File Workflow".to_string(),
        description: "Loaded by CLI".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "file-step".to_string(),
            action_id: None,
            step_type: StepType::Action,
            parameters: serde_json::json!({}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    std::fs::write(&path, serde_json::to_string_pretty(&workflow).unwrap()).unwrap();

    let output = run_cli(["std", "run", path.to_str().unwrap()]).unwrap();

    assert!(output.contains("\"step_name\": \"file-step\""));
}

#[test]
fn run_command_persists_failed_workflow_history() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let workflow_path = temp.path().join("bad.workflow.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &workflow_path,
        serde_json::json!({
            "id": Uuid::new_v4(),
            "name": "Bad Workflow",
            "description": "Fails with missing required user response",
            "steps": [{
                "id": Uuid::new_v4(),
                "name": "Confirm release",
                "action_id": null,
                "step_type": "UserInteraction",
                "parameters": {
                    "prompt": "Continue release?",
                    "choices": ["yes", "no"],
                    "required": true
                }
            }],
            "created_at": Utc::now(),
            "updated_at": Utc::now()
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let run = run_cli(["std", "run", workflow_path.to_str().unwrap()]).unwrap();
    let history = run_cli(["std", "workflow", "history"]).unwrap();
    let events = run_cli(["std", "events", "--audit"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(run.contains("\"status\": \"Failed\""));
    assert!(run.contains("requires response"));
    assert!(history.contains("\"status\": \"Failed\""));
    assert!(history.contains("Bad Workflow"));
    assert!(events.contains("WorkflowStarted"));
    assert!(events.contains("WorkflowFailed"));
}
