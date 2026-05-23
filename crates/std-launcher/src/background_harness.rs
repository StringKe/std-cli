use crate::app::LauncherApp;
use crate::ui;
use crate::window::{apply_host_window_command, LauncherHostWindowCommand};
use eframe::egui;
use std::env;
use std::time::{Duration, Instant};
use std_launcher::LauncherState;

const HARNESS_TITLE_PREFIX: &str = "std-cli Background UI Harness";
const HARNESS_QUERY: &str = "Echo";
const HARNESS_TOKEN_ENV: &str = "STD_BACKGROUND_UI_HARNESS_TOKEN";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BackgroundHarnessRequest {
    Run(u64),
    Blocked(String),
}

struct BackgroundHarnessApp {
    app: LauncherApp,
    started_at: Instant,
    timeout_ms: u64,
}

impl BackgroundHarnessApp {
    fn new(timeout_ms: u64) -> Self {
        let mut app = LauncherApp::for_background_harness();
        app.state = visible_harness_state();
        Self {
            app,
            started_at: Instant::now(),
            timeout_ms,
        }
    }
}

fn visible_harness_state() -> LauncherState {
    let mut state = LauncherState::new();
    state.controller.show();
    state.update_query(HARNESS_QUERY);
    state
}

impl eframe::App for BackgroundHarnessApp {
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        self.app.clear_color(visuals)
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.update(ctx, frame);
        if self.started_at.elapsed() >= Duration::from_millis(self.timeout_ms) {
            apply_host_window_command(
                ctx,
                LauncherHostWindowCommand::Close,
                ui::launcher_window_inner_size(&self.app.state),
            );
        } else {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}

pub(crate) fn background_harness_request_from_args(
    args: &[String],
) -> Option<BackgroundHarnessRequest> {
    if args.get(1).map(String::as_str) != Some("--background-ui-harness") {
        return None;
    }
    if !background_harness_allowed() {
        return Some(BackgroundHarnessRequest::Blocked(
            background_harness_blocked_reason(),
        ));
    }
    if background_harness_token().is_none() {
        return Some(BackgroundHarnessRequest::Blocked(
            "background UI harness requires STD_BACKGROUND_UI_HARNESS_TOKEN".to_string(),
        ));
    }
    Some(BackgroundHarnessRequest::Run(
        args.get(2)
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(30_000),
    ))
}

fn background_harness_allowed() -> bool {
    !std_core::std_test_mode_enabled()
        && env::var("STD_ALLOW_BACKGROUND_UI_AUTOMATION")
            .map(|value| value == "1")
            .unwrap_or(false)
}

fn background_harness_blocked_reason() -> String {
    if std_core::std_test_mode_enabled() {
        "STD_TEST_MODE blocked background UI harness startup".to_string()
    } else {
        "background UI harness requires STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 explicit opt-in"
            .to_string()
    }
}

pub(crate) fn blocked_background_harness_summary(reason: &str) -> String {
    format!("launcher_background_ui_harness SKIP\nreason={reason}")
}

pub(crate) fn run_background_harness(timeout_ms: u64) -> eframe::Result<()> {
    eframe::run_native(
        &background_harness_title(),
        background_harness_native_options(),
        Box::new(|_cc| Ok(Box::new(BackgroundHarnessApp::new(timeout_ms)))),
    )
}

fn background_harness_title() -> String {
    let token = background_harness_token().unwrap_or_else(|| "missing-token".to_string());
    format!("{HARNESS_TITLE_PREFIX} {token}")
}

fn background_harness_token() -> Option<String> {
    env::var(HARNESS_TOKEN_ENV)
        .ok()
        .map(|token| token.trim().to_string())
        .filter(|token| !token.is_empty())
}

fn background_harness_native_options() -> eframe::NativeOptions {
    std_launcher::launcher_panel_native_options(
        ui::launcher_window_inner_size(&visible_harness_state()),
        true,
    )
}

#[cfg(test)]
fn background_harness_window_contract() -> String {
    std_launcher::transparent_visible_panel_contract(ui::launcher_window_inner_size(
        &visible_harness_state(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    static HARNESS_ENV_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn background_harness_is_blocked_in_test_mode() {
        let args = vec![
            "std-launcher".to_string(),
            "--background-ui-harness".to_string(),
        ];

        let request = background_harness_request_from_args(&args).unwrap();

        assert_eq!(
            request,
            BackgroundHarnessRequest::Blocked(
                "STD_TEST_MODE blocked background UI harness startup".to_string()
            )
        );
    }

    #[test]
    fn background_harness_uses_whitelisted_window_title() {
        let _guard = harness_env_guard();
        std::env::set_var(HARNESS_TOKEN_ENV, "test-token");
        let options = background_harness_native_options();
        let description = format!("{:?}", options.viewport);

        assert_eq!(HARNESS_TITLE_PREFIX, "std-cli Background UI Harness");
        assert_eq!(
            background_harness_title(),
            "std-cli Background UI Harness test-token"
        );
        assert!(description.contains("transparent: Some(true)"));
        assert!(description.contains("decorations: Some(false)"));
        assert!(description.contains("resizable: Some(false)"));
        assert!(description.contains("visible: Some(true)"));
        assert!(background_harness_window_contract().starts_with(
            "native_host=transparent,transparent=true,decorations=false,resizable=false,visible=true,panel_surface=opaque-bg-surface-0,host_background=none,host_gutter=0px,size=720x"
        ));
        std::env::remove_var(HARNESS_TOKEN_ENV);
    }

    #[test]
    fn background_harness_token_contract_rejects_blank_values() {
        let _guard = harness_env_guard();
        std::env::remove_var(HARNESS_TOKEN_ENV);
        assert_eq!(background_harness_token(), None);
        assert_eq!(
            background_harness_title(),
            "std-cli Background UI Harness missing-token"
        );

        std::env::set_var(HARNESS_TOKEN_ENV, "   ");
        assert_eq!(background_harness_token(), None);

        std::env::set_var(HARNESS_TOKEN_ENV, "run-42");
        assert_eq!(background_harness_token().as_deref(), Some("run-42"));
        assert_eq!(
            background_harness_title(),
            "std-cli Background UI Harness run-42"
        );
        std::env::remove_var(HARNESS_TOKEN_ENV);
    }

    fn harness_env_guard() -> MutexGuard<'static, ()> {
        HARNESS_ENV_LOCK.lock().unwrap()
    }

    #[test]
    fn background_harness_app_does_not_register_system_resources() {
        let app = BackgroundHarnessApp::new(1000);

        assert_eq!(
            app.app.system_resource_contract(),
            "hotkey=false,resident=false,status=harness"
        );
    }

    #[test]
    fn background_harness_shows_launcher_panel_inside_visible_window() {
        let state = visible_harness_state();

        assert!(state.controller.visible);
        assert_eq!(state.view.query, HARNESS_QUERY);
        assert!(!state.view.results.is_empty());
        assert_eq!(state.view.results[0].action.name, HARNESS_QUERY);
        assert!(crate::ui_metrics::panel_is_only_visible_surface(&state));
    }

    #[test]
    fn background_harness_forwards_transparent_clear_color() {
        let source = include_str!("background_harness.rs");

        assert!(source.contains("fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4]"));
        assert!(source.contains("self.app.clear_color(visuals)"));
    }
}
