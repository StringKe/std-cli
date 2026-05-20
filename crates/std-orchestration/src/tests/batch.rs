use super::*;
use std_core::StdCore;
use std_types::ActionExecutionStatus;

#[test]
fn batch_executor_runs_actions_and_workflows_with_deferred_external_steps() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(std_core::StdConfig {
        data_dir: temp.path().join("data"),
        ..std_core::StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let plan = BatchPlan {
        stop_on_error: false,
        allow_external: false,
        steps: vec![
            BatchStep {
                name: "rebuild".to_string(),
                kind: BatchStepKind::Action,
                target: "index".to_string(),
            },
            BatchStep {
                name: "smoke".to_string(),
                kind: BatchStepKind::Workflow,
                target: "smoke".to_string(),
            },
            BatchStep {
                name: "terminal".to_string(),
                kind: BatchStepKind::Action,
                target: "terminal".to_string(),
            },
        ],
    };

    let report = BatchExecutor::new(core).execute(&plan);

    assert_eq!(report.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(report.steps.len(), 3);
    assert_eq!(report.steps[0].status, ActionExecutionStatus::Completed);
    assert_eq!(
        report.steps[1]
            .execution
            .as_ref()
            .unwrap()
            .action_name
            .as_str(),
        "Run Workflow: smoke"
    );
    assert_eq!(
        report.steps[2].status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(
        report.steps[2]
            .execution
            .as_ref()
            .unwrap()
            .output
            .as_ref()
            .unwrap()["deferred"]
            .as_bool(),
        Some(true)
    );
}

#[test]
fn batch_executor_stops_on_first_failed_step() {
    let core = StdCore::new();
    core.seed_builtin_actions().unwrap();
    let plan = BatchPlan {
        stop_on_error: true,
        allow_external: false,
        steps: vec![
            BatchStep {
                name: "missing".to_string(),
                kind: BatchStepKind::Action,
                target: "missing-action".to_string(),
            },
            BatchStep {
                name: "rebuild".to_string(),
                kind: BatchStepKind::Action,
                target: "index".to_string(),
            },
        ],
    };

    let report = BatchExecutor::new(core).execute(&plan);

    assert_eq!(report.status, ActionExecutionStatus::Failed);
    assert_eq!(report.steps.len(), 1);
    assert!(report.steps[0]
        .error
        .as_ref()
        .unwrap()
        .contains("No action matched query"));
}
