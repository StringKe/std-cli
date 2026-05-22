use crate::LauncherState;
use std_types::{ActionExecution, ActionExecutionStatus};

impl LauncherState {
    pub(crate) fn complete_action_panel_copy(&mut self, command: String) -> ActionExecution {
        let action_name = self
            .view
            .selected_result()
            .map(|result| result.action.name.clone())
            .unwrap_or_else(|| "Selected Action".to_string());
        let execution = ActionExecution {
            action_id: self
                .view
                .selected_result()
                .map(|result| result.action.id)
                .unwrap_or_default(),
            action_name: format!("Copy Command: {action_name}"),
            status: ActionExecutionStatus::Completed,
            message: command.clone(),
            output: Some(serde_json::json!({ "copied": command })),
            created_at: chrono::Utc::now(),
        };
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        execution
    }

    pub(crate) fn review_action_panel_command(&mut self) -> ActionExecution {
        let action = self
            .view
            .selected_result()
            .map(|result| result.action.clone());
        let action_name = action
            .as_ref()
            .map(|action| action.name.clone())
            .unwrap_or_else(|| "Selected Action".to_string());
        let command = action
            .as_ref()
            .and_then(|action| action.examples.first().cloned())
            .unwrap_or_else(|| action_name.clone());
        let execution = ActionExecution {
            action_id: action.as_ref().map(|action| action.id).unwrap_or_default(),
            action_name: format!("Review Command: {action_name}"),
            status: ActionExecutionStatus::NeedsExternalRunner,
            message: command.clone(),
            output: Some(serde_json::json!({
                "deferred": true,
                "reason": "review command before running external action",
                "command": command,
            })),
            created_at: chrono::Utc::now(),
        };
        self.view.last_execution = Some(execution.clone());
        self.view.feedback = Some(std_egui::LauncherFeedback::from_execution(&execution));
        self.view.selected_feedback_action = 0;
        self.view.phase = std_egui::LauncherPhase::Feedback;
        execution
    }
}
