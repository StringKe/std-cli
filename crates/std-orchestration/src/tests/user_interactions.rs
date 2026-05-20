use super::*;

fn user_step(parameters: serde_json::Value) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: "Confirm release".to_string(),
        action_id: None,
        step_type: StepType::UserInteraction,
        parameters,
    }
}

fn workflow_with_step(step: WorkflowStep) -> Workflow {
    Workflow {
        id: Uuid::new_v4(),
        name: "User Interaction Workflow".to_string(),
        description: "Captures deterministic user input".to_string(),
        steps: vec![step],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn user_interaction_step_uses_pre_filled_response() {
    let wf = workflow_with_step(user_step(serde_json::json!({
        "prompt": "Continue release?",
        "choices": ["yes", "no"],
        "response": "yes",
        "required": true
    })));

    let executor = WorkflowExecutor::default();
    let preview = executor.dry_run(&wf).unwrap();
    let execution = executor.execute(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Completed);
    assert_eq!(
        preview.steps[0].message,
        "user interaction validated: Continue release?"
    );
    assert_eq!(execution.status, ExecutionStatus::Completed);
    assert_eq!(
        execution.results[0].output,
        serde_json::json!({
            "step_type": "user_interaction",
            "prompt": "Continue release?",
            "choices": ["yes", "no"],
            "response": "yes",
            "required": true
        })
    );
}

#[test]
fn required_user_interaction_fails_without_response() {
    let wf = workflow_with_step(user_step(serde_json::json!({
        "prompt": "Continue release?",
        "choices": ["yes", "no"],
        "required": true
    })));

    let captured = WorkflowExecutor::default().execute_capture(&wf).unwrap();

    assert_eq!(captured.status, ExecutionStatus::Failed);
    assert!(captured.results[0].output["error"]
        .as_str()
        .unwrap()
        .contains("requires response"));
}

#[test]
fn dry_run_rejects_response_outside_declared_choices() {
    let wf = workflow_with_step(user_step(serde_json::json!({
        "prompt": "Continue release?",
        "choices": ["yes", "no"],
        "response": "later"
    })));

    let preview = WorkflowExecutor::default().dry_run(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Failed);
    assert!(preview.steps[0]
        .message
        .contains("response is not one of the declared choices"));
}
