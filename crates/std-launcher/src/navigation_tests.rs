use crate::{LauncherFocusSection, LauncherFocusSource, LauncherKey, LauncherState};
use std_egui::{LauncherFeedback, LauncherFeedbackAction};
use std_types::{ActionExecution, ActionExecutionStatus, ActionId};

#[test]
fn mod_arrow_keys_jump_to_result_edges() {
    let mut state = LauncherState::new();
    state.update_query("index");
    assert!(state.view.results.len() > 1);

    state.handle_keyboard_input(LauncherKey::JumpToLast, false);
    assert_eq!(state.view.selected, state.view.results.len() - 1);

    state.handle_keyboard_input(LauncherKey::JumpToFirst, false);
    assert_eq!(state.view.selected, 0);
}

#[test]
fn mod_arrow_keys_jump_inside_action_panel() {
    let mut state = LauncherState::new();
    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    assert!(state.action_panel.open);
    assert!(state.action_panel.visible_items().len() > 1);

    state.handle_keyboard_input(LauncherKey::JumpToLast, false);
    assert_eq!(
        state.action_panel.selected,
        state.action_panel.visible_items().len() - 1
    );

    state.handle_keyboard_input(LauncherKey::JumpToFirst, false);
    assert_eq!(state.action_panel.selected, 0);
    assert!(state.action_panel.open);
}

#[test]
fn escape_clears_query_before_hiding_launcher() {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query("open app");

    state.handle_keyboard_input(LauncherKey::Escape, false);
    assert_eq!(state.view.query, "");
    assert!(state.controller.visible);

    state.handle_keyboard_input(LauncherKey::Escape, false);
    assert!(!state.controller.visible);
}

#[test]
fn escape_closes_action_panel_before_clearing_query() {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query("open");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    assert!(state.action_panel.open);

    state.handle_keyboard_input(LauncherKey::Escape, false);
    assert!(!state.action_panel.open);
    assert_eq!(state.view.query, "open");
    assert!(state.controller.visible);
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Search));
}

#[test]
fn tab_keys_cycle_launcher_focus_sections_without_wrapping_into_mouse_only_state() {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query("index");
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Search));

    state.handle_keyboard_input(LauncherKey::FocusNext, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Results);
    assert_eq!(state.focus_source, LauncherFocusSource::Keyboard);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Results));

    state.handle_keyboard_input(LauncherKey::FocusNext, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Search);

    state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Results);

    state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
}

#[test]
fn complete_selected_query_uses_current_selected_result_keyword() {
    let mut state = LauncherState::new();
    state.update_query("reb");
    assert_eq!(state.focus_section, LauncherFocusSection::Search);

    state.handle_keyboard_input(LauncherKey::CompleteSelectedQuery, false);

    assert_eq!(state.view.query, "rebuild index");
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Search));
}

#[test]
fn pointer_result_selection_suppresses_keyboard_focus_ring() {
    let mut state = LauncherState::new();
    state.update_query("index");

    state.mark_pointer_focus(LauncherFocusSection::Results);

    assert_eq!(state.focus_section, LauncherFocusSection::Results);
    assert_eq!(state.focus_source, LauncherFocusSource::Pointer);
    assert!(!state.keyboard_focus_visible(LauncherFocusSection::Results));
}

#[test]
fn tab_keys_include_action_panel_when_it_is_open() {
    let mut state = LauncherState::new();
    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    assert_eq!(state.focus_section, LauncherFocusSection::ActionPanel);

    state.handle_keyboard_input(LauncherKey::FocusNext, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Search);

    state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
    assert_eq!(state.focus_section, LauncherFocusSection::ActionPanel);
}

#[test]
fn tab_keys_include_feedback_actions_when_feedback_is_visible() {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query("index");
    state.view.feedback = Some(feedback(ActionExecutionStatus::Failed));
    assert_eq!(state.focus_section, LauncherFocusSection::Search);

    state.handle_keyboard_input(LauncherKey::FocusPrevious, false);

    assert_eq!(state.focus_section, LauncherFocusSection::Feedback);
    assert_eq!(
        state.view.selected_feedback_action(),
        Some(LauncherFeedbackAction::Copy)
    );
}

