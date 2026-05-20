use crate::{LauncherFocusSection, LauncherKey, LauncherState};

#[test]
fn mod_arrow_keys_jump_to_result_edges() {
    let mut state = LauncherState::new();
    state.update_query("");
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
    assert_eq!(state.focus_section, LauncherFocusSection::Results);
}

#[test]
fn tab_keys_cycle_launcher_focus_sections_without_wrapping_into_mouse_only_state() {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query("index");
    assert_eq!(state.focus_section, LauncherFocusSection::Search);

    state.handle_keyboard_input(LauncherKey::FocusNext, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Results);

    state.handle_keyboard_input(LauncherKey::FocusNext, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Search);

    state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Results);

    state.handle_keyboard_input(LauncherKey::FocusPrevious, false);
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
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
fn ime_composition_blocks_focus_section_shortcuts() {
    let mut state = LauncherState::new();
    state.update_query("index");

    state.handle_keyboard_input(LauncherKey::FocusNext, true);

    assert_eq!(state.focus_section, LauncherFocusSection::Search);
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
