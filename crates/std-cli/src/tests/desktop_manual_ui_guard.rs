use std::{fs, path::Path};

use super::desktop_guard_scan::assert_order;

#[test]
fn screenshot_capture_script_requires_ui_preview_opt_in() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("scripts/capture-window.sh")).unwrap();

    assert!(body.contains("STD_ALLOW_UI_PREVIEW"));
    assert!(body.contains("capture-window SKIP"));
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cg-capture-window.swift");
}

#[test]
fn screenshot_matrix_script_requires_ui_preview_opt_in() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("scripts/capture-ui-matrix.sh")).unwrap();

    assert!(body.contains("STD_ALLOW_UI_PREVIEW"));
    assert!(body.contains("STD_TEST_MODE blocks UI preview"));
    assert!(body.contains("cargo run -p std-launcher -- --ui-preview"));
    assert!(body.contains("cargo run -p std-studio -- --ui-preview"));
    assert!(body.contains("scripts/capture-window.sh"));
    for required in launcher_required_capture_states() {
        assert!(
            body.contains(required),
            "capture matrix must include Launcher required state: {required}"
        );
    }
    for required in studio_required_capture_states() {
        assert!(
            body.contains(required),
            "capture matrix must include Studio required state: {required}"
        );
    }
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cargo run -p std-launcher");
    assert_order(&body, "STD_TEST_MODE", "cargo run -p std-launcher");
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "scripts/capture-window.sh");
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

fn launcher_required_capture_states() -> [&'static str; 20] {
    [
        "capture_launcher light collapsed",
        "capture_launcher dark collapsed",
        "capture_launcher light empty",
        "capture_launcher dark empty",
        "capture_launcher light results",
        "capture_launcher dark results",
        "capture_launcher light no-results",
        "capture_launcher dark no-results",
        "capture_launcher light searching",
        "capture_launcher dark searching",
        "capture_launcher light loading",
        "capture_launcher dark loading",
        "capture_launcher light executing",
        "capture_launcher dark executing",
        "capture_launcher light defer",
        "capture_launcher dark defer",
        "capture_launcher light error",
        "capture_launcher dark error",
        "capture_launcher light action-panel",
        "capture_launcher dark action-panel",
    ]
}

fn studio_required_capture_states() -> [&'static str; 16] {
    [
        "capture_studio light dashboard",
        "capture_studio dark dashboard",
        "capture_studio light workflow",
        "capture_studio dark workflow",
        "capture_studio light workflow-error",
        "capture_studio dark workflow-error",
        "capture_studio light analysis",
        "capture_studio dark analysis",
        "capture_studio light plugins",
        "capture_studio dark plugins",
        "capture_studio light operations",
        "capture_studio dark operations",
        "capture_studio light settings",
        "capture_studio dark settings",
        "capture_studio light panes",
        "capture_studio dark panes",
    ]
}
