use crate::StudioWorkspacePolicy;
use std::{fs, path::Path};

const ALLOWED_VIEWPORT_FILES: &[&str] = StudioWorkspacePolicy::VIEWPORT_TOUCHPOINTS;
const ALLOWED_NATIVE_ENTRY_FILES: &[&str] = StudioWorkspacePolicy::NATIVE_ENTRYPOINTS;

#[test]
fn studio_main_path_forbids_detached_or_native_child_windows() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = crate_dir.join("src");
    let mut violations = Vec::new();

    scan_rs_files(&src_dir, &mut violations);
    scan_allowed_viewport_files(&src_dir, &mut violations);

    assert!(
        violations.is_empty(),
        "Studio v1 must stay on one borderless egui host viewport with internal workspace panes: {}",
        violations.join(", ")
    );
}

#[test]
fn studio_settings_must_be_workspace_pane_not_overlay() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let src_dir = crate_dir.join("src");
    let mut violations = Vec::new();

    scan_rs_files_for_settings_overlay(&src_dir, &mut violations);

    assert!(
        violations.is_empty(),
        "Studio settings must open as an internal workspace pane, not a detached overlay: {}",
        violations.join(", ")
    );
}

#[test]
fn viewport_allowlist_uses_exact_src_relative_paths() {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

    assert!(viewport_file_allowed(&crate_dir.join("src/preview.rs")));
    assert!(!viewport_file_allowed(
        &crate_dir.join("src/views/preview.rs")
    ));
    assert!(native_entry_file_allowed(
        &crate_dir.join("src/native_app.rs")
    ));
    assert!(!native_entry_file_allowed(
        &crate_dir.join("src/views/native_app.rs")
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
            || viewport_file_allowed(&path)
            || is_guard_file(&path)
            || is_policy_evidence_file(&path)
        {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        if !native_entry_file_allowed(&path) && body.contains("eframe::run_native") {
            violations.push(format!("{} contains eframe::run_native", path.display()));
        }
        if path.ends_with("studio_open.rs") && body.contains("run_studio_native_app_with") {
            violations.push(format!(
                "{} must emit internal pane intent, not start a host window",
                path.display()
            ));
        }
        for pattern in forbidden_studio_window_patterns() {
            if body.contains(&pattern) {
                violations.push(format!("{} contains {}", path.display(), pattern));
            }
        }
    }
}

fn scan_rs_files_for_settings_overlay(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_rs_files_for_settings_overlay(&path, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs") || is_guard_file(&path) {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        for pattern in ["settings_open", "studio_settings_overlay"] {
            if body.contains(pattern) {
                violations.push(format!("{} contains {}", path.display(), pattern));
            }
        }
    }
}

fn scan_allowed_viewport_files(src_dir: &Path, violations: &mut Vec<String>) {
    for file in ALLOWED_VIEWPORT_FILES {
        let path = src_dir
            .parent()
            .map(|crate_dir| crate_dir.join(file))
            .unwrap_or_else(|| src_dir.join(file));
        let Ok(body) = fs::read_to_string(&path) else {
            violations.push(format!("{} missing allowed viewport file", path.display()));
            continue;
        };
        for pattern in forbidden_studio_window_patterns() {
            if !allowed_viewport_pattern(file, &pattern) && body.contains(&pattern) {
                violations.push(format!("{} contains {}", path.display(), pattern));
            }
        }
        for pattern in required_viewport_patterns(file) {
            if !body.contains(pattern) {
                violations.push(format!("{} missing {}", path.display(), pattern));
            }
        }
    }
}

fn forbidden_studio_window_patterns() -> Vec<String> {
    StudioWorkspacePolicy::FORBIDDEN_WORKBENCH_APIS
        .iter()
        .map(|pattern| pattern.to_string())
        .collect()
}

fn allowed_viewport_pattern(file: &str, pattern: &str) -> bool {
    matches!(
        (file, pattern),
        ("src/viewport.rs", "ViewportBuilder::default")
            | ("src/host_chrome.rs", "ViewportCommand::")
            | ("src/host_chrome.rs", "send_viewport_cmd")
            | ("src/host_chrome_drag.rs", "ViewportCommand::")
            | ("src/host_chrome_drag.rs", "send_viewport_cmd")
            | ("src/preview.rs", "ViewportCommand::")
            | ("src/preview.rs", "send_viewport_cmd")
            | ("src/preview_tests.rs", "ViewportCommand::")
    )
}

fn required_viewport_patterns(file: &str) -> &'static [&'static str] {
    match file {
        "src/viewport.rs" => &[
            "StudioWorkspacePolicy::studio_v1()",
            "HostWindowPolicy::SingleBorderlessEguiViewport",
            "with_decorations(false)",
        ],
        "src/host_chrome.rs" => &[
            "ViewportCommand::Close",
            "ViewportCommand::Minimized(true)",
            "ViewportCommand::Maximized",
            "open_current_pane_from_host_chrome",
        ],
        "src/host_chrome_drag.rs" => &["ViewportCommand::StartDrag"],
        "src/preview.rs" => &[
            "STD_ALLOW_UI_PREVIEW",
            "ViewportCommand::Close",
            "studio_native_options()",
        ],
        "src/preview_tests.rs" => &["ViewportCommand::Close"],
        _ => &[],
    }
}

fn viewport_file_allowed(path: &Path) -> bool {
    studio_src_relative_path(path)
        .as_deref()
        .map(|path| ALLOWED_VIEWPORT_FILES.contains(&path))
        .unwrap_or(false)
}

fn native_entry_file_allowed(path: &Path) -> bool {
    studio_src_relative_path(path)
        .as_deref()
        .map(|path| ALLOWED_NATIVE_ENTRY_FILES.contains(&path))
        .unwrap_or(false)
}

fn is_guard_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == "workspace_policy_guard.rs")
        .unwrap_or(false)
}

fn is_policy_evidence_file(path: &Path) -> bool {
    studio_src_relative_path(path)
        .as_deref()
        .map(|path| {
            matches!(
                path,
                "src/workspace_policy.rs"
                    | "src/smoke/workspace_policy_smoke.rs"
                    | "src/studio_smoke_cli.rs"
            )
        })
        .unwrap_or(false)
}

fn studio_src_relative_path(path: &Path) -> Option<String> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    path.strip_prefix(crate_dir)
        .ok()
        .and_then(|relative| relative.to_str())
        .map(|relative| relative.replace('\\', "/"))
}
