use crate::StudioEguiApp;
use std::path::Path;

pub(crate) fn flow_contract() -> &'static str {
    "flow=goal-input|plan|save|simulate|test|trace"
}

impl StudioEguiApp {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flow_contract_covers_docs22_toolbar_sequence() {
        assert_eq!(
            flow_contract(),
            "flow=goal-input|plan|save|simulate|test|trace"
        );
    }
}
