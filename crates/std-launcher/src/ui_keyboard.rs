use eframe::egui;
use std_egui::{input, tokens};
use std_launcher::{LauncherKey, LauncherState};
use std_types::{ActionExecution, ActionExecutionStatus};

pub(crate) fn handle_search_shortcuts(
    ctx: &egui::Context,
    state: &mut LauncherState,
    hide_requested: &mut bool,
) {
    if tokens::ime_composing(ctx) {
        return;
    }
    if input::mod_arrow_down().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::JumpToLast, false);
    } else if input::arrow_down().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    }
    if input::mod_arrow_up().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::JumpToFirst, false);
    } else if input::arrow_up().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::ArrowUp, false);
    }
    if input::enter().pressed(ctx) {
        handle_user_execution(state, LauncherKey::Enter, hide_requested);
    }
    if input::shift_tab().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
    } else if input::tab().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::FocusNext, false);
    }
    if input::launcher_action_panel().pressed(ctx) {
        state.handle_keyboard_input_by_user(LauncherKey::ActionPanel, false);
    }
    if input::launcher_delete_previous_token().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::DeletePreviousToken, false);
    }
    if let Some(index) = input::pressed_mod_number(ctx, 9) {
        handle_user_execution(state, LauncherKey::TriggerResult(index), hide_requested);
    }
}

fn handle_user_execution(state: &mut LauncherState, key: LauncherKey, hide_requested: &mut bool) {
    if let Some(execution) = state.handle_keyboard_input_by_user(key, false) {
        *hide_requested = execution_hides_launcher(&execution);
    }
}

pub(crate) fn execution_hides_launcher(execution: &ActionExecution) -> bool {
    execution.status == ActionExecutionStatus::Completed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_hides_only_after_completed_execution() {
        assert!(execution_hides_launcher(&execution(
            ActionExecutionStatus::Completed
        )));
        assert!(!execution_hides_launcher(&execution(
            ActionExecutionStatus::NeedsExternalRunner
        )));
        assert!(!execution_hides_launcher(&execution(
            ActionExecutionStatus::Failed
        )));
    }

    fn execution(status: ActionExecutionStatus) -> ActionExecution {
        ActionExecution {
            action_id: Default::default(),
            action_name: "Fixture".to_string(),
            status,
            message: "fixture".to_string(),
            output: None,
            created_at: chrono::Utc::now(),
        }
    }
}
