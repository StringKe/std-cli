use crate::{
    ExecutionStatus, OrchestrationError, StepResult, StepType, WorkflowExecution, WorkflowStep,
};
use chrono::Utc;
use std_types::ActionId;

pub(crate) struct ResolvedStepAction {
    pub(crate) id: ActionId,
    pub(crate) name: String,
    pub(crate) input_schema: Option<serde_json::Value>,
    pub(crate) output_schema: Option<serde_json::Value>,
}

pub(crate) fn failed_step_result(
    step: &WorkflowStep,
    error: OrchestrationError,
    started_at: chrono::DateTime<chrono::Utc>,
) -> StepResult {
    StepResult {
        step_id: step.id,
        step_name: step.name.clone(),
        status: ExecutionStatus::Failed,
        output: serde_json::json!({
            "error": error.to_string(),
            "step_type": format!("{:?}", step.step_type),
            "parameters": step.parameters,
        }),
        started_at,
        finished_at: Utc::now(),
    }
}

pub(crate) fn step_failure_error(execution: &WorkflowExecution) -> OrchestrationError {
    execution
        .results
        .last()
        .and_then(|result| result.output["error"].as_str())
        .map(classify_step_failure)
        .unwrap_or_else(|| OrchestrationError::StepFailed("workflow failed".to_string()))
}

pub(crate) fn action_schema_tuple(entry: &std_types::RegistryEntry) -> ResolvedStepAction {
    ResolvedStepAction {
        id: entry.action.id,
        name: entry.action.name.clone(),
        input_schema: entry.action.input_schema.clone(),
        output_schema: entry.action.output_schema.clone(),
    }
}

pub(crate) fn summarize_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Bool(value) => value.to_string(),
        serde_json::Value::Number(value) => value.to_string(),
        serde_json::Value::String(value) => value.clone(),
        serde_json::Value::Array(items) => format!("array({})", items.len()),
        serde_json::Value::Object(items) => format!("object({})", items.len()),
    }
}

pub(crate) fn validate_step_parameters(
    step: &WorkflowStep,
    schema: Option<&serde_json::Value>,
) -> Result<(), OrchestrationError> {
    let Some(schema) = schema else {
        return Ok(());
    };
    let validator = jsonschema::validator_for(schema)
        .map_err(|error| OrchestrationError::StepSchemaInvalid(error.to_string()))?;
    let evaluation = validator.evaluate(&step.parameters);
    let errors = evaluation
        .iter_errors()
        .map(|error| format!("{} at {}", error.error, error.instance_location.as_str()))
        .collect::<Vec<_>>();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(OrchestrationError::StepSchemaInvalid(format!(
            "{}: {}",
            step.name,
            errors.join("; ")
        )))
    }
}

fn classify_step_failure(message: &str) -> OrchestrationError {
    if let Some(step_type) = message
        .strip_prefix("Unsupported step type: ")
        .and_then(parse_step_type)
    {
        OrchestrationError::UnsupportedStepType(step_type)
    } else if let Some(detail) = message.strip_prefix("Step parameters failed schema validation: ")
    {
        OrchestrationError::StepSchemaInvalid(detail.to_string())
    } else {
        OrchestrationError::StepFailed(message.to_string())
    }
}

fn parse_step_type(value: &str) -> Option<StepType> {
    match value {
        "Action" => Some(StepType::Action),
        "Condition" => Some(StepType::Condition),
        "Loop" => Some(StepType::Loop),
        "AiSubtask" => Some(StepType::AiSubtask),
        "UserInteraction" => Some(StepType::UserInteraction),
        _ => None,
    }
}
