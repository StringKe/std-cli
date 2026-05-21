use crate::{preview::*, smoke::smoke_from_args, StudioEguiApp, StudioPane};
use std_core::{StdConfig, StdCore};

#[test]
fn ui_preview_args_are_explicit_opt_in() {
    let args = vec![
        "std-studio".to_string(),
        "--ui-preview".to_string(),
        "light".to_string(),
        "panes".to_string(),
        "900".to_string(),
    ];
    let config = studio_preview_config_from_args(&args).unwrap();

    assert_eq!(config.theme, "light");
    assert_eq!(config.scenario, "panes");
    assert_eq!(config.timeout_ms, 900);
    assert!(smoke_from_args(args).is_none());
}

#[test]
fn ui_preview_uses_product_window_title() {
    assert_eq!(studio_preview_window_title(), "std-cli Studio");
}

#[test]
fn ui_preview_args_are_blocked_without_opt_in() {
    std::env::remove_var("STD_ALLOW_UI_PREVIEW");
    let args = vec![
        "std-studio".to_string(),
        "--ui-preview".to_string(),
        "light".to_string(),
        "panes".to_string(),
        "900".to_string(),
    ];

    let Some(StudioPreviewRequest::Blocked(reason)) = studio_preview_request_from_args(&args)
    else {
        panic!("expected blocked Studio UI preview request");
    };
    assert!(reason.contains("STD_TEST_MODE blocked Studio UI preview"));
    assert!(blocked_studio_preview_summary(&reason).contains("studio_ui_preview SKIP"));
}

#[test]
fn workflow_preview_seeds_builder_runtime_state() {
    let core = StdCore::with_config(StdConfig {
        data_dir: preview_data_dir(),
        ..StdConfig::default()
    });
    let mut app = StudioEguiApp {
        app: std_studio::StudioApp::with_core(core),
        ..StudioEguiApp::default()
    };

    seed_workflow_preview(&mut app);

    assert_eq!(app.app.active_pane, StudioPane::Workflows);
    assert!(app.workflow_selected_path.is_some());
    assert!(app.app.workflow_debug.is_some());
    assert!(app.app.last_workflow_execution.is_some());
    assert_eq!(app.app.open_workspace_panes().count(), 1);
}

#[test]
fn preview_smoke_reports_required_studio_screenshot_matrix() {
    let report = StudioPreviewSmokeReport::new();
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_eq!(report.scenarios.len(), 14);
    assert!(summary.contains("studio_preview_smoke PASS"));
    assert!(summary.contains("dark-dashboard"));
    assert!(summary.contains("dark-workflow"));
    assert!(summary.contains("light-workflow"));
    assert!(summary.contains("dark-analysis"));
    assert!(summary.contains("light-analysis"));
    assert!(summary.contains("dark-plugins"));
    assert!(summary.contains("light-plugins"));
    assert!(summary.contains("dark-operations"));
    assert!(summary.contains("light-operations"));
    assert!(summary.contains("dark-settings"));
    assert!(summary.contains("light-settings"));
    assert!(summary.contains("dark-panes"));
    assert!(summary.contains("light-panes"));
    assert!(summary.contains("STD_ALLOW_UI_PREVIEW=1"));
    assert!(summary.contains("preview_capture_contract=explicit-opt-in-only"));
    assert!(summary.contains("blocked-in-STD_TEST_MODE"));
    assert!(summary.contains("no-default-window"));
}