#[test]
fn feedback_actions_are_keyboard_reachable() {
    let mut state = LauncherState::new();
    state.view.feedback = Some(feedback(ActionExecutionStatus::Failed));
    state.focus_section = LauncherFocusSection::Feedback;

    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    assert_eq!(
        state.view.selected_feedback_action(),
        Some(LauncherFeedbackAction::Retry)
    );
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    assert_eq!(
        state.view.selected_feedback_action(),
        Some(LauncherFeedbackAction::OpenStudio)
    );
    let execution = state.handle_keyboard_input(LauncherKey::Enter, false);

    assert!(execution.is_none());
    assert_eq!(
        state
            .studio_intent
            .as_ref()
            .map(|intent| intent.command.as_str()),
        Some("studio-pane://history")
    );
}

#[test]
fn feedback_copy_updates_shared_model_for_mouse_and_keyboard_paths() {
    let mut state = LauncherState::new();
    state.view.feedback = Some(feedback(ActionExecutionStatus::Failed));
    state.focus_section = LauncherFocusSection::Feedback;

    let execution = state.copy_feedback_to_clipboard_model().unwrap();

    assert_eq!(execution.action_name, "Copy Feedback");
    assert_eq!(execution.status, ActionExecutionStatus::Completed);
    assert_eq!(
        state
            .view
            .last_execution
            .as_ref()
            .map(|execution| execution.action_name.as_str()),
        Some("Copy Feedback")
    );
    assert_eq!(
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.status.clone()),
        Some(ActionExecutionStatus::Completed)
    );
}

fn feedback(status: ActionExecutionStatus) -> LauncherFeedback {
    LauncherFeedback::from_execution(&ActionExecution {
        action_id: ActionId::default(),
        action_name: "Fixture Feedback".to_string(),
        status,
        message: "fixture feedback".to_string(),
        output: None,
        created_at: chrono::Utc::now(),
    })
}

#[test]
fn ime_composition_blocks_focus_section_shortcuts() {
    let mut state = LauncherState::new();
    state.update_query("index");

    state.handle_keyboard_input(LauncherKey::FocusNext, true);

    assert_eq!(state.focus_section, LauncherFocusSection::Search);
}

#[test]
fn ime_composition_blocks_launcher_action_navigation_and_escape() {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let selected_before = state.view.selected;
    let action_panel_before = state.action_panel.selected;

    let execution = state.handle_keyboard_input(LauncherKey::Enter, true);
    state.handle_keyboard_input(LauncherKey::ArrowDown, true);
    state.handle_keyboard_input(LauncherKey::Escape, true);

    assert!(execution.is_none());
    assert!(state.controller.visible);
    assert!(state.action_panel.open);
    assert_eq!(state.view.query, "terminal");
    assert_eq!(state.view.selected, selected_before);
    assert_eq!(state.action_panel.selected, action_panel_before);
    assert_eq!(state.focus_section, LauncherFocusSection::ActionPanel);
}

#[test]
fn no_match_enter_uses_ask_ai_fallback() {
    let mut state = LauncherState::new();
    state.update_query("missing launcher item");

    let execution = state.handle_keyboard_input(LauncherKey::Enter, false);

    assert!(execution.is_none());
    assert_eq!(state.view.query, "? missing launcher item");
}

#[test]
fn no_match_enter_respects_ime_composition() {
    let mut state = LauncherState::new();
    state.update_query("missing launcher item");

    let execution = state.handle_keyboard_input(LauncherKey::Enter, true);

    assert!(execution.is_none());
    assert_eq!(state.view.query, "missing launcher item");
}

#[test]
fn empty_query_suggestions_are_keyboard_reachable() {
    let mut state = LauncherState::new();
    state.update_query("");
    assert!(state.empty_query_suggestions_visible());
    assert_eq!(state.empty_suggestion_selected, 0);

    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    assert_eq!(state.empty_suggestion_selected, 1);
    assert_eq!(state.focus_section, LauncherFocusSection::Results);
    assert!(state.keyboard_focus_visible(LauncherFocusSection::Results));

    state.handle_keyboard_input(LauncherKey::JumpToLast, false);
    assert_eq!(state.empty_suggestion_selected, 2);

    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    assert_eq!(state.empty_suggestion_selected, 2);

    let execution = state.handle_keyboard_input(LauncherKey::Enter, false);

    assert!(execution.is_none());
    assert_eq!(state.view.query, "> studio");
    assert_eq!(state.empty_suggestion_selected, 0);
}

#[test]
fn empty_query_suggestions_respect_ime_composition() {
    let mut state = LauncherState::new();
    state.update_query("");

    state.handle_keyboard_input(LauncherKey::ArrowDown, true);
    let execution = state.handle_keyboard_input(LauncherKey::Enter, true);

    assert!(execution.is_none());
    assert_eq!(state.empty_suggestion_selected, 0);
    assert_eq!(state.view.query, "");
}
