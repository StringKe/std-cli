use std_studio::StudioApp;

pub(crate) struct WorkflowBuilderSmoke {
    pub(crate) created: bool,
    pub(crate) added_step: bool,
    pub(crate) updated_step: bool,
    pub(crate) moved_step: bool,
    pub(crate) simulated: bool,
    pub(crate) run_status: String,
    pub(crate) planned_run_status: String,
    pub(crate) trace_steps: usize,
    pub(crate) trace_events: usize,
    pub(crate) interaction_sequence: String,
    pub(crate) selected_step_title: String,
    pub(crate) trace_status: String,
    pub(crate) side_effect_model: String,
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
    let planned = studio.plan_workflow("terminal")?;
    let planned_run = studio.run_planned_workflow()?.clone();
    let execution = studio.run_workflow_path(&workflow_path)?.clone();
    let run_status = format!("{:?}", execution.status);
    let traces = studio.recent_workflow_traces(10)?;
    let trace = traces
        .iter()
        .find(|trace| trace.execution.workflow_id == execution.workflow_id);
    let trace_status = trace
        .map(|trace| format!("{:?}", trace.execution.status))
        .unwrap_or_else(|| "Missing".to_string());

    Ok(WorkflowBuilderSmoke {
        created,
        added_step,
        updated_step: updated.name == "Validate edited output",
        moved_step: moved.name == "Validate edited output",
        simulated,
        run_status,
        planned_run_status: format!("{:?}:{}", planned_run.status, planned.name),
        trace_steps: trace.map(|trace| trace.steps.len()).unwrap_or_default(),
        trace_events: trace
            .map(|trace| trace.audit_events.len())
            .unwrap_or_default(),
        interaction_sequence: "create>add>edit>move>simulate>run-planned>run-saved>trace"
            .to_string(),
        selected_step_title: moved.name,
        trace_status,
        side_effect_model: "simulate=dry-run,run=audit-log".to_string(),
    })
}
