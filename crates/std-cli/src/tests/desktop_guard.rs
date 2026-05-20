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

fn forbidden_test_app_terms() -> Vec<String> {
    vec![
        ["1", "Password"].join(""),
        ["We", "Chat"].join(""),
        ["Wei", "xin"].join(""),
        String::from("\u{5fae}\u{4fe1}"),
        ["open -a ", "Terminal"].join(""),
        ["open", " -a "].join(""),
        ["/usr/bin/", "open", " -a "].join(""),
        ["osa", "script"].join(""),
        ["System", " Events"].join(""),
        ["tell ", "application"].join(""),
        ["/Applications/", "1", "Password.app"].join(""),
        ["tell application \"", "1", "Password\""].join(""),
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
