use crate::{preview::*, ui};
use eframe::egui;
use std_egui::tokens::ThemeMode;
use std_types::ActionExecutionStatus;

#[test]
fn ui_preview_args_are_explicit_opt_in() {
    let args = vec![
        "std-launcher".to_string(),
        "--ui-preview".to_string(),
        "light".to_string(),
        "defer".to_string(),
        "1200".to_string(),
    ];
    let config = preview_config_from_args(&args).unwrap();

    assert_eq!(config.theme_mode, ThemeMode::Light);
    assert_eq!(config.scenario, "defer");
    assert_eq!(config.timeout_ms, 1200);
}

#[test]
fn ui_preview_args_are_blocked_without_opt_in() {
    std::env::remove_var("STD_ALLOW_UI_PREVIEW");
    let args = vec![
        "std-launcher".to_string(),
        "--ui-preview".to_string(),
        "light".to_string(),
        "defer".to_string(),
        "1200".to_string(),
    ];

    let Some(LauncherPreviewRequest::Blocked(reason)) = preview_request_from_args(&args) else {
        panic!("expected blocked UI preview request");
    };
    assert!(reason.contains("STD_TEST_MODE blocked UI preview"));
    assert!(blocked_preview_summary(&reason).contains("launcher_ui_preview SKIP"));
}

#[test]
fn preview_smoke_commands_match_ui_preview_parser_contract() {
    let report = LauncherPreviewSmokeReport::new();

    assert!(report.pass(), "{}", report.summary());
    assert_eq!(report.scenarios.len(), 16);
    assert!(report
        .commands
        .iter()
        .all(|command| command.starts_with("STD_ALLOW_UI_PREVIEW=1 ")));
    assert!(report
        .commands
        .iter()
        .all(|command| command.contains(" --ui-preview light ")
            || command.contains(" --ui-preview dark ")));
    assert!(report
        .states
        .iter()
        .any(|state| state.starts_with("light-empty=PASS")));
    assert_eq!(report.sizes.len(), report.scenarios.len());
    assert!(report.sizes.iter().all(|size| size.contains("=PASS")));
    assert!(report
        .sizes
        .iter()
        .any(|size| size.starts_with("light-defer=PASS")));
    assert!(report
        .sizes
        .iter()
        .any(|size| size.starts_with("dark-error=PASS")));
    assert!(report.summary().contains("preview_sizes=light-empty=PASS"));
    assert!(report.summary().contains("bottom_clearance=0"));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("panel=#FAFBFD,search=#F2F5F8")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("panel=#1C1E22,search=#24272C")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("selected=#0A6BFF@31")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("selected=#4E9CFF@46")));
    assert!(report
        .states
        .iter()
        .any(|state| state.starts_with("dark-searching=PASS")));
    assert!(report
        .states
        .iter()
        .any(|state| state.starts_with("light-executing=PASS")));
    assert!(report
        .states
        .iter()
        .any(|state| state.starts_with("light-no-results=PASS")));
    assert!(report
        .states
        .iter()
        .any(|state| state.starts_with("dark-error=PASS")));
    assert!(report
        .states
        .iter()
        .any(|state| state.starts_with("light-action-panel=PASS")));
    assert!(report
        .summary()
        .contains("preview_capture_contract=explicit-opt-in-only"));
    assert!(report.summary().contains("blocked-in-STD_TEST_MODE"));
    assert!(report.summary().contains("no-default-window"));
}

#[test]
fn ui_preview_uses_transparent_visible_chrome() {
    let options = preview_native_options();
    let description = format!("{:?}", options.viewport);

    assert_eq!(preview_window_title(), "std-cli Launcher");
    assert!(description.contains("transparent: Some(true)"));
    assert!(description.contains("decorations: Some(false)"));
    assert!(description.contains("visible: Some(true)"));
    assert_eq!(
        ui::launcher_initial_window_inner_size(),
        egui::vec2(720.0, 64.0)
    );
}

#[test]
fn ui_preview_scenarios_seed_visible_launcher_states() {
    let mut state = std_launcher::LauncherState::new();

    apply_preview_scenario(&mut state, "no-results");
    assert!(state.view.results.is_empty());
    assert_eq!(state.view.phase, std_egui::LauncherPhase::NoMatches);

    apply_preview_scenario(&mut state, "searching");
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Searching);

    apply_preview_scenario(&mut state, "executing");
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Executing);

    apply_preview_scenario(&mut state, "defer");
    assert_eq!(
        state.view.feedback.as_ref().unwrap().status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Feedback);

    apply_preview_scenario(&mut state, "action-panel");
    assert!(state.action_panel.open);
    assert_eq!(state.action_panel.action_name, "Open Terminal");

    apply_preview_scenario(&mut state, "error");
    assert_eq!(
        state.view.feedback.as_ref().unwrap().status,
        ActionExecutionStatus::Failed
    );
}
