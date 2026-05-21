use eframe::egui;
use std_egui::input;
use std_launcher::{launcher_execution_hides_window, LauncherKey, LauncherState};
use std_types::ActionExecution;

#[cfg(test)]
use std_types::ActionExecutionStatus;

pub(crate) fn handle_search_shortcuts(
    ctx: &egui::Context,
    state: &mut LauncherState,
    hide_requested: &mut bool,
) {
    if input::ime_composing(ctx) {
        return;
    }
    if input::launcher_cancel().pressed(ctx) {
        state.handle_keyboard_input(LauncherKey::CancelExecuting, false);
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
    } else if input::tab().pressed(ctx)
        && state.focus_section == std_launcher::LauncherFocusSection::Search
    {
        state.handle_keyboard_input(LauncherKey::CompleteSelectedQuery, false);
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
    if key == LauncherKey::Enter && state.view.phase == std_egui::LauncherPhase::Executing {
        state.handle_keyboard_input(LauncherKey::MoveExecutingToBackground, false);
        *hide_requested = true;
        return;
    }
    if let Some(execution) = state.handle_keyboard_input_by_user(key, false) {
        *hide_requested = execution_hides_launcher(&execution);
    }
}

pub(crate) fn execution_hides_launcher(execution: &ActionExecution) -> bool {
    launcher_execution_hides_window(execution)
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

    #[test]
    fn ui_keyboard_routes_executing_enter_and_cancel_before_normal_trigger() {
        let source = include_str!("ui_keyboard.rs");
        let cancel_index = source
            .find("input::launcher_cancel().pressed(ctx)")
            .unwrap();
        let enter_index = source.find("input::enter().pressed(ctx)").unwrap();
        let executing_index = source.find("LauncherPhase::Executing").unwrap();
        let user_trigger_index = source
            .find("state.handle_keyboard_input_by_user(key, false)")
            .unwrap();

        assert!(cancel_index < enter_index);
        assert!(executing_index < user_trigger_index);
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
