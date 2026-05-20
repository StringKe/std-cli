//! std-launcher - Full product foundation
//!
//! Extremely restrained global hotkey launcher with Workflow support.

mod gui_smoke;
mod resident;
mod ui;
mod ui_action_panel;
mod ui_empty;
mod ui_parts;
mod ui_results;

use eframe::egui;
use gui_smoke::{run_gui_hotkey_smoke, GuiHotkeySmokeConfig};
use resident::{ResidentCommand, ResidentEntry};
use std::time::Duration;
use std_egui::tokens::{apply_theme, Color, Space, ThemeMode};
use std_launcher::{
    hotkey_smoke, GlobalHotkeyRuntime, HotkeySmokeReport, LauncherKeyboardReport,
    LauncherSmokeReport, LauncherState, LauncherUiSemanticsReport, LauncherWindowCommand,
    LauncherWindowSmokeReport,
};
use std_types::{ActionExecution, ActionExecutionStatus};

struct LauncherApp {
    state: LauncherState,
    hotkey_runtime: GlobalHotkeyRuntime,
    hotkey_status: String,
    resident_entry: Option<ResidentEntry>,
    resident_status: String,
    voice_transcript: String,
    theme_mode: ThemeMode,
    allow_close: bool,
}

struct LauncherPreviewConfig {
    theme_mode: ThemeMode,
    scenario: String,
    timeout_ms: u64,
}

impl Default for LauncherApp {
    fn default() -> Self {
        let mut state = LauncherState::new();
        state.hide();
        let plan = state.controller.registration_plan();
        let (hotkey_runtime, hotkey_status) = match GlobalHotkeyRuntime::register(plan.clone()) {
            Ok(runtime) => (runtime, "registered".to_string()),
            Err(error) => (
                GlobalHotkeyRuntime::disabled(plan),
                format!("disabled: {error}"),
            ),
        };
        let (resident_entry, resident_status) = match ResidentEntry::new() {
            Ok(entry) => {
                let status = entry.status().to_string();
                (Some(entry), status)
            }
            Err(error) => (None, format!("menu bar disabled: {error}")),
        };
        Self {
            theme_mode: ThemeMode::resolve(&state.core.config.theme),
            state,
            hotkey_runtime,
            hotkey_status,
            resident_entry,
            resident_status,
            voice_transcript: String::new(),
            allow_close: false,
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx, self.theme_mode);
        if ctx.input(|input| input.viewport().close_requested()) && !self.allow_close {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            apply_window_commands(ctx, self.state.handle_escape_hide());
        }

        if self.take_hotkey_toggle() {
            apply_window_commands(ctx, self.state.handle_hotkey_toggle());
        }
        if let Some(command) = self
            .resident_entry
            .as_ref()
            .and_then(ResidentEntry::poll_command)
        {
            match command {
                ResidentCommand::Show => apply_window_commands(ctx, self.state.handle_show()),
                ResidentCommand::Hide => {
                    apply_window_commands(ctx, self.state.handle_escape_hide());
                }
                ResidentCommand::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
            }
        }

        if ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            apply_window_commands(ctx, self.state.handle_escape_hide());
        }

        if !self.state.controller.visible {
            ctx.request_repaint_after(Duration::from_millis(50));
            return;
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(Color::bg_surface_0(ctx))
                    .inner_margin(egui::Margin::same(Space::MD)),
            )
            .show(ctx, |ui| {
                ui.set_width(ui.available_width());
                if ui::render_launcher_panel(
                    ui,
                    &mut self.state,
                    &self.hotkey_status,
                    &self.resident_status,
                    &mut self.voice_transcript,
                ) {
                    apply_window_commands(ctx, self.state.handle_escape_hide());
                }
            });
    }
}

struct LauncherPreviewApp {
    app: LauncherApp,
    started_at: std::time::Instant,
    timeout_ms: u64,
}

