use crate::studio_open::*;
use std_studio::StudioPane;

#[test]
fn parses_supported_open_requests() {
    for (target, request) in [
        ("analysis", StudioOpenRequest::Analysis),
        ("apps", StudioOpenRequest::Apps),
        ("history", StudioOpenRequest::History),
        ("memory", StudioOpenRequest::Memory),
        ("plugins", StudioOpenRequest::Plugins),
        ("settings", StudioOpenRequest::Settings),
        ("workflows", StudioOpenRequest::Workflows),
    ] {
        let args = open_args(target);
        assert_eq!(studio_open_request_from_args(&args), Some(request));
    }
}

#[test]
fn applies_open_requests_to_internal_workspace_panes() {
    for request in all_open_requests() {
        let app = app_for_open_request(request);

        assert_eq!(app.app.open_workspace_panes().count(), 1);
        assert!(app.app.focused_pane.is_some());
        assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
        assert!(app.status.contains(request.target()));
    }
}

#[test]
fn open_smoke_reports_internal_pane_intents_without_native_windows() {
    let args = vec!["std-studio".to_string(), "--open-smoke".to_string()];
    let report = studio_open_smoke_from_args(&args).unwrap();

    assert!(report.pass(), "{}", report.summary());
    assert!(report.summary().contains("studio_open_smoke PASS"));
    assert!(report
        .summary()
        .contains("route=internal-egui-workspace-pane-intent"));
    assert!(report.summary().contains("native_child_windows=false"));
    assert!(report.summary().contains("detached_panels=false"));
}

#[test]
fn open_request_emits_internal_intent_without_launching_host_window() {
    let summary = studio_open_intent_summary(StudioOpenRequest::Plugins);

    assert!(summary.contains("studio_open_intent PASS"));
    assert!(summary.contains("target=plugins"));
    assert!(summary.contains("route=internal-egui-workspace-pane-intent"));
    assert!(summary.contains("host_window=existing-studio-host"));
    assert!(summary.contains("workspace_panes=1"));
    assert!(summary.contains("native_child_windows=false"));
    assert!(summary.contains("detached_panels=false"));
}

#[test]
fn settings_request_uses_internal_workspace_pane() {
    let app = app_for_open_request(StudioOpenRequest::Settings);
    let spec = crate::workspace_panes::focused_workspace_spec(&app.app).unwrap();

    assert_eq!(app.app.active_pane, StudioPane::Settings);
    assert_eq!(spec.content_key, "settings");
    assert_eq!(spec.heading, "Settings");
    assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
    assert!(app.status.contains("opened studio settings"));
}

#[test]
fn blocked_summary_keeps_test_mode_from_opening_native_window() {
    let summary = studio_open_blocked_summary(
        StudioOpenRequest::Settings,
        "studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup",
    );

    assert!(summary.contains("studio_open SKIP"));
    assert!(summary.contains("target=settings"));
    assert!(summary.contains("workspace_panes=1"));
    assert!(summary.contains("STD_TEST_MODE blocked native app startup"));
}

fn open_args(target: &str) -> Vec<String> {
    vec![
        "std-studio".to_string(),
        "--open".to_string(),
        target.to_string(),
    ]
}
