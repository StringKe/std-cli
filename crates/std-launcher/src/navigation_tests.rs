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
    state.update_query("");
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
