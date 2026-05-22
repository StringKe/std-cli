use crate::{
    ui,
    views::{
        workflow_builder_ai, workflow_builder_properties, workflow_builder_run,
        workflow_builder_toolbar, workflow_rows,
    },
    StudioEguiApp,
};
use eframe::egui;
use std::path::Path;
use std_egui::{i18n, input};

impl StudioEguiApp {
    pub(crate) fn render_builder_toolbar(&mut self, ui: &mut egui::Ui) {
        let run_control = workflow_builder_run::WorkflowRunControl::from_execution(
            self.app.last_workflow_execution.as_ref(),
        );
        let response =
            workflow_builder_toolbar::render(ui, &mut self.workflow_goal, run_control.can_cancel);
        for action in response.actions {
            match action {
                workflow_builder_toolbar::WorkflowToolbarAction::Plan => {
                    self.plan_workflow_from_goal();
                }
                workflow_builder_toolbar::WorkflowToolbarAction::Save => {
                    self.save_planned_workflow();
                }
                workflow_builder_toolbar::WorkflowToolbarAction::Simulate => {
                    self.preview_active_workflow();
                }
                workflow_builder_toolbar::WorkflowToolbarAction::Test => {
                    self.run_active_workflow();
                }
                workflow_builder_toolbar::WorkflowToolbarAction::Cancel => {
                    self.cancel_active_workflow();
                }
                workflow_builder_toolbar::WorkflowToolbarAction::History => {
                    self.open_workflow_history();
                }
            }
        }
    }

    pub(crate) fn render_builder_steps(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.steps.title"),
                &workflow_step_shortcuts_detail(),
            );
            if self.app.planned_workflow.is_some() {
                self.render_planned_steps(ui);
                return;
            }
            if self.workflow_selected_path.is_some() {
                self.render_debug_steps(ui);
            } else {
                ui::empty_state(ui, i18n::t("studio.workflow_builder.steps.empty"));
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
        workflow_rows::builder_step_summary(ui, &name, rows.len());
        for (index, (name, detail)) in rows.iter().enumerate() {
            self.step_row(ui, index, name, detail);
        }
    }

    fn render_debug_steps(&mut self, ui: &mut egui::Ui) {
        let Some(debug) = &self.app.workflow_debug else {
            ui::empty_state(ui, i18n::t("studio.workflow_builder.preview.empty"));
            return;
        };
        let name = debug.workflow_name.clone();
        let rows = debug
            .steps
            .iter()
            .map(|step| (step.step_name.clone(), format!("{:?}", step.step_type)))
            .collect::<Vec<_>>();
        workflow_rows::builder_step_summary(ui, &name, rows.len());
        for (index, (name, detail)) in rows.iter().enumerate() {
            self.step_row(ui, index, name, detail);
        }
    }

    fn step_row(&mut self, ui: &mut egui::Ui, index: usize, name: &str, detail: &str) {
        let selected = self.workflow_edit_index.trim() == index.to_string();
        let response = workflow_rows::builder_step_row(ui, index, name, detail, selected);
        if response.clicked() {
            self.workflow_edit_index = index.to_string();
            self.workflow_step_name = name.to_string();
            self.sync_selected_step_parameters(index);
        }
    }

