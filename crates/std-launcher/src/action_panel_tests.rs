use super::*;
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

#[test]
fn action_panel_opens_for_selected_result_and_closes_on_search_change() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    let opened = state.open_action_panel();
    let first_item = state
        .action_panel
        .selected_item()
        .unwrap()
        .title()
        .to_string();
    state.update_query("index");

    assert!(opened);
    assert!(state.action_panel.action_name.contains("Terminal"));
    assert_eq!(first_item, "Review first");
    assert!(!state.action_panel.open);
}

#[test]
fn action_panel_includes_open_in_studio_for_launcher_results() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("index");
    state.open_action_panel();

    let titles = state
        .action_panel
        .items
        .iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>();

    assert!(titles.contains(&"Open in Studio".to_string()));
}

#[test]
fn action_panel_labels_external_primary_as_review_first() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.open_action_panel();

    let titles = state
        .action_panel
        .items
        .iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>();

    assert_eq!(titles.first().unwrap(), "Review first");
    assert!(titles.contains(&"Defer".to_string()));
    assert!(!titles.iter().any(|title| title == "Run"));
}

#[test]
fn action_panel_labels_safe_primary_as_run() {
    let mut state = LauncherState::new();

    state.update_query("index");
    state.open_action_panel();

    assert_eq!(state.action_panel.items.first().unwrap().title(), "Run");
}

#[test]
fn action_panel_open_in_studio_records_intent_without_launching() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("index");
    state.open_action_panel();
    state.update_action_panel_query("studio");
    let execution = state.trigger_action_panel_selection();

    assert!(execution.is_none());
    let intent = state.studio_intent.unwrap();
    assert_eq!(intent.command, "studio-pane://analysis");
    assert_eq!(intent.source_action, "Rebuild Index");
}

#[test]
fn action_panel_keyboard_path_defers_external_runner_by_default() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(state.action_panel.selected_item().unwrap().title(), "Defer");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
}

#[test]
fn action_panel_default_enter_on_review_first_still_defers_external_runner() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(
        state.action_panel.selected_item().unwrap().title(),
        "Review first"
    );
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert!(execution.action_name.starts_with("Review Command:"));
    assert_eq!(
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.deferred),
        Some(true)
    );
}

#[test]
fn review_first_shows_command_without_triggering_external_action() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(
        state.action_panel.selected_item().unwrap().title(),
        "Review first"
    );
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        execution
            .output
            .as_ref()
            .and_then(|output| output.get("reason"))
            .and_then(|value| value.as_str()),
        Some("review command before running external action")
    );
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Feedback);
}

#[test]
fn action_panel_selection_api_separates_default_and_user_enter_routes() {
    let source = include_str!("action_panel_state.rs");
    let default_route = source
        .find("pub fn trigger_action_panel_selection(&mut self)")
        .unwrap();
    let user_route = source
        .find("pub fn trigger_action_panel_selection_by_user(&mut self)")
        .unwrap();
    let false_route = source
        .find("self.trigger_action_panel_selection_with_external_runner(false)")
        .unwrap();
    let true_route = source
        .find("self.trigger_action_panel_selection_with_external_runner(true)")
        .unwrap();

    assert!(default_route < false_route);
    assert!(user_route < true_route);
    assert!(source.contains("ActionPanelItem::Run"));
    assert!(source.contains("ActionPanelItem::Run =>"));
    assert!(source.contains("self.review_action_panel_command()"));
    assert!(source.contains("ActionPanelItem::ReviewFirst =>"));
}

#[test]
fn keyboard_enter_uses_user_route_only_for_explicit_user_execution() {
    let source = include_str!("keyboard.rs");
    let action_panel_branch = source.find("if self.action_panel.open").unwrap();
    let by_user = source
        .find("self.trigger_action_panel_selection_by_user()")
        .unwrap();
    let default = source
        .find("self.trigger_action_panel_selection()")
        .unwrap();

    assert!(action_panel_branch < by_user);
    assert!(by_user < default);
    assert!(source.contains("if allow_external_runner"));
}

#[test]
fn action_panel_ime_blocks_open_and_trigger() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("index");
    let open = state.handle_keyboard_input(LauncherKey::ActionPanel, true);
    state.open_action_panel();
    let trigger = state.handle_keyboard_input(LauncherKey::Enter, true);

    assert!(open.is_none());
    assert!(state.action_panel.open);
    assert!(trigger.is_none());
    assert!(state.view.feedback.is_none());
}

#[test]
fn action_panel_filters_actions_and_resets_selection() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.open_action_panel();
    state.move_action_panel_selection(1);
    state.update_action_panel_query("copy");

    let visible_titles = state
        .action_panel
        .visible_items()
        .into_iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>();

    assert_eq!(state.action_panel.selected, 0);
    assert_eq!(visible_titles, vec!["Copy command"]);
    assert_eq!(
        state.action_panel.selected_item().unwrap().title(),
        "Copy command"
    );
}

#[test]
fn action_panel_filter_is_keyboard_reachable_from_panel_focus() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    state.handle_keyboard_input(LauncherKey::TypeActionPanelQuery('c'), false);
    state.handle_keyboard_input(LauncherKey::TypeActionPanelQuery('o'), false);

    let visible_titles = state
        .action_panel
        .visible_items()
        .into_iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>();

    assert_eq!(state.action_panel.query, "co");
    assert_eq!(state.action_panel.selected, 0);
    assert_eq!(visible_titles, vec!["Copy command"]);
}

#[test]
fn action_panel_filter_respects_ime_composition() {
    let mut state = LauncherState::new();
    state.update_query("terminal");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);

    state.handle_keyboard_input(LauncherKey::TypeActionPanelQuery('c'), true);

    assert_eq!(state.action_panel.query, "");
    assert!(state.action_panel.open);
}

#[test]
fn mod_backspace_deletes_previous_query_token() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("  open   terminal now ");
    state.handle_keyboard_input(LauncherKey::DeletePreviousToken, false);

    assert_eq!(state.view.query, "open terminal");
}
