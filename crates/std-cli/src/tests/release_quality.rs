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
        "background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 std ui background-smoke"
    ));
    for forbidden in [
        "smoke=STD_ALLOW_DESKTOP_AUTOMATION=1",
        "smoke=STD_ALLOW_UI_PREVIEW=1",
        "command=STD_ALLOW_DESKTOP_AUTOMATION=1",
        "command=STD_ALLOW_UI_PREVIEW=1",
        "smoke=std-launcher --",
        "smoke=std-studio --",
    ] {
        assert!(!report.contains(forbidden), "{forbidden}");
    }
    for required in [
        "command=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test --workspace -- --test-threads=1",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --preview-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --surface-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --preview-smoke",
    ] {
        assert!(report.contains(required), "{required}");
    }
}
