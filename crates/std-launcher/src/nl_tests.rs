use super::*;
use std_core::StdConfig;
use std_types::{Action, ActionExecutionStatus, ActionType, RegistryEntry};

#[test]
fn launcher_state_defers_external_runner_actions() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.register_action(fixture_app_action(temp.path()))
        .unwrap();
    let mut state = LauncherState::with_core(core);

    let preview = state.update_query("StdNeverLaunchFixture").unwrap();
    let execution = state.trigger_selected().unwrap();

    assert_eq!(preview.title, "Open App: StdNeverLaunchFixture");
    assert_eq!(execution.action_name, "Open App: StdNeverLaunchFixture");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    let feedback = state.view.feedback.as_ref().unwrap();
    assert_eq!(
        feedback.title,
        std_egui::i18n::t("launcher.feedback.deferred")
    );
    assert!(feedback.deferred);
    assert_eq!(
        feedback.detail,
        std_egui::i18n::t("launcher.feedback.deferred.detail")
    );
    assert_eq!(
        execution
            .output
            .as_ref()
            .unwrap()
            .get("deferred")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
}

#[test]
fn launcher_natural_language_enter_defers_ai_planner() {
    let mut state = LauncherState::new();

    state.update_query("? rebuild index");
    let execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(execution.action_name, "Natural Language: Ask AI");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
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
fn launcher_natural_language_search_actions_keeps_keyboard_flow() {
    let mut state = LauncherState::new();

    state.update_query("? rebuild index");
    state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let execution = state.handle_keyboard_input(LauncherKey::Enter, false);

    assert!(execution.is_none());
    assert_eq!(state.view.query, "> rebuild index");
    assert!(state.view.results.iter().all(|result| matches!(
        result.action.action_type,
        ActionType::Command | ActionType::Workflow
    )));
    assert_eq!(state.focus_section, LauncherFocusSection::Search);
}

fn fixture_app_action(root: &std::path::Path) -> RegistryEntry {
    let app = root.join("StdNeverLaunchFixture.app");
    RegistryEntry::from_action(
        Action::new(
            "Open App: StdNeverLaunchFixture",
            format!("Launch fixture app at {}", app.display()),
            "When testing external runner deferral",
            ActionType::AppLaunch,
        ),
        vec!["app".to_string(), "fixture".to_string()],
    )
    .with_metadata("path", app.display().to_string())
}
