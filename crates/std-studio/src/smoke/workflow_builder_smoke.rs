use crate::{
    bottom_panel_model::BottomPanelTabModel,
    views::{self, workflow_builder_fields, workflow_builder_toolbar, workflow_builder_trace},
};
use std_egui::input;
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
    pub(crate) toolbar_contract: String,
    pub(crate) properties_contract: String,
    pub(crate) keyboard_move_path: String,
    pub(crate) selected_step_title: String,
    pub(crate) trace_status: String,
    pub(crate) side_effect_model: String,
    pub(crate) next_action: String,
    pub(crate) bottom_panel_contract: String,
    pub(crate) debug_panel_contract: String,
    pub(crate) visual_contract: String,
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
    let keyboard_move_path = workflow_step_keyboard_path();
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

    let debug_panel_contract = workflow_builder_trace::builder_debug_contract(
        studio.workflow_debug.as_ref(),
        studio.last_workflow_execution.as_ref(),
    );
    let visual_contract = builder_visual_contract();

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
        toolbar_contract: workflow_builder_toolbar::toolbar_contract().to_string(),
        properties_contract: workflow_builder_fields::fields_contract().to_string(),
        keyboard_move_path,
        selected_step_title: moved.name,
        trace_status,
        side_effect_model: "simulate=dry-run,run=audit-log".to_string(),
        next_action: "complete".to_string(),
        bottom_panel_contract: builder_bottom_panel_contract(),
        debug_panel_contract,
        visual_contract,
    })
}

fn builder_bottom_panel_contract() -> String {
    let builder_source = include_str!("../views/workflow_builder.rs");
    let bottom_panel_source = include_str!("../bottom_panel.rs");
    let simulate = action_opens_batch_debug(builder_source, "preview_workflow_path");
    let run = action_opens_batch_debug(builder_source, "run_workflow_path");
    let planned_run = action_opens_batch_debug(builder_source, "run_planned_workflow");
    let history = action_opens_batch_debug(builder_source, "open_workflow_history");
    let helper = bottom_panel_source.contains("self.layout.open_bottom_panel();")
        && bottom_panel_source.contains("self.bottom_panel_tab = BottomPanelTab::BatchDebug;");
    format!(
        "batch-debug=simulate:{simulate}|run:{run}|planned-run:{planned_run}|history:{history};helper={};{}",
        open_closed(helper),
        BottomPanelTabModel::docs22_default().contract()
    )
}

fn action_opens_batch_debug(source: &str, action: &str) -> &'static str {
    let Some(action_index) = source.find(&format!("fn {action}")) else {
        return "missing";
    };
    let tail = &source[action_index..];
    let next_method_index = tail
        .find("\n    pub(crate) fn ")
        .filter(|index| *index > 0)
        .unwrap_or(tail.len());
    let action_body = &tail[..next_method_index];
    open_closed(action_body.contains("self.open_batch_debug_panel();"))
}

fn open_closed(open: bool) -> &'static str {
    if open {
        "open"
    } else {
        "closed"
    }
}

fn builder_visual_contract() -> String {
    [
        "builder_visual=single-pane-workbench",
        views::workflow_builder_flow::flow_contract(),
        views::workflow_builder_step_visual_contract(),
        "properties=step-name|parameters-json|index|add|update|move|remove",
        "debug=dry-run|execution|trace",
        "bottom-panel=batch-debug|logs|problems|performance",
        "history=execution-history-pane|timeline",
    ]
    .join(";")
}

fn workflow_step_keyboard_path() -> String {
    format!(
        "{}:0>1;{}:1>0",
        input::studio_workflow_step_move_down().label(),
        input::studio_workflow_step_move_up().label()
    )
}
