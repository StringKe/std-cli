use std_orchestration::{OrchestrationError, WorkflowStep};

use crate::StudioApp;

impl StudioApp {
    pub fn selected_planned_step(&self, step_index: usize) -> Option<&WorkflowStep> {
        self.planned_workflow
            .as_ref()
            .and_then(|workflow| workflow.steps.get(step_index))
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
