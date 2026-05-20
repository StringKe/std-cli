use crate::{ExecutionStatus, OrchestrationError, WorkflowExecution};
use std_core::StdCore;
use std_types::{ActionExecutionStatus, StdEvent, StdEventType};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowExecutionTrace {
    pub execution: WorkflowExecution,
    pub steps: Vec<WorkflowTraceStep>,
    pub audit_events: Vec<StdEvent>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct WorkflowTraceStep {
    pub name: String,
    pub status: ExecutionStatus,
    pub action_status: Option<ActionExecutionStatus>,
    pub message: Option<String>,
    pub error: Option<String>,
}

impl WorkflowExecutionTrace {
    pub fn summary(&self) -> String {
        let failed = self
            .steps
            .iter()
            .filter(|step| step.status == ExecutionStatus::Failed)
            .count();
        let deferred = self
            .steps
            .iter()
            .filter(|step| step.action_status == Some(ActionExecutionStatus::NeedsExternalRunner))
            .count();
        format!(
            "{} {:?} steps={} failed={} deferred={} events={}",
            self.execution.workflow_name,
            self.execution.status,
            self.steps.len(),
            failed,
            deferred,
            self.audit_events.len()
        )
    }
}

pub fn recent_workflow_traces(
    core: &StdCore,
    limit: usize,
) -> Result<Vec<WorkflowExecutionTrace>, OrchestrationError> {
    let executions = crate::read_workflow_executions(&core.config.history_dir(), limit)?;
    let events = core.read_audit_events()?;
    Ok(executions
        .into_iter()
        .map(|execution| build_trace(execution, &events))
        .collect())
}

pub fn build_trace(execution: WorkflowExecution, events: &[StdEvent]) -> WorkflowExecutionTrace {
    let workflow_id = execution.workflow_id.to_string();
    let audit_events = events
        .iter()
        .filter(|event| workflow_event_matches(event, &workflow_id))
        .cloned()
        .collect::<Vec<_>>();
    let steps = execution.results.iter().map(trace_step).collect();
    WorkflowExecutionTrace {
        execution,
        steps,
        audit_events,
    }
}

fn workflow_event_matches(event: &StdEvent, workflow_id: &str) -> bool {
    matches!(
        event.event_type,
        StdEventType::WorkflowStarted
            | StdEventType::WorkflowStepCompleted
            | StdEventType::WorkflowCompleted
            | StdEventType::WorkflowFailed
    ) && event.payload["workflow_id"].as_str() == Some(workflow_id)
}

fn trace_step(result: &crate::StepResult) -> WorkflowTraceStep {
    WorkflowTraceStep {
        name: result.step_name.clone(),
        status: result.status.clone(),
        action_status: parse_action_status(&result.output["status"]),
        message: result
            .output
            .get("message")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        error: result
            .output
            .get("error")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
    }
}

fn parse_action_status(value: &serde_json::Value) -> Option<ActionExecutionStatus> {
    match value.as_str()? {
        "Completed" => Some(ActionExecutionStatus::Completed),
        "Failed" => Some(ActionExecutionStatus::Failed),
        "NeedsExternalRunner" => Some(ActionExecutionStatus::NeedsExternalRunner),
        _ => None,
    }
}
