use crate::{
    views::{
        workflow_builder_ai::{self, WorkflowAiAction},
        workflow_builder_properties::StepPropertyActions,
    },
    StudioEguiApp,
};
use std::path::Path;

impl StudioEguiApp {
    pub(crate) fn apply_loaded_step_actions(&mut self, path: &Path, actions: StepPropertyActions) {
        if actions.add_requested() {
            self.add_step_to_selected(path);
        }
        if actions.update_requested() {
            self.update_selected_step(path);
        }
        if actions.move_up_requested() {
            self.move_selected_step(path, -1);
        }
        if actions.move_down_requested() {
            self.move_selected_step(path, 1);
        }
        if actions.remove_requested() {
            self.remove_selected_step(path);
        }
    }

    pub(crate) fn apply_planned_step_actions(&mut self, actions: StepPropertyActions) {
        if actions.update_requested() {
            self.update_planned_step();
        }
        if actions.move_up_requested() {
            self.move_planned_step(-1);
        }
        if actions.move_down_requested() {
            self.move_planned_step(1);
        }
        if actions.remove_requested() {
            self.remove_planned_step();
        }
    }

    pub(crate) fn apply_workflow_ai_action(&mut self, action: WorkflowAiAction) {
        let suggestions = workflow_builder_ai::suggestions(&self.workflow_goal);
        let (mode, index) = ai_action_mode_and_index(action);
        let Some(suggestion) = suggestions.get(index) else {
            self.status = "missing AI suggestion".to_string();
            return;
        };
        self.app.ensure_planned_workflow("AI assisted workflow");
        let result = match action {
            WorkflowAiAction::Apply(_) => self
                .app
                .append_planned_workflow_step(suggestion.step_name, suggestion.parameters.clone()),
            WorkflowAiAction::Insert(_) => {
                let index = self.selected_step_index().unwrap_or_default();
                self.app.insert_planned_workflow_step(
                    index,
                    suggestion.step_name,
                    suggestion.parameters.clone(),
                )
            }
            WorkflowAiAction::Replace(_) => {
                let Some(index) = self.selected_step_index() else {
                    return;
                };
                self.app.update_planned_workflow_step(
                    index,
                    Some(suggestion.step_name),
                    Some(suggestion.parameters.clone()),
                )
            }
        };
        match result {
            Ok(step) => {
                self.workflow_step_name = step.name.clone();
                self.workflow_step_parameters = step.parameters.to_string();
                self.status = format!("AI {mode} step {}", step.name);
            }
            Err(error) => self.status = error.to_string(),
        }
    }
}

fn ai_action_mode_and_index(action: WorkflowAiAction) -> (&'static str, usize) {
    match action {
        WorkflowAiAction::Apply(index) => ("applied", index),
        WorkflowAiAction::Insert(index) => ("inserted", index),
        WorkflowAiAction::Replace(index) => ("replaced", index),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::views::workflow_builder_properties::StepPropertyAction;

    #[test]
    fn action_dispatch_preserves_loaded_step_order() {
        let actions = StepPropertyActions {
            actions: vec![
                StepPropertyAction::Add,
                StepPropertyAction::Update,
                StepPropertyAction::MoveUp,
                StepPropertyAction::MoveDown,
                StepPropertyAction::Remove,
            ],
        };

        assert!(actions.add_requested());
        assert!(actions.update_requested());
        assert!(actions.move_up_requested());
        assert!(actions.move_down_requested());
        assert!(actions.remove_requested());
    }

    #[test]
    fn ai_action_mode_tracks_toolbar_labels() {
        assert_eq!(
            ai_action_mode_and_index(WorkflowAiAction::Apply(1)),
            ("applied", 1)
        );
        assert_eq!(
            ai_action_mode_and_index(WorkflowAiAction::Insert(2)),
            ("inserted", 2)
        );
        assert_eq!(
            ai_action_mode_and_index(WorkflowAiAction::Replace(3)),
            ("replaced", 3)
        );
    }
}
