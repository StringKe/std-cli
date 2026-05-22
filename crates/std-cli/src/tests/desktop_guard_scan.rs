use std::{fs, path::Path};

pub(crate) fn assert_order(body: &str, first: &str, second: &str) {
    let first_index = body.find(first).unwrap();
    let second_index = body.find(second).unwrap();
    assert!(
        first_index < second_index,
        "{first} must appear before {second}"
    );
}

pub(crate) fn forbidden_test_app_terms() -> Vec<String> {
    vec![
        "[\"1\", \"Password\"]".to_string(),
        "[\"1P\", \"assword\"]".to_string(),
        "[\"We\", \"Chat\"]".to_string(),
        "[\"wei\", \"xin\"]".to_string(),
        "weixin://".to_string(),
        "wechat://".to_string(),
        ["open -a ", "Terminal"].join(""),
        ["open", " -a "].join(""),
        "[\"op\", \"en\"".to_string(),
        "[\"op\", \"en\", \"-a\"".to_string(),
        ["/usr/bin/", "open", " -a "].join(""),
        ["osa", "script"].join(""),
        "[\"osa\", \"script\"]".to_string(),
        ["System", " Events"].join(""),
        ["tell ", "application"].join(""),
        ["/Applications/", "1", "Password.app"].join(""),
        "/Applications/".to_string(),
        "/System/Applications".to_string(),
    ]
}

pub(crate) fn forbidden_test_mode_clear_terms() -> Vec<String> {
    vec![
        ".env_remove(\"STD_TEST_MODE\")".to_string(),
        "remove_var(\"STD_TEST_MODE\")".to_string(),
    ]
}

pub(crate) fn task_has_std_test_mode(body: &str, task: &str) -> bool {
    task_has_env(body, task, "STD_TEST_MODE", "1")
}

pub(crate) fn task_blocks_desktop_opt_ins(body: &str, task: &str) -> bool {
    task_has_env(body, task, "STD_ALLOW_DESKTOP_AUTOMATION", "0")
        && task_has_env(body, task, "STD_ALLOW_UI_PREVIEW", "0")
        && task_has_env(body, task, "STD_ALLOW_BACKGROUND_UI_AUTOMATION", "0")
}

pub(crate) fn workspace_blocks_desktop_opt_ins(body: &str) -> bool {
    body.contains("STD_ALLOW_DESKTOP_AUTOMATION = \"0\"")
        && body.contains("STD_ALLOW_UI_PREVIEW = \"0\"")
        && body.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION = \"0\"")
}

pub(crate) fn command_sets_desktop_safe_env(line: &str) -> bool {
    line.contains("STD_TEST_MODE=1")
        && line.contains("STD_ALLOW_DESKTOP_AUTOMATION=0")
        && line.contains("STD_ALLOW_UI_PREVIEW=0")
        && line.contains("STD_ALLOW_BACKGROUND_UI_AUTOMATION=0")
}

fn task_has_env(body: &str, task: &str, key: &str, value: &str) -> bool {
    let header = format!("[tasks.{task}]");
    let Some(start) = body.find(&header) else {
        return false;
    };
    let rest = &body[start + header.len()..];
    let end = rest.find("\n[tasks.").unwrap_or(rest.len());
    rest[..end].contains(&format!("{key} = \"{value}\""))
}

pub(crate) fn task_inherits_workspace_test_mode(body: &str, task: &str) -> bool {
    body.contains("[env]\nSTD_TEST_MODE = \"1\"") && body.contains(&format!("[tasks.{task}]"))
}

pub(crate) fn source_section<'a>(body: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = body.find(start).unwrap();
    let end_index = body[start_index..]
        .find(end)
        .map(|offset| start_index + offset)
        .unwrap_or(body.len());
    &body[start_index..end_index]
}

pub(crate) fn scan_rs_files(dir: &Path, forbidden_terms: &[String], violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if ignored_scan_dir(&path) {
                continue;
            }
            scan_rs_files(&path, forbidden_terms, violations);
            continue;
        }
        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        if !eligible_test_source(&path, &body) {
            continue;
        }
        for term in forbidden_terms {
            if body.contains(term) {
                violations.push(format!("{} contains {}", path.display(), term));
            }
        }
    }
}

