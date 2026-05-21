use std::{fs, path::Path};

const PRODUCT_UI_DIRS: [&str; 2] = ["crates/std-launcher/src", "crates/std-studio/src"];

#[test]
fn launcher_and_studio_ui_do_not_hardcode_theme_colors() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let mut violations = Vec::new();

    for relative in PRODUCT_UI_DIRS {
        scan_product_ui_sources(&root.join(relative), &mut violations);
    }

    assert!(
        violations.is_empty(),
        "Launcher and Studio UI must use std-egui color tokens instead of hardcoded theme colors: {}",
        violations.join(", ")
    );
}

fn scan_product_ui_sources(dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            scan_product_ui_sources(&path, violations);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
            || allowed_non_ui_color_file(&path)
        {
            continue;
        }
        let body = fs::read_to_string(&path).unwrap();
        for pattern in forbidden_color_patterns() {
            if body.contains(pattern) {
                violations.push(format!("{} contains {}", path.display(), pattern));
            }
        }
    }
}

fn forbidden_color_patterns() -> [&'static str; 8] {
    [
        "Color32::from_rgb(",
        "Color32::from_rgba_",
        "Color32::from_gray",
        "Color32::BLACK",
        "Color32::WHITE",
        "egui::Visuals::dark",
        "egui::Visuals::light",
        "#000",
    ]
}

fn allowed_non_ui_color_file(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| {
            matches!(
                name,
                "resident.rs" | "preview_evidence.rs" | "preview_tests.rs"
            )
        })
        .unwrap_or(false)
}
