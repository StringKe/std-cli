use crate::{ui, StudioEguiApp};
use eframe::egui;
use std::path::Path;

impl StudioEguiApp {
    pub(crate) fn render_workflow_builder(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Workflow Builder", "steps, properties, AI assist");
            self.render_builder_toolbar(ui);
            ui.add_space(8.0);
            ui.columns(2, |columns| {
                columns[0].vertical(|ui| self.render_builder_steps(ui));
                columns[1].vertical(|ui| self.render_step_properties(ui));
            });
            ui.add_space(8.0);
            self.render_ai_assist_panel(ui);
        });
    }

    fn render_builder_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add_sized(
                [ui.available_width().min(260.0), 28.0],
                egui::TextEdit::singleline(&mut self.workflow_goal)
                    .hint_text("Describe workflow goal"),
            );
            if ui::quiet_button(ui, "Plan").clicked() {
                self.plan_workflow_from_goal();
            }
            if ui::quiet_button(ui, "Simulate").clicked() {
                self.preview_active_workflow();
            }
            if ui::quiet_button(ui, "Run").clicked() {
                self.run_active_workflow();
            }
            if ui::quiet_button(ui, "Save").clicked() {
                self.save_planned_workflow();
            }
        });
    }

    fn render_builder_steps(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Steps", "Alt+Up Alt+Down");
            if self.app.planned_workflow.is_some() {
                self.render_planned_steps(ui);
                return;
            }
            if self.workflow_selected_path.is_some() {
                self.render_debug_steps(ui);
            } else {
                ui::empty_state(ui, "Select or plan a workflow");
            }
        });
    }

    fn render_planned_steps(&mut self, ui: &mut egui::Ui) {
        let Some(workflow) = &self.app.planned_workflow else {
            return;
        };
        let name = workflow.name.clone();
        let rows = workflow
            .steps
            .iter()
            .map(|step| (step.name.clone(), format!("{:?}", step.step_type)))
            .collect::<Vec<_>>();
        ui.label(format!("{} steps={}", name, rows.len()));
        for (index, (name, detail)) in rows.iter().enumerate() {
            self.step_row(ui, index, name, detail);
        }
    }

    fn render_debug_steps(&mut self, ui: &mut egui::Ui) {
        let Some(debug) = &self.app.workflow_debug else {
            ui::empty_state(ui, "No preview yet");
            return;
        };
        let name = debug.workflow_name.clone();
        let rows = debug
            .steps
            .iter()
            .map(|step| (step.step_name.clone(), format!("{:?}", step.step_type)))
            .collect::<Vec<_>>();
        ui.label(format!("{} steps={}", name, rows.len()));
        for (index, (name, detail)) in rows.iter().enumerate() {
            self.step_row(ui, index, name, detail);
        }
    }

    fn step_row(&mut self, ui: &mut egui::Ui, index: usize, name: &str, detail: &str) {
        let selected = self.workflow_edit_index.trim() == index.to_string();
        let response = ui.selectable_label(selected, format!("{}  {}", index + 1, name));
        if response.clicked() {
            self.workflow_edit_index = index.to_string();
            self.workflow_step_name = name.to_string();
        }
        ui.small(detail);
        ui.add_space(4.0);
    }

    fn render_step_properties(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Step Properties", "schema JSON");
            let selected = self.workflow_selected_path.clone();
            let Some(path) = selected else {
                ui::empty_state(ui, "Select a saved workflow to edit steps");
                return;
            };
            ui.small(path.display().to_string());
            ui.label("Step name");
            ui.text_edit_singleline(&mut self.workflow_step_name);
            ui.label("Parameters JSON");
            ui.add_sized(
                [ui.available_width(), 92.0],
                egui::TextEdit::multiline(&mut self.workflow_step_parameters),
            );
            ui.horizontal(|ui| {
                ui.label("Index");
                ui.add_sized(
                    [48.0, 24.0],
                    egui::TextEdit::singleline(&mut self.workflow_edit_index),
                );
                if ui::quiet_button(ui, "Add").clicked() {
                    self.add_step_to_selected(&path);
                }
                if ui::quiet_button(ui, "Update").clicked() {
                    self.update_selected_step(&path);
                }
            });
            ui.horizontal_wrapped(|ui| {
                if ui::quiet_button(ui, "Move Up").clicked() {
                    self.move_selected_step(&path, -1);
                }
                if ui::quiet_button(ui, "Move Down").clicked() {
                    self.move_selected_step(&path, 1);
                }
                if ui::quiet_button(ui, "Remove").clicked() {
                    self.remove_selected_step(&path);
                }
            });
        });
    }

    fn render_ai_assist_panel(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "AI Assist", "plan from goal");
            ui.label("Describe what this workflow should do");
            ui.add_sized(
                [ui.available_width(), 32.0],
                egui::TextEdit::singleline(&mut self.workflow_goal),
            );
        });
    }

    fn preview_active_workflow(&mut self) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.preview_workflow_path(&path);
        } else {
            self.preview_planned_workflow();
        }
    }

    fn run_active_workflow(&mut self) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.run_workflow_path(&path);
        } else {
            self.status = "select saved workflow before running".to_string();
        }
    }

    pub(crate) fn plan_workflow_from_goal(&mut self) {
        let goal = self.workflow_goal.clone();
        match self.app.plan_workflow(&goal) {
            Ok(workflow) => {
                self.status = format!("planned {} steps={}", workflow.name, workflow.steps.len())
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn preview_planned_workflow(&mut self) {
        match self.app.planned_workflow.clone() {
            Some(workflow) => match self.app.preview_workflow(&workflow) {
                Ok(preview) => {
                    self.status = format!(
                        "dry-run {} steps={}",
                        preview.workflow_name,
                        preview.steps.len()
                    )
                }
                Err(error) => self.status = error.to_string(),
            },
            None => self.status = "missing planned workflow".to_string(),
        }
    }

    pub(crate) fn save_planned_workflow(&mut self) {
        match self.app.save_planned_workflow() {
            Ok(path) => {
                self.workflow_selected_path = Some(path.clone());
                self.status = format!("saved {}", path.display());
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn preview_workflow_path(&mut self, path: &Path) {
        match self.app.preview_workflow_path(path) {
            Ok(preview) => {
                self.status = format!(
                    "dry-run {} steps={}",
                    preview.workflow_name,
                    preview.steps.len()
                )
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn run_workflow_path(&mut self, path: &Path) {
        match self.app.run_workflow_path(path) {
            Ok(execution) => {
                self.status = format!(
                    "run {:?} steps={}",
                    execution.status,
                    execution.results.len()
                )
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn add_step_to_selected(&mut self, path: &Path) {
        let Ok(parameters) = serde_json::from_str(&self.workflow_step_parameters) else {
            self.status = "invalid step parameters JSON".to_string();
            return;
        };
        match self
            .app
            .add_workflow_step(path, &self.workflow_step_name, parameters)
        {
            Ok(step) => self.status = format!("added step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn update_selected_step(&mut self, path: &Path) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let Ok(parameters) = serde_json::from_str(&self.workflow_step_parameters) else {
            self.status = "invalid step parameters JSON".to_string();
            return;
        };
        match self.app.update_workflow_step(
            path,
            index,
            Some(&self.workflow_step_name),
            Some(parameters),
        ) {
            Ok(step) => self.status = format!("updated step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn move_selected_step(&mut self, path: &Path, offset: isize) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let target = index.saturating_add_signed(offset);
        match self.app.move_workflow_step(path, index, target) {
            Ok(step) => {
                self.workflow_edit_index = target.to_string();
                self.status = format!("moved step {}", step.name);
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn remove_selected_step(&mut self, path: &Path) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        match self.app.remove_workflow_step(path, index) {
            Ok(step) => self.status = format!("removed step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn selected_step_index(&mut self) -> Option<usize> {
        match self.workflow_edit_index.trim().parse::<usize>() {
            Ok(index) => Some(index),
            Err(_) => {
                self.status = "invalid step index".to_string();
                None
            }
        }
    }
}
