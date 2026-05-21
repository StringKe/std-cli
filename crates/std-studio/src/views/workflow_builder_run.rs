use std_orchestration::{ExecutionStatus, WorkflowExecution};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WorkflowRunControl {
    pub(crate) can_cancel: bool,
    pub(crate) status: &'static str,
}

impl WorkflowRunControl {
    pub(crate) fn from_execution(execution: Option<&WorkflowExecution>) -> Self {
        let status = execution
            .map(|execution| execution_status_label(&execution.status))
            .unwrap_or("idle");
        Self {
            can_cancel: execution
                .map(|execution| execution.status == ExecutionStatus::Running)
                .unwrap_or(false),
            status,
        }
    }
}

fn execution_status_label(status: &ExecutionStatus) -> &'static str {
    match status {
        ExecutionStatus::Pending => "pending",
        ExecutionStatus::Running => "running",
        ExecutionStatus::Completed => "completed",
        ExecutionStatus::Failed => "failed",
        ExecutionStatus::Cancelled => "cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn run_control_only_allows_cancel_during_running_execution() {
        let running = execution_with_status(ExecutionStatus::Running);
        let completed = execution_with_status(ExecutionStatus::Completed);

        assert!(WorkflowRunControl::from_execution(Some(&running)).can_cancel);
        assert!(!WorkflowRunControl::from_execution(Some(&completed)).can_cancel);
        assert!(!WorkflowRunControl::from_execution(None).can_cancel);
    }

    #[test]
    fn run_control_contract_exposes_status_for_toolbar_and_trace() {
        let running = execution_with_status(ExecutionStatus::Running);

        assert_eq!(
            run_control_contract(WorkflowRunControl::from_execution(Some(&running))),
            "run_control_status=running,can_cancel=true"
        );
        assert_eq!(
            run_control_contract(WorkflowRunControl::from_execution(None)),
            "run_control_status=idle,can_cancel=false"
        );
    }

    fn run_control_contract(control: WorkflowRunControl) -> String {
        format!(
            "run_control_status={},can_cancel={}",
            control.status, control.can_cancel
        )
    }

    fn execution_with_status(status: ExecutionStatus) -> WorkflowExecution {
        WorkflowExecution {
            workflow_id: Uuid::new_v4(),
            workflow_name: "run control".to_string(),
            status,
            current_step: 0,
            started_at: Utc::now(),
            finished_at: None,
            results: Vec::new(),
        }
    }
}
