//! std-launcher - Full product foundation
//!
//! Extremely restrained global hotkey launcher with Workflow support.

mod app;
mod cli;
mod gui_smoke;
mod preview;
mod resident;
mod ui;
mod ui_action_bar;
mod ui_action_panel;
mod ui_empty;
mod ui_feedback;
mod ui_keyboard;
mod ui_metrics;
mod ui_metrics_action_panel;
mod ui_metrics_empty;
mod ui_metrics_results;
mod ui_metrics_search;
mod ui_parts;
mod ui_result_model;
mod ui_results;
mod window;

use app::LauncherApp;
use eframe::egui;
use preview::{
    blocked_preview_summary, preview_request_from_args, run_preview, LauncherPreviewRequest,
};

fn main() -> eframe::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
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
    run_launcher_app()
}

fn run_launcher_app() -> eframe::Result<()> {
    eframe::run_native(
        "std-cli Launcher",
        launcher_native_options(),
        Box::new(|_cc| Ok(Box::new(LauncherApp::default()))),
    )
}

fn launcher_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(ui::launcher_initial_window_inner_size())
            .with_decorations(false)
            .with_transparent(true)
            .with_visible(false),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_window_uses_transparent_hidden_chrome() {
        let options = launcher_native_options();
        let description = format!("{:?}", options.viewport);

        assert!(description.contains("transparent: Some(true)"));
        assert!(description.contains("decorations: Some(false)"));
        assert!(description.contains("visible: Some(false)"));
        assert_eq!(
            ui::launcher_initial_window_inner_size(),
            egui::vec2(720.0, 64.0)
        );
    }

    #[test]
    fn launcher_window_opens_with_suggested_results_for_empty_query() {
        let mut state = std_launcher::LauncherState::new();
        let empty_size = ui::launcher_window_inner_size(&state);

        state.update_query("index");
        let expanded_size = ui::launcher_window_inner_size(&state);

        assert!(empty_size.y > 64.0);
        assert_eq!(expanded_size.x, empty_size.x);
        assert!(expanded_size.y > 64.0);
    }
}
