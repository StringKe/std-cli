use std::{fs, path::Path};

use super::desktop_guard_scan::{
    assert_order, command_sets_desktop_safe_env, forbidden_test_app_terms,
    forbidden_test_mode_clear_terms, scan_rs_files, scan_rs_files_for_binary_spawns,
    scan_rs_files_for_desktop_process_commands, scan_rs_files_for_raw_process_spawns,
    scan_rs_files_for_unsafe_opt_ins, source_section, task_blocks_desktop_opt_ins,
    task_has_std_test_mode, task_inherits_workspace_test_mode, workspace_blocks_desktop_opt_ins,
};

#[test]
fn test_sources_do_not_reference_real_app_launch_targets() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files(root, &forbidden_test_app_terms(), &mut violations);

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

    scan_rs_files_for_unsafe_opt_ins(root, &mut violations);

    assert!(
        violations.is_empty(),
        "test sources must set desktop opt-in env vars to 0 instead of inheriting or removing them: {}",
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

    scan_rs_files_for_binary_spawns(root, &mut violations);

    assert!(
        violations.is_empty(),
        "test binary spawns must set STD_TEST_MODE=1 and remove desktop opt-ins: {}",
        violations.join(", ")
    );
}

#[test]
fn test_sources_only_spawn_repo_binaries() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files_for_raw_process_spawns(root, &mut violations);

    assert!(
        violations.is_empty(),
        "test sources must not spawn raw desktop or shell processes: {}",
        violations.join(", ")
    );
}

#[test]
fn test_sources_do_not_spawn_desktop_process_commands() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files_for_desktop_process_commands(root, &mut violations);

    assert!(
        violations.is_empty(),
        "default tests must not spawn desktop process commands: {}",
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

    scan_rs_files(root, &forbidden_test_mode_clear_terms(), &mut violations);

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
    assert!(
        workspace_blocks_desktop_opt_ins(&body),
        "mise workspace env must force desktop opt-ins off"
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
        assert!(
            task_blocks_desktop_opt_ins(&body, task) || workspace_blocks_desktop_opt_ins(&body),
            "mise task {task} must force desktop opt-ins off"
        );
    }
    assert!(
        !body.contains("STD_ALLOW_DESKTOP_AUTOMATION = \"1\""),
        "mise default tasks must not opt into desktop automation"
    );
    assert!(
        !body.contains("STD_ALLOW_UI_PREVIEW = \"1\""),
        "mise default tasks must not opt into UI preview"
    );
    assert!(
        !body.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"1\""),
        "mise default tasks must not opt into background UI automation"
    );
}

#[test]
fn workspace_test_binaries_carry_desktop_safe_env_by_default() {
    assert_eq!(option_env!("STD_TEST_MODE"), Some("1"));
    assert_eq!(option_env!("STD_ALLOW_DESKTOP_AUTOMATION"), Some("0"));
    assert_eq!(option_env!("STD_ALLOW_UI_PREVIEW"), Some("0"));
    assert_eq!(option_env!("STD_ALLOW_BACKGROUND_UI_AUTOMATION"), Some("0"));
}

#[test]
fn workspace_package_forces_test_env_for_all_crates() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let build_body = fs::read_to_string(root.join("build.rs")).unwrap();
    for relative in [
        "crates/std-types/Cargo.toml",
        "crates/std-core/Cargo.toml",
        "crates/std-orchestration/Cargo.toml",
        "crates/std-index/Cargo.toml",
        "crates/std-egui/Cargo.toml",
        "crates/std-launcher/Cargo.toml",
        "crates/std-studio/Cargo.toml",
        "crates/std-cli/Cargo.toml",
    ] {
        let body = fs::read_to_string(root.join(relative)).unwrap();
        assert!(
            body.contains("build = \"../../build.rs\""),
            "{relative} must attach shared build.rs"
        );
    }

    for required in [
        "cargo:rustc-env=STD_TEST_MODE=1",
        "cargo:rustc-env=STD_ALLOW_DESKTOP_AUTOMATION=0",
        "cargo:rustc-env=STD_ALLOW_UI_PREVIEW=0",
        "cargo:rustc-env=STD_ALLOW_BACKGROUND_UI_AUTOMATION=0",
    ] {
        assert!(
            build_body.contains(required),
            "build.rs must force safe test env into test binaries: {required}"
        );
    }
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
            command_sets_desktop_safe_env(line),
            "release default smoke command must force safe desktop env: {line}"
        );
    }
    let quality_commands = source_section(&body, "const QUALITY_COMMANDS", "const SMOKE_COMMANDS");
    for line in quality_commands
        .lines()
        .filter(|line| line.contains("\"cargo"))
    {
        assert!(
            command_sets_desktop_safe_env(line)
                || line.contains("cargo deny")
                || line.contains("cargo machete"),
            "release default cargo command must force safe desktop env unless it is static dependency analysis: {line}"
        );
    }
    for forbidden in [
        "STD_ALLOW_DESKTOP_AUTOMATION=1",
        "STD_ALLOW_UI_PREVIEW=1",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1",
    ] {
        assert!(
            !quality_commands.contains(forbidden) && !smoke_commands.contains(forbidden),
            "release default quality gates must not include desktop opt-in: {forbidden}"
        );
    }
}

#[test]
fn background_ui_smoke_contract_requires_isolated_harness() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let cli_ui = fs::read_to_string(root.join("crates/std-cli/src/ui.rs")).unwrap();
    let quality_doc = fs::read_to_string(root.join("docs/14_Code_Quality.md")).unwrap();

    for required in [
        "STD_TEST_MODE blocks background UI automation",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required",
        "isolated_background_ui_harness_only",
        "AX_or_CGEvent_postToPid_after_explicit_opt_in",
        "event_tap_then_appkit_defined_primer_then_center_primer",
        "fallback=never_frontmost_desktop_click",
    ] {
        assert!(
            cli_ui.contains(required),
            "background-smoke must keep isolated opt-in boundary: {required}"
        );
    }
    for required in [
        "per-process event tap",
        "appKitDefined primer",
        "center primer",
        "隔离 harness",
        "window title 白名单",
    ] {
        assert!(
            quality_doc.contains(required),
            "background UI acceptance docs must describe safe harness boundary: {required}"
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
