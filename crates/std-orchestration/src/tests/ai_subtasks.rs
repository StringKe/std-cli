use super::*;
use std_core::StdCore;

fn ai_subtask_step(goal: &str) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: "Plan subtask".to_string(),
        action_id: None,
        step_type: StepType::AiSubtask,
        parameters: serde_json::json!({ "goal": goal }),
    }
}

fn workflow_with_step(step: WorkflowStep) -> Workflow {
    Workflow {
        id: Uuid::new_v4(),
        name: "AI Subtask Workflow".to_string(),
        description: "Plans a local AI subtask".to_string(),
        steps: vec![step],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

#[test]
fn ai_subtask_step_produces_local_plan_and_workflow() {
    let core = StdCore::new();
    core.seed_builtin_actions().unwrap();
    let wf = workflow_with_step(ai_subtask_step("terminal"));

    let executor = WorkflowExecutor::new(core);
    let preview = executor.dry_run(&wf).unwrap();
    let execution = executor.execute(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Completed);
    assert_eq!(preview.steps[0].message, "ai subtask validated: terminal");
    assert_eq!(execution.status, ExecutionStatus::Completed);
    assert_eq!(
        execution.results[0].output["step_type"].as_str(),
        Some("ai_subtask")
    );
    assert_eq!(
        execution.results[0].output["plan"]["goal"].as_str(),
        Some("terminal")
    );
    assert_eq!(
        execution.results[0].output["plan"]["steps"][0]["action_name"].as_str(),
        Some("StdFixtureTerminal")
    );
    assert_eq!(
        execution.results[0].output["workflow"]["steps"][0]["step_type"].as_str(),
        Some("Action")
    );
}

#[test]
fn dry_run_rejects_ai_subtask_without_goal() {
    let wf = workflow_with_step(WorkflowStep {
        id: Uuid::new_v4(),
        name: "Plan subtask".to_string(),
        action_id: None,
        step_type: StepType::AiSubtask,
        parameters: serde_json::json!({}),
    });

    let preview = WorkflowExecutor::default().dry_run(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Failed);
    assert!(preview.steps[0].message.contains("ai_subtask missing goal"));
}
