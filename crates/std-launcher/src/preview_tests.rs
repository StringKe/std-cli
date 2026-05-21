use crate::{preview::*, ui};
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
    std::env::set_var("STD_ALLOW_UI_PREVIEW", "0");
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
    assert_eq!(report.scenarios.len(), 20);
    assert!(report
        .commands
        .iter()
        .all(|command| command.starts_with("STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher")));
    assert!(report
        .commands
        .iter()
        .all(|command| command.contains(" --ui-preview light ")
            || command.contains(" --ui-preview dark ")));
    assert_eq!(report.sizes.len(), report.scenarios.len());
    assert!(report.sizes.iter().all(|size| size.contains("=PASS")));
    assert_preview_state_matrix(&report);
    assert_preview_theme_tokens(&report);
    assert_preview_affordance_contract(&report);
    assert_required_capture_state_contract(&report);
    assert_preview_capture_contract(&report);
}

fn assert_preview_state_matrix(report: &LauncherPreviewSmokeReport) {
    for state in [
        "light-empty=PASS",
        "light-collapsed=PASS",
        "dark-searching=PASS",
        "light-loading=PASS",
        "dark-loading=PASS",
        "light-executing=PASS",
        "light-no-results=PASS",
        "dark-error=PASS",
        "light-action-panel=PASS",
    ] {
        assert!(report.states.iter().any(|entry| entry.starts_with(state)));
    }
    assert!(report
        .sizes
        .iter()
        .any(|size| size.starts_with("light-defer=PASS")));
    assert!(report
        .sizes
        .iter()
        .any(|size| size.starts_with("dark-error=PASS")));
}

fn assert_preview_theme_tokens(report: &LauncherPreviewSmokeReport) {
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("panel_token=bg/surface-0:#FAFBFD")
            && state.contains("search_token=bg/surface-1:#F2F5F8")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("panel_token=bg/surface-0:#1C1E22")
            && state.contains("search_token=bg/surface-1:#24272C")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("selected_token=accent/weak:#0A6BFF@31")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("selected_token=accent/weak:#4E9CFF@46")));
}

fn assert_preview_affordance_contract(report: &LauncherPreviewSmokeReport) {
    let summary = report.summary();

    assert!(summary.contains("suggested=3"));
    assert!(summary.contains("ask_ai=true"));
    assert!(summary.contains("feedback_actions=Copy,Retry"));
    assert!(summary.contains("feedback_actions=Copy,Retry,OpenStudio"));
    assert!(summary.contains("feedback_action_shortcuts=Copy:Enter,Retry:Enter"));
    assert!(summary.contains("feedback_action_shortcuts=Copy:Enter,Retry:Enter,OpenStudio:Enter"));
    assert!(summary.contains("action_panel_actions=Review first,Defer,Open in Studio,Copy command"));
}

fn assert_required_capture_state_contract(report: &LauncherPreviewSmokeReport) {
    assert_eq!(
        report.required_capture_states,
        [
            "light-results",
            "dark-results",
            "light-no-results",
            "dark-no-results",
            "light-defer",
            "dark-defer",
            "light-error",
            "dark-error",
            "light-action-panel",
            "dark-action-panel",
        ]
    );

    let summary = report.summary();
    assert!(summary.contains(
        "required_capture_states=light-results,dark-results,light-no-results,dark-no-results,light-defer,dark-defer,light-error,dark-error,light-action-panel,dark-action-panel"
    ));
}

fn assert_preview_capture_contract(report: &LauncherPreviewSmokeReport) {
    let summary = report.summary();

    assert!(summary.contains("preview_capture_contract=native-panel-surface,opt-in-only"));
    assert!(summary.contains("checkout-binary-only"));
    assert!(summary.contains("blocked-in-STD_TEST_MODE"));
    assert!(summary.contains("no-default-window"));
    assert!(summary.contains("no-carrier-background"));
}

#[test]
fn preview_smoke_sizes_prove_capture_window_has_panel_only_surface() {
    let report = LauncherPreviewSmokeReport::new();

    assert!(report.pass(), "{}", report.summary());
    assert!(report
        .summary()
        .contains("preview_sizes=light-collapsed=PASS"));
    assert!(report.summary().contains("light-empty=PASS"));
    assert!(report
        .summary()
        .contains("panel_frame=native_panel_surface"));
    assert!(report
        .summary()
        .contains("search_surface=panel_as_search_surface"));
    assert!(report
        .summary()
        .contains("search_surface=nested_search_surface"));
}

#[test]
fn ui_preview_uses_native_panel_surface_window() {
    let config = LauncherPreviewConfig {
        theme_mode: ThemeMode::Light,
        scenario: "empty".to_string(),
        timeout_ms: 8000,
    };
    let options = preview_native_options_for_config(&config);
    let description = format!("{:?}", options.viewport);

    assert_eq!(preview_window_title(), "std-cli Launcher");
    assert!(description.contains("transparent: Some(true)"));
    assert!(description.contains("decorations: Some(false)"));
    assert!(description.contains("visible: Some(true)"));
    assert_eq!(
        preview_capture_window_contract(&config),
        "native=panel-surface,transparent=true,decorations=false,visible=true,size=720x460"
    );
}

#[test]
fn preview_evidence_names_native_panel_surface_not_preview_viewport() {
    let surface = include_str!("surface_smoke.rs");
    let preview = include_str!("preview.rs");

    assert!(surface.contains("capture_window=panel_surface,opt_in_only"));
    assert!(surface.contains("capture_surface=native_panel_surface"));
    assert!(!surface.contains("preview_viewport="));
    assert!(preview.contains("native-panel-surface"));
    assert!(preview.contains("no-carrier-background"));
    assert!(!preview.contains("preview_viewport"));
    assert!(!preview.contains("preview_viewport_contract"));
}

#[test]
fn ui_preview_window_size_matches_seeded_panel_state() {
    for scenario in ["results", "defer", "error", "action-panel"] {
        let config = LauncherPreviewConfig {
            theme_mode: ThemeMode::Light,
            scenario: scenario.to_string(),
            timeout_ms: 8000,
        };
        let preview_size = preview_window_inner_size(&config);
        let mut state = std_launcher::LauncherState::new();
        apply_preview_scenario(&mut state, scenario);
        let expected_size = ui::launcher_window_inner_size(&state);

        assert!(preview_size.y > ui::launcher_initial_window_inner_size().y);
        assert_eq!(preview_size, expected_size);
    }
}

#[test]
fn ui_preview_scenarios_seed_visible_launcher_states() {
    let mut state = std_launcher::LauncherState::new();

    apply_preview_scenario(&mut state, "no-results");
    assert!(state.view.results.is_empty());
    assert_eq!(state.view.phase, std_egui::LauncherPhase::NoMatches);

    apply_preview_scenario(&mut state, "searching");
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Searching);

    apply_preview_scenario(&mut state, "loading");
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
    assert_eq!(state.action_panel.action_name, "StdFixtureTerminal");

    apply_preview_scenario(&mut state, "error");
    assert_eq!(
        state.view.feedback.as_ref().unwrap().status,
        ActionExecutionStatus::Failed
    );
}
