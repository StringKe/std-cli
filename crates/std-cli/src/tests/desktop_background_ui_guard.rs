use std::{fs, path::Path};

use super::desktop_guard_scan::source_section;

#[test]
fn background_ui_smoke_contract_requires_isolated_harness() {
    let root = workspace_root();
    let cli_ui = fs::read_to_string(root.join("crates/std-cli/src/ui/background.rs")).unwrap();
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
    assert!(smoke.contains(
        "--window-title \\\"std-cli Background UI Harness ${HARNESS_TOKEN:?set HARNESS_TOKEN}\\\""
    ));
    assert!(smoke.contains("--harness-token ${HARNESS_TOKEN:?set HARNESS_TOKEN}"));
    assert!(acceptance.contains("Manual opt-in"));
    assert!(acceptance.contains("STD_TEST_MODE = \"0\""));
    assert!(acceptance.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(acceptance.contains("scripts/background-ui-acceptance.sh"));
}

#[test]
fn default_tests_never_use_desktop_event_delivery() {
    let root = workspace_root();
    let checked_files = [
        "crates/std-cli/src/tests/desktop_manual_ui_guard.rs",
        "crates/std-launcher/tests/desktop_guard.rs",
        "crates/std-studio/tests/desktop_guard.rs",
        "crates/std-cli/src/tests/release.rs",
        "crates/std-cli/src/release/quality.rs",
    ];
    let forbidden = [
        "/usr/bin/open",
        "osascript",
        "CGEvent",
        "AXUIElement",
        "postToPid",
        "tapCreateForPid",
        "System Events",
        "com.1password",
        "com.tencent.xinWeChat",
        "com.tencent.WeChat",
    ];

    for file in checked_files {
        let body = fs::read_to_string(root.join(file)).unwrap();
        for term in forbidden {
            assert!(
                !body.contains(term),
                "default test or quality file must not trigger desktop delivery: {file} contains {term}"
            );
        }
    }
}

#[test]
fn background_ui_acceptance_matches_background_click_design() {
    let root = workspace_root();
    let runner = fs::read_to_string(root.join("scripts/background-ui-smoke.swift")).unwrap();
    let docs = fs::read_to_string(root.join("docs/14_Code_Quality.md")).unwrap();

    for required in [
        "CGEvent.tapCreateForPid",
        "place: .headInsertEventTap",
        "eventsOfInterest: focusEventMask()",
        "raw == 13 || raw == 19 || raw == 20",
        "NSEvent.otherEvent",
        "with: .appKitDefined",
        "subtype: subtype",
        "sendAppKitActivation(to: config.harnessPid, windowId: config.windowId, subtype: 1)",
        "postCenterPrimer(to: config.harnessPid, windowId: config.windowId, window: window)",
        "postKeySmoke(to: config.harnessPid, windowId: config.windowId)",
        "sendAppKitActivation(to: config.harnessPid, windowId: config.windowId, subtype: 2)",
        "event.postToPid(pid)",
        "finalFrontmostPid == previousPid",
    ] {
        assert!(
            runner.contains(required),
            "background runner must keep non-frontmost event delivery: {required}"
        );
    }

    for required in [
        "真正打开可见预览窗口必须显式设置",
        "后台 UI 自动化验收使用 macOS AX / CGEvent / postToPid 方案",
        "浮动光标不是输入机制",
        "先装 per-process event tap",
        "center primer",
        "只能投递到 harness window center",
        "PASS 输出必须包含 `frontmost_preserved=true`",
        "不能 fallback 到前台点击真实桌面",
    ] {
        assert!(
            docs.contains(required),
            "background UI docs must preserve safe background testing model: {required}"
        );
    }
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}

fn assert_background_runner_contract(root: &Path) {
    let runner = fs::read_to_string(root.join("scripts/background-ui-smoke.swift")).unwrap();
    for required in background_runner_contract_terms() {
        assert!(
            runner.contains(required),
            "background runner must implement isolated per-process delivery: {required}"
        );
    }
    assert_no_previous_pid_event_delivery(&runner);
    assert_no_global_or_frontmost_delivery(&runner);
}

fn assert_background_harness_contract(root: &Path) {
    let harness = fs::read_to_string(root.join("scripts/background-ui-harness.sh")).unwrap();
    let acceptance = fs::read_to_string(root.join("scripts/background-ui-acceptance.sh")).unwrap();
    for required in [
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "dev.std-cli.background-ui-harness",
        "std-cli Background UI Harness",
        "STD_BACKGROUND_UI_HARNESS_TOKEN",
        "harness_token=\"run-$$\"",
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
        "background-ui-acceptance manifest",
        "identity_rule=pid+window-id+bundle-id+window-title+harness-token",
        "completion_rule=background-ui-smoke-PASS-and-frontmost-preserved",
        "default_gate=manual-opt-in-only",
        "STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST",
        "--manifest",
        "bundle_id outside whitelist",
        "window_title outside whitelist",
        "harness_token missing",
        "smoke_command identity mismatch",
        "expected_smoke_command=",
        "--harness-token",
        "cargo run -p std-cli -- ui background-smoke",
        "driver_frontmost_preserved=\"false\"",
        "driver_stdout=$driver_stdout",
        "driver_identity=target-pid-window-id-and-frontmost-pid",
        "target_pid=$harness_pid",
        "window_id=$window_id",
        "frontmost_before=",
        "frontmost_after=",
        "frontmost_before_equals_after=required",
        "frontmost_evidence_source=background_driver_stdout",
        "target_not_frontmost=required",
        "previous_app_policy=event_tap_only_no_input_delivery",
        "frontmost_user_app_policy=identify_and_preserve_current_frontmost_app",
        "cleanup_harness()",
        "trap cleanup_harness EXIT",
        "cleanup_attempted=true",
        "cleanup_target_pid=$harness_pid",
        "cleanup_signal=TERM",
        "cleanup_scope=validated_background_ui_harness_pid_only",
        "frontmost preservation missing from driver stdout",
        "background_ui_acceptance PASS manifest=$manifest",
    ] {
        assert!(
            acceptance.contains(required),
            "background acceptance must preserve isolated harness workflow: {required}"
        );
    }
}

fn background_cli_contract_terms() -> &'static [&'static str] {
    &[
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "isolated_background_ui_harness_only",
        "HARNESS_BUNDLE_ID",
        "HARNESS_WINDOW_TITLE_PREFIX",
        "BACKGROUND_RUNNER",
        "scripts/background-ui-smoke.swift",
        "scripts/background-ui-harness.sh",
        "required_bundle_id=",
        "required_window_title_prefix=",
        "harness_token=",
        "harness_pid required",
        "window_id required",
        "harness_token required",
        "--harness-token",
        "/usr/bin/swift",
        "driver_sequence=",
        "per-process-event-tap",
        "appKitDefined-activation-primer",
        "window-center-primer",
        "postToPid-target-pid-input",
        "cursor_visual=floating_cursor_not_required_for_event_delivery",
        "harness_origin=spawned_by_scripts_background_ui_harness_only",
        "target_identity=fixed_bundle_pid_window_title_quadruple",
        "run_identity=harness_token_required_to_reject_stale_windows",
        "tap_order=install_previous_and_target_taps_before_primer",
        "trust_gate=AXIsProcessTrusted_before_event_tap",
        "event_tap_then_appkit_defined_primer_then_center_primer",
        "event_route=postToPid_target_pid_only",
        "key_smoke=Enter_to_isolated_echo_result",
        "frontmost_policy=previous_app_never_targeted",
        "frontmost_user_app_policy=identify_and_preserve_current_frontmost_app",
        "frontmost_sensitive_app_policy=fail_before_event_tap",
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

fn background_doc_contract_terms() -> &'static [&'static str] {
    &[
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
        "安装 event tap 前直接 `FAIL`",
        "真实 App 名称",
        "WeChat、weixin、wechat、微信",
        "用户当前 frontmost app",
        "默认质量门禁",
        "cargo run -p std-cli",
    ]
}

fn background_runner_contract_terms() -> &'static [&'static str] {
    &[
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "AXIsProcessTrusted()",
        "macOS Accessibility trust required before event tap",
        "CGEvent.tapCreateForPid",
        ".headInsertEventTap",
        "NSEvent.otherEvent",
        "appKitDefined",
        "focusEventMask()",
        "NSRunningApplication(processIdentifier: config.harnessPid)",
        "pid bundle_id outside whitelist",
        "app.localizedName ?? \"UNKNOWN\"",
        "let previousApp = frontmostAppInfo()",
        "guard !isForbiddenFrontmostApp(previousApp)",
        "frontmost app is forbidden for event tap",
        "previous event tap install failed before primer",
        "target event tap install failed before primer",
        "com.googlecode.iterm2",
        "com.1password",
        "com.tencent.xinwechat",
        "com.tencent.wechat",
        "\"wechat\", \"weixin\", \"微信\"",
        "let finalFrontmostPid = frontmostPid()",
        "finalFrontmostPid == previousPid",
        "frontmost app changed from",
        "frontmost_before=",
        "frontmost_after=",
        "previousPid != config.harnessPid",
        "harness is frontmost; refusing to target active user window",
        "ownerPid == config.harnessPid",
        "number == config.windowId",
        "postToPid",
        "mouseEventWindowUnderMousePointer",
        "mouseEventWindowUnderMousePointerThatCanHandleThisEvent",
        "CGEventField(rawValue: 51)",
        "CGEventField(rawValue: 58)",
        "virtualKey: 36",
        "requiredBundleId",
        "requiredWindowTitle",
        "harnessToken",
    ]
}

fn assert_no_previous_pid_event_delivery(runner: &str) {
    for forbidden in [
        "postToPid(previousPid)",
        "postMouse(type: .leftMouseDown, to: previousPid",
        "sendAppKitActivation(to: previousPid",
        "postCenterPrimer(to: previousPid",
        "postKeySmoke(to: previousPid",
        "virtualKey: 53",
    ] {
        assert!(
            !runner.contains(forbidden),
            "background runner must never deliver events to previous app: {forbidden}"
        );
    }
}

fn assert_no_global_or_frontmost_delivery(runner: &str) {
    for forbidden in [
        "CGEvent.post(",
        "post(tap:",
        "kCGHIDEventTap",
        "/usr/bin/open",
        "System Events",
        "AXPress",
        "CGWarpMouseCursorPosition",
        "mouseCursorPosition: frontmost",
    ] {
        assert!(
            !runner.contains(forbidden),
            "background runner must not use global or frontmost delivery: {forbidden}"
        );
    }
}
