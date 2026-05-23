use std::{fs, path::Path};

use super::desktop_guard_scan::assert_order;

#[test]
fn screenshot_capture_script_requires_ui_preview_opt_in() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("scripts/capture-window.sh")).unwrap();
    let driver = fs::read_to_string(root.join("scripts/cg-capture-window.swift")).unwrap();

    assert!(body.contains("STD_ALLOW_UI_PREVIEW"));
    assert!(body.contains("capture-window SKIP"));
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cg-capture-window.swift");
    assert!(body.contains("<process-pid> <process-name>"));
    assert!(driver.contains("kCGWindowOwnerPID"));
    assert!(driver.contains("pid == ownerPid"));
    assert!(driver.contains("title.contains(titleFragment)"));
    assert!(!driver.contains("fallback"));
}

#[test]
fn screenshot_matrix_script_requires_ui_preview_opt_in() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("scripts/capture-ui-matrix.sh")).unwrap();

    assert!(body.contains("STD_ALLOW_UI_PREVIEW"));
    assert!(body.contains("STD_TEST_MODE blocks UI preview"));
    assert!(body.contains("cargo build -p std-launcher -p std-studio"));
    assert!(body.contains("launcher_bin=\"target/debug/std-launcher\""));
    assert!(body.contains("studio_bin=\"target/debug/std-studio\""));
    assert!(body.contains("\"$launcher_bin\" --ui-preview"));
    assert!(body.contains("\"$studio_bin\" --ui-preview"));
    assert!(!body.contains("cargo run -p std-launcher -- --ui-preview"));
    assert!(!body.contains("cargo run -p std-studio -- --ui-preview"));
    assert!(body.contains("scripts/capture-window.sh"));
    assert!(body.contains("scripts/capture-window.sh \"$pid\" std-launcher"));
    assert!(body.contains("scripts/capture-window.sh \"$pid\" std-studio"));
    for required in launcher_required_capture_states() {
        assert!(
            body.contains(required),
            "capture matrix must include Launcher required state: {required}"
        );
    }
    assert_eq!(
        launcher_capture_sequence(&body),
        launcher_required_capture_states()
    );
    for required in studio_required_capture_states() {
        assert!(
            body.contains(required),
            "capture matrix must include Studio required state: {required}"
        );
    }
    assert_eq!(
        studio_capture_sequence(&body),
        studio_required_capture_states()
    );
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cargo build -p std-launcher");
    assert_order(&body, "STD_TEST_MODE", "cargo build -p std-launcher");
    assert_order(
        &body,
        "cargo build -p std-launcher",
        "\"$launcher_bin\" --ui-preview",
    );
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "scripts/capture-window.sh");
}

#[test]
fn mise_ui_capture_matrix_is_manual_preview_only() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("mise.toml")).unwrap();
    let task = source_section(&body, "[tasks.ui-capture-matrix]", "[tasks.quality]");
    let quality = source_section(&body, "[tasks.quality]", "[tasks.release-build]");

    assert!(task.contains("STD_ALLOW_UI_PREVIEW = \"1\""));
    assert!(task.contains("STD_TEST_MODE = \"0\""));
    assert!(task.contains("scripts/capture-ui-matrix.sh"));
    assert!(!quality.contains("ui-capture-matrix"));
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

fn source_section<'a>(body: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = body.find(start).unwrap();
    let tail = &body[start_index..];
    let end_index = tail.find(end).unwrap_or(tail.len());
    &tail[..end_index]
}

fn launcher_required_capture_states() -> [&'static str; 22] {
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
        "capture_launcher light ime",
        "capture_launcher dark ime",
        "capture_launcher light action-panel",
        "capture_launcher dark action-panel",
    ]
}

fn launcher_capture_sequence(body: &str) -> Vec<String> {
    body.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("capture_launcher "))
        .map(str::to_string)
        .collect()
}

fn studio_required_capture_states() -> [&'static str; 18] {
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
        "capture_studio light plugin-permission",
        "capture_studio dark plugin-permission",
        "capture_studio light operations",
        "capture_studio dark operations",
        "capture_studio light settings",
        "capture_studio dark settings",
        "capture_studio light panes",
        "capture_studio dark panes",
    ]
}

fn studio_capture_sequence(body: &str) -> Vec<String> {
    body.lines()
        .map(str::trim)
        .filter(|line| line.starts_with("capture_studio "))
        .map(str::to_string)
        .collect()
}
