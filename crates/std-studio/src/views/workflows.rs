use crate::{ui, views::schema_label, StudioEguiApp};
use eframe::egui;
use std::path::{Path, PathBuf};
use std_egui::tokens::Space;

impl StudioEguiApp {
    pub(crate) fn render_workflows(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            "Workflow Workbench",
            "create, edit, simulate, run, trace",
        );
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_workflow_library(ui));
            columns[1].vertical(|ui| self.render_workflow_builder(ui));
            columns[2].vertical(|ui| self.render_workflow_runtime(ui));
        });
    }

    fn render_workflow_library(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Create", "markdown workflow");
            ui.label("Name");
            ui.text_edit_singleline(&mut self.workflow_name);
            ui.label("Description");
            ui.text_edit_multiline(&mut self.workflow_description);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, "Create").clicked() {
                    self.create_workflow_from_form();
                }
                if ui::quiet_button(ui, "Refresh").clicked() {
                    self.app.refresh();
                    self.status = "workflow library refreshed".to_string();
                }
            });
        });

        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Saved", "local definitions");
            match self.app.saved_workflows() {
                Ok(workflows) if workflows.is_empty() => ui::empty_state(ui, "No saved workflows"),
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
                    let label = workflow_label(&path);
                    ui.horizontal(|ui| {
                        if ui.selectable_label(selected, label).clicked() {
                            self.workflow_selected_path = Some(path.clone());
                            self.preview_workflow_path(&path);
                        }
                        if ui.small_button("Open").clicked() {
                            self.app.open_workflow_builder(path.clone());
                            self.workflow_selected_path = Some(path.clone());
                        }
                    });
                    ui.small(path.display().to_string());
                    ui.add_space(Space::XS as f32);
                }
            });
    }

    fn render_workflow_runtime(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Dry Run", "schema, defer, errors");
            self.render_workflow_debug(ui);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Execution", "last captured run");
            self.render_execution_trace(ui);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Batch", "external defaults defer");
            self.render_batch_debug(ui);
        });
    }

    fn render_workflow_debug(&self, ui: &mut egui::Ui) {
        let Some(debug) = &self.app.workflow_debug else {
            ui::empty_state(ui, "No dry-run report");
            return;
        };
        ui.horizontal(|ui| {
            ui::chip(
                ui,
                &format!("{:?}", debug.status),
                status_fill(ui.ctx(), &debug.status),
            );
            ui.label(format!(
                "{} steps={}",
                debug.workflow_name,
                debug.steps.len()
            ));
        });
        for step in &debug.steps {
            ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui::chip(
                        ui,
                        &format!("{:?}", step.status),
                        status_fill(ui.ctx(), &step.status),
                    );
                    ui.label(format!("{} {:?}", step.step_name, step.step_type));
                });
                ui.small(&step.message);
                ui.small(format!(
                    "schema input={} output={}",
                    schema_label(step.input_schema.as_ref()),
                    schema_label(step.output_schema.as_ref())
                ));
            });
        }
    }

    fn render_execution_trace(&self, ui: &mut egui::Ui) {
        let Some(execution) = &self.app.last_workflow_execution else {
            ui::empty_state(ui, "No execution captured");
            return;
        };
        ui.horizontal(|ui| {
            ui::chip(
                ui,
                &format!("{:?}", execution.status),
                status_fill(ui.ctx(), &execution.status),
            );
            ui.label(format!(
                "{} steps={}",
                execution.workflow_name,
                execution.results.len()
            ));
        });
        for step in &execution.results {
            ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                ui.label(format!("{} {:?}", step.step_name, step.status));
                ui.small(format!("started={}", step.started_at));
                ui.small(format!("finished={}", step.finished_at));
            });
        }
    }

    fn render_batch_debug(&mut self, ui: &mut egui::Ui) {
        ui.add_sized(
            [ui.available_width(), 110.0],
            egui::TextEdit::multiline(&mut self.batch_json),
        );
        if ui::quiet_button(ui, "Run Batch").clicked() {
            let body = self.batch_json.clone();
            match self.app.run_batch_json(&body) {
                Ok(report) => {
                    self.status = format!("batch {:?} steps={}", report.status, report.steps.len())
                }
                Err(error) => self.status = error.to_string(),
            }
        }
        if let Some(report) = &self.app.last_batch_report {
            ui.horizontal(|ui| {
                ui::chip(
                    ui,
                    &format!("{:?}", report.status),
                    action_status_fill(ui.ctx(), &report.status),
                );
                ui.label(format!("steps={}", report.steps.len()));
            });
            for step in &report.steps {
                ui.small(format!("{} {:?} {}", step.name, step.status, step.target));
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

fn workflow_label(path: &Path) -> String {
    path.parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .or_else(|| path.file_stem().and_then(|name| name.to_str()))
        .unwrap_or("workflow")
        .to_string()
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
