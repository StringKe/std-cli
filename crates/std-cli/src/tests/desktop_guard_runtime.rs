use std::{fs, path::Path};

use super::desktop_guard_scan::{assert_order, source_section};

#[test]
fn std_core_test_mode_limits_app_discovery_to_local_fixtures() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-core/src/app_bundle.rs")).unwrap();

    for required in [
        "fn app_discovery_dirs",
        "crate::std_test_mode_enabled()",
        "return vec![local_apps_dir.to_path_buf()]",
        "test_mode_app_discovery_uses_only_local_fixture_dir",
    ] {
        assert!(
            body.contains(required),
            "app discovery must hard block system app dirs in STD_TEST_MODE: {required}"
        );
    }
}

#[test]
fn std_core_external_runner_requires_desktop_opt_in() {
    let root = workspace_root();
    let execution_body = fs::read_to_string(root.join("crates/std-core/src/execution.rs")).unwrap();
    let actions_body = fs::read_to_string(root.join("crates/std-core/src/actions.rs")).unwrap();

    assert!(
        execution_body.contains("ExternalExecutionMode::DesktopAutomation"),
        "external runners must resolve to a desktop automation execution mode"
    );
    assert!(
        actions_body.contains("allow_external_runner && crate::desktop_automation_allowed()"),
        "CLI external runners must require CLI opt-in and STD_ALLOW_DESKTOP_AUTOMATION"
    );
    let external_runner_gate = source_section(
        &execution_body,
        "fn external_runner_allowed(external_mode: ExternalExecutionMode) -> bool",
        "fn user_desktop_open_allowed",
    );
    assert!(!external_runner_gate.contains("std_test_mode_enabled"));
}

#[test]
fn std_core_test_mode_detection_does_not_spawn_process_inspection() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-core/src/lib.rs")).unwrap();
    let detector = source_section(
        &body,
        "pub fn std_test_mode_enabled() -> bool",
        "pub fn desktop_automation_allowed() -> bool",
    );

    for forbidden in [
        "Command::new(\"/bin/ps\")",
        "parent_process_chain",
        "std::process::id()",
        "-o\", \"ppid=",
    ] {
        assert!(
            !detector.contains(forbidden),
            "STD_TEST_MODE detection must not inspect or spawn desktop process state: {forbidden}"
        );
    }
}

#[test]
fn std_core_app_open_allows_launcher_user_enter_outside_test_mode() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-core/src/execution.rs")).unwrap();

    assert!(
        body.contains("fn user_desktop_open_allowed_for_test_mode("),
        "app and file open must have a distinct user-enter permission gate with test-mode input"
    );
    assert!(
        body.contains("ExternalExecutionMode::LauncherUser"),
        "Launcher user Enter should use a distinct app/file open mode"
    );
    assert!(
        body.contains("user_desktop_open_allowed_for_test_mode")
            && body.contains("crate::std_test_mode_enabled()")
            && body.contains(") && !test_mode"),
        "Launcher user Enter must stay blocked in STD_TEST_MODE"
    );
    assert!(
        body.contains("ActionType::Command => execute_command_action"),
        "shell commands must stay on the external runner gate"
    );
}

#[test]
fn plugin_shell_runner_requires_desktop_opt_in() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-core/src/plugins/command.rs")).unwrap();

    assert!(
        body.contains("if !crate::desktop_automation_allowed()"),
        "shell plugins must require STD_ALLOW_DESKTOP_AUTOMATION before spawning sh"
    );
    assert_order(&body, "desktop_automation_allowed", "Command::new(\"sh\")");
}

#[test]
fn launcher_hotkey_registration_uses_product_desktop_integration_gate() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-launcher/src/hotkey.rs")).unwrap();

    assert!(
        body.contains("if !std_core::desktop_integration_allowed()"),
        "product global hotkey registration must be allowed outside tests and STD_TEST_MODE"
    );
    assert_order(
        &body,
        "desktop_integration_allowed",
        "GlobalHotKeyManager::new",
    );
    assert!(
        body.contains("fn hotkey_smoke_blocked() -> bool")
            && body.contains("!std_core::desktop_automation_allowed()"),
        "hotkey smoke must still require explicit desktop automation opt-in"
    );
}

#[test]
fn binary_entrypoints_sanitize_desktop_opt_ins_before_dispatch() {
    let root = workspace_root();
    for relative in [
        "crates/std-cli/src/lib.rs",
        "crates/std-launcher/src/main.rs",
        "crates/std-studio/src/main.rs",
    ] {
        let body = fs::read_to_string(root.join(relative)).unwrap();
        assert!(
            body.contains("std_core::sanitize_desktop_opt_ins_for_test_mode();"),
            "{relative} must clear desktop opt-ins when STD_TEST_MODE=1"
        );
    }
}

