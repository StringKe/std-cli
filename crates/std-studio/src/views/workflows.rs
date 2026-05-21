use crate::{
    ui,
    views::{
        schema_label,
        workflow_rows::{self, WorkflowFileAction},
    },
    StudioEguiApp,
};
use eframe::egui;
use std::path::PathBuf;
use std_egui::{i18n, tokens::Space};

const WORKFLOW_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_workflows(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.workflows.title"),
            i18n::t("studio.workflows.detail"),
        );
        self.render_workflows_workspace(ui);
    }

    fn render_workflows_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 900.0 {
            self.render_workflow_library(ui);
            ui.add_space(WORKFLOW_PANEL_GAP);
            self.render_workflow_builder(ui);
            ui.add_space(WORKFLOW_PANEL_GAP);
            self.render_workflow_runtime(ui);
            return;
        }
        let column_width = (available_width - WORKFLOW_PANEL_GAP * 2.0) / 3.0;
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_workflow_library(ui),
            );
            ui.add_space(WORKFLOW_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_workflow_builder(ui),
            );
            ui.add_space(WORKFLOW_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_workflow_runtime(ui),
            );
        });
    }

    fn render_workflow_library(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflows.create.title"),
                i18n::t("studio.workflows.create.detail"),
            );
            workflow_text_input(
                ui,
                i18n::t("studio.workflows.name"),
                &mut self.workflow_name,
            );
            workflow_multiline_input(
                ui,
                i18n::t("studio.workflows.description"),
                &mut self.workflow_description,
                72.0,
            );
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.workflows.create")).clicked() {
                    self.create_workflow_from_form();
                }
                if ui::quiet_button(ui, i18n::t("studio.workflows.refresh")).clicked() {
                    self.app.refresh();
                    self.status = "workflow library refreshed".to_string();
                }
            });
        });

        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflows.saved.title"),
                i18n::t("studio.workflows.saved.detail"),
            );
            match self.app.saved_workflows() {
                Ok(workflows) if workflows.is_empty() => {
                    ui::empty_state(ui, i18n::t("studio.workflows.saved.empty"))
                }
                Ok(workflows) => self.render_saved_workflow_rows(ui, workflows),
                Err(error) => {
                    ui.colored_label(ui::warn_bg(ui.ctx()), error.to_string());
                }
            }
        });
    }

    fn render_saved_workflow_rows(&mut self, ui: &mut egui::Ui, workflows: Vec<PathBuf>) {
        egui::ScrollArea::vertical()
            .max_height(420.0)
            .show(ui, |ui| {
                for path in workflows {
                    let selected = self.workflow_selected_path.as_ref() == Some(&path);
                    match workflow_rows::workflow_file_row(ui, &path, selected) {
                        WorkflowFileAction::Select(path) => {
                            self.workflow_selected_path = Some(path.clone());
                            self.preview_workflow_path(&path);
                        }
                        WorkflowFileAction::Open(path) => {
                            self.app.open_workflow_builder(path.clone());
                            self.workflow_selected_path = Some(path);
                        }
                        WorkflowFileAction::None => {}
                    }
                }
            });
    }

    fn render_workflow_runtime(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflows.dry_run.title"),
                i18n::t("studio.workflows.dry_run.detail"),
            );
            self.render_workflow_debug(ui);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflows.execution.title"),
                i18n::t("studio.workflows.execution.detail"),
            );
            self.render_execution_trace(ui);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflows.batch.title"),
                i18n::t("studio.workflows.batch.detail"),
            );
            self.render_batch_debug(ui);
        });
    }

    fn render_workflow_debug(&self, ui: &mut egui::Ui) {
        let Some(debug) = &self.app.workflow_debug else {
            ui::empty_state(ui, i18n::t("studio.workflows.no_dry_run"));
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
                &format!("{} {:?}", step.step_name, step.step_type),
                &format!("{:?}", step.status),
                &format!(
                    "{} input={} output={}",
                    step.message,
                    schema_label(step.input_schema.as_ref()),
                    schema_label(step.output_schema.as_ref())
                ),
                status_fill(ui.ctx(), &step.status),
            );
        }
    }

    fn render_execution_trace(&self, ui: &mut egui::Ui) {
        let Some(execution) = &self.app.last_workflow_execution else {
            ui::empty_state(ui, i18n::t("studio.workflows.no_execution"));
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

    fn render_batch_debug(&mut self, ui: &mut egui::Ui) {
        workflow_multiline_input(
            ui,
            i18n::t("studio.workflows.batch.title"),
            &mut self.batch_json,
            110.0,
        );
        if ui::quiet_button(ui, i18n::t("studio.workflows.run_batch")).clicked() {
            let body = self.batch_json.clone();
            match self.app.run_batch_json(&body) {
                Ok(report) => {
                    self.layout.open_bottom_panel();
                    self.status = format!("batch {:?} steps={}", report.status, report.steps.len())
                }
                Err(error) => self.status = error.to_string(),
            }
        }
        if let Some(report) = &self.app.last_batch_report {
            workflow_rows::workflow_summary(
                ui,
                "batch",
                &format!("{:?}", report.status),
                report.steps.len(),
            );
            for step in &report.steps {
                workflow_rows::status_row(
                    ui,
                    &step.name,
                    &format!("{:?}", step.status),
                    &format!("{:?} {}", step.kind, step.target),
                    action_status_fill(ui.ctx(), &step.status),
                );
            }
        }
    }
}

impl StudioEguiApp {
    fn create_workflow_from_form(&mut self) {
        match self
            .app
            .create_workflow(&self.workflow_name, &self.workflow_description)
        {
            Ok(path) => {
                self.workflow_selected_path = Some(path.clone());
                self.status = format!("created {}", path.display());
            }
            Err(error) => self.status = error.to_string(),
        }
    }
}

fn status_fill(ctx: &egui::Context, status: &std_orchestration::ExecutionStatus) -> egui::Color32 {
    match status {
        std_orchestration::ExecutionStatus::Completed => ui::ok_bg(ctx),
        std_orchestration::ExecutionStatus::Failed => ui::warn_bg(ctx),
        std_orchestration::ExecutionStatus::Cancelled => ui::warn_bg(ctx),
        std_orchestration::ExecutionStatus::Running => ui::selected_bg(ctx),
        std_orchestration::ExecutionStatus::Pending => ui::panel_alt(ctx),
    }
}

fn action_status_fill(
    ctx: &egui::Context,
    status: &std_types::ActionExecutionStatus,
) -> egui::Color32 {
    match status {
        std_types::ActionExecutionStatus::Completed => ui::ok_bg(ctx),
        std_types::ActionExecutionStatus::Failed => ui::warn_bg(ctx),
        std_types::ActionExecutionStatus::NeedsExternalRunner => ui::warn_bg(ctx),
    }
}

fn workflow_text_input(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.label(label);
    let response = ui.add_sized(
        [ui.available_width(), 28.0],
        egui::TextEdit::singleline(value),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            workflow_input_a11y_label(label, value),
        )
    });
}

