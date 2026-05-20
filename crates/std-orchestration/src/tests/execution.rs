use super::*;
use std_core::{EventBus, StdCore};

#[test]
fn workflow_can_be_created_and_executed() {
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Test Workflow".to_string(),
        description: "Simple test".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Step 1".to_string(),
            action_id: None,
            step_type: StepType::Action,
            parameters: serde_json::json!({}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let executor = WorkflowExecutor::default();
    let result = executor.execute(&wf).unwrap();

    assert_eq!(result.status, ExecutionStatus::Completed);
    assert_eq!(result.results.len(), 1);
}

#[test]
fn workflow_action_step_executes_registered_action() {
    let core = StdCore::new();
    core.seed_builtin_actions().unwrap();
    let action_id = core.search("index", 1).unwrap()[0].action.id;
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Execute Action".to_string(),
        description: "Runs a registered action".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Rebuild index".to_string(),
            action_id: Some(action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let executor = WorkflowExecutor::new(core);
    let result = executor.execute(&wf).unwrap();

    assert_eq!(result.status, ExecutionStatus::Completed);
    assert_eq!(result.results[0].status, ExecutionStatus::Completed);
    assert_eq!(
        result.results[0].output["action_name"].as_str(),
        Some("Rebuild Index")
    );
    assert_eq!(
        result.results[0].output["status"].as_str(),
        Some("Completed")
    );
}

#[test]
fn workflow_action_step_falls_back_to_name_when_action_id_is_stale() {
    let core = StdCore::new();
    core.seed_builtin_actions().unwrap();
    let stale_action_id = Uuid::new_v4();
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Saved Plan".to_string(),
        description: "Runs a planner workflow saved in another process".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Open Terminal".to_string(),
            action_id: Some(stale_action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let executor = WorkflowExecutor::new(core);
    let preview = executor.dry_run(&wf).unwrap();
    let result = executor.execute(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Completed);
    assert_eq!(
        preview.steps[0].action_name.as_deref(),
        Some("Open Terminal")
    );
    assert_eq!(result.status, ExecutionStatus::Completed);
    assert_eq!(
        result.results[0].output["action_name"].as_str(),
        Some("Open Terminal")
    );
    assert_eq!(
        result.results[0].output["status"].as_str(),
        Some("NeedsExternalRunner")
    );
    assert_eq!(
        result.results[0].output["output"]["deferred"].as_bool(),
        Some(true)
    );
}

#[test]
fn dry_run_exposes_action_input_and_output_schema() {
    let core = StdCore::new();
    let mut action = std_types::Action::new(
        "Generate Report",
        "Generate a structured report",
        "When a report workflow needs typed input",
        std_types::ActionType::Command,
    );
    action.input_schema = Some(serde_json::json!({
        "title": "ReportInput",
        "type": "object",
        "properties": {
            "topic": { "type": "string" }
        }
    }));
    action.output_schema = Some(serde_json::json!({
        "title": "ReportOutput",
        "type": "object",
        "properties": {
            "path": { "type": "string" }
        }
    }));
    let action_id = action.id;
    core.register_action(
        std_types::RegistryEntry::from_action(action, vec!["report".to_string()])
            .with_metadata("command", "printf report"),
    )
    .unwrap();
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Report Workflow".to_string(),
        description: "Uses typed action schemas".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Generate Report".to_string(),
            action_id: Some(action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({"topic": "quality"}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let preview = WorkflowExecutor::new(core).dry_run(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Completed);
    assert_eq!(
        preview.steps[0].input_schema.as_ref().unwrap()["title"],
        "ReportInput"
    );
    assert_eq!(
        preview.steps[0].output_schema.as_ref().unwrap()["title"],
        "ReportOutput"
    );
    assert_eq!(preview.steps[0].parameter_summary, "object(1)");
}

#[test]
fn dry_run_and_execute_reject_schema_invalid_parameters() {
    let core = StdCore::new();
    let mut action = std_types::Action::new(
        "Generate Report",
        "Generate a structured report",
        "When validating schema before execution",
        std_types::ActionType::Command,
    );
    action.input_schema = Some(serde_json::json!({
        "type": "object",
        "required": ["topic"],
        "properties": {
            "topic": { "type": "string" }
        }
    }));
    let action_id = action.id;
    core.register_action(
        std_types::RegistryEntry::from_action(action, vec!["report".to_string()])
            .with_metadata("command", "printf should-not-run"),
    )
    .unwrap();
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Invalid Report Workflow".to_string(),
        description: "Rejects invalid step parameters".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Generate Report".to_string(),
            action_id: Some(action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({"topic": 42}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let executor = WorkflowExecutor::new(core);
    let preview = executor.dry_run(&wf).unwrap();
    let error = executor.execute(&wf).unwrap_err();

    assert_eq!(preview.status, ExecutionStatus::Failed);
    assert_eq!(preview.steps[0].status, ExecutionStatus::Failed);
    assert!(preview.steps[0].message.contains("Generate Report"));
    assert!(preview.steps[0].message.contains("/topic"));
    assert!(matches!(error, OrchestrationError::StepSchemaInvalid(_)));
}

#[test]
fn workflow_external_runner_can_be_explicitly_allowed() {
    let core = StdCore::new();
    let action = std_types::Action::new(
        "Open Missing App",
        "Launch missing app for opt-in test",
        "When testing explicit external workflow execution",
        std_types::ActionType::AppLaunch,
    );
    let action_id = action.id;
    core.register_action(
        std_types::RegistryEntry::from_action(action, vec!["app".to_string()])
            .with_metadata("path", "/tmp/std-cli-missing-app"),
    )
    .unwrap();
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "External Opt In".to_string(),
        description: "Allows external runner execution".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Open Missing App".to_string(),
            action_id: Some(action_id),
            step_type: StepType::Action,
            parameters: serde_json::json!({}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let executor = WorkflowExecutor::with_options(
        core,
        WorkflowExecutionOptions {
            allow_external_runner: true,
        },
    );
    let result = executor.execute(&wf).unwrap();

    assert_eq!(result.status, ExecutionStatus::Completed);
    assert_ne!(
        result.results[0].output["status"].as_str(),
        Some("NeedsExternalRunner")
    );
    assert!(result.results[0].output["output"].get("deferred").is_none());
    assert_eq!(
        result.results[0].output["output"]["command"].as_str(),
        Some("open /tmp/std-cli-missing-app")
    );
}

#[test]
fn execute_capture_returns_failed_execution_with_audit_events() {
    let core = StdCore::new();
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Captured Failure".to_string(),
        description: "Captures unsupported step failure".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Ask user".to_string(),
            action_id: None,
            step_type: StepType::UserInteraction,
            parameters: serde_json::json!({
                "prompt": "Continue?",
                "required": true
            }),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let executor = WorkflowExecutor::new(core.clone());

    let captured = executor.execute_capture(&wf).unwrap();
    let events = core.events().unwrap();

    assert_eq!(captured.status, ExecutionStatus::Failed);
    assert_eq!(captured.results.len(), 1);
    assert_eq!(captured.results[0].status, ExecutionStatus::Failed);
    assert!(captured.results[0].output["error"]
        .as_str()
        .unwrap()
        .contains("requires response"));
    assert!(events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::WorkflowStarted));
    assert!(events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::WorkflowFailed));
}

#[test]
fn dry_run_reports_failed_steps_without_executing() {
    let wf = Workflow {
        id: Uuid::new_v4(),
        name: "Preview Workflow".to_string(),
        description: "Dry run checks every step".to_string(),
        steps: vec![
            WorkflowStep {
                id: Uuid::new_v4(),
                name: "Inline".to_string(),
                action_id: None,
                step_type: StepType::Action,
                parameters: serde_json::json!({"ok": true}),
            },
            WorkflowStep {
                id: Uuid::new_v4(),
                name: "Bad interaction".to_string(),
                action_id: None,
                step_type: StepType::UserInteraction,
                parameters: serde_json::json!({"choices": ["yes"]}),
            },
        ],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let executor = WorkflowExecutor::default();
    let preview = executor.dry_run(&wf).unwrap();

    assert_eq!(preview.status, ExecutionStatus::Failed);
    assert_eq!(preview.steps.len(), 2);
    assert_eq!(preview.steps[0].status, ExecutionStatus::Completed);
    assert_eq!(preview.steps[1].status, ExecutionStatus::Failed);
    assert!(preview.steps[1].message.contains("missing prompt"));
}