impl LauncherPreviewApp {
    fn new(config: LauncherPreviewConfig) -> Self {
        let mut app = LauncherApp {
            theme_mode: config.theme_mode,
            allow_close: true,
            ..LauncherApp::default()
        };
        app.state.controller.show();
        apply_preview_scenario(&mut app.state, &config.scenario);
        Self {
            app,
            started_at: std::time::Instant::now(),
            timeout_ms: config.timeout_ms,
        }
    }
}

impl eframe::App for LauncherPreviewApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.app.update(ctx, frame);
        if self.started_at.elapsed() >= Duration::from_millis(self.timeout_ms) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        } else {
            ctx.request_repaint_after(Duration::from_millis(100));
        }
    }
}

fn main() -> eframe::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(config) = preview_from_args(&args) {
        return run_preview(config);
    }
    match smoke_from_args(args) {
        Some(LauncherCliSmoke::Launcher(report)) => {
            println!("{}", report.summary());
            return Ok(());
        }
        Some(LauncherCliSmoke::Hotkey(report)) => {
            println!("{}", report.summary());
            return Ok(());
        }
        Some(LauncherCliSmoke::Window(report)) => {
            println!("{}", report.summary());
            return Ok(());
        }
        Some(LauncherCliSmoke::Keyboard(report)) => {
            println!("{}", report.summary());
            return Ok(());
        }
        Some(LauncherCliSmoke::UiSemantics(report)) => {
            println!("{}", report.summary());
            return Ok(());
        }
        Some(LauncherCliSmoke::GuiHotkey(config)) => {
            let report = run_gui_hotkey_smoke(config)?;
            println!("{}", report.summary());
            return Ok(());
        }
        None => {}
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([680.0, 420.0])
            .with_decorations(false)
            .with_transparent(false)
            .with_visible(false),
        ..Default::default()
    };

    eframe::run_native(
        "std-cli Launcher",
        options,
        Box::new(|_cc| Ok(Box::new(LauncherApp::default()))),
    )
}

enum LauncherCliSmoke {
    Launcher(LauncherSmokeReport),
    Hotkey(HotkeySmokeReport),
    Window(LauncherWindowSmokeReport),
    Keyboard(LauncherKeyboardReport),
    UiSemantics(LauncherUiSemanticsReport),
    GuiHotkey(GuiHotkeySmokeConfig),
}

fn smoke_from_args(args: Vec<String>) -> Option<LauncherCliSmoke> {
    match args.get(1).map(String::as_str) {
        Some("--smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("rebuild index");
            LauncherState::smoke(query).map(LauncherCliSmoke::Launcher)
        }
        Some("--hotkey-smoke") => {
            let accelerator = args
                .get(2)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("Alt+Space");
            Some(LauncherCliSmoke::Hotkey(hotkey_smoke(accelerator)))
        }
        Some("--window-smoke") => Some(LauncherCliSmoke::Window(LauncherState::window_smoke())),
        Some("--keyboard-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::Keyboard(LauncherState::keyboard_smoke(
                query,
            )))
        }
        Some("--ui-semantics-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::UiSemantics(
                LauncherState::ui_semantics_smoke(query),
            ))
        }
        Some("--gui-hotkey-smoke") => Some(LauncherCliSmoke::GuiHotkey(GuiHotkeySmokeConfig {
            accelerator: args
                .get(2)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("Alt+Space")
                .to_string(),
            timeout_ms: args
                .get(3)
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(5_000),
            trigger_delay_ms: 500,
        })),
        _ => None,
    }
}

fn preview_from_args(args: &[String]) -> Option<LauncherPreviewConfig> {
    if args.get(1).map(String::as_str) != Some("--ui-preview") {
        return None;
    }
    Some(LauncherPreviewConfig {
        theme_mode: args
            .get(2)
            .map(String::as_str)
            .map(ThemeMode::resolve)
            .unwrap_or(ThemeMode::Dark),
        scenario: args
            .get(3)
            .cloned()
            .unwrap_or_else(|| "results".to_string()),
        timeout_ms: args
            .get(4)
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(8_000),
    })
}

