use super::*;

#[test]
fn batch_command_runs_actions_and_workflows_with_deferred_external_steps() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let batch_path = temp.path().join("batch.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &batch_path,
        serde_json::json!({
            "steps": [
                {
                    "name": "rebuild",
                    "kind": "action",
                    "target": "index"
                },
                {
                    "name": "smoke",
                    "kind": "workflow",
                    "target": "smoke"
                },
                {
                    "name": "terminal",
                    "kind": "action",
                    "target": "terminal"
                }
            ]
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "batch", batch_path.to_str().unwrap()]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(output.contains("\"name\": \"rebuild\""));
    assert!(output.contains("\"action_name\": \"Rebuild Index\""));
    assert!(output.contains("\"name\": \"smoke\""));
    assert!(output.contains("\"action_name\": \"Run Workflow: smoke\""));
    assert!(output.contains("\"name\": \"terminal\""));
    assert!(output.contains("\"deferred\": true"));
}

#[test]
fn batch_command_can_stop_on_first_error() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let batch_path = temp.path().join("batch.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &batch_path,
        serde_json::json!({
            "steps": [
                {
                    "name": "missing",
                    "kind": "action",
                    "target": "missing-action"
                },
                {
                    "name": "rebuild",
                    "kind": "action",
                    "target": "index"
                }
            ]
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli([
        "std",
        "batch",
        batch_path.to_str().unwrap(),
        "--stop-on-error",
    ])
    .unwrap();
    let report: serde_json::Value = serde_json::from_str(&output).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert_eq!(report["status"].as_str(), Some("Failed"));
    assert_eq!(report["steps"].as_array().unwrap().len(), 1);
    assert!(report["steps"][0]["error"]
        .as_str()
        .unwrap()
        .contains("No action matched query"));
}
