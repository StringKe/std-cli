use crate::{ExecutionStatus, OrchestrationError, StepDryRun, StepType, WorkflowStep};

const MAX_LOOP_COUNT: usize = 100;

pub(crate) struct LoopDefinition {
    pub(crate) count: usize,
    pub(crate) steps: Vec<WorkflowStep>,
}

impl LoopDefinition {
    pub(crate) fn parse(step: &WorkflowStep) -> Result<Self, OrchestrationError> {
        let object = step.parameters.as_object().ok_or_else(|| {
            OrchestrationError::StepSchemaInvalid(format!(
                "{}: loop parameters must be an object",
                step.name
            ))
        })?;
        let count = object
            .get("count")
            .and_then(|value| value.as_u64())
            .ok_or_else(|| {
                OrchestrationError::StepSchemaInvalid(format!("{}: loop missing count", step.name))
            })? as usize;
        if count == 0 || count > MAX_LOOP_COUNT {
            return Err(OrchestrationError::StepSchemaInvalid(format!(
                "{}: loop count must be between 1 and {MAX_LOOP_COUNT}",
                step.name
            )));
        }
        let steps = object
            .get("steps")
            .ok_or_else(|| {
                OrchestrationError::StepSchemaInvalid(format!("{}: loop missing steps", step.name))
            })
            .and_then(|value| {
                serde_json::from_value::<Vec<WorkflowStep>>(value.clone())
                    .map_err(OrchestrationError::Json)
            })?;
        if steps.is_empty() {
            return Err(OrchestrationError::StepSchemaInvalid(format!(
                "{}: loop steps must not be empty",
                step.name
            )));
        }
        Ok(Self { count, steps })
    }
}

pub(crate) fn preview_loop_step(step: &WorkflowStep, parameter_summary: String) -> StepDryRun {
    let message = match LoopDefinition::parse(step) {
        Ok(definition) => format!(
            "loop validated: count={}, steps={}",
            definition.count,
            definition.steps.len()
        ),
        Err(error) => error.to_string(),
    };
    let status = if message.starts_with("loop validated") {
        ExecutionStatus::Completed
    } else {
        ExecutionStatus::Failed
    };
    StepDryRun {
        step_id: step.id,
        step_name: step.name.clone(),
        step_type: StepType::Loop,
        status,
        action_name: None,
        input_schema: None,
        output_schema: None,
        parameter_summary,
        message,
    }
}
