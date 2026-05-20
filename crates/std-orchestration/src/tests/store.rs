use super::*;

#[test]
fn json_workflow_can_be_loaded() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().join("workflow.json");
    let workflow = Workflow {
        id: Uuid::new_v4(),
        name: "Load JSON".to_string(),
        description: "Loaded from disk".to_string(),
        steps: vec![WorkflowStep {
            id: Uuid::new_v4(),
            name: "Step".to_string(),
            action_id: None,
            step_type: StepType::Action,
            parameters: serde_json::json!({"ok": true}),
        }],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    fs::write(&path, serde_json::to_string_pretty(&workflow).unwrap()).unwrap();

    let loaded = load_workflow(&path).unwrap();

    assert_eq!(loaded.name, "Load JSON");
    assert_eq!(loaded.steps.len(), 1);
}

#[test]
fn workflow_markdown_can_be_created_listed_and_loaded() {
    let temp = tempfile::tempdir().unwrap();

    let path = write_workflow_markdown(
        temp.path(),
        "Deploy Preview",
        "Create and verify a preview deployment",
    )
    .unwrap();
    let workflows = list_workflows(temp.path()).unwrap();
    let loaded = load_workflow(&path).unwrap();

    assert!(path.ends_with("deploy-preview/workflow.md"));
    assert_eq!(workflows, vec![path]);
    assert_eq!(loaded.name, "Deploy Preview");
    assert_eq!(loaded.description, "Create and verify a preview deployment");
}

#[test]
fn workflow_markdown_step_can_be_added_and_loaded() {
    let temp = tempfile::tempdir().unwrap();
    let path =
        write_workflow_markdown(temp.path(), "Release Notes", "Draft release notes").unwrap();

    let step = add_workflow_step(
        &path,
        "Collect commits",
        StepType::Action,
        serde_json::json!({"command": "git log --oneline"}),
    )
    .unwrap();
    let loaded = load_workflow(&path).unwrap();

    assert_eq!(loaded.steps.len(), 1);
    assert_eq!(loaded.steps[0].id, step.id);
    assert_eq!(loaded.steps[0].name, "Collect commits");
    assert_eq!(loaded.steps[0].parameters["command"], "git log --oneline");
}

#[test]
fn workflow_markdown_steps_can_be_updated_removed_and_moved() {
    let temp = tempfile::tempdir().unwrap();
    let path =
        write_workflow_markdown(temp.path(), "Release Notes", "Draft release notes").unwrap();
    add_workflow_step(
        &path,
        "Collect commits",
        StepType::Action,
        serde_json::json!({"command": "git log"}),
    )
    .unwrap();
    add_workflow_step(
        &path,
        "Draft notes",
        StepType::Action,
        serde_json::json!({"format": "markdown"}),
    )
    .unwrap();

    let updated = update_workflow_step(
        &path,
        0,
        Some("Collect merged commits"),
        Some(serde_json::json!({"command": "git log --merges"})),
    )
    .unwrap();
    let moved = move_workflow_step(&path, 1, 0).unwrap();
    let removed = remove_workflow_step(&path, 1).unwrap();
    let loaded = load_workflow(&path).unwrap();

    assert_eq!(updated.name, "Collect merged commits");
    assert_eq!(moved.name, "Draft notes");
    assert_eq!(removed.name, "Collect merged commits");
    assert_eq!(loaded.steps.len(), 1);
    assert_eq!(loaded.steps[0].name, "Draft notes");
}

#[test]
fn workflow_execution_history_can_be_appended_and_read() {
    let temp = tempfile::tempdir().unwrap();
    let execution = WorkflowExecution {
        workflow_id: Uuid::new_v4(),
        workflow_name: "Stored Workflow".to_string(),
        status: ExecutionStatus::Completed,
        current_step: 0,
        started_at: Utc::now(),
        finished_at: Some(Utc::now()),
        results: vec![StepResult {
            step_id: Uuid::new_v4(),
            step_name: "Collect commits".to_string(),
            status: ExecutionStatus::Completed,
            output: serde_json::json!({"ok": true}),
            started_at: Utc::now(),
            finished_at: Utc::now(),
        }],
    };

    let path = append_workflow_execution(temp.path(), &execution).unwrap();
    let executions = read_workflow_executions(temp.path(), 10).unwrap();

    assert!(path.ends_with("workflow-executions.jsonl"));
    assert_eq!(executions.len(), 1);
    assert_eq!(executions[0].workflow_id, execution.workflow_id);
    assert_eq!(executions[0].results[0].step_name, "Collect commits");
}
