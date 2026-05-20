use crate::{
    planning::workflow_from_plan, ExecutionStatus, OrchestrationError, StepDryRun, StepType,
    WorkflowStep,
};
use std_core::{AiPlanner, StdCore};

pub(crate) fn execute_ai_subtask_step(
    core: &StdCore,
    step: &WorkflowStep,
) -> Result<serde_json::Value, OrchestrationError> {
    let definition = AiSubtaskDefinition::parse(step)?;
    let plan = AiPlanner::plan(core, &definition.goal)?;
    let workflow = workflow_from_plan(&plan);
    Ok(serde_json::json!({
        "step_type": "ai_subtask",
        "goal": definition.goal,
        "plan": plan,
        "workflow": workflow,
    }))
}

pub(crate) fn preview_ai_subtask_step(
    step: &WorkflowStep,
    parameter_summary: String,
) -> StepDryRun {
    let message = match AiSubtaskDefinition::parse(step) {
        Ok(definition) => format!("ai subtask validated: {}", definition.goal),
        Err(error) => error.to_string(),
    };
    let status = if message.starts_with("ai subtask validated") {
        ExecutionStatus::Completed
    } else {
        ExecutionStatus::Failed
    };
    StepDryRun {
        step_id: step.id,
        step_name: step.name.clone(),
        step_type: StepType::AiSubtask,
        status,
        action_name: None,
        input_schema: None,
        output_schema: None,
        parameter_summary,
        message,
    }
}

struct AiSubtaskDefinition {
    goal: String,
}

impl AiSubtaskDefinition {
    fn parse(step: &WorkflowStep) -> Result<Self, OrchestrationError> {
        let object = step.parameters.as_object().ok_or_else(|| {
            OrchestrationError::StepSchemaInvalid(format!(
                "{}: ai_subtask parameters must be an object",
                step.name
            ))
        })?;
        let goal = object
            .get("goal")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|goal| !goal.is_empty())
            .ok_or_else(|| {
                OrchestrationError::StepSchemaInvalid(format!(
                    "{}: ai_subtask missing goal",
                    step.name
                ))
            })?
            .to_string();
        Ok(Self { goal })
    }
}
