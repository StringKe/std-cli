use crate::LauncherState;
use std_types::ActionExecution;

impl LauncherState {
    pub fn trigger_selected_primary_action_by_user(&mut self) -> Option<ActionExecution> {
        if self
            .view
            .selected_result()
            .is_some_and(|result| result.action.action_type.needs_external_runner())
        {
            Some(self.review_action_panel_command())
        } else {
            self.trigger_selected_by_user()
        }
    }

    pub fn trigger_result_primary_action_by_user(
        &mut self,
        index: usize,
    ) -> Option<ActionExecution> {
        if index >= self.view.results.len() || index >= 9 {
            return None;
        }
        self.action_panel.close();
        self.view.selected = index;
        self.view.refresh_preview(&self.core);
        self.trigger_selected_primary_action_by_user()
    }
}