fn workflow_multiline_input(ui: &mut egui::Ui, label: &str, value: &mut String, height: f32) {
    ui.label(label);
    let response = ui.add_sized(
        [ui.available_width(), height],
        egui::TextEdit::multiline(value),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            workflow_input_a11y_label(label, value),
        )
    });
}

fn workflow_input_a11y_label(label: &str, value: &str) -> String {
    let value = if value.trim().is_empty() {
        "empty"
    } else {
        value.trim()
    };
    format!("{label}, text box, value {value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_inputs_use_text_edit_widget_info() {
        let source = include_str!("workflows.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("workflow_text_input"));
        assert!(implementation.contains("workflow_multiline_input"));
        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("workflow_input_a11y_label"));
        assert!(implementation.contains("TextEdit::singleline"));
        assert!(implementation.contains("TextEdit::multiline"));
        assert!(!implementation.contains("ui.text_edit_singleline"));
        assert!(!implementation.contains("ui.text_edit_multiline"));
    }

    #[test]
    fn workflow_input_a11y_label_exposes_value() {
        assert_eq!(
            workflow_input_a11y_label("Workflow name", "Daily run"),
            "Workflow name, text box, value Daily run"
        );
        assert_eq!(
            workflow_input_a11y_label("Batch", " "),
            "Batch, text box, value empty"
        );
    }
}
