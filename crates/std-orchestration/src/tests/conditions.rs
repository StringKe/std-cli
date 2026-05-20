use super::*;
use std_core::{EventBus, StdCore};

fn inline_step(name: &str, parameters: serde_json::Value) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: name.to_string(),
        action_id: None,
        step_type: StepType::Action,
        parameters,
    }
}

fn condition_step(name: &str, parameters: serde_json::Value) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: name.to_string(),
        action_id: None,
        step_type: StepType::Condition,
        parameters,
    }
}

fn workflow_with_steps(steps: Vec<WorkflowStep>) -> Workflow {
    Workflow {
        id: Uuid::new_v4(),
        name: "Condition Workflow".to_string(),
        description: "Evaluates declarative conditions".to_string(),
        steps,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn condition_step_can_assert_previous_step_output() {
    let wf = workflow_with_steps(vec![
        inline_step("Prepare", serde_json::json!({"status": "ready"})),
        condition_step(
            "Require ready",
            serde_json::json!({
                "operator": "equals",
                "left": {
                    "previous": true,
                    "path": "/output/parameters/status"
                },
                "right": "ready"
            }),
        ),
    ]);

    let executor = WorkflowExecutor::default();
    let preview = executor.dry_run(&wf).unwrap();
    let execution = executor.execute(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Completed);
    assert_eq!(preview.steps[1].message, "condition validated: equals");
    assert_eq!(execution.status, ExecutionStatus::Completed);
    assert_eq!(execution.results.len(), 2);
    assert_eq!(
        execution.results[1].output,
        serde_json::json!({
            "step_type": "condition",
            "operator": "equals",
            "matched": true,
            "left": "ready",
            "right": "ready"
        })
    );
}

#[test]
fn condition_step_failure_is_captured_with_audit_event() {
    let core = StdCore::new();
    let wf = workflow_with_steps(vec![
        inline_step("Prepare", serde_json::json!({"status": "blocked"})),
        condition_step(
            "Require ready",
            serde_json::json!({
                "operator": "equals",
                "left": {
                    "previous": true,
                    "path": "/output/parameters/status"
                },
                "right": "ready"
            }),
        ),
    ]);

    let executor = WorkflowExecutor::new(core.clone());
    let execution = executor.execute_capture(&wf).unwrap();
    let events = core.events().unwrap();

    assert_eq!(execution.status, ExecutionStatus::Failed);
    assert_eq!(execution.results.len(), 2);
    assert_eq!(execution.results[1].status, ExecutionStatus::Failed);
    assert!(execution.results[1].output["error"]
        .as_str()
        .unwrap()
        .contains("condition failed"));
    assert!(events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::WorkflowFailed));
}

#[test]
fn dry_run_rejects_invalid_condition_parameters() {
    let wf = workflow_with_steps(vec![condition_step(
        "Invalid condition",
        serde_json::json!({
            "operator": "equals",
            "left": {
                "previous": true,
                "path": "/output/parameters/status"
            }
        }),
    )]);

    let preview = WorkflowExecutor::default().dry_run(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Failed);
    assert_eq!(preview.steps[0].status, ExecutionStatus::Failed);
    assert!(preview.steps[0]
        .message
        .contains("operator equals requires right operand"));
}
