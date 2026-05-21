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
        body.contains("fn user_desktop_open_allowed(external_mode: ExternalExecutionMode) -> bool"),
        "app and file open must have a distinct user-enter permission gate"
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
fn launcher_hotkey_registration_requires_desktop_opt_in() {
    let root = workspace_root();
    let body = fs::read_to_string(root.join("crates/std-launcher/src/hotkey.rs")).unwrap();

    assert!(
        body.contains("if !std_core::desktop_automation_allowed()"),
        "global hotkey registration must require STD_ALLOW_DESKTOP_AUTOMATION"
    );
    assert_order(
        &body,
        "desktop_automation_allowed",
        "GlobalHotKeyManager::new",
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
