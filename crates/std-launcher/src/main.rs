//! std-launcher - Full product foundation
//!
//! Extremely restrained global hotkey launcher with Workflow support.

mod app;
mod cli;
mod gui_smoke;
mod preview;
mod resident;
mod ui;
mod ui_action_panel;
mod ui_empty;
mod ui_parts;
mod ui_results;
mod window;

use app::LauncherApp;
use eframe::egui;
use preview::{preview_from_args, run_preview};

fn main() -> eframe::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(config) = preview_from_args(&args) {
        return run_preview(config);
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
            .with_inner_size([880.0, 520.0])
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
    }
}