#[test]
fn background_ui_smoke_accepts_only_isolated_harness_identity() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-cli/src/ui/background.rs")).unwrap();

    for required in [
        "const HARNESS_BUNDLE_ID: &str = \"dev.std-cli.background-ui-harness\";",
        "const HARNESS_WINDOW_TITLE_PREFIX: &str = \"std-cli Background UI Harness\";",
        "harness_token required",
        "bundle_id outside whitelist",
        "window_title outside whitelist",
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        &[
            "forbidden_targets=frontmost_app,Terminal,1",
            "Password,WeChat,weixin,wechat,",
            "微信,System_Settings",
        ]
        .join(""),
        "fallback=never_frontmost_desktop_click",
    ] {
        assert!(
            body.contains(required),
            "background UI smoke must be harness-only and deny real apps: {required}"
        );
    }
}

#[test]
fn background_ui_smoke_documents_safe_background_event_route() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-cli/src/ui/background.rs")).unwrap();

    for required in [
        "per-process-event-tap",
        "appKitDefined-activation-primer",
        "window-center-primer",
        "postToPid-target-pid-input",
        "tap_order=install_previous_and_target_taps_before_primer",
        "tap_failure=fail_before_any_primer_or_input",
        "focus_guard=drop_previous_app_deactivation",
        "focus_policy=allow_target_activation_only",
        "event_route=postToPid_target_pid_only",
        "key_smoke=Enter_to_isolated_echo_result",
        "forbidden_route=global_HID,System_Events,frontmost_click,screen_coordinate_click",
    ] {
        assert!(
            body.contains(required),
            "background UI smoke must keep the isolated event route explicit: {required}"
        );
    }
}

#[test]
fn mise_background_ui_tasks_are_manual_harness_only() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("mise.toml")).unwrap();
    let quality = source_section(&body, "[tasks.quality]", "[tasks.release-build]");
    let harness = source_section(
        &body,
        "[tasks.ui-background-harness]",
        "[tasks.ui-background-smoke]",
    );
    let smoke = source_section(&body, "[tasks.ui-background-smoke]", "[tasks.quality]");

    assert!(harness.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(harness.contains("STD_TEST_MODE = \"0\""));
    assert!(harness.contains("scripts/background-ui-harness.sh"));
    assert!(smoke.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
    assert!(smoke.contains("${HARNESS_PID:?set HARNESS_PID}"));
    assert!(smoke.contains("${WINDOW_ID:?set WINDOW_ID}"));
    assert!(smoke.contains("${HARNESS_TOKEN:?set HARNESS_TOKEN}"));
    assert!(smoke.contains("cargo run -p std-cli -- ui background-smoke"));
    assert!(smoke.contains("--harness-token"));
    assert!(!smoke.contains("run = \"std ui background-smoke"));
    assert!(smoke.contains("dev.std-cli.background-ui-harness"));
    assert!(smoke.contains("std-cli Background UI Harness"));
    assert!(!smoke.contains("<pid>"));
    assert!(!smoke.contains("<window-id>"));
    assert!(!quality.contains("ui-background-harness"));
    assert!(!quality.contains("ui-background-smoke"));
    assert!(!quality.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""));
}

#[test]
fn background_ui_runner_fails_when_event_taps_are_unavailable() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("scripts/background-ui-smoke.swift")).unwrap();

    assert!(
        body.contains("guard session.start() else"),
        "background UI runner must require event taps before primer or input"
    );
    assert!(
        body.contains("fail(\"event tap install failed\")"),
        "background UI runner must hard fail when event taps cannot be installed"
    );
    assert_order(
        &body,
        "guard session.start() else",
        "sendAppKitActivation(to: config.harnessPid",
    );
    assert!(body.contains("virtualKey: 36"));
    assert!(!body.contains("virtualKey: 53"));
}

#[test]
fn std_core_does_not_use_build_env_as_runtime_test_mode_guard() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-core/src/lib.rs")).unwrap();
    let detector = source_section(
        &body,
        "pub fn std_test_mode_enabled() -> bool",
        "fn running_under_cargo_test_context() -> bool",
    );

    for forbidden in [
        "compiled_for_safe_tests()",
        "option_env!(\"STD_TEST_MODE\")",
        "option_env!(\"STD_ALLOW_DESKTOP_AUTOMATION\")",
        "option_env!(\"STD_ALLOW_UI_PREVIEW\")",
    ] {
        assert!(
            !detector.contains(forbidden),
            "compiled safe env must not force normal runtime binaries into test mode: {forbidden}"
        );
    }
    assert_order(
        detector,
        "running_under_cargo_test_context()",
        "STD_TEST_MODE",
    );
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}
