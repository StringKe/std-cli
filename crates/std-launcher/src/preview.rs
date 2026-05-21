use crate::app::LauncherApp;
use crate::ui;
use eframe::egui;
use std::env;
use std::time::{Duration, Instant};
use std_egui::tokens::{apply_theme, Color, ThemeMode};
use std_launcher::LauncherState;
use std_types::{ActionExecution, ActionExecutionStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherPreviewConfig {
    pub(crate) theme_mode: ThemeMode,
    pub(crate) scenario: String,
    pub(crate) timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LauncherPreviewRequest {
    Run(LauncherPreviewConfig),
    Blocked(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherPreviewSmokeReport {
    pub(crate) scenarios: Vec<LauncherPreviewScenario>,
    pub(crate) commands: Vec<String>,
    pub(crate) states: Vec<String>,
    pub(crate) capture_contract: &'static str,
}

impl LauncherPreviewSmokeReport {
    pub(crate) fn new() -> Self {
        let scenarios = preview_matrix();
        Self {
            commands: scenarios
                .iter()
                .map(LauncherPreviewScenario::command)
                .collect(),
            states: scenarios.iter().map(preview_state_summary).collect(),
            scenarios,
            capture_contract: preview_capture_contract(),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.scenarios == preview_matrix()
            && self.commands.len() == self.scenarios.len()
            && self.states.iter().all(|state| state.contains("PASS"))
            && self.capture_contract == preview_capture_contract()
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "launcher_preview_smoke {}\npreview_scenarios={}\npreview_commands={}\npreview_states={}\npreview_capture_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.scenarios
                .iter()
                .map(LauncherPreviewScenario::label)
                .collect::<Vec<_>>()
                .join(","),
            self.commands.join(";"),
            self.states.join(";"),
            self.capture_contract
        )
    }
}

struct LauncherPreviewApp {
    app: LauncherApp,
    started_at: Instant,
    timeout_ms: u64,
}

impl LauncherPreviewApp {
    fn new(config: LauncherPreviewConfig) -> Self {
        let mut app = LauncherApp::for_preview(config.theme_mode);
        apply_preview_scenario(&mut app.state, &config.scenario);
        Self {
            app,
            started_at: Instant::now(),
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

pub(crate) fn preview_request_from_args(args: &[String]) -> Option<LauncherPreviewRequest> {
    if args.get(1).map(String::as_str) != Some("--ui-preview") {
        return None;
    }
    if !ui_preview_allowed() {
        return Some(LauncherPreviewRequest::Blocked(ui_preview_blocked_reason()));
    }
    preview_config_from_args(args).map(LauncherPreviewRequest::Run)
}

pub(crate) fn preview_config_from_args(args: &[String]) -> Option<LauncherPreviewConfig> {
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

fn ui_preview_allowed() -> bool {
    !std_core::std_test_mode_enabled()
        && env::var("STD_ALLOW_UI_PREVIEW")
            .map(|value| value == "1")
            .unwrap_or(false)
}

fn ui_preview_blocked_reason() -> String {
    if std_core::std_test_mode_enabled() {
        "STD_TEST_MODE blocked UI preview; use explicit UI preview opt-in outside tests".to_string()
    } else {
        "UI preview requires STD_ALLOW_UI_PREVIEW=1 explicit opt-in".to_string()
    }
}

pub(crate) fn blocked_preview_summary(reason: &str) -> String {
    format!("launcher_ui_preview SKIP\nreason={reason}")
}

pub(crate) fn run_preview(config: LauncherPreviewConfig) -> eframe::Result<()> {
    eframe::run_native(
        preview_window_title(),
        preview_native_options(),
        Box::new(|_cc| Ok(Box::new(LauncherPreviewApp::new(config)))),
    )
}

fn preview_capture_contract() -> &'static str {
    "explicit-opt-in-only,blocked-in-STD_TEST_MODE,no-default-window"
}

pub(crate) fn preview_window_title() -> &'static str {
    "std-cli Launcher"
}

pub(crate) fn preview_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(ui::launcher_initial_window_inner_size())
            .with_decorations(false)
            .with_transparent(true)
            .with_visible(true),
        ..Default::default()
    }
}

pub(crate) fn apply_preview_scenario(state: &mut LauncherState, scenario: &str) {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LauncherPreviewScenario {
    theme: &'static str,
    state: &'static str,
}

impl LauncherPreviewScenario {
    fn label(&self) -> String {
        format!("{}-{}", self.theme, self.state)
    }

    fn command(&self) -> String {
        format!(
            "STD_ALLOW_UI_PREVIEW=1 std-launcher --ui-preview {} {} 8000",
            self.theme, self.state
        )
    }
}

fn preview_matrix() -> Vec<LauncherPreviewScenario> {
    [
        LauncherPreviewScenario {
            theme: "light",
            state: "empty",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "empty",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "results",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "results",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "no-results",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "no-results",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "searching",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "searching",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "executing",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "executing",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "defer",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "defer",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "error",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "error",
        },
        LauncherPreviewScenario {
            theme: "light",
            state: "action-panel",
        },
        LauncherPreviewScenario {
            theme: "dark",
            state: "action-panel",
        },
    ]
    .into_iter()
    .collect()
}

fn preview_state_summary(scenario: &LauncherPreviewScenario) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let valid =
        matches!(scenario.theme, "dark" | "light") && preview_state_passes(&state, scenario.state);
    let theme = ThemeMode::resolve(scenario.theme);
    let surface = preview_surface_summary(theme);
    format!(
        "{}={}:phase={:?},results={},feedback={},{}",
        scenario.label(),
        if valid { "PASS" } else { "FAIL" },
        state.view.phase,
        state.view.results.len(),
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.title.as_str())
            .unwrap_or("none"),
        surface
    )
}

fn preview_state_passes(state: &LauncherState, state_name: &str) -> bool {
    match state_name {
        "empty" => {
            state.view.phase == std_egui::LauncherPhase::Empty
                && state.view.result_mode == std_egui::LauncherResultMode::SuggestedWorkflows
        }
        "results" => {
            state.view.phase == std_egui::LauncherPhase::WithResults
                && !state.view.results.is_empty()
        }
        "no-results" => {
            state.view.phase == std_egui::LauncherPhase::NoMatches && state.view.results.is_empty()
        }
        "defer" => state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.status == ActionExecutionStatus::NeedsExternalRunner)
            .unwrap_or(false),
        "error" => state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.status == ActionExecutionStatus::Failed)
            .unwrap_or(false),
        "searching" => state.view.phase == std_egui::LauncherPhase::Searching,
        "executing" => state.view.phase == std_egui::LauncherPhase::Executing,
        "action-panel" => state.action_panel.open,
        _ => false,
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

fn preview_surface_summary(theme: ThemeMode) -> String {
    let ctx = egui::Context::default();
    apply_theme(&ctx, theme);
    format!(
        "panel={},search={},result={},selected={}",
        color_hex(Color::bg_surface_0(&ctx)),
        color_hex(Color::bg_surface_1(&ctx)),
        color_hex(Color::bg_surface_1(&ctx)),
        color_hex_alpha(Color::accent_weak(&ctx))
    )
}

fn color_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

fn color_hex_alpha(color: egui::Color32) -> String {
    format!(
        "#{:02X}{:02X}{:02X}@{}",
        color.r(),
        color.g(),
        color.b(),
        color.a()
    )
}
