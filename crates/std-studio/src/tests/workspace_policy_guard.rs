use std::{fs, path::Path};

const ALLOWED_VIEWPORT_FILES: &[&str] = &["viewport.rs", "host_chrome.rs"];

#[test]
fn studio_main_path_forbids_detached_or_native_child_windows() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = crate_dir.join("src");
    let mut violations = Vec::new();

    scan_rs_files(&src_dir, &mut violations);

    assert!(
        violations.is_empty(),
        "Studio v1 must stay on one borderless egui host viewport with internal workspace panes: {}",
        violations.join(", ")
    );
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
            || viewport_file_allowed(&path)
        {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        for pattern in forbidden_studio_window_patterns() {
            if body.contains(&pattern) {
                violations.push(format!("{} contains {}", path.display(), pattern));
            }
        }
    }
}

fn forbidden_studio_window_patterns() -> Vec<String> {
    vec![
        ["egui::", "Window", "::new"].join(""),
        ["Window", "::new"].join(""),
        ["Viewport", "Builder::default"].join(""),
        ["Viewport", "Command::"].join(""),
        ["send_", "viewport_cmd"].join(""),
    ]
}

fn viewport_file_allowed(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| ALLOWED_VIEWPORT_FILES.contains(&name))
        .unwrap_or(false)
}
