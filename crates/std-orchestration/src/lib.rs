//! std-orchestration - Workflow execution engine.
//!
//! Workflow is the highest productivity abstraction in std-cli.
//! This crate provides the state machine and execution logic.

mod ai_subtasks;
mod batch;
mod conditions;
mod executor;
mod executor_support;
mod loops;
mod planning;
mod trace;
mod user_interactions;
mod workflow_store;

pub use batch::{BatchExecutor, BatchPlan, BatchReport, BatchStep, BatchStepKind, BatchStepReport};
pub use executor::{WorkflowExecutionOptions, WorkflowExecutor};
pub use planning::workflow_from_plan;
pub use trace::{build_trace, recent_workflow_traces, WorkflowExecutionTrace, WorkflowTraceStep};
pub use workflow_store::{
    append_workflow_execution, format_workflow_markdown, list_workflows, load_workflow,
    read_workflow_executions, resolve_workflow_input, workflow_history_path, write_workflow,
    write_workflow_markdown,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std_types::ActionId;
use thiserror::Error;
use uuid::Uuid;

pub type WorkflowId = Uuid;

#[derive(Error, Debug)]
pub enum OrchestrationError {
    #[error("Workflow not found")]
    WorkflowNotFound,
    #[error("No action matched query: {0}")]
    ActionSearchEmpty(String),
    #[error("Step execution failed: {0}")]
    StepFailed(String),
    #[error("Unsupported step type: {0:?}")]
    UnsupportedStepType(StepType),
    #[error("Step parameters failed schema validation: {0}")]
    StepSchemaInvalid(String),
    #[error("Core error: {0}")]
    Core(#[from] std_core::CoreError),
    #[error("Workflow file error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Workflow JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid workflow document: {0}")]
    InvalidWorkflow(String),
}

/// A single step inside a Workflow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub id: Uuid,
    pub name: String,
    pub action_id: Option<ActionId>,
    pub step_type: StepType,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepType {
    Action,
    Condition,
    Loop,
    AiSubtask,
    UserInteraction,
}

/// The main Workflow definition (declarative style)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: WorkflowId,
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Workflow {
    pub fn simple(name: impl Into<String>, description: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }
}

pub fn add_workflow_step(
    workflow_path: &Path,
    name: &str,
    step_type: StepType,
    parameters: serde_json::Value,
) -> Result<WorkflowStep, OrchestrationError> {
    let path = workflow_store::resolve_workflow_path(workflow_path);
    let mut workflow = load_workflow(&path)?;
    let step = WorkflowStep {
        id: Uuid::new_v4(),
        name: name.to_string(),
        action_id: None,
        step_type,
        parameters,
    };
    workflow.steps.push(step.clone());
    workflow_store::update_workflow_timestamp(&mut workflow);
    workflow_store::write_workflow(&path, &workflow)?;
    Ok(step)
}

pub fn update_workflow_step(
    workflow_path: &Path,
    step_index: usize,
    name: Option<&str>,
    parameters: Option<serde_json::Value>,
) -> Result<WorkflowStep, OrchestrationError> {
    let path = workflow_store::resolve_workflow_path(workflow_path);
    let mut workflow = load_workflow(&path)?;
    let step = workflow
        .steps
        .get_mut(step_index)
        .ok_or_else(|| workflow_store::invalid_step_index(step_index))?;
    if let Some(name) = name {
        step.name = name.to_string();
    }
    if let Some(parameters) = parameters {
        step.parameters = parameters;
    }
    let updated = step.clone();
    workflow_store::update_workflow_timestamp(&mut workflow);
    workflow_store::write_workflow(&path, &workflow)?;
    Ok(updated)
}

pub fn remove_workflow_step(
    workflow_path: &Path,
    step_index: usize,
) -> Result<WorkflowStep, OrchestrationError> {
    let path = workflow_store::resolve_workflow_path(workflow_path);
    let mut workflow = load_workflow(&path)?;
    if step_index >= workflow.steps.len() {
        return Err(workflow_store::invalid_step_index(step_index));
    }
    let removed = workflow.steps.remove(step_index);
    workflow_store::update_workflow_timestamp(&mut workflow);
    workflow_store::write_workflow(&path, &workflow)?;
    Ok(removed)
}

pub fn move_workflow_step(
    workflow_path: &Path,
    from_index: usize,
    to_index: usize,
) -> Result<WorkflowStep, OrchestrationError> {
    let path = workflow_store::resolve_workflow_path(workflow_path);
    let mut workflow = load_workflow(&path)?;
    if from_index >= workflow.steps.len() {
        return Err(workflow_store::invalid_step_index(from_index));
    }
    if to_index >= workflow.steps.len() {
        return Err(workflow_store::invalid_step_index(to_index));
    }
    let step = workflow.steps.remove(from_index);
    workflow.steps.insert(to_index, step.clone());
    workflow_store::update_workflow_timestamp(&mut workflow);
    workflow_store::write_workflow(&path, &workflow)?;
    Ok(step)
}

/// Execution state of a Workflow run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecution {
    pub workflow_id: WorkflowId,
    #[serde(default)]
    pub workflow_name: String,
    pub status: ExecutionStatus,
    pub current_step: usize,
    pub started_at: DateTime<Utc>,
    pub finished_at: Option<DateTime<Utc>>,
    pub results: Vec<StepResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepResult {
    pub step_id: Uuid,
    pub step_name: String,
    pub status: ExecutionStatus,
    pub output: serde_json::Value,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkflowDryRun {
    pub workflow_id: WorkflowId,
    pub workflow_name: String,
    pub status: ExecutionStatus,
    pub checked_at: DateTime<Utc>,
    pub steps: Vec<StepDryRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StepDryRun {
    pub step_id: Uuid,
    pub step_name: String,
    pub step_type: StepType,
    pub status: ExecutionStatus,
    pub action_name: Option<String>,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
    pub parameter_summary: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests;
