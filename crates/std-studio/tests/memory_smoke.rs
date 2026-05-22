use std_core::{StdConfig, StdCore};
use std_studio::{StudioApp, StudioPane};

#[test]
fn studio_memory_pane_uses_shared_core_storage() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let core = StdCore::with_config(config);
    core.seed_builtin_actions().unwrap();
    core.remember(
        "cli",
        "CLI memory",
        "Memory written through shared core",
        vec!["cli".to_string()],
    )
    .unwrap();

    let mut studio = StudioApp::with_core(core);
    studio.open_memory_browser_pane();

    let found = studio.search_memory("CLI memory");
    let written = studio
        .remember_from_studio(
            "studio",
            "Studio shared memory",
            "Memory written through Studio pane",
            vec!["studio".to_string()],
        )
        .unwrap();
    let search_results = studio.core.search("Studio shared memory", 10).unwrap();

    assert_eq!(found.len(), 1);
    assert_eq!(found[0].scope, "cli");
    assert_eq!(written.scope, "studio");
    assert!(search_results
        .iter()
        .any(|result| result.action.name.contains("Memory: Studio shared memory")));
}

#[test]
fn studio_workflow_run_writes_shared_execution_history() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut studio = StudioApp::with_core(core);
    studio.open_workspace_pane(StudioPane::Workflows);
    let path = studio
        .create_workflow("Smoke Run", "Run from Studio smoke")
        .unwrap();
    studio
        .add_workflow_step(&path, "Collect smoke", serde_json::json!({"smoke": true}))
        .unwrap();

    let execution = studio.run_workflow_path(&path).unwrap().clone();
    let history = studio.recent_workflow_executions(1).unwrap();

    assert_eq!(
        execution.status,
        std_orchestration::ExecutionStatus::Completed
    );
    assert_eq!(execution.results[0].step_name, "Collect smoke");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].workflow_id, execution.workflow_id);
}
