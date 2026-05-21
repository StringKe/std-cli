use std::{fs, path::Path};

use super::desktop_guard_scan::{assert_order, source_section};

#[test]
fn background_ui_smoke_contract_requires_isolated_harness() {
    let root = workspace_root();
    let cli_ui = fs::read_to_string(root.join("crates/std-cli/src/ui.rs")).unwrap();
    let quality_doc = fs::read_to_string(root.join("docs/14_Code_Quality.md")).unwrap();

    for required in background_cli_contract_terms() {
        assert!(
            cli_ui.contains(required),
            "background-smoke must keep isolated opt-in boundary: {required}"
        );
    }
    for required in background_doc_contract_terms() {
        assert!(
            quality_doc.contains(required),
            "background UI acceptance docs must describe safe harness boundary: {required}"
        );
    }
    assert_background_runner_contract(root);
    assert_background_harness_contract(root);
}

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

#[test]
fn release_quality_keeps_manual_background_ui_out_of_default_gate() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-cli/src/release/quality.rs")).unwrap();
    let quality_commands = source_section(&body, "const QUALITY_COMMANDS", "const SMOKE_COMMANDS");
    let smoke_commands = source_section(&body, "const SMOKE_COMMANDS", "const MANUAL_DESKTOP");

    assert!(!quality_commands.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION=1"));
    assert!(!smoke_commands.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION=1"));
    assert!(body.contains("background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1"));
    assert!(body.contains("scripts/background-ui-acceptance.sh"));
    assert!(body.contains("cargo run -p std-cli -- ui background-smoke --harness-pid <pid>"));
    assert!(!body.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 std ui background-smoke"));
    assert!(body.contains("dev.std-cli.background-ui-harness"));
}

#[test]
fn mise_quality_keeps_background_ui_manual_only() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("mise.toml")).unwrap();
    let quality = source_section(&body, "[tasks.quality]", "[tasks.release-build]");
    let harness = source_section(
        &body,
        "[tasks.ui-background-harness]",
        "[tasks.ui-background-smoke]",
    );
    let smoke = source_section(&body, "[tasks.ui-background-smoke]", "[tasks.quality]");
    let acceptance = source_section(&body, "[tasks.ui-background-acceptance]", "[tasks.quality]");

    assert!(!quality.contains("ui-background-harness"));
    assert!(!quality.contains("ui-background-smoke"));
    assert!(!quality.contains("ui-background-acceptance"));
    assert!(!quality.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(harness.contains("Manual opt-in"));
    assert!(harness.contains("STD_TEST_MODE = \"0\""));
    assert!(harness.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(harness.contains("scripts/background-ui-harness.sh"));
    assert!(smoke.contains("Manual opt-in"));
    assert!(smoke.contains("STD_TEST_MODE = \"0\""));
    assert!(smoke.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(smoke.contains("--bundle-id dev.std-cli.background-ui-harness"));
    assert!(smoke.contains("--window-title \\\"std-cli Background UI Harness\\\""));
    assert!(acceptance.contains("Manual opt-in"));
    assert!(acceptance.contains("STD_TEST_MODE = \"0\""));
    assert!(acceptance.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(acceptance.contains("scripts/background-ui-acceptance.sh"));
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

fn launcher_required_capture_states() -> [&'static str; 10] {
    [
        "capture_launcher light results",
        "capture_launcher dark results",
        "capture_launcher light no-results",
        "capture_launcher dark no-results",
        "capture_launcher light defer",
        "capture_launcher dark defer",
        "capture_launcher light error",
        "capture_launcher dark error",
        "capture_launcher light action-panel",
        "capture_launcher dark action-panel",
    ]
}

fn studio_required_capture_states() -> [&'static str; 14] {
    [
        "capture_studio light dashboard",
        "capture_studio dark dashboard",
        "capture_studio light workflow",
        "capture_studio dark workflow",
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

fn assert_background_runner_contract(root: &Path) {
    let runner = fs::read_to_string(root.join("scripts/background-ui-smoke.swift")).unwrap();
    for required in background_runner_contract_terms() {
        assert!(
            runner.contains(required),
            "background runner must implement isolated per-process delivery: {required}"
        );
    }
}

fn assert_background_harness_contract(root: &Path) {
    let harness = fs::read_to_string(root.join("scripts/background-ui-harness.sh")).unwrap();
    let acceptance = fs::read_to_string(root.join("scripts/background-ui-acceptance.sh")).unwrap();
    for required in [
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "dev.std-cli.background-ui-harness",
        "std-cli Background UI Harness",
        "open -n -g",
        "unset STD_TEST_MODE",
        "--background-ui-harness",
        "background-ui-harness-window.swift",
    ] {
        assert!(
            harness.contains(required),
            "background harness must stay isolated and background-launched: {required}"
        );
    }
    for required in [
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "scripts/background-ui-harness.sh",
        "bundle_id outside whitelist",
        "window_title outside whitelist",
        "cargo run -p std-cli -- ui background-smoke",
    ] {
        assert!(
            acceptance.contains(required),
            "background acceptance must preserve isolated harness workflow: {required}"
        );
    }
}

fn background_cli_contract_terms() -> [&'static str; 34] {
    [
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "isolated_background_ui_harness_only",
        "HARNESS_BUNDLE_ID",
        "HARNESS_WINDOW_TITLE",
        "BACKGROUND_RUNNER",
        "scripts/background-ui-smoke.swift",
        "scripts/background-ui-harness.sh",
        "required_bundle_id=",
        "required_window_title=",
        "harness_pid required",
        "window_id required",
        "/usr/bin/swift",
        "driver_sequence=",
        "per-process-event-tap",
        "appKitDefined-activation-primer",
        "window-center-primer",
        "postToPid-target-pid-input",
        "cursor_visual=floating_cursor_not_required_for_event_delivery",
        "harness_origin=spawned_by_scripts_background_ui_harness_only",
        "target_identity=fixed_bundle_pid_window_title_quadruple",
        "tap_order=install_previous_and_target_taps_before_primer",
        "event_tap_then_appkit_defined_primer_then_center_primer",
        "event_route=postToPid_target_pid_only",
        "frontmost_policy=previous_app_never_targeted",
        "real_app_policy=deny_user_apps_by_bundle_pid_window_title_mismatch",
        "focus_guard=drop_previous_app_deactivation",
        "focus_policy=allow_target_activation_only",
        "focus_messages=raw_13_19_20",
        "tap_mask=focus_raw_13_19_20_only",
        "window_addressing=windowUnderMouse_windowThatCanHandle_fields_51_58",
        "forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click",
        "fallback=never_frontmost_desktop_click",
        "manual_only=excluded_from_default_quality_and_release_smoke",
    ]
}

fn background_doc_contract_terms() -> [&'static str; 24] {
    [
        "per-process event tap",
        "浮动光标不是输入机制",
        "先安装 previous 和 target",
        "appKitDefined primer",
        "center primer",
        "raw value 13、19、20",
        "applicationActivated",
        "applicationDeactivated",
        "windowUnderMouse",
        "windowThatCanHandle",
        "field 51/58",
        "隔离 harness",
        "四重匹配",
        "previous app 永远不能作为输入目标",
        "window title 白名单",
        "scripts/background-ui-harness.sh",
        "scripts/background-ui-acceptance.sh",
        "cargo run -p std-cli -- ui background-smoke",
        "open -g",
        "dev.std-cli.background-ui-harness",
        "真实 App 名称",
        "用户当前 frontmost app",
        "默认质量门禁",
        "cargo run -p std-cli",
    ]
}

fn background_runner_contract_terms() -> [&'static str; 20] {
    [
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "CGEvent.tapCreateForPid",
        ".headInsertEventTap",
        "NSEvent.otherEvent",
        "appKitDefined",
        "focusEventMask()",
        "NSRunningApplication(processIdentifier: config.harnessPid)",
        "pid bundle_id outside whitelist",
        "previousPid != config.harnessPid",
        "harness is frontmost; refusing to target active user window",
        "ownerPid == config.harnessPid",
        "number == config.windowId",
        "postToPid",
        "mouseEventWindowUnderMousePointer",
        "mouseEventWindowUnderMousePointerThatCanHandleThisEvent",
        "CGEventField(rawValue: 51)",
        "CGEventField(rawValue: 58)",
        "requiredBundleId",
        "requiredWindowTitle",
    ]
}
