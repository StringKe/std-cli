use super::*;
use std_orchestration::{StepType, Workflow, WorkflowStep};

#[test]
fn studio_traces_successful_workflow_execution_with_audit_events() {
    let mut studio = test_studio();
    let path = studio
        .create_workflow("Trace Success", "Trace completed workflow")
        .unwrap();
    studio
        .add_workflow_step(&path, "Collect context", serde_json::json!({"ok": true}))
        .unwrap();
    let execution = studio.run_workflow_path(&path).unwrap().clone();

    let traces = studio.recent_workflow_traces(5).unwrap();
    let trace = traces
        .iter()
        .find(|trace| trace.execution.workflow_id == execution.workflow_id)
        .unwrap();

    assert_eq!(
        trace.execution.status,
        std_orchestration::ExecutionStatus::Completed
    );
    assert_eq!(trace.steps.len(), 1);
    assert_eq!(trace.steps[0].name, "Collect context");
    assert_eq!(
        trace.steps[0].status,
        std_orchestration::ExecutionStatus::Completed
    );
    assert!(trace
        .audit_events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::WorkflowStarted));
    assert!(trace
        .audit_events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::WorkflowCompleted));
    assert!(trace.summary().contains("Trace Success Completed steps=1"));
}

#[test]
fn studio_traces_failed_workflow_execution() {
    let mut studio = test_studio();
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: "Trace Failure".to_string(),
        description: "Trace failed workflow".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Needs user".to_string(),
            action_id: None,
            step_type: StepType::UserInteraction,
            parameters: serde_json::json!({"prompt": "approve", "required": true}),
        }],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let workflow_path = studio
        .core
        .config
        .workflows_dir()
        .join("trace-failure")
        .join("workflow.json");
    std::fs::create_dir_all(workflow_path.parent().unwrap()).unwrap();
    std_orchestration::write_workflow(&workflow_path, &workflow).unwrap();

    let execution = studio.run_workflow_path(&workflow_path).unwrap().clone();
    let traces = studio.recent_workflow_traces(5).unwrap();
    let trace = traces
        .iter()
        .find(|trace| trace.execution.workflow_id == execution.workflow_id)
        .unwrap();

    assert_eq!(
        trace.execution.status,
        std_orchestration::ExecutionStatus::Failed
    );
    assert_eq!(
        trace.steps[0].status,
        std_orchestration::ExecutionStatus::Failed
    );
    assert!(trace.steps[0]
        .error
        .as_ref()
        .unwrap()
        .contains("requires response"));
    assert!(trace
        .audit_events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::WorkflowFailed));
    assert!(trace.summary().contains("failed=1"));
}

#[test]
fn studio_traces_deferred_external_runner_steps() {
    let mut studio = test_studio();
    studio.plan_workflow("terminal").unwrap();
    let path = studio.save_planned_workflow().unwrap();
    let execution = studio.run_workflow_path(&path).unwrap().clone();

    let traces = studio.recent_workflow_traces(5).unwrap();
    let trace = traces
        .iter()
        .find(|trace| trace.execution.workflow_id == execution.workflow_id)
        .unwrap();

    assert_eq!(
        trace.execution.status,
        std_orchestration::ExecutionStatus::Completed
    );
    assert_eq!(
        trace.steps[0].action_status,
        Some(std_types::ActionExecutionStatus::NeedsExternalRunner)
    );
    let output = &trace.execution.results[0].output;
    assert_eq!(output["output"]["deferred"].as_bool(), Some(true));
    assert_eq!(
        output["output"]["reason"].as_str(),
        Some("workflow external runner action requires explicit user trigger")
    );
    assert!(trace.summary().contains("deferred=1"));
}
