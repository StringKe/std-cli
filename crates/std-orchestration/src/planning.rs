use crate::{StepType, Workflow, WorkflowStep};
use chrono::Utc;
use std_types::AiPlan;
use uuid::Uuid;

pub fn workflow_from_plan(plan: &AiPlan) -> Workflow {
    let now = Utc::now();
    Workflow {
        id: Uuid::new_v4(),
        name: plan.goal.clone(),
        description: format!("AI planned workflow for {}", plan.goal),
        steps: plan
            .steps
            .iter()
            .map(|step| WorkflowStep {
                id: Uuid::new_v4(),
                name: step.action_name.clone(),
                action_id: step.action_id,
                step_type: StepType::Action,
                parameters: step.parameters.clone(),
            })
            .collect(),
        created_at: now,
        updated_at: now,
    }
}
