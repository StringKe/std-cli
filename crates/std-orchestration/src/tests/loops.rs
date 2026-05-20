use super::*;

fn inline_step(name: &str, parameters: serde_json::Value) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: name.to_string(),
        action_id: None,
        step_type: StepType::Action,
        parameters,
    }
}

fn loop_step(name: &str, count: usize, steps: Vec<WorkflowStep>) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: name.to_string(),
        action_id: None,
        step_type: StepType::Loop,
        parameters: serde_json::json!({
            "count": count,
            "steps": steps,
        }),
    }
}

fn workflow_with_steps(steps: Vec<WorkflowStep>) -> Workflow {
    Workflow {
        id: Uuid::new_v4(),
        name: "Loop Workflow".to_string(),
        description: "Evaluates declarative loops".to_string(),
        steps,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn loop_step_executes_body_for_each_iteration() {
    let wf = workflow_with_steps(vec![loop_step(
        "Repeat collect",
        3,
        vec![inline_step(
            "Collect",
            serde_json::json!({"kind": "context"}),
        )],
    )]);

    let executor = WorkflowExecutor::default();
    let preview = executor.dry_run(&wf).unwrap();
    let execution = executor.execute(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Completed);
    assert_eq!(preview.steps[0].message, "loop validated: count=3, steps=1");
    assert_eq!(execution.status, ExecutionStatus::Completed);
    assert_eq!(execution.results.len(), 1);
    assert_eq!(execution.results[0].output["count"].as_u64(), Some(3));
    assert_eq!(
        execution.results[0].output["iterations"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        execution.results[0].output["iterations"][2]["results"][0]["step_name"].as_str(),
        Some("Collect")
    );
}

#[test]
fn loop_step_can_use_condition_inside_body() {
    let condition = WorkflowStep {
        id: Uuid::new_v4(),
        name: "Require ready".to_string(),
        action_id: None,
        step_type: StepType::Condition,
        parameters: serde_json::json!({
            "operator": "equals",
            "left": {
                "previous": true,
                "path": "/output/parameters/status"
            },
            "right": "ready"
        }),
    };
    let wf = workflow_with_steps(vec![loop_step(
        "Repeat guarded collect",
        2,
        vec![
            inline_step("Collect", serde_json::json!({"status": "ready"})),
            condition,
        ],
    )]);

    let execution = WorkflowExecutor::default().execute(&wf).unwrap();

    assert_eq!(execution.status, ExecutionStatus::Completed);
    assert_eq!(
        execution.results[0].output["iterations"][1]["results"][1]["output"]["matched"].as_bool(),
        Some(true)
    );
}

#[test]
fn dry_run_rejects_invalid_loop_count() {
    let wf = workflow_with_steps(vec![loop_step(
        "Bad loop",
        0,
        vec![inline_step("Collect", serde_json::json!({}))],
    )]);

    let preview = WorkflowExecutor::default().dry_run(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Failed);
    assert!(preview.steps[0]
        .message
        .contains("loop count must be between 1 and 100"));
}
