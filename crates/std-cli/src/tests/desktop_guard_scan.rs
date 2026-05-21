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
        ["1", "Password"].join(""),
        "[\"1\", \"Password\"]".to_string(),
        "[\"1P\", \"assword\"]".to_string(),
        ["We", "Chat"].join(""),
        "[\"We\", \"Chat\"]".to_string(),
        ["we", "chat"].join(""),
        ["wei", "xin"].join(""),
        "[\"wei\", \"xin\"]".to_string(),
        "微信".to_string(),
        "\\u{5fae}".to_string(),
        "\\u{4fe1}".to_string(),
        ["open -a ", "Terminal"].join(""),
        "Terminal\".to_string()".to_string(),
        ["open", " -a "].join(""),
        "[\"op\", \"en\"".to_string(),
        "[\"op\", \"en\", \"-a\"".to_string(),
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
}

pub(crate) fn workspace_blocks_desktop_opt_ins(body: &str) -> bool {
    body.contains("STD_ALLOW_DESKTOP_AUTOMATION = \"0\"")
        && body.contains("STD_ALLOW_UI_PREVIEW = \"0\"")
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
            scan_rs_files(&path, forbidden_terms, violations);
            continue;
        }
        if !eligible_test_source(&path) {
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

pub(crate) fn scan_rs_files_for_binary_spawns(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_rs_files_for_binary_spawns(&path, violations);
            continue;
        }
        if !eligible_test_source(&path) {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        if !body.contains("CARGO_BIN_EXE_") {
            continue;
        }
        scan_binary_spawn_blocks(&path, &body, violations);
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
            ".env_remove(\"STD_ALLOW_DESKTOP_AUTOMATION\")",
            ".env_remove(\"STD_ALLOW_UI_PREVIEW\")",
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

pub(crate) fn scan_rs_files_for_unsafe_opt_ins(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_rs_files_for_unsafe_opt_ins(&path, violations);
            continue;
        }
        if !eligible_test_source(&path) {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        for term in forbidden_test_opt_in_terms() {
            if body.contains(&term) {
                violations.push(format!("{} contains {}", path.display(), term));
            }
        }
    }
}

fn forbidden_test_opt_in_terms() -> Vec<String> {
    vec![
        ".env(\"STD_ALLOW_DESKTOP_AUTOMATION\"".to_string(),
        ".env(\"STD_ALLOW_UI_PREVIEW\"".to_string(),
        "set_var(\"STD_ALLOW_DESKTOP_AUTOMATION\"".to_string(),
        "set_var(\"STD_ALLOW_UI_PREVIEW\"".to_string(),
    ]
}

fn eligible_test_source(path: &Path) -> bool {
    path.extension().and_then(|ext| ext.to_str()) == Some("rs")
        && is_test_path(path)
        && !is_static_desktop_guard_file(path)
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
}
