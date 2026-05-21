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
fn ui_preview_closes_with_viewport_command_not_process_exit() {
    let source = include_str!("preview.rs");

    assert!(!source.contains("process::exit"));
    assert!(source.contains("ViewportCommand::Close"));
}

#[test]
fn ui_preview_args_are_blocked_without_opt_in() {
    std::env::set_var("STD_ALLOW_UI_PREVIEW", "0");
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
    assert_eq!(app.app.open_workspace_panes().count(), 2);
    assert!(app.layout.bottom_panel_open);
}

#[test]
fn workflow_error_preview_seeds_failed_execution_and_problems_panel() {
    let app = seeded_preview_app("light", "workflow-error");

    assert_eq!(app.app.active_pane, StudioPane::Workflows);
    assert_eq!(
        app.app
            .last_workflow_execution
            .as_ref()
            .map(|execution| &execution.status),
        Some(&std_orchestration::ExecutionStatus::Failed)
    );
    assert_eq!(
        app.bottom_panel_tab,
        crate::bottom_panel::BottomPanelTab::Problems
    );
    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_snapshot().status, "1 issues");
}

#[test]
fn preview_smoke_reports_required_studio_screenshot_matrix() {
    let report = StudioPreviewSmokeReport::new();
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_eq!(report.scenarios.len(), 16);
    assert_eq!(report.sizes.len(), report.scenarios.len());
    assert!(report.sizes.iter().all(|size| size.contains("=PASS")));
    assert_preview_summary_has_scenarios(&summary);
    assert_required_capture_state_contract(&report);
    assert_preview_summary_has_surfaces(&summary);
    assert_preview_summary_has_viewport_policy(&summary);
}

fn assert_preview_summary_has_scenarios(summary: &str) {
    assert!(summary.contains("studio_preview_smoke PASS"));
    assert!(summary.contains("dark-dashboard"));
    assert!(summary.contains("dark-workflow"));
    assert!(summary.contains("light-workflow"));
    assert!(summary.contains("dark-workflow-error"));
    assert!(summary.contains("light-workflow-error"));
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
}

fn assert_preview_summary_has_surfaces(summary: &str) {
    assert!(summary.contains("canvas_token=bg/surface-0:#1C1E22"));
    assert!(summary.contains("canvas_token=bg/surface-0:#FAFBFD"));
    assert!(summary.contains("panel_token=bg/surface-1:#24272C"));
    assert!(summary.contains("panel_token=bg/surface-1:#F2F5F8"));
    assert!(summary.contains("inspector_token=bg/surface-1:#24272C"));
    assert!(summary.contains("inspector_token=bg/surface-1:#F2F5F8"));
    assert!(summary.contains("selected_token=accent/weak:#4E9CFF@46"));
    assert!(summary.contains("selected_token=accent/weak:#0A6BFF@31"));
}

fn assert_required_capture_state_contract(report: &StudioPreviewSmokeReport) {
    assert_eq!(
        report.required_capture_states,
        [
            "light-dashboard",
            "dark-dashboard",
            "light-workflow",
            "dark-workflow",
            "light-workflow-error",
            "dark-workflow-error",
            "light-analysis",
            "dark-analysis",
            "light-plugins",
            "dark-plugins",
            "light-operations",
            "dark-operations",
            "light-settings",
            "dark-settings",
            "light-panes",
            "dark-panes",
        ]
    );

    assert!(report.summary().contains(
        "required_capture_states=light-dashboard,dark-dashboard,light-workflow,dark-workflow,light-workflow-error,dark-workflow-error,light-analysis,dark-analysis,light-plugins,dark-plugins,light-operations,dark-operations,light-settings,dark-settings,light-panes,dark-panes"
    ));
}

fn assert_preview_summary_has_viewport_policy(summary: &str) {
    assert!(summary.contains("preview_sizes=dark-dashboard=PASS"));
    assert!(summary.contains("host=1280x800,min=1080x640"));
    assert!(summary.contains("native_child_windows=false,detached_panels=false"));
    assert!(summary.contains("dark-settings=PASS"));
    assert!(summary.contains("light-settings=PASS"));
    assert!(summary.contains("settings_surface=internal-workspace-pane"));
    assert!(summary.contains("STD_ALLOW_UI_PREVIEW=1"));
    assert!(summary.contains("cargo run -p std-studio -- --ui-preview"));
    assert!(summary.contains("workflow_e2e=builder|dry-run|execution|trace|history-pane"));
    assert!(summary.contains("workflow_error=failed-execution|problems-panel|error-row"));
    assert!(summary
        .contains("pane_management=open|focus|close|restore|state-preserved|single-egui-viewport"));
    assert!(summary.contains("preview_capture_contract=explicit-opt-in-only"));
    assert!(summary.contains("checkout-binary-only"));
    assert!(summary.contains("blocked-in-STD_TEST_MODE"));
    assert!(summary.contains("no-default-window"));
    assert!(summary.contains("normal-viewport-close"));
}
