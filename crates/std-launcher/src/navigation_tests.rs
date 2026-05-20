use crate::{LauncherKey, LauncherState};

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
}
