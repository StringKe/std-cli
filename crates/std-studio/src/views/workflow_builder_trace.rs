use crate::{bottom_panel_model::workflow_status_label, ui, views::workflow_rows, StudioEguiApp};
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
        workflow_status_label(&debug.status),
        debug.steps.len(),
    );
    for step in &debug.steps {
        workflow_rows::status_row(
            ui,
            &step.step_name,
            workflow_status_label(&step.status),
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
        workflow_status_label(&execution.status),
        execution.results.len(),
    );
    for step in &execution.results {
        workflow_rows::status_row(
            ui,
            &step.step_name,
            workflow_status_label(&step.status),
            &format!("started={} finished={}", step.started_at, step.finished_at),
            status_fill(ui.ctx(), &step.status),
        );
    }
}

fn status_path(dry_run: Option<&WorkflowDryRun>, execution: Option<&WorkflowExecution>) -> String {
    let dry = dry_run
        .map(|debug| {
            debug
                .steps
                .iter()
                .map(|step| workflow_status_label(&step.status))
        })
        .into_iter()
        .flatten();
    let run = execution
        .map(|execution| {
            execution
                .results
                .iter()
                .map(|step| workflow_status_label(&step.status))
        })
        .into_iter()
        .flatten();
    dry.chain(run).collect::<Vec<_>>().join(">")
}

fn status_fill(ctx: &egui::Context, status: &ExecutionStatus) -> egui::Color32 {
    match status {
        ExecutionStatus::Completed => ui::ok_bg(ctx),
        ExecutionStatus::Failed => ui::danger_bg(ctx),
        ExecutionStatus::Cancelled => ui::warn_bg(ctx),
        ExecutionStatus::Running => ui::selected_bg(ctx),
        ExecutionStatus::Pending => ui::panel_alt(ctx),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_debug_contract_keeps_panel_visible() {
        assert_eq!(
            builder_debug_contract(None, None),
            "debug_panel=true,dry_run=false,execution=false,statuses="
        );
    }

    #[test]
    fn workflow_trace_failed_status_uses_danger_not_warning() {
        let ctx = egui::Context::default();
        std_egui::tokens::apply_theme(&ctx, std_egui::tokens::ThemeMode::Light);

        assert_eq!(
            status_fill(&ctx, &ExecutionStatus::Failed),
            ui::danger_bg(&ctx)
        );
        assert_eq!(
            status_fill(&ctx, &ExecutionStatus::Cancelled),
            ui::warn_bg(&ctx)
        );
    }
}
