use crate::{ui, views::workflow_rows, StudioEguiApp};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_orchestration::{ExecutionStatus, WorkflowDryRun, WorkflowExecution};

pub(crate) fn render(ui: &mut egui::Ui, app: &StudioEguiApp) {
    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.workflow_builder.debug.title"),
            i18n::t("studio.workflow_builder.debug.detail"),
        );
        render_debug(ui, app.app.workflow_debug.as_ref());
        ui.add_space(Space::XS as f32);
        render_execution(ui, app.app.last_workflow_execution.as_ref());
    });
}

pub(crate) fn builder_debug_contract(
    dry_run: Option<&WorkflowDryRun>,
    execution: Option<&WorkflowExecution>,
) -> String {
    format!(
        "debug_panel={},dry_run={},execution={},statuses={}",
        true,
        dry_run.is_some(),
        execution.is_some(),
        status_path(dry_run, execution)
    )
}

fn render_debug(ui: &mut egui::Ui, debug: Option<&WorkflowDryRun>) {
    let Some(debug) = debug else {
        ui::empty_state(ui, i18n::t("studio.workflow_builder.debug.no_dry_run"));
        return;
    };
    workflow_rows::workflow_summary(
        ui,
        &debug.workflow_name,
        &format!("{:?}", debug.status),
        debug.steps.len(),
    );
    for step in &debug.steps {
        workflow_rows::status_row(
            ui,
            &step.step_name,
            &format!("{:?}", step.status),
            &format!("{:?} {}", step.step_type, step.message),
            status_fill(ui.ctx(), &step.status),
        );
    }
}

fn render_execution(ui: &mut egui::Ui, execution: Option<&WorkflowExecution>) {
    let Some(execution) = execution else {
        ui::empty_state(ui, i18n::t("studio.workflow_builder.debug.no_execution"));
        return;
    };
    workflow_rows::workflow_summary(
        ui,
        &execution.workflow_name,
        &format!("{:?}", execution.status),
        execution.results.len(),
    );
    for step in &execution.results {
        workflow_rows::status_row(
            ui,
            &step.step_name,
            &format!("{:?}", step.status),
            &format!("started={} finished={}", step.started_at, step.finished_at),
            status_fill(ui.ctx(), &step.status),
        );
    }
}

fn status_path(dry_run: Option<&WorkflowDryRun>, execution: Option<&WorkflowExecution>) -> String {
    let dry = dry_run
        .map(|debug| debug.steps.iter().map(|step| status_label(&step.status)))
        .into_iter()
        .flatten();
    let run = execution
        .map(|execution| {
            execution
                .results
                .iter()
                .map(|step| status_label(&step.status))
        })
        .into_iter()
        .flatten();
    dry.chain(run).collect::<Vec<_>>().join(">")
}

fn status_label(status: &ExecutionStatus) -> &'static str {
    match status {
        ExecutionStatus::Pending => "pending",
        ExecutionStatus::Running => "running",
        ExecutionStatus::Completed => "success",
        ExecutionStatus::Failed => "error",
        ExecutionStatus::Cancelled => "skipped",
    }
}

fn status_fill(ctx: &egui::Context, status: &ExecutionStatus) -> egui::Color32 {
    match status {
        ExecutionStatus::Completed => ui::ok_bg(ctx),
        ExecutionStatus::Failed | ExecutionStatus::Cancelled => ui::warn_bg(ctx),
        ExecutionStatus::Running => ui::selected_bg(ctx),
        ExecutionStatus::Pending => ui::panel_alt(ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_labels_match_docs_22_batch_debug_states() {
        assert_eq!(status_label(&ExecutionStatus::Pending), "pending");
        assert_eq!(status_label(&ExecutionStatus::Running), "running");
        assert_eq!(status_label(&ExecutionStatus::Completed), "success");
        assert_eq!(status_label(&ExecutionStatus::Failed), "error");
        assert_eq!(status_label(&ExecutionStatus::Cancelled), "skipped");
    }

    #[test]
    fn empty_debug_contract_keeps_panel_visible() {
        assert_eq!(
            builder_debug_contract(None, None),
            "debug_panel=true,dry_run=false,execution=false,statuses="
        );
    }
}