pub(crate) fn scan_rs_files_for_binary_spawns(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if ignored_scan_dir(&path) {
                continue;
            }
            scan_rs_files_for_binary_spawns(&path, violations);
            continue;
        }
        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        if !eligible_test_source(&path, &body) {
            continue;
        }
        if !body.contains("CARGO_BIN_EXE_") {
            continue;
        }
        scan_binary_spawn_blocks(&path, &body, violations);
    }
}

pub(crate) fn scan_rs_files_for_raw_process_spawns(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if ignored_scan_dir(&path) {
                continue;
            }
            scan_rs_files_for_raw_process_spawns(&path, violations);
            continue;
        }
        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        if !eligible_raw_process_spawn_source(&path, &body) {
            continue;
        }
        scan_raw_process_spawn_blocks(&path, &body, violations);
    }
}

fn scan_binary_spawn_blocks(path: &Path, body: &str, violations: &mut Vec<String>) {
    let mut rest = body;
    let mut offset = 0;
    while let Some(index) = rest.find("Command::new(env!(\"CARGO_BIN_EXE_") {
        let start = offset + index;
        let after_start = &body[start..];
        let end = after_start
            .find(".output()")
            .map(|end| start + end)
            .unwrap_or(body.len());
        let block = &body[start..end];
        for required in [
            ".env(\"STD_TEST_MODE\", \"1\")",
            ".env(\"STD_ALLOW_DESKTOP_AUTOMATION\", \"0\")",
            ".env(\"STD_ALLOW_UI_PREVIEW\", \"0\")",
        ] {
            if !block.contains(required) {
                violations.push(format!(
                    "{} spawn at byte {} missing {}",
                    path.display(),
                    start,
                    required
                ));
            }
        }
        offset = start + "Command::new(env!(\"CARGO_BIN_EXE_".len();
        rest = &body[offset..];
    }
}

fn scan_raw_process_spawn_blocks(path: &Path, body: &str, violations: &mut Vec<String>) {
    let mut rest = body;
    let mut offset = 0;
    while let Some(index) = rest.find("Command::new(") {
        let start = offset + index;
        if command_new_is_string_literal(body, start) {
            offset = start + "Command::new(".len();
            rest = &body[offset..];
            continue;
        }
        let after_start = &body[start..];
        let end = after_start
            .find(".output()")
            .map(|end| start + end)
            .unwrap_or(body.len());
        let block = &body[start..end];
        if !block.contains("Command::new(env!(\"CARGO_BIN_EXE_") {
            violations.push(format!(
                "{} raw process spawn at byte {} must use CARGO_BIN_EXE test binary",
                path.display(),
                start
            ));
        }
        offset = start + "Command::new(".len();
        rest = &body[offset..];
    }
}

fn eligible_raw_process_spawn_source(path: &Path, body: &str) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("rs")
        && is_test_path(path)
        && !is_static_desktop_guard_file(path)
        && !is_runtime_desktop_support_file(path)
        && body.contains("Command::new(")
}

fn command_new_is_string_literal(body: &str, start: usize) -> bool {
    let line_start = body[..start]
        .rfind('\n')
        .map(|index| index + 1)
        .unwrap_or(0);
    body[line_start..start].contains('"')
}

pub(crate) fn scan_rs_files_for_desktop_process_commands(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if ignored_scan_dir(&path) {
                continue;
            }
            scan_rs_files_for_desktop_process_commands(&path, violations);
            continue;
        }
        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        if !eligible_test_source(&path, &body) {
            continue;
        }
        for term in forbidden_desktop_process_terms() {
            if body.contains(&term) {
                violations.push(format!("{} contains {}", path.display(), term));
            }
        }
    }
}

fn forbidden_desktop_process_terms() -> Vec<String> {
    vec![
        "Command::new(\"op\")".to_string(),
        "Command::new(\"/usr/local/bin/op\")".to_string(),
        "Command::new(\"/opt/homebrew/bin/op\")".to_string(),
        "Command::new(\"open\")".to_string(),
        "Command::new(\"/usr/bin/open\")".to_string(),
        "Command::new(\"osascript\")".to_string(),
        "Command::new(\"/usr/bin/osascript\")".to_string(),
        "Command::new(\"screencapture\")".to_string(),
        "Command::new(\"/usr/sbin/screencapture\")".to_string(),
        "Command::new(\"/bin/ps\")".to_string(),
        "Command::new(\"ps\")".to_string(),
    ]
}

