use super::test_studio;
use std_orchestration::{ExecutionStatus, WorkflowExecution};
use uuid::Uuid;

#[test]
fn studio_cancels_running_workflow_and_persists_cancel_trace() {
    let mut studio = test_studio();
    studio.last_workflow_execution = Some(WorkflowExecution {
        workflow_id: Uuid::new_v4(),
        workflow_name: "Cancellable Workflow".to_string(),
        status: ExecutionStatus::Running,
        current_step: 0,
        started_at: chrono::Utc::now(),
        finished_at: None,
        results: vec![],
    });

    let execution = studio.cancel_last_workflow_execution().unwrap().clone();
    let history = studio.recent_workflow_executions(5).unwrap();

    assert_eq!(execution.status, ExecutionStatus::Cancelled);
    assert!(execution.finished_at.is_some());
    assert_eq!(history[0].workflow_id, execution.workflow_id);
    assert_eq!(history[0].status, ExecutionStatus::Cancelled);
}

#[test]
fn studio_rejects_cancel_when_workflow_is_not_running() {
    let mut studio = test_studio();
    studio.last_workflow_execution = Some(WorkflowExecution {
        workflow_id: Uuid::new_v4(),
        workflow_name: "Completed Workflow".to_string(),
        status: ExecutionStatus::Completed,
        current_step: 0,
        started_at: chrono::Utc::now(),
        finished_at: Some(chrono::Utc::now()),
        results: vec![],
    });

    let error = studio.cancel_last_workflow_execution().unwrap_err();

    assert_eq!(
        error.to_string(),
        "Invalid workflow document: workflow is not running"
    );
}
