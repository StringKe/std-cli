use crate::{
    preview::*, preview_smoke::StudioPreviewSmokeReport, smoke::smoke_from_args,
    workspace_panes::focused_workspace_spec, StudioEguiApp,
};
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
    assert_eq!(studio_preview_window_title(), "std-cli-Studio");
}

#[test]
fn ui_preview_scenarios_do_not_alias_panes_as_windows_or_viewports() {
    let source = include_str!("preview.rs");

    assert!(source.contains("\"panes\" =>"));
    assert!(!source.contains("\"windows\""));
    assert!(!source.contains("\"viewports\""));
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

    assert_eq!(
        focused_workspace_spec(&app.app)
            .map(|spec| spec.content_key)
            .unwrap_or("none"),
        "workflows"
    );
    assert!(app.workflow_selected_path.is_some());
    assert!(app.app.workflow_debug.is_some());
    assert!(app.app.last_workflow_execution.is_some());
    assert_eq!(app.app.open_workspace_panes().count(), 4);
    assert!(app
        .app
        .open_workspace_panes()
        .any(|pane| pane.kind.content_key() == "history"));
    assert!(app.layout.bottom_panel_open);
    assert_eq!(
        app.bottom_panel_tab,
        crate::bottom_panel_model::BottomPanelTab::BatchDebug
    );
}

#[test]
fn workflow_error_preview_seeds_failed_execution_and_problems_panel() {
    let app = seeded_preview_app("light", "workflow-error");

    assert_eq!(
        focused_workspace_spec(&app.app)
            .map(|spec| spec.content_key)
            .unwrap_or("none"),
        "workflows"
    );
    assert_eq!(
        app.app
            .last_workflow_execution
            .as_ref()
            .map(|execution| &execution.status),
        Some(&std_orchestration::ExecutionStatus::Failed)
    );
    assert_eq!(
        app.bottom_panel_tab,
        crate::bottom_panel_model::BottomPanelTab::Problems
    );
    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_snapshot().status, "1 issues");
}

#[test]
fn preview_smoke_reports_required_studio_screenshot_matrix() {
    let report = StudioPreviewSmokeReport::new();
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_eq!(report.scenarios.len(), 22);
    assert_eq!(report.sizes.len(), report.scenarios.len());
    assert!(report.sizes.iter().all(|size| size.contains("=PASS")));
    assert_preview_summary_has_scenarios(&summary);
    assert_required_capture_state_contract(&report);
    assert_preview_capture_manifest_contract(&report);
    assert_preview_screenshot_acceptance_contract(&report);
    assert_preview_summary_has_surfaces(&summary);
    assert_preview_summary_has_viewport_policy(&summary);
    assert_preview_summary_has_workspace_structure(&summary);
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
    assert!(summary.contains("dark-plugin-permission"));
    assert!(summary.contains("light-plugin-permission"));
    assert!(summary.contains("dark-operations"));
    assert!(summary.contains("light-operations"));
    assert!(summary.contains("dark-memory"));
    assert!(summary.contains("light-memory"));
    assert!(summary.contains("dark-history"));
    assert!(summary.contains("light-history"));
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
            "light-plugin-permission",
            "dark-plugin-permission",
            "light-operations",
            "dark-operations",
            "light-memory",
            "dark-memory",
            "light-history",
            "dark-history",
            "light-settings",
            "dark-settings",
            "light-panes",
            "dark-panes",
        ]
    );

    assert!(report.summary().contains(
        "required_capture_states=light-dashboard,dark-dashboard,light-workflow,dark-workflow,light-workflow-error,dark-workflow-error,light-analysis,dark-analysis,light-plugins,dark-plugins,light-plugin-permission,dark-plugin-permission,light-operations,dark-operations,light-memory,dark-memory,light-history,dark-history,light-settings,dark-settings,light-panes,dark-panes"
    ));
}

fn assert_preview_summary_has_viewport_policy(summary: &str) {
    assert_preview_summary_has_host_policy(summary);
    assert_preview_summary_has_studio_surfaces(summary);
    assert_preview_summary_has_capture_runtime_policy(summary);
    assert_preview_summary_has_workbench_contracts(summary);
}

fn assert_preview_summary_has_host_policy(summary: &str) {
    assert!(summary.contains("light-dashboard=PASS"));
    assert!(summary.contains("dark-dashboard=PASS"));
    assert!(summary.contains("host=1280x800,min=1080x640"));
    assert!(summary.contains("host_layout=1280x800:chrome=52,sidebar=240"));
    assert!(summary.contains("min_layout=1080x640:chrome=52,sidebar=240"));
    assert!(summary.contains("status=24"));
    assert!(summary.contains("fits=true"));
    assert!(summary.contains("native_child_windows=false,detached_panels=false"));
}

