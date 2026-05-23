use super::*;

#[test]
fn release_quality_report_keeps_desktop_automation_manual_only() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let source_dir = temp.path().join("release");
    let dist_dir = temp.path().join("dist").join("1.0.0");
    std::fs::create_dir_all(&source_dir).unwrap();
    for binary in ["std", "std-launcher", "std-studio"] {
        std::fs::write(source_dir.join(binary), format!("{binary}\n")).unwrap();
    }
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    run_cli([
        "std",
        "release",
        "package",
        "--version",
        "1.0.0",
        "--from",
        source_dir.to_str().unwrap(),
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap();
    std::env::remove_var("STDCLI_CONFIG");

    let report =
        std::fs::read_to_string(dist_dir.join("quality").join("quality-report.txt")).unwrap();
    assert!(report.contains("quality_command=mise run quality"));
    assert!(report.contains(
        "manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space"
    ));
    assert!(report.contains(
        "background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-acceptance.sh"
    ));
    assert!(report.contains(
        "background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title \"std-cli Background UI Harness <token>\" --harness-token <token>"
    ));
    for required in [
        "manual_ui_evidence=ui_capture_manifest=STD_UI_CAPTURE_MANIFEST=artifacts/ui/manual-acceptance/manifest.txt",
        "manual_ui_evidence=ui_capture_command=STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix",
        "manual_ui_evidence=ui_capture_doctor=STD_UI_CAPTURE_MANIFEST=artifacts/ui/manual-acceptance/manifest.txt std doctor",
        "manual_ui_evidence=ui_capture_rule=current-run-png-only",
        "manual_ui_evidence=background_ui_manifest=STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST=artifacts/ui/background-acceptance/manifest.txt",
        "manual_ui_evidence=background_ui_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance",
        "manual_ui_evidence=background_ui_rule=isolated-harness-only",
        "manual_ui_matrix=launcher_delivery=light-empty,dark-empty,light-results,dark-results,light-no-results,dark-no-results,light-defer,dark-defer,light-error,dark-error",
        "manual_ui_matrix=launcher_diagnostic=light-collapsed,dark-collapsed,light-searching,dark-searching,light-loading,dark-loading,light-executing,dark-executing,light-ime,dark-ime,light-action-panel,dark-action-panel",
        "manual_ui_matrix=studio_delivery=light-dashboard,dark-dashboard,light-analysis,dark-analysis,light-plugins,dark-plugins,light-operations,dark-operations,light-settings,dark-settings",
        "manual_ui_matrix=studio_workflow=light-workflow,dark-workflow,light-workflow-error,dark-workflow-error",
        "manual_ui_matrix=studio_diagnostic=light-plugin-permission,dark-plugin-permission,light-panes,dark-panes",
        "manual_ui_evidence_rule=ui_capture_pixels=samples+opaque_samples+unique_colors+black_pixels+white_pixels+transparent_pixels+edge_samples+edge_transparent_pixels+edge_black_pixels+edge_white_pixels",
        "manual_ui_evidence_rule=ui_capture_rejects=single-color+dominant-black+dominant-white+edge-black+edge-white-carrier",
    ] {
        assert!(report.contains(required), "{required}");
    }
    for forbidden in [
        "smoke=STD_ALLOW_DESKTOP_AUTOMATION=1",
        "smoke=STD_ALLOW_UI_PREVIEW=1",
        "smoke=std-launcher --",
        "smoke=std-studio --",
    ] {
        assert!(!report.contains(forbidden), "{forbidden}");
    }
    for line in report.lines() {
        if line.starts_with("command=") || line.starts_with("smoke=") {
            assert!(!line.contains("STD_ALLOW_DESKTOP_AUTOMATION=1"), "{line}");
            assert!(!line.contains("STD_ALLOW_UI_PREVIEW=1"), "{line}");
        }
    }
    for required in [
        "command=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test --workspace -- --test-threads=1",
        "command=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo run -p std-egui --example a11y-audit",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --preview-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --user-enter-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --surface-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --preview-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std install runtime-evidence --prefix .std-cli/install-check",
    ] {
        assert!(report.contains(required), "{required}");
    }
}
