use std::{fs, path::Path};

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
        "test sources must use fake app fixtures, not real launch targets: {}",
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

    scan_rs_files(
        &root.join("crates"),
        &forbidden_test_opt_in_terms(),
        &mut violations,
    );

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

    for task in ["clippy", "dylint", "dylint-test", "file-limits", "test"] {
        assert!(
            task_has_std_test_mode(&body, task),
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

fn assert_order(body: &str, first: &str, second: &str) {
    let first_index = body.find(first).unwrap();
    let second_index = body.find(second).unwrap();
    assert!(
        first_index < second_index,
        "{first} must appear before {second}"
    );
}

fn forbidden_test_app_terms() -> Vec<String> {
    vec![
        ["1", "Password"].join(""),
        "[\"1\", \"Password\"]".to_string(),
        ["open -a ", "Terminal"].join(""),
        "Terminal\".to_string()".to_string(),
        ["open", " -a "].join(""),
        ["/usr/bin/", "open", " -a "].join(""),
        ["osa", "script"].join(""),
        "[\"osa\", \"script\"]".to_string(),
        ["System", " Events"].join(""),
        ["tell ", "application"].join(""),
        ["/Applications/", "1", "Password.app"].join(""),
        ["tell application \"", "1", "Password\""].join(""),
        "/Applications/".to_string(),
        "/System/Applications".to_string(),
    ]
}

fn forbidden_test_opt_in_terms() -> Vec<String> {
    vec![
        ".env(\"STD_ALLOW_DESKTOP_AUTOMATION\"".to_string(),
        ".env(\"STD_ALLOW_UI_PREVIEW\"".to_string(),
        "set_var(\"STD_ALLOW_DESKTOP_AUTOMATION\"".to_string(),
        "set_var(\"STD_ALLOW_UI_PREVIEW\"".to_string(),
    ]
}

fn forbidden_test_mode_clear_terms() -> Vec<String> {
    vec![
        ".env_remove(\"STD_TEST_MODE\")".to_string(),
        "remove_var(\"STD_TEST_MODE\")".to_string(),
    ]
}

fn task_has_std_test_mode(body: &str, task: &str) -> bool {
    let header = format!("[tasks.{task}]");
    let Some(start) = body.find(&header) else {
        return false;
    };
    let rest = &body[start + header.len()..];
    let end = rest.find("\n[tasks.").unwrap_or(rest.len());
    rest[..end].contains("env = { STD_TEST_MODE = \"1\" }")
}

fn source_section<'a>(body: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = body.find(start).unwrap();
    let end_index = body[start_index..]
        .find(end)
        .map(|offset| start_index + offset)
        .unwrap_or(body.len());
    &body[start_index..end_index]
}

fn scan_rs_files(dir: &Path, forbidden_terms: &[String], violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_rs_files(&path, forbidden_terms, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
            || !is_test_path(&path)
            || is_guard_file(&path)
        {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        for term in forbidden_terms {
            if body.contains(term) {
                violations.push(format!("{} contains {}", path.display(), term));
            }
        }
    }
}

fn scan_rs_files_for_binary_spawns(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_rs_files_for_binary_spawns(&path, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
            || !is_test_path(&path)
            || is_guard_file(&path)
        {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        if !body.contains("CARGO_BIN_EXE_") {
            continue;
        }
        for required in [
            ".env(\"STD_TEST_MODE\", \"1\")",
            ".env_remove(\"STD_ALLOW_DESKTOP_AUTOMATION\")",
            ".env_remove(\"STD_ALLOW_UI_PREVIEW\")",
        ] {
            if !body.contains(required) {
                violations.push(format!("{} missing {}", path.display(), required));
            }
        }
    }
}

fn is_test_path(path: &Path) -> bool {
    path.components().any(|part| part.as_os_str() == "tests")
        || path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.ends_with("_tests.rs") || name == "tests.rs")
            .unwrap_or(false)
}

fn is_guard_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == "desktop_guard.rs")
        .unwrap_or(false)
}
