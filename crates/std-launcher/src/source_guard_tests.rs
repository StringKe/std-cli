use std::{fs, path::Path};

const ALLOWED_NATIVE_ENTRY_FILES: &[&str] = &[
    "src/main.rs",
    "src/preview.rs",
    "src/gui_smoke.rs",
    "src/background_harness.rs",
];

const ALLOWED_HOST_WINDOW_FILES: &[&str] = &[
    "src/window.rs",
    "src/preview.rs",
    "src/gui_smoke.rs",
    "src/viewport_contract.rs",
];

#[test]
fn launcher_product_ui_cannot_create_native_hosts_or_extra_viewports() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = crate_dir.join("src");
    let mut violations = Vec::new();

    scan_rs_files(&src_dir, &mut violations);

    assert!(
        violations.is_empty(),
        "Launcher UI must stay on one transparent native host with an opaque token panel: {}",
        violations.join(", ")
    );
}

#[test]
fn launcher_native_host_allowlist_uses_exact_src_relative_paths() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(native_entry_file_allowed(&crate_dir.join("src/main.rs")));
    assert!(host_window_file_allowed(&crate_dir.join("src/window.rs")));
    assert!(!native_entry_file_allowed(
        &crate_dir.join("src/ui/main.rs")
    ));
    assert!(!host_window_file_allowed(
        &crate_dir.join("src/ui/window.rs")
    ));
}

fn scan_rs_files(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_rs_files(&path, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
            || is_guard_file(&path)
            || is_test_module_file(&path)
        {
            continue;
        }
        let body = production_source(&fs::read_to_string(&path).unwrap()).to_string();
        if !native_entry_file_allowed(&path) && body.contains("eframe::run_native") {
            violations.push(format!("{} contains eframe::run_native", path.display()));
        }
        for pattern in forbidden_host_window_patterns() {
            if !host_window_file_allowed(&path) && body.contains(pattern) {
                violations.push(format!("{} contains {}", path.display(), pattern));
            }
        }
    }
}

fn production_source(body: &str) -> &str {
    body.split("#[cfg(test)]").next().unwrap_or(body)
}

fn forbidden_host_window_patterns() -> &'static [&'static str] {
    &[
        "ViewportBuilder::default",
        "ViewportCommand::",
        "send_viewport_cmd",
        "egui::Window::new",
        "Window::new",
    ]
}

fn native_entry_file_allowed(path: &Path) -> bool {
    launcher_src_relative_path(path)
        .as_deref()
        .map(|path| ALLOWED_NATIVE_ENTRY_FILES.contains(&path))
        .unwrap_or(false)
}

fn host_window_file_allowed(path: &Path) -> bool {
    launcher_src_relative_path(path)
        .as_deref()
        .map(|path| ALLOWED_HOST_WINDOW_FILES.contains(&path))
        .unwrap_or(false)
}

fn is_guard_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == "source_guard_tests.rs")
        .unwrap_or(false)
}

fn is_test_module_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == "tests.rs" || name.ends_with("_tests.rs"))
        .unwrap_or(false)
}

fn launcher_src_relative_path(path: &Path) -> Option<String> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    path.strip_prefix(crate_dir)
        .ok()
        .and_then(|relative| relative.to_str())
        .map(|relative| relative.replace('\\', "/"))
}
