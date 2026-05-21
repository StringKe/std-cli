use chrono::Utc;
use std_orchestration::{OrchestrationError, StepType, Workflow, WorkflowStep};
use uuid::Uuid;

use crate::StudioApp;

impl StudioApp {
    pub fn selected_planned_step(&self, step_index: usize) -> Option<&WorkflowStep> {
        self.planned_workflow
            .as_ref()
            .and_then(|workflow| workflow.steps.get(step_index))
    }

    pub fn ensure_planned_workflow(&mut self, name: &str) {
        if self.planned_workflow.is_some() {
            return;
        }
        let now = Utc::now();
        self.planned_workflow = Some(Workflow {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: "Draft workflow from Studio AI Assist".to_string(),
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
        });
    }

    pub fn update_planned_workflow_step(
        &mut self,
        step_index: usize,
        name: Option<&str>,
        parameters: Option<serde_json::Value>,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let workflow = self
            .planned_workflow
            .as_mut()
            .ok_or_else(missing_planned_workflow)?;
        let step = workflow
            .steps
            .get_mut(step_index)
            .ok_or_else(|| invalid_planned_step_index(step_index))?;
        if let Some(name) = name {
            step.name = name.to_string();
        }
        if let Some(parameters) = parameters {
            step.parameters = parameters;
        }
        Ok(step.clone())
    }

    pub fn insert_planned_workflow_step(
        &mut self,
        step_index: usize,
        name: &str,
        parameters: serde_json::Value,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let workflow = self
            .planned_workflow
            .as_mut()
            .ok_or_else(missing_planned_workflow)?;
        if step_index > workflow.steps.len() {
            return Err(invalid_planned_step_index(step_index));
        }
        let step = workflow_step(name, parameters);
        workflow.steps.insert(step_index, step.clone());
        Ok(step)
    }

    pub fn append_planned_workflow_step(
        &mut self,
        name: &str,
        parameters: serde_json::Value,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let index = self
            .planned_workflow
            .as_ref()
            .ok_or_else(missing_planned_workflow)?
            .steps
            .len();
        self.insert_planned_workflow_step(index, name, parameters)
    }

    pub fn remove_planned_workflow_step(
        &mut self,
        step_index: usize,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let workflow = self
            .planned_workflow
            .as_mut()
            .ok_or_else(missing_planned_workflow)?;
        if step_index >= workflow.steps.len() {
            return Err(invalid_planned_step_index(step_index));
        }
        Ok(workflow.steps.remove(step_index))
    }

    pub fn move_planned_workflow_step(
        &mut self,
        from_index: usize,
        to_index: usize,
    ) -> Result<WorkflowStep, OrchestrationError> {
        let workflow = self
            .planned_workflow
            .as_mut()
            .ok_or_else(missing_planned_workflow)?;
        if from_index >= workflow.steps.len() {
            return Err(invalid_planned_step_index(from_index));
        }
        if to_index >= workflow.steps.len() {
            return Err(invalid_planned_step_index(to_index));
        }
        let step = workflow.steps.remove(from_index);
        workflow.steps.insert(to_index, step.clone());
        Ok(step)
    }
}

fn missing_planned_workflow() -> OrchestrationError {
    OrchestrationError::InvalidWorkflow("missing planned workflow".to_string())
}

fn invalid_planned_step_index(index: usize) -> OrchestrationError {
    OrchestrationError::InvalidWorkflow(format!("invalid planned step index {index}"))
}

