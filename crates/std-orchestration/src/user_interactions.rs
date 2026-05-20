use crate::{ExecutionStatus, OrchestrationError, StepDryRun, StepType, WorkflowStep};

pub(crate) fn execute_user_interaction_step(
    step: &WorkflowStep,
) -> Result<serde_json::Value, OrchestrationError> {
    let definition = UserInteractionDefinition::parse(step)?;
    if definition.required && definition.response.is_none() {
        return Err(OrchestrationError::StepFailed(format!(
            "user interaction requires response: {}",
            step.name
        )));
    }
    Ok(serde_json::json!({
        "step_type": "user_interaction",
        "prompt": definition.prompt,
        "choices": definition.choices,
        "response": definition.response,
        "required": definition.required,
    }))
}

pub(crate) fn preview_user_interaction_step(
    step: &WorkflowStep,
    parameter_summary: String,
) -> StepDryRun {
    let message = match UserInteractionDefinition::parse(step) {
        Ok(definition) => format!("user interaction validated: {}", definition.prompt),
        Err(error) => error.to_string(),
    };
    let status = if message.starts_with("user interaction validated") {
        ExecutionStatus::Completed
    } else {
        ExecutionStatus::Failed
    };
    StepDryRun {
        step_id: step.id,
        step_name: step.name.clone(),
        step_type: StepType::UserInteraction,
        status,
        action_name: None,
        input_schema: None,
        output_schema: None,
        parameter_summary,
        message,
    }
}

struct UserInteractionDefinition {
    prompt: String,
    choices: Vec<String>,
    response: Option<String>,
    required: bool,
}

impl UserInteractionDefinition {
    fn parse(step: &WorkflowStep) -> Result<Self, OrchestrationError> {
        let object = step.parameters.as_object().ok_or_else(|| {
            OrchestrationError::StepSchemaInvalid(format!(
                "{}: user_interaction parameters must be an object",
                step.name
            ))
        })?;
        let prompt = object
            .get("prompt")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                OrchestrationError::StepSchemaInvalid(format!(
                    "{}: user_interaction missing prompt",
                    step.name
                ))
            })?
            .to_string();
        let choices = parse_choices(object)?;
        let response = object
            .get("response")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string);
        if let Some(response) = &response {
            validate_response(step, response, &choices)?;
        }
        Ok(Self {
            prompt,
            choices,
            response,
            required: object
                .get("required")
                .and_then(|value| value.as_bool())
                .unwrap_or(false),
        })
    }
}

fn parse_choices(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<Vec<String>, OrchestrationError> {
    let Some(raw_choices) = object.get("choices") else {
        return Ok(Vec::new());
    };
    let choices = raw_choices
        .as_array()
        .ok_or_else(|| {
            OrchestrationError::StepSchemaInvalid("choices must be an array".to_string())
        })?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::trim)
                .filter(|choice| !choice.is_empty())
                .map(str::to_string)
                .ok_or_else(|| {
                    OrchestrationError::StepSchemaInvalid(
                        "choices must contain non-empty strings".to_string(),
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(choices)
}

fn validate_response(
    step: &WorkflowStep,
    response: &str,
    choices: &[String],
) -> Result<(), OrchestrationError> {
    if !choices.is_empty() && !choices.iter().any(|choice| choice == response) {
        return Err(OrchestrationError::StepSchemaInvalid(format!(
            "{}: response is not one of the declared choices",
            step.name
        )));
    }
    Ok(())
}
