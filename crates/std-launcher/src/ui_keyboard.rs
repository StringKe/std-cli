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
    if input::ime_action_guard(ctx).blocks_actions() {
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
    if input::launcher_defer().pressed(ctx) {
        handle_pinned_user_execution(state);
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

fn handle_pinned_user_execution(state: &mut LauncherState) {
    let _ = state.handle_keyboard_input_by_user(LauncherKey::Enter, false);
}

pub(crate) fn execution_hides_launcher(execution: &ActionExecution) -> bool {
    launcher_execution_hides_window(execution)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_core::{StdConfig, StdCore};

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
        let guard_index = source.find("input::ime_action_guard(ctx)").unwrap();
        let cancel_index = source
            .find("input::launcher_cancel().pressed(ctx)")
            .unwrap();
        let enter_index = source.find("input::enter().pressed(ctx)").unwrap();
        let pinned_index = source.find("input::launcher_defer().pressed(ctx)").unwrap();
        let executing_index = source.find("LauncherPhase::Executing").unwrap();
        let user_trigger_index = source
            .find("state.handle_keyboard_input_by_user(key, false)")
            .unwrap();

        assert!(guard_index < cancel_index);
        assert!(cancel_index < enter_index);
        assert!(enter_index < pinned_index);
        assert!(executing_index < user_trigger_index);
    }

    #[test]
    fn launcher_pinned_execution_keeps_window_open_without_hide_request() {
        let ctx = egui::Context::default();
        let mut state = LauncherState::new();
        state.controller.show();
        state.update_query("index");
        let mut hide_requested = false;

        let _ = ctx.run(mod_shift_enter_input(), |ctx| {
            handle_search_shortcuts(ctx, &mut state, &mut hide_requested);
        });

        assert_eq!(
            state
                .view
                .last_execution
                .as_ref()
                .map(|execution| execution.status.clone()),
            Some(ActionExecutionStatus::Completed)
        );
        assert!(state.controller.visible);
        assert!(!hide_requested);
    }

    #[test]
    fn ui_enter_requests_hide_only_for_completed_execution() {
        let ctx = egui::Context::default();
        let mut completed = LauncherState::new();
        completed.controller.show();
        completed.update_query("index");
        let mut completed_hide = false;

        let _ = ctx.run(enter_input(), |ctx| {
            handle_search_shortcuts(ctx, &mut completed, &mut completed_hide);
        });

        assert_eq!(
            completed
                .view
                .last_execution
                .as_ref()
                .map(|execution| execution.status.clone()),
            Some(ActionExecutionStatus::Completed)
        );
        assert!(completed_hide);

        let mut deferred = launcher_with_fixture_app();
        deferred.controller.show();
        deferred.update_query("UI Enter Fixture");
        let mut deferred_hide = false;

        let _ = ctx.run(enter_input(), |ctx| {
            handle_search_shortcuts(ctx, &mut deferred, &mut deferred_hide);
        });

        assert_eq!(
            deferred
                .view
                .last_execution
                .as_ref()
                .map(|execution| execution.status.clone()),
            Some(ActionExecutionStatus::NeedsExternalRunner)
        );
        assert!(!deferred_hide);
        assert!(deferred.controller.visible);
    }

    fn mod_shift_enter_input() -> egui::RawInput {
        let modifiers = egui::Modifiers {
            command: true,
            shift: true,
            ..Default::default()
        };
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers,
            }],
            modifiers,
            ..Default::default()
        }
    }

    fn enter_input() -> egui::RawInput {
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..Default::default()
        }
    }

    fn launcher_with_fixture_app() -> LauncherState {
        let root = std::env::temp_dir().join(format!(
            "std-launcher-ui-enter-fixture-{}",
            std::process::id()
        ));
        let config = StdConfig {
            data_dir: root.join("data"),
            ..StdConfig::default()
        };
        let app = config.apps_dir().join("UIEnterFixture.app");
        let contents = app.join("Contents");
        let _ = std::fs::create_dir_all(&contents);
        let _ = std::fs::write(
            contents.join("Info.plist"),
            r#"<plist><dict>
<key>CFBundleDisplayName</key><string>UI Enter Fixture</string>
<key>CFBundleName</key><string>UIEnterFixture</string>
</dict></plist>"#,
        );
        LauncherState::with_core(StdCore::with_config(config))
    }

    #[test]
    fn ime_preedit_frame_owns_launcher_shortcuts() {
        let ctx = egui::Context::default();
        let mut state = LauncherState::new();
        state.update_query("index");
        state.view.preview_executing();
        let before_query = state.view.query.clone();
        let before_selected = state.view.selected;
        let mut hide_requested = false;

        let _ = ctx.run(ime_preedit_enter_input(), |ctx| {
            handle_search_shortcuts(ctx, &mut state, &mut hide_requested);
        });

        assert_eq!(state.view.query, before_query);
        assert_eq!(state.view.selected, before_selected);
        assert_eq!(state.view.phase, std_egui::LauncherPhase::Executing);
        assert!(state.view.feedback.is_none());
        assert!(!state.action_panel.open);
        assert!(!hide_requested);
    }

    fn ime_preedit_enter_input() -> egui::RawInput {
        egui::RawInput {
            events: vec![
                egui::Event::Ime(egui::ImeEvent::Preedit("zhong".to_string())),
                egui::Event::Key {
                    key: egui::Key::Enter,
                    physical_key: Some(egui::Key::Enter),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::Key {
                    key: egui::Key::C,
                    physical_key: Some(egui::Key::C),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers {
                        ctrl: true,
                        ..Default::default()
                    },
                },
                egui::Event::Key {
                    key: egui::Key::ArrowDown,
                    physical_key: Some(egui::Key::ArrowDown),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
            ..Default::default()
        }
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