    pub(crate) fn render_step_properties(&mut self, ui: &mut egui::Ui) {
        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.properties.title"),
                i18n::t("studio.workflow_builder.properties.detail"),
            );
            if self.app.planned_workflow.is_some() && self.workflow_selected_path.is_none() {
                self.render_planned_step_properties(ui);
                return;
            }
            let selected = self.workflow_selected_path.clone();
            let Some(path) = selected else {
                ui::empty_state(ui, i18n::t("studio.workflow_builder.properties.empty"));
                return;
            };
            workflow_rows::path_row(ui, "workflow", &path);
            let actions = workflow_builder_properties::render_loaded_step_properties(
                ui,
                &mut self.workflow_step_name,
                &mut self.workflow_step_parameters,
                &mut self.workflow_edit_index,
            );
            self.apply_loaded_step_actions(&path, actions);
        });
    }

    fn render_planned_step_properties(&mut self, ui: &mut egui::Ui) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let Some(step) = self.app.selected_planned_step(index).cloned() else {
            ui::empty_state(ui, i18n::t("studio.workflow_builder.properties.empty"));
            return;
        };
        workflow_builder_properties::render_step_identity(ui, index, &step);
        let actions = workflow_builder_properties::render_planned_step_properties(
            ui,
            &mut self.workflow_step_name,
            &mut self.workflow_step_parameters,
            &mut self.workflow_edit_index,
        );
        self.apply_planned_step_actions(actions);
    }

    pub(crate) fn render_ai_assist_panel(&mut self, ui: &mut egui::Ui) {
        if let Some(action) = workflow_builder_ai::render(ui, &mut self.workflow_goal) {
            self.apply_workflow_ai_action(action);
        }
    }

    pub(crate) fn preview_active_workflow(&mut self) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.preview_workflow_path(&path);
        } else {
            self.preview_planned_workflow();
        }
    }

    pub(crate) fn run_active_workflow(&mut self) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.run_workflow_path(&path);
        } else {
            self.run_planned_workflow();
        }
    }

    pub(crate) fn open_workflow_history(&mut self) {
        self.app.open_execution_history_pane();
        self.open_batch_debug_panel();
        self.status = "workflow history opened".to_string();
    }

    pub(crate) fn cancel_active_workflow(&mut self) {
        match self.app.cancel_last_workflow_execution() {
            Ok(execution) => {
                let status = format!(
                    "cancelled {} steps={}",
                    execution.workflow_name,
                    execution.results.len()
                );
                self.open_batch_debug_panel();
                self.status = status;
            }
            Err(error) => self.status = error.to_string(),
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
                    let status = format!(
                        "dry-run {} steps={}",
                        preview.workflow_name,
                        preview.steps.len()
                    );
                    self.open_batch_debug_panel();
                    self.status = status;
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
                let status = format!(
                    "dry-run {} steps={}",
                    preview.workflow_name,
                    preview.steps.len()
                );
                self.open_batch_debug_panel();
                self.status = status;
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn run_workflow_path(&mut self, path: &Path) {
        match self.app.run_workflow_path(path) {
            Ok(execution) => {
                let status = format!(
                    "run {:?} steps={}",
                    execution.status,
                    execution.results.len()
                );
                self.open_batch_debug_panel();
                self.status = status;
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn run_planned_workflow(&mut self) {
        match self.app.run_planned_workflow() {
            Ok(execution) => {
                let status = format!(
                    "run planned {:?} steps={}",
                    execution.status,
                    execution.results.len()
                );
                self.open_batch_debug_panel();
                self.status = status;
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn add_step_to_selected(&mut self, path: &Path) {
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

    pub(crate) fn update_selected_step(&mut self, path: &Path) {
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

    pub(crate) fn move_selected_step(&mut self, path: &Path, offset: isize) {
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

    pub(crate) fn move_workflow_builder_step_by_keyboard(&mut self, offset: isize) {
        if let Some(path) = self.workflow_selected_path.clone() {
            self.move_selected_step(&path, offset);
        } else {
            self.move_planned_step(offset);
        }
    }

    pub(crate) fn select_workflow_builder_step_by_keyboard(&mut self, offset: isize) {
        if !self.focused_workspace_is_workflow_builder() {
            return;
        }
        let Some(current) = self.selected_step_index() else {
            return;
        };
        let Some(last) = self
            .workflow_step_count()
            .and_then(|count| count.checked_sub(1))
        else {
            return;
        };
        let next = current.saturating_add_signed(offset).min(last);
        self.select_workflow_builder_step(next);
    }

    pub(crate) fn remove_selected_step(&mut self, path: &Path) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        match self.app.remove_workflow_step(path, index) {
            Ok(step) => self.status = format!("removed step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn update_planned_step(&mut self) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let Ok(parameters) = serde_json::from_str(&self.workflow_step_parameters) else {
            self.status = "invalid step parameters JSON".to_string();
            return;
        };
        match self.app.update_planned_workflow_step(
            index,
            Some(&self.workflow_step_name),
            Some(parameters),
        ) {
            Ok(step) => self.status = format!("updated planned step {}", step.name),
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn move_planned_step(&mut self, offset: isize) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        let target = index.saturating_add_signed(offset);
        match self.app.move_planned_workflow_step(index, target) {
            Ok(step) => {
                self.workflow_edit_index = target.to_string();
                self.workflow_step_name = step.name.clone();
                self.workflow_step_parameters = step.parameters.to_string();
                self.status = format!("moved planned step {}", step.name);
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    pub(crate) fn remove_planned_step(&mut self) {
        let Some(index) = self.selected_step_index() else {
            return;
        };
        match self.app.remove_planned_workflow_step(index) {
            Ok(step) => {
                self.workflow_edit_index = "0".to_string();
                self.status = format!("removed planned step {}", step.name);
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn sync_selected_step_parameters(&mut self, index: usize) {
        if let Some(step) = self.app.selected_planned_step(index) {
            self.workflow_step_parameters = step.parameters.to_string();
        }
    }

    fn select_workflow_builder_step(&mut self, index: usize) {
        self.workflow_edit_index = index.to_string();
        self.sync_selected_step_parameters(index);
    }

    fn workflow_step_count(&self) -> Option<usize> {
        if let Some(workflow) = &self.app.planned_workflow {
            return Some(workflow.steps.len());
        }
        self.app
            .workflow_debug
            .as_ref()
            .map(|debug| debug.steps.len())
    }

    pub(crate) fn selected_step_index(&mut self) -> Option<usize> {
        match self.workflow_edit_index.trim().parse::<usize>() {
            Ok(index) => Some(index),
            Err(_) => {
                self.status = "invalid step index".to_string();
                None
            }
        }
    }
}

fn workflow_step_shortcuts_detail() -> String {
    format!(
        "{} {}",
        input::studio_workflow_step_move_up().label(),
        input::studio_workflow_step_move_down().label()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_step_shortcuts_detail_uses_platform_labels() {
        let detail = workflow_step_shortcuts_detail();

        assert!(detail.contains(&input::studio_workflow_step_move_up().label()));
        assert!(detail.contains(&input::studio_workflow_step_move_down().label()));
        assert!(!detail.contains("Alt+Up Alt+Down"));
    }
}