fn assert_preview_summary_has_studio_surfaces(summary: &str) {
    assert!(summary.contains("dark-settings=PASS"));
    assert!(summary.contains("light-settings=PASS"));
    assert!(summary.contains("dark-memory=PASS"));
    assert!(summary.contains("light-memory=PASS"));
    assert!(summary.contains("dark-history=PASS"));
    assert!(summary.contains("light-history=PASS"));
    assert!(summary.contains("settings_surface=internal-workspace-pane"));
}

fn assert_preview_summary_has_capture_runtime_policy(summary: &str) {
    assert!(summary.contains("STD_ALLOW_UI_PREVIEW=1"));
    assert!(summary.contains("target/ui-capture/debug/std-studio --ui-preview"));
    assert!(!summary.contains("cargo run -p std-studio -- --ui-preview"));
    assert!(summary.contains("preview_capture_contract=explicit-opt-in-only"));
    assert!(summary.contains("checkout-binary-only"));
    assert!(summary.contains("blocked-in-STD_TEST_MODE"));
    assert!(summary.contains("no-default-window"));
    assert!(summary.contains("normal-viewport-close"));
}

fn assert_preview_summary_has_workbench_contracts(summary: &str) {
    assert!(summary.contains("workflow_e2e=builder|dry-run|execution|trace|history-pane"));
    assert!(summary.contains("workflow_error=failed-execution|problems-panel|error-row"));
    assert!(summary.contains(
        "analysis_preview=coverage=overview:PASS|components:PASS|relations:PASS|history:PASS|complete:PASS"
    ));
    assert!(
        summary.contains("plugin_runtime=runtime=js:Completed:deno_core|ts:Completed:deno_core")
    );
    assert!(summary.contains("plugin_permission=permissions|fs|network|review-prompt"));
    assert!(summary.contains(
        "pane_management=open|focus|switch|close|reopen|restore|state-preserved|single-egui-viewport"
    ));
}

fn assert_preview_summary_has_workspace_structure(summary: &str) {
    for expected in [
        "structure=host:single-borderless-egui-viewport",
        "panes:internal-workspace-panes",
        "shell:host-chrome|sidebar|canvas|status-bar",
        "focused:dashboard",
        "focused:workflows",
        "focused:analysis",
        "focused:plugins",
        "focused:operations",
        "focused:memory",
        "focused:history",
        "focused:settings",
        "bottom_panel:visible",
        "native_child_windows:false",
        "detached_panels:false",
    ] {
        assert!(summary.contains(expected), "{expected}");
    }
}

fn assert_preview_capture_manifest_contract(report: &StudioPreviewSmokeReport) {
    let summary = report.summary();

    assert!(report.capture_manifest.pass(&report.scenarios));
    assert!(
        summary.contains("expected_capture_manifest=artifacts/ui/manual-acceptance/manifest.txt")
    );
    assert!(summary.contains("capture_out_dir=artifacts/ui/manual-acceptance"));
    assert!(summary.contains("expected_capture_files=studio-light-dashboard.png"));
    assert!(summary.contains("studio-dark-panes.png"));
    assert!(summary.contains("capture_command=STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix"));
    assert!(summary.contains("verify_rule=manifest-current-run-png-files-by-theme-state"));
    assert!(summary.contains("source_rule=pid+process-name+window-title-per-capture"));
    assert!(summary.contains(
        "pixel_evidence_rule=samples+opaque_samples+unique_colors+black_pixels+white_pixels+transparent_pixels+edge_samples+edge_transparent_pixels+edge_black_pixels+edge_white_pixels"
    ));
    assert!(summary
        .contains("carrier_reject_rule=reject-single-color+dominant-black+dominant-white+edge-black+edge-white-carrier"));
}

fn assert_preview_screenshot_acceptance_contract(report: &StudioPreviewSmokeReport) {
    let summary = report.summary();

    assert!(report.screenshot_acceptance.pass());
    assert!(summary.contains("studio_screenshot_acceptance PASS"));
    assert!(summary.contains(
        "delivery_capture_states=light-dashboard,dark-dashboard,light-analysis,dark-analysis,light-plugins,dark-plugins,light-operations,dark-operations,light-memory,dark-memory,light-history,dark-history,light-settings,dark-settings"
    ));
    assert!(summary.contains(
        "workflow_capture_states=light-workflow,dark-workflow,light-workflow-error,dark-workflow-error"
    ));
    assert!(summary.contains(
        "diagnostic_capture_states=light-plugin-permission,dark-plugin-permission,light-panes,dark-panes"
    ));
    assert!(summary.contains(
        "evidence_rule=docs22-delivery=theme-baseline+core-workbenches+memory+history+operations+settings;theme-pairs=light|dark"
    ));
    assert!(summary.contains("opt_in_rule=STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless"));
    assert!(summary.contains("capture_verify_rule=manifest-current-run-png-files-by-theme-state"));
    assert!(summary.contains("capture_source_rule=pid+process-name+window-title-per-capture"));
}