pub(crate) fn scan_rs_files_for_unsafe_opt_ins(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            if ignored_scan_dir(&path) {
                continue;
            }
            scan_rs_files_for_unsafe_opt_ins(&path, violations);
            continue;
        }
        let Ok(body) = fs::read_to_string(&path) else {
            continue;
        };
        if !eligible_test_source(&path, &body) {
            continue;
        }
        for term in forbidden_test_opt_in_terms() {
            if body.contains(&term) {
                violations.push(format!("{} contains {}", path.display(), term));
            }
        }
    }
}

fn forbidden_test_opt_in_terms() -> Vec<String> {
    vec![
        ".env(\"STD_ALLOW_DESKTOP_AUTOMATION\", \"1\")".to_string(),
        ".env(\"STD_ALLOW_UI_PREVIEW\", \"1\")".to_string(),
        ".env(\"STD_ALLOW_BACKGROUND_UI_AUTOMATION\", \"1\")".to_string(),
        ".env_remove(\"STD_ALLOW_DESKTOP_AUTOMATION\")".to_string(),
        ".env_remove(\"STD_ALLOW_UI_PREVIEW\")".to_string(),
        ".env_remove(\"STD_ALLOW_BACKGROUND_UI_AUTOMATION\")".to_string(),
        ".env([\"STD_ALLOW\", \"DESKTOP_AUTOMATION\"]".to_string(),
        ".env([\"STD_ALLOW\", \"UI_PREVIEW\"]".to_string(),
        ".env([\"STD_ALLOW\", \"BACKGROUND_UI_AUTOMATION\"]".to_string(),
        "set_var(\"STD_ALLOW_DESKTOP_AUTOMATION\", \"1\")".to_string(),
        "set_var(\"STD_ALLOW_UI_PREVIEW\", \"1\")".to_string(),
        "set_var(\"STD_ALLOW_BACKGROUND_UI_AUTOMATION\", \"1\")".to_string(),
        "remove_var(\"STD_ALLOW_DESKTOP_AUTOMATION\")".to_string(),
        "remove_var(\"STD_ALLOW_UI_PREVIEW\")".to_string(),
        "remove_var(\"STD_ALLOW_BACKGROUND_UI_AUTOMATION\")".to_string(),
        "set_var([\"STD_ALLOW\", \"DESKTOP_AUTOMATION\"]".to_string(),
        "set_var([\"STD_ALLOW\", \"UI_PREVIEW\"]".to_string(),
        "set_var([\"STD_ALLOW\", \"BACKGROUND_UI_AUTOMATION\"]".to_string(),
    ]
}

fn eligible_test_source(path: &Path, body: &str) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("rs")
        && (is_test_path(path) || body.contains("#[cfg(test)]"))
        && !is_static_desktop_guard_file(path)
        && !is_runtime_desktop_support_file(path)
}

fn ignored_scan_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| matches!(name, ".git" | ".std-cli" | "dist" | "target"))
        .unwrap_or(false)
}

fn is_test_path(path: &Path) -> bool {
    path.components().any(|part| part.as_os_str() == "tests")
        || path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.ends_with("_tests.rs") || name == "tests.rs")
            .unwrap_or(false)
}

fn is_static_desktop_guard_file(path: &Path) -> bool {
    path.ends_with("std-cli/src/tests/desktop_guard.rs")
        || path.ends_with("std-cli/src/tests/desktop_guard_scan.rs")
        || path.ends_with("std-cli/src/tests/desktop_background_ui_guard.rs")
}

fn is_runtime_desktop_support_file(path: &Path) -> bool {
    path.ends_with("std-core/src/lib.rs")
        || path.ends_with("std-core/src/app_bundle.rs")
        || path.ends_with("std-core/src/bootstrap.rs")
        || path.ends_with("std-cli/src/ui/background.rs")
        || path.ends_with("std-launcher/src/gui_smoke.rs")
}
