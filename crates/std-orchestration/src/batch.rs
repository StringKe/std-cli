use crate::{
    append_workflow_execution, load_workflow, resolve_workflow_input, ExecutionStatus,
    OrchestrationError, StepType, Workflow, WorkflowExecution, WorkflowExecutionOptions,
    WorkflowExecutor, WorkflowStep,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std_core::StdCore;
use std_types::{ActionExecution, ActionExecutionStatus};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatchPlan {
    #[serde(default)]
    pub stop_on_error: bool,
    #[serde(default)]
    pub allow_external: bool,
    #[serde(default)]
    pub steps: Vec<BatchStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BatchStep {
    pub name: String,
    #[serde(default = "default_batch_step_kind")]
    pub kind: BatchStepKind,
    pub target: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BatchStepKind {
    #[default]
    Action,
    Workflow,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchReport {
    pub status: ActionExecutionStatus,
    pub steps: Vec<BatchStepReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchStepReport {
    pub name: String,
    pub kind: BatchStepKind,
    pub target: String,
    pub status: ActionExecutionStatus,
    pub execution: Option<ActionExecution>,
    pub error: Option<String>,
}

pub struct BatchExecutor {
    core: StdCore,
}

impl BatchExecutor {
    pub fn new(core: StdCore) -> Self {
        Self { core }
    }

    pub fn execute(&self, plan: &BatchPlan) -> BatchReport {
        let mut reports = Vec::new();
        for step in &plan.steps {
            let result = self.execute_step(step, plan.allow_external);
            let report = match result {
                Ok(execution) => BatchStepReport {
                    name: step.name.clone(),
                    kind: step.kind.clone(),
                    target: step.target.clone(),
                    status: execution.status.clone(),
                    execution: Some(execution),
                    error: None,
                },
                Err(error) => BatchStepReport {
                    name: step.name.clone(),
                    kind: step.kind.clone(),
                    target: step.target.clone(),
                    status: ActionExecutionStatus::Failed,
                    execution: None,
                    error: Some(error.to_string()),
                },
            };
            let should_stop =
                plan.stop_on_error && matches!(report.status, ActionExecutionStatus::Failed);
            reports.push(report);
            if should_stop {
                break;
            }
        }

        BatchReport {
            status: batch_status(&reports),
            steps: reports,
        }
    }

    fn execute_step(
        &self,
        step: &BatchStep,
        allow_external: bool,
    ) -> Result<ActionExecution, OrchestrationError> {
        match step.kind {
            BatchStepKind::Action => execute_batch_action(&self.core, &step.target, allow_external),
            BatchStepKind::Workflow => {
                let execution = execute_batch_workflow(&self.core, &step.target, allow_external)?;
                Ok(workflow_action_execution(&step.target, execution))
            }
        }
    }
}

fn execute_batch_action(
    core: &StdCore,
    query: &str,
    allow_external: bool,
) -> Result<ActionExecution, OrchestrationError> {
    let result = core
        .search(query, 1)?
        .into_iter()
        .next()
        .ok_or_else(|| OrchestrationError::ActionSearchEmpty(query.to_string()))?;
    if !allow_external && result.action.action_type.needs_external_runner() {
        let preview = core.preview_action(result.action.id)?;
        return Ok(ActionExecution {
            action_id: result.action.id,
            action_name: result.action.name,
            status: ActionExecutionStatus::NeedsExternalRunner,
            message: preview.primary_command,
            output: Some(serde_json::json!({
                "deferred": true,
                "reason": "external runner action requires explicit user trigger",
            })),
            created_at: Utc::now(),
        });
    }
    Ok(core.execute_action_with_external_runner(result.action.id, allow_external)?)
}

fn execute_batch_workflow(
    core: &StdCore,
    workflow_name: &str,
    allow_external: bool,
) -> Result<WorkflowExecution, OrchestrationError> {
    core.ensure_storage()?;
    let workflow = match resolve_workflow_input(&core.config, workflow_name) {
        Some(path) => load_workflow(&path)?,
        None => built_in_batch_workflow(core, workflow_name)?,
    };
    let executor = WorkflowExecutor::with_options(
        core.clone(),
        WorkflowExecutionOptions {
            allow_external_runner: allow_external,
        },
    );
    let execution = executor.execute_capture(&workflow)?;
    append_workflow_execution(&core.config.history_dir(), &execution)?;
    Ok(execution)
}

fn workflow_action_execution(workflow_name: &str, execution: WorkflowExecution) -> ActionExecution {
    ActionExecution {
        action_id: execution.workflow_id,
        action_name: format!("Run Workflow: {workflow_name}"),
        status: match execution.status {
            ExecutionStatus::Completed => ActionExecutionStatus::Completed,
            ExecutionStatus::Failed => ActionExecutionStatus::Failed,
            _ => ActionExecutionStatus::NeedsExternalRunner,
        },
        message: format!(
            "workflow executed: {} steps, status {:?}",
            execution.results.len(),
            execution.status
        ),
        output: Some(serde_json::to_value(execution).unwrap_or(serde_json::Value::Null)),
        created_at: Utc::now(),
    }
}

fn built_in_batch_workflow(
    core: &StdCore,
    workflow_name: &str,
) -> Result<Workflow, OrchestrationError> {
    let action = std_types::Action::new(
        "CLI Echo",
        "Echo workflow execution parameters",
        "When validating the terminal Workflow path",
        std_types::ActionType::Command,
    );
    let action_id = action.id;
    core.register_action(std_types::RegistryEntry::from_action(
        action,
        vec!["workflow".to_string(), "cli".to_string()],
    ))?;

    Ok(Workflow {
        id: Uuid::new_v4(),
        name: workflow_name.to_string(),
        description: "Built-in CLI smoke workflow".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "echo".to_string(),
            action_id: Some(action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({
                "workflow": workflow_name,
            }),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}

fn batch_status(reports: &[BatchStepReport]) -> ActionExecutionStatus {
    if reports
        .iter()
        .any(|step| matches!(step.status, ActionExecutionStatus::Failed))
    {
        ActionExecutionStatus::Failed
    } else if reports
        .iter()
        .any(|step| matches!(step.status, ActionExecutionStatus::NeedsExternalRunner))
    {
        ActionExecutionStatus::NeedsExternalRunner
    } else {
        ActionExecutionStatus::Completed
    }
}

fn default_batch_step_kind() -> BatchStepKind {
    BatchStepKind::Action
}
