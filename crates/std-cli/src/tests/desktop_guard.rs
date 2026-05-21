use std::{fs, path::Path};

use super::desktop_guard_scan::{
    assert_order, forbidden_test_app_terms, forbidden_test_mode_clear_terms, scan_rs_files,
    scan_rs_files_for_binary_spawns, scan_rs_files_for_unsafe_opt_ins, source_section,
    task_has_std_test_mode, task_inherits_workspace_test_mode,
};

#[test]
fn test_sources_do_not_reference_real_app_launch_targets() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files(
        &root.join("crates"),
        &forbidden_test_app_terms(),
        &mut violations,
    );

    assert!(
        violations.is_empty(),
        "test sources must use fake app fixtures, not real desktop apps or launch targets: {}",
        violations.join(", ")
    );
}

#[test]
fn test_sources_do_not_inherit_desktop_automation_opt_ins() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files_for_unsafe_opt_ins(&root.join("crates"), &mut violations);

    assert!(
        violations.is_empty(),
        "test sources must clear desktop opt-in env vars instead of inheriting them: {}",
        violations.join(", ")
    );
}

#[test]
fn test_binary_spawns_are_forced_into_test_mode() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files_for_binary_spawns(&root.join("crates"), &mut violations);

    assert!(
        violations.is_empty(),
        "test binary spawns must set STD_TEST_MODE=1 and remove desktop opt-ins: {}",
        violations.join(", ")
    );
}

#[test]
fn test_sources_must_not_clear_test_mode() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files(
        &root.join("crates"),
        &forbidden_test_mode_clear_terms(),
        &mut violations,
    );

    assert!(
        violations.is_empty(),
        "test sources must not clear STD_TEST_MODE: {}",
        violations.join(", ")
    );
}

#[test]
fn mise_quality_keeps_default_tests_in_desktop_safe_mode() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let body = fs::read_to_string(root.join("mise.toml")).unwrap();

    assert!(
        body.contains("[env]\nSTD_TEST_MODE = \"1\""),
        "mise workspace env must force STD_TEST_MODE=1"
    );
    for task in [
        "clippy",
        "dylint",
        "dylint-test",
        "file-limits",
        "test",
        "deny",
        "machete",
        "release-build",
        "release-package",
        "release-verify",
        "install-run",
        "install-verify",
    ] {
        assert!(
            task_has_std_test_mode(&body, task) || task_inherits_workspace_test_mode(&body, task),
            "mise task {task} must set STD_TEST_MODE=1"
        );
    }
    assert!(
        !body.contains("STD_ALLOW_DESKTOP_AUTOMATION"),
        "mise default tasks must not opt into desktop automation"
    );
    assert!(
        !body.contains("STD_ALLOW_UI_PREVIEW"),
        "mise default tasks must not opt into UI preview"
    );
}

#[test]
fn release_quality_keeps_desktop_smoke_manual_only() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let body = fs::read_to_string(root.join("crates/std-cli/src/release/quality.rs")).unwrap();

    assert!(
        body.contains("manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1"),
        "release quality report must document real desktop acceptance as manual only"
    );
    let smoke_commands = source_section(&body, "const SMOKE_COMMANDS", "const MANUAL_DESKTOP");
    for forbidden in ["STD_ALLOW_DESKTOP_AUTOMATION", "STD_ALLOW_UI_PREVIEW"] {
        assert!(
            !smoke_commands.contains(forbidden),
            "release default quality gates must not include desktop opt-in: {forbidden}"
        );
    }
    for forbidden in [
        "--ui-preview",
        "gui-hotkey-smoke",
        "1Password",
        "WeChat",
        "微信",
    ] {
        assert!(
            !smoke_commands.contains(forbidden),
            "release default quality gates must not touch desktop apps: {forbidden}"
        );
    }
    for line in smoke_commands
        .lines()
        .filter(|line| line.contains("\"STD_"))
    {
        assert!(
            line.contains("STD_TEST_MODE=1"),
            "release default smoke command must force STD_TEST_MODE=1: {line}"
        );
    }
    let quality_commands = source_section(&body, "const QUALITY_COMMANDS", "const SMOKE_COMMANDS");
    for line in quality_commands
        .lines()
        .filter(|line| line.contains("\"cargo"))
    {
        assert!(
            line.contains("STD_TEST_MODE=1") || line.contains("cargo deny") || line.contains("cargo machete"),
            "release default cargo command must force STD_TEST_MODE=1 unless it is static dependency analysis: {line}"
        );
    }
}

#[test]
fn screenshot_capture_script_requires_ui_preview_opt_in() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let body = fs::read_to_string(root.join("scripts/capture-window.sh")).unwrap();

    assert!(body.contains("STD_ALLOW_UI_PREVIEW"));
    assert!(body.contains("capture-window SKIP"));
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cg-capture-window.swift");
}

#[test]
fn screenshot_matrix_script_requires_ui_preview_opt_in() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let body = fs::read_to_string(root.join("scripts/capture-ui-matrix.sh")).unwrap();

    assert!(body.contains("STD_ALLOW_UI_PREVIEW"));
    assert!(body.contains("STD_TEST_MODE blocks UI preview"));
    assert!(body.contains("std-launcher -- --ui-preview"));
    assert!(body.contains("std-studio -- --ui-preview"));
    assert!(body.contains("scripts/capture-window.sh"));
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cargo run -p std-launcher");
    assert_order(&body, "STD_TEST_MODE", "cargo run -p std-launcher");
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "scripts/capture-window.sh");
}

#[test]
fn std_core_test_mode_limits_app_discovery_to_local_fixtures() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
        body.contains(") && !crate::std_test_mode_enabled()"),
        "Launcher user Enter must stay blocked in STD_TEST_MODE"
    );
    assert!(
        body.contains("ActionType::Command => execute_command_action"),
        "shell commands must stay on the external runner gate"
    );
}

#[test]
fn plugin_shell_runner_requires_desktop_opt_in() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let body = fs::read_to_string(root.join("crates/std-core/src/plugins/command.rs")).unwrap();

    assert!(
        body.contains("if !crate::desktop_automation_allowed()"),
        "shell plugins must require STD_ALLOW_DESKTOP_AUTOMATION before spawning sh"
    );
    assert_order(&body, "desktop_automation_allowed", "Command::new(\"sh\")");
}

#[test]
fn launcher_hotkey_registration_requires_desktop_opt_in() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
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
