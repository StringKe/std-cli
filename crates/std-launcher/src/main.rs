//! std-launcher - Full product foundation
//!
//! Extremely restrained global hotkey launcher with Workflow support.

mod app;
mod background_harness;
mod cli;
mod gui_smoke;
mod preview;
mod preview_affordance;
mod preview_behavior;
mod preview_evidence;
#[cfg(test)]
mod preview_tests;
mod resident;
mod ui;
mod ui_action_bar;
mod ui_action_panel;
mod ui_completion_boundary;
mod ui_empty;
mod ui_feedback;
mod ui_keyboard;
mod ui_metrics;
mod ui_metrics_action_panel;
mod ui_metrics_empty;
mod ui_metrics_layout;
mod ui_metrics_results;
mod ui_metrics_search;
mod ui_parts;
mod ui_preview_panel;
mod ui_result_icons;
mod ui_result_model;
mod ui_result_nl;
mod ui_result_rows;
mod ui_results;
mod ui_results_surface;
mod ui_results_virtual;
mod ui_search;
mod ui_shortcut_help;
mod window;

use app::LauncherApp;
use background_harness::{
    background_harness_request_from_args, blocked_background_harness_summary,
    run_background_harness, BackgroundHarnessRequest,
};
use preview::{
    blocked_preview_summary, preview_request_from_args, run_preview, LauncherPreviewRequest,
};

fn main() -> eframe::Result<()> {
    std_core::sanitize_desktop_opt_ins_for_test_mode();
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(request) = background_harness_request_from_args(&args) {
        match request {
            BackgroundHarnessRequest::Run(timeout_ms) => return run_background_harness(timeout_ms),
            BackgroundHarnessRequest::Blocked(reason) => {
                println!("{}", blocked_background_harness_summary(&reason));
                return Ok(());
            }
        }
    }
    if let Some(request) = preview_request_from_args(&args) {
        match request {
            LauncherPreviewRequest::Run(config) => return run_preview(config),
            LauncherPreviewRequest::Blocked(reason) => {
                println!("{}", blocked_preview_summary(&reason));
                return Ok(());
            }
        }
    }
    if cli::run_smoke_from_args(args)? {
        return Ok(());
    }
    if let Some(reason) = native_app_blocked_by_test_mode() {
        println!("{reason}");
        return Ok(());
    }
    run_launcher_app()
}

fn native_app_blocked_by_test_mode() -> Option<&'static str> {
    std_core::std_test_mode_enabled()
        .then_some("launcher_native_app SKIP reason=STD_TEST_MODE blocked native app startup")
}

fn run_launcher_app() -> eframe::Result<()> {
    eframe::run_native(
        "std-cli Launcher",
        launcher_native_options(),
        Box::new(|_cc| Ok(Box::new(LauncherApp::default()))),
    )
}

fn launcher_native_options() -> eframe::NativeOptions {
    std_launcher::launcher_panel_native_options(ui::launcher_initial_window_inner_size(), false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui;
    use std::{fs, path::Path};

    #[test]
    fn launcher_window_uses_transparent_hidden_chrome() {
        let options = launcher_native_options();
        let description = format!("{:?}", options.viewport);
        let contract = std_launcher::transparent_hidden_panel_contract(
            ui::launcher_initial_window_inner_size(),
        );

        assert!(description.contains("transparent: Some(true)"));
        assert!(description.contains("decorations: Some(false)"));
        assert!(description.contains("resizable: Some(false)"));
        assert!(description.contains("visible: Some(false)"));
        assert_eq!(
            contract,
            "native_host=transparent,transparent=true,decorations=false,resizable=false,visible=false,panel_surface=opaque,host_gap=0x0,size=720x64"
        );
        assert_eq!(
            ui::launcher_initial_window_inner_size(),
            egui::vec2(720.0, 64.0)
        );
    }

    #[test]
    fn launcher_window_expands_from_collapsed_to_results() {
        let mut state = std_launcher::LauncherState::new();
        state.view.results.clear();
        state.view.preview = None;
        let collapsed_size = ui::launcher_window_inner_size(&state);

        state.update_query("index");
        let expanded_size = ui::launcher_window_inner_size(&state);

        assert_eq!(collapsed_size.y, 64.0);
        assert_eq!(expanded_size.x, collapsed_size.x);
        assert!(expanded_size.y > collapsed_size.y);
    }

    #[test]
    fn test_mode_blocks_native_launcher_startup() {
        assert_eq!(
            native_app_blocked_by_test_mode(),
            Some("launcher_native_app SKIP reason=STD_TEST_MODE blocked native app startup")
        );
    }

    #[test]
    fn launcher_product_ui_uses_theme_tokens_not_hardcoded_colors() {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let src_dir = crate_dir.join("src");
        let mut violations = Vec::new();

        scan_rs_files_for_hardcoded_colors(&src_dir, &mut violations);

        assert!(
            violations.is_empty(),
            "Launcher product UI must use std-egui theme tokens instead of hardcoded colors: {}",
            violations.join(", ")
        );
    }

    fn scan_rs_files_for_hardcoded_colors(dir: &Path, violations: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                scan_rs_files_for_hardcoded_colors(&path, violations);
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) != Some("rs")
                || hardcoded_color_file_allowed(&path)
            {
                continue;
            }
            let body = fs::read_to_string(&path).unwrap();
            for pattern in hardcoded_color_patterns() {
                if body.contains(pattern) {
                    violations.push(format!("{} contains {}", path.display(), pattern));
                }
            }
        }
    }

    fn hardcoded_color_file_allowed(path: &Path) -> bool {
        let Some(relative) = launcher_src_relative_path(path) else {
            return false;
        };
        matches!(
            relative.as_str(),
            "src/main.rs"
                | "src/resident.rs"
                | "src/surface_smoke.rs"
                | "src/preview_evidence.rs"
                | "src/preview_tests.rs"
                | "src/cli.rs"
        )
    }

    fn hardcoded_color_patterns() -> &'static [&'static str] {
        &[
            concat!("Color32", "::from_rgb"),
            concat!("Color32", "::from_rgba"),
            "from_black_alpha",
            concat!("#00", "0"),
            concat!("#ff", "f"),
            concat!("#FF", "F"),
        ]
    }

    fn launcher_src_relative_path(path: &Path) -> Option<String> {
        let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        path.strip_prefix(crate_dir)
            .ok()
            .and_then(|relative| relative.to_str())
            .map(|relative| relative.replace('\\', "/"))
    }
}
