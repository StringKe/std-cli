use super::*;
use std_types::AiPlan;

#[test]
fn ai_plan_can_be_converted_to_executable_workflow() {
    let action_id = Uuid::new_v4();
    let plan = AiPlan {
        goal: "fixture shell".to_string(),
        steps: vec![std_types::PlanStep {
            action_id: Some(action_id),
            action_name: "Open Fixture Shell".to_string(),
            reason: "Matched fields: name".to_string(),
            parameters: serde_json::json!({"source": "planner"}),
            evidence: vec!["action: Open Fixture Shell".to_string()],
        }],
        created_at: Utc::now(),
    };

    let workflow = workflow_from_plan(&plan);

    assert_eq!(workflow.name, "fixture shell");
    assert_eq!(workflow.steps.len(), 1);
    assert_eq!(workflow.steps[0].action_id, Some(action_id));
    assert_eq!(workflow.steps[0].step_type, StepType::Action);
    assert_eq!(
        workflow.steps[0].parameters,
        serde_json::json!({"source": "planner"})
    );
}
