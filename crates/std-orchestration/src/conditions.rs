use crate::{ExecutionStatus, OrchestrationError, StepDryRun, StepResult, StepType, WorkflowStep};

pub(crate) fn execute_condition_step(
    step: &WorkflowStep,
    previous_results: &[StepResult],
) -> Result<serde_json::Value, OrchestrationError> {
    let condition = Condition::parse(step)?;
    let result = condition.evaluate(previous_results)?;
    if !result.matched {
        return Err(OrchestrationError::StepFailed(format!(
            "condition failed: {}",
            step.name
        )));
    }
    Ok(result.into_json())
}

pub(crate) fn preview_condition_step(step: &WorkflowStep, parameter_summary: String) -> StepDryRun {
    let message = match Condition::parse(step) {
        Ok(condition) => format!("condition validated: {}", condition.operator.as_str()),
        Err(error) => error.to_string(),
    };
    let status = if message.starts_with("condition validated") {
        ExecutionStatus::Completed
    } else {
        ExecutionStatus::Failed
    };
    StepDryRun {
        step_id: step.id,
        step_name: step.name.clone(),
        step_type: StepType::Condition,
        status,
        action_name: None,
        input_schema: None,
        output_schema: None,
        parameter_summary,
        message,
    }
}

struct Condition {
    operator: ConditionOperator,
    left: Operand,
    right: Option<Operand>,
}

struct ConditionResult {
    matched: bool,
    operator: ConditionOperator,
    left: serde_json::Value,
    right: Option<serde_json::Value>,
}

#[derive(Clone, Copy)]
enum ConditionOperator {
    Equals,
    NotEquals,
    Exists,
    NotExists,
    Contains,
    GreaterThan,
    LessThan,
}

enum Operand {
    Literal(serde_json::Value),
    Previous { path: String },
    Step { index: usize, path: String },
}

impl Condition {
    fn parse(step: &WorkflowStep) -> Result<Self, OrchestrationError> {
        let object = step.parameters.as_object().ok_or_else(|| {
            OrchestrationError::StepSchemaInvalid(format!(
                "{}: condition parameters must be an object",
                step.name
            ))
        })?;
        let operator = object
            .get("operator")
            .and_then(|value| value.as_str())
            .ok_or_else(|| missing_field(step, "operator"))
            .and_then(ConditionOperator::parse)?;
        let left = object
            .get("left")
            .or_else(|| object.get("value"))
            .ok_or_else(|| missing_field(step, "left"))?;
        let right = object.get("right").or_else(|| object.get("equals"));
        let condition = Self {
            operator,
            left: Operand::parse(left)?,
            right: right.map(Operand::parse).transpose()?,
        };
        condition.validate(step)?;
        Ok(condition)
    }

    fn validate(&self, step: &WorkflowStep) -> Result<(), OrchestrationError> {
        if self.operator.needs_right_operand() && self.right.is_none() {
            return Err(OrchestrationError::StepSchemaInvalid(format!(
                "{}: condition operator {} requires right operand",
                step.name,
                self.operator.as_str()
            )));
        }
        Ok(())
    }

    fn evaluate(
        &self,
        previous_results: &[StepResult],
    ) -> Result<ConditionResult, OrchestrationError> {
        let left = self.left.resolve(previous_results)?;
        let right = self
            .right
            .as_ref()
            .map(|operand| operand.resolve(previous_results))
            .transpose()?;
        let matched = match self.operator {
            ConditionOperator::Equals => Some(&left) == right.as_ref(),
            ConditionOperator::NotEquals => Some(&left) != right.as_ref(),
            ConditionOperator::Exists => !left.is_null(),
            ConditionOperator::NotExists => left.is_null(),
            ConditionOperator::Contains => contains_value(&left, right.as_ref()),
            ConditionOperator::GreaterThan => {
                compare_number(&left, right.as_ref(), |left, right| left > right)
            }
            ConditionOperator::LessThan => {
                compare_number(&left, right.as_ref(), |left, right| left < right)
            }
        };
        Ok(ConditionResult {
            matched,
            operator: self.operator,
            left,
            right,
        })
    }
}