fn workflow_step(name: &str, parameters: serde_json::Value) -> WorkflowStep {
    WorkflowStep {
        id: Uuid::new_v4(),
        name: name.to_string(),
        action_id: None,
        step_type: StepType::Action,
        parameters,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_orchestration::{StepType, Workflow};
    use uuid::Uuid;

    #[test]
    fn planned_workflow_steps_can_be_edited_before_save() {
        let mut app = StudioApp::with_core(std_core::StdCore::new());
        app.planned_workflow = Some(Workflow {
            id: Uuid::new_v4(),
            name: "Draft".to_string(),
            description: "Draft workflow".to_string(),
            steps: vec![
                step("Collect", serde_json::json!({"phase": "collect"})),
                step("Validate", serde_json::json!({"phase": "validate"})),
            ],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

        let updated = app
            .update_planned_workflow_step(
                1,
                Some("Validate output"),
                Some(serde_json::json!({"phase": "edited"})),
            )
            .unwrap();
        let moved = app.move_planned_workflow_step(1, 0).unwrap();
        let removed = app.remove_planned_workflow_step(1).unwrap();

        assert_eq!(updated.name, "Validate output");
        assert_eq!(moved.name, "Validate output");
        assert_eq!(removed.name, "Collect");
        assert_eq!(app.planned_workflow.as_ref().unwrap().steps.len(), 1);
    }

    #[test]
    fn planned_workflow_steps_can_be_inserted_and_appended() {
        let mut app = StudioApp::with_core(std_core::StdCore::new());
        app.ensure_planned_workflow("Draft");
        app.append_planned_workflow_step("Collect", serde_json::json!({"phase": "collect"}))
            .unwrap();

        let inserted = app
            .insert_planned_workflow_step(0, "Prepare", serde_json::json!({"phase": "prepare"}))
            .unwrap();
        let appended = app
            .append_planned_workflow_step("Verify", serde_json::json!({"phase": "verify"}))
            .unwrap();
        let workflow = app.planned_workflow.as_ref().unwrap();

        assert_eq!(inserted.name, "Prepare");
        assert_eq!(appended.name, "Verify");
        assert_eq!(workflow.steps[0].name, "Prepare");
        assert_eq!(workflow.steps[2].name, "Verify");
    }

    #[test]
    fn ensure_planned_workflow_creates_empty_draft_once() {
        let mut app = StudioApp::with_core(std_core::StdCore::new());

        app.ensure_planned_workflow("AI Draft");
        app.ensure_planned_workflow("Other");

        let workflow = app.planned_workflow.as_ref().unwrap();
        assert_eq!(workflow.name, "AI Draft");
        assert!(workflow.steps.is_empty());
    }

    #[test]
    fn planned_workflow_can_be_previewed_after_step_edits() {
        let mut app = StudioApp::with_core(std_core::StdCore::new());
        app.planned_workflow = Some(Workflow {
            id: Uuid::new_v4(),
            name: "Draft".to_string(),
            description: "Draft workflow".to_string(),
            steps: vec![step("Collect", serde_json::json!({"phase": "collect"}))],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

        app.update_planned_workflow_step(
            0,
            Some("Edited planned step"),
            Some(serde_json::json!({"phase": "edited"})),
        )
        .unwrap();
        let workflow = app.planned_workflow.clone().unwrap();
        let preview = app.preview_workflow(&workflow).unwrap();

        assert_eq!(workflow.steps[0].name, "Edited planned step");
        assert_eq!(workflow.steps[0].parameters["phase"], "edited");
        assert_eq!(preview.steps.len(), 1);
    }

    #[test]
    fn planned_workflow_can_run_before_save_and_write_history() {
        let temp = tempfile::tempdir().unwrap();
        let mut app = StudioApp::with_core(std_core::StdCore::with_config(std_core::StdConfig {
            data_dir: temp.path().join("data"),
            ..std_core::StdConfig::default()
        }));
        app.planned_workflow = Some(Workflow {
            id: Uuid::new_v4(),
            name: "Draft Run".to_string(),
            description: "Run without saving first".to_string(),
            steps: vec![step("Collect", serde_json::json!({"phase": "collect"}))],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        });

        let execution = app.run_planned_workflow().unwrap().clone();
        let history = app.recent_workflow_executions(5).unwrap();

        assert_eq!(execution.workflow_name, "Draft Run");
        assert_eq!(execution.results.len(), 1);
        assert_eq!(history[0].workflow_id, execution.workflow_id);
    }

    fn step(name: &str, parameters: serde_json::Value) -> WorkflowStep {
        WorkflowStep {
            id: Uuid::new_v4(),
            name: name.to_string(),
            action_id: None,
            step_type: StepType::Action,
            parameters,
        }
    }
}
