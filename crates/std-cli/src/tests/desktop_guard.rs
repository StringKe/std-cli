use std::{fs, path::Path};

use crate::run_cli;

use super::desktop_guard_scan::{
    assert_order, forbidden_test_app_terms, forbidden_test_mode_clear_terms, scan_rs_files,
    scan_rs_files_for_binary_spawns, scan_rs_files_for_desktop_process_commands,
    scan_rs_files_for_unsafe_opt_ins, source_section, task_blocks_desktop_opt_ins,
    task_has_std_test_mode, task_inherits_workspace_test_mode, workspace_blocks_desktop_opt_ins,
};

#[test]
fn run_cli_clears_polluted_desktop_opt_ins_before_dispatch() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();

    std::env::set_var("STDCLI_CONFIG", &config_path);
    std::env::set_var("STD_ALLOW_DESKTOP_AUTOMATION", "1");
    std::env::set_var("STD_ALLOW_UI_PREVIEW", "1");

    let define = run_cli([
        "std",
        "command",
        "define",
        "Polluted Opt In Guard",
        "External runner guard",
        "printf polluted-opt-in-guard",
    ])
    .unwrap();
    assert!(define.contains("\"name\": \"Polluted Opt In Guard\""));

    let output = run_cli([
        "std",
        "trigger",
        "Polluted Opt In Guard",
        "--allow-external",
    ])
    .unwrap();

    assert_eq!(
        std::env::var("STD_ALLOW_DESKTOP_AUTOMATION"),
        Err(std::env::VarError::NotPresent)
    );
    assert_eq!(
        std::env::var("STD_ALLOW_UI_PREVIEW"),
        Err(std::env::VarError::NotPresent)
    );
    assert!(output.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(output.contains("printf polluted-opt-in-guard"));
    assert!(!output.contains("\"status\": \"Completed\""));
    std::env::remove_var("STDCLI_CONFIG");
}

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
fn test_sources_do_not_spawn_desktop_process_commands() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    scan_rs_files_for_desktop_process_commands(&root.join("crates"), &mut violations);

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
}

#[test]
fn workspace_test_binaries_carry_desktop_safe_env_by_default() {
    assert_eq!(option_env!("STD_TEST_MODE"), Some("1"));
    assert_eq!(option_env!("STD_ALLOW_DESKTOP_AUTOMATION"), Some("0"));
    assert_eq!(option_env!("STD_ALLOW_UI_PREVIEW"), Some("0"));
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