fn run_preview(config: LauncherPreviewConfig) -> eframe::Result<()> {
    eframe::run_native(
        "std-cli Launcher UI Preview",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([680.0, 420.0])
                .with_decorations(false)
                .with_transparent(false)
                .with_visible(true),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(LauncherPreviewApp::new(config)))),
    )
}

fn apply_preview_scenario(state: &mut LauncherState, scenario: &str) {
    match scenario {
        "empty" => {
            state.update_query("");
        }
        "none" | "no-results" => {
            state.update_query("zzzz-no-launcher-match");
        }
        "searching" => {
            state.view.preview_searching("slow query");
        }
        "executing" => {
            state.update_query("index");
            state.view.preview_executing();
        }
        "defer" => {
            state.update_query("terminal");
            select_external_runner_result(state);
            state.trigger_selected();
        }
        "action-panel" => {
            state.update_query("terminal");
            select_external_runner_result(state);
            state.open_action_panel();
        }
        "error" => {
            state.update_query("index");
            state.view.feedback = Some(std_egui::LauncherFeedback::from_execution(
                &ActionExecution {
                    action_id: uuid::Uuid::nil(),
                    action_name: "Preview Failure".to_string(),
                    status: ActionExecutionStatus::Failed,
                    message: "UI preview error state".to_string(),
                    output: None,
                    created_at: chrono::Utc::now(),
                },
            ));
            state.view.phase = std_egui::LauncherPhase::Feedback;
        }
        _ => {
            state.update_query("index");
        }
    }
}

fn apply_window_commands(ctx: &egui::Context, commands: Vec<LauncherWindowCommand>) {
    for command in commands {
        match command {
            LauncherWindowCommand::SetVisible(visible) => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Visible(visible));
            }
            LauncherWindowCommand::Focus => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            }
        }
    }
}

fn select_external_runner_result(state: &mut LauncherState) {
    if let Some(index) = state
        .view
        .results
        .iter()
        .position(|result| result.action.action_type.needs_external_runner())
    {
        state.view.selected = index;
        state.view.refresh_preview(&state.core);
    }
}
impl LauncherApp {
    fn take_hotkey_toggle(&self) -> bool {
        self.hotkey_runtime.poll_toggle_event()
    }
}

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn ui_preview_args_are_explicit_opt_in() {
        let args = vec![
            "std-launcher".to_string(),
            "--ui-preview".to_string(),
            "light".to_string(),
            "defer".to_string(),
            "1200".to_string(),
        ];
        let config = preview_from_args(&args).unwrap();

        assert_eq!(config.theme_mode, ThemeMode::Light);
        assert_eq!(config.scenario, "defer");
        assert_eq!(config.timeout_ms, 1200);
        assert!(smoke_from_args(args).is_none());
    }

    #[test]
    fn ui_preview_scenarios_seed_visible_launcher_states() {
        let mut state = LauncherState::new();

        apply_preview_scenario(&mut state, "no-results");
        assert!(state.view.results.is_empty());
        assert_eq!(state.view.phase, std_egui::LauncherPhase::NoMatches);

        apply_preview_scenario(&mut state, "searching");
        assert_eq!(state.view.phase, std_egui::LauncherPhase::Searching);

        apply_preview_scenario(&mut state, "executing");
        assert_eq!(state.view.phase, std_egui::LauncherPhase::Executing);

        apply_preview_scenario(&mut state, "defer");
        assert_eq!(
            state.view.feedback.as_ref().unwrap().status,
            ActionExecutionStatus::NeedsExternalRunner
        );
        assert_eq!(state.view.phase, std_egui::LauncherPhase::Feedback);

        apply_preview_scenario(&mut state, "action-panel");
        assert!(state.action_panel.open);
        assert_eq!(state.action_panel.action_name, "Open Terminal");

        apply_preview_scenario(&mut state, "error");
        assert_eq!(
            state.view.feedback.as_ref().unwrap().status,
            ActionExecutionStatus::Failed
        );
    }
}
