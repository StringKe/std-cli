use std_studio::StudioApp;

pub(crate) struct WorkflowBuilderSmoke {
    pub(crate) created: bool,
    pub(crate) added_step: bool,
    pub(crate) updated_step: bool,
    pub(crate) moved_step: bool,
    pub(crate) simulated: bool,
    pub(crate) run_status: String,
    pub(crate) trace_steps: usize,
    pub(crate) trace_events: usize,
}

pub(crate) fn run_workflow_builder_smoke(
    studio: &mut StudioApp,
) -> Result<WorkflowBuilderSmoke, Box<dyn std::error::Error>> {
    let workflow_path = studio.create_workflow("Builder Smoke", "Builder interaction smoke")?;
    let created = workflow_path.ends_with("builder-smoke/workflow.md");
    let first = studio.add_workflow_step(
        &workflow_path,
        "Collect builder context",
        serde_json::json!({"phase": "collect"}),
    )?;
    let second = studio.add_workflow_step(
        &workflow_path,
        "Validate builder output",
        serde_json::json!({"phase": "validate"}),
    )?;
    let added_step =
        first.name == "Collect builder context" && second.name == "Validate builder output";
    let updated = studio.update_workflow_step(
        &workflow_path,
        1,
        Some("Validate edited output"),
        Some(serde_json::json!({"phase": "edited"})),
    )?;
    let moved = studio.move_workflow_step(&workflow_path, 1, 0)?;
    let simulated = studio.preview_workflow_path(&workflow_path)?.steps.len() == 2;
    let execution = studio.run_workflow_path(&workflow_path)?.clone();
    let run_status = format!("{:?}", execution.status);
    let traces = studio.recent_workflow_traces(10)?;
    let trace = traces
        .iter()
        .find(|trace| trace.execution.workflow_id == execution.workflow_id);

    Ok(WorkflowBuilderSmoke {
        created,
        added_step,
        updated_step: updated.name == "Validate edited output",
        moved_step: moved.name == "Validate edited output",
        simulated,
        run_status,
        trace_steps: trace.map(|trace| trace.steps.len()).unwrap_or_default(),
        trace_events: trace
            .map(|trace| trace.audit_events.len())
            .unwrap_or_default(),
    })
}