impl ConditionOperator {
    fn parse(value: &str) -> Result<Self, OrchestrationError> {
        match value {
            "equals" => Ok(Self::Equals),
            "not_equals" => Ok(Self::NotEquals),
            "exists" => Ok(Self::Exists),
            "not_exists" => Ok(Self::NotExists),
            "contains" => Ok(Self::Contains),
            "greater_than" => Ok(Self::GreaterThan),
            "less_than" => Ok(Self::LessThan),
            _ => Err(OrchestrationError::StepSchemaInvalid(format!(
                "unsupported condition operator: {value}"
            ))),
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Equals => "equals",
            Self::NotEquals => "not_equals",
            Self::Exists => "exists",
            Self::NotExists => "not_exists",
            Self::Contains => "contains",
            Self::GreaterThan => "greater_than",
            Self::LessThan => "less_than",
        }
    }

    fn needs_right_operand(self) -> bool {
        !matches!(self, Self::Exists | Self::NotExists)
    }
}

impl Operand {
    fn parse(value: &serde_json::Value) -> Result<Self, OrchestrationError> {
        let Some(object) = value.as_object() else {
            return Ok(Self::Literal(value.clone()));
        };
        if let Some(literal) = object.get("literal") {
            return Ok(Self::Literal(literal.clone()));
        }
        if object.get("previous").and_then(|value| value.as_bool()) == Some(true) {
            return Ok(Self::Previous {
                path: read_path(object)?,
            });
        }
        if let Some(step) = object.get("step").and_then(|value| value.as_u64()) {
            return Ok(Self::Step {
                index: step as usize,
                path: read_path(object)?,
            });
        }
        Ok(Self::Literal(value.clone()))
    }

    fn resolve(
        &self,
        previous_results: &[StepResult],
    ) -> Result<serde_json::Value, OrchestrationError> {
        match self {
            Self::Literal(value) => Ok(value.clone()),
            Self::Previous { path } => previous_results
                .last()
                .ok_or_else(|| {
                    OrchestrationError::StepFailed("condition has no previous step".to_string())
                })
                .and_then(|result| resolve_result_path(result, path)),
            Self::Step { index, path } => previous_results
                .get(*index)
                .ok_or_else(|| {
                    OrchestrationError::StepFailed(format!(
                        "condition step index out of range: {index}"
                    ))
                })
                .and_then(|result| resolve_result_path(result, path)),
        }
    }
}

impl ConditionResult {
    fn into_json(self) -> serde_json::Value {
        serde_json::json!({
            "step_type": "condition",
            "operator": self.operator.as_str(),
            "matched": self.matched,
            "left": self.left,
            "right": self.right,
        })
    }
}

fn missing_field(step: &WorkflowStep, field: &str) -> OrchestrationError {
    OrchestrationError::StepSchemaInvalid(format!("{}: condition missing {field}", step.name))
}

fn read_path(
    object: &serde_json::Map<String, serde_json::Value>,
) -> Result<String, OrchestrationError> {
    Ok(object
        .get("path")
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .to_string())
}

fn resolve_result_path(
    result: &StepResult,
    path: &str,
) -> Result<serde_json::Value, OrchestrationError> {
    let value = serde_json::to_value(result)?;
    if path.is_empty() {
        return Ok(value);
    }
    Ok(value
        .pointer(path)
        .cloned()
        .unwrap_or(serde_json::Value::Null))
}

fn contains_value(left: &serde_json::Value, right: Option<&serde_json::Value>) -> bool {
    match (left, right) {
        (serde_json::Value::String(left), Some(serde_json::Value::String(right))) => {
            left.contains(right)
        }
        (serde_json::Value::Array(items), Some(right)) => items.iter().any(|item| item == right),
        (serde_json::Value::Object(object), Some(serde_json::Value::String(key))) => {
            object.contains_key(key)
        }
        _ => false,
    }
}

fn compare_number(
    left: &serde_json::Value,
    right: Option<&serde_json::Value>,
    compare: impl FnOnce(f64, f64) -> bool,
) -> bool {
    left.as_f64()
        .zip(right.and_then(|value| value.as_f64()))
        .map(|(left, right)| compare(left, right))
        .unwrap_or(false)
}
