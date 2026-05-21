use crate::{viewport::studio_native_options, StudioEguiApp, StudioPane};
use eframe::egui;
use std::env;
use std_core::{StdConfig, StdCore};
use std_egui::tokens::ThemeMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioPreviewConfig {
    pub theme: String,
    pub scenario: String,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StudioPreviewRequest {
    Run(StudioPreviewConfig),
    Blocked(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioPreviewSmokeReport {
    scenarios: Vec<String>,
    commands: Vec<String>,
    states: Vec<String>,
}

impl StudioPreviewSmokeReport {
    pub(crate) fn new() -> Self {
        let scenarios = preview_matrix();
        Self {
            commands: scenarios
                .iter()
                .map(|scenario| {
                    let (theme, name) = scenario.split_once('-').unwrap_or(("dark", "dashboard"));
                    format!("STD_ALLOW_UI_PREVIEW=1 std-studio --ui-preview {theme} {name} 8000")
                })
                .collect(),
            states: scenarios
                .iter()
                .map(|scenario| preview_state_summary(scenario))
                .collect(),
            scenarios,
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.scenarios == preview_matrix()
            && self.commands.len() == self.scenarios.len()
            && self.states.iter().all(|state| state.contains("PASS"))
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_preview_smoke {}\npreview_scenarios={}\npreview_commands={}\npreview_states={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.scenarios.join(","),
            self.commands.join(";"),
            self.states.join(";")
        )
    }
}

struct StudioPreviewApp {
    app: StudioEguiApp,
    started_at: std::time::Instant,
    timeout_ms: u64,
}

impl StudioPreviewApp {
    fn new(config: StudioPreviewConfig) -> Self {
        let app = seeded_preview_app(&config.theme, &config.scenario);
        Self {
            app,
            started_at: std::time::Instant::now(),
            timeout_ms: config.timeout_ms,
        }
    }
}

fn preview_data_dir() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "std-cli-studio-ui-preview-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ))
}

impl eframe::App for StudioPreviewApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.started_at.elapsed() >= std::time::Duration::from_millis(self.timeout_ms) {
            std::process::exit(0);
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(50));
        self.app.update(ctx, frame);
    }
}

pub(crate) fn studio_preview_request_from_args(args: &[String]) -> Option<StudioPreviewRequest> {
    if args.get(1).map(String::as_str) != Some("--ui-preview") {
        return None;
    }
    if !studio_preview_allowed() {
        return Some(StudioPreviewRequest::Blocked(
            studio_preview_blocked_reason(),
        ));
    }
    studio_preview_config_from_args(args).map(StudioPreviewRequest::Run)
}

fn studio_preview_config_from_args(args: &[String]) -> Option<StudioPreviewConfig> {
    let theme = args
        .get(2)
        .map(String::as_str)
        .map(ThemeMode::resolve)
        .map(|mode| match mode {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
            ThemeMode::System => "system",
        })
        .unwrap_or("dark")
        .to_string();
    Some(StudioPreviewConfig {
        theme,
        scenario: args
            .get(3)
            .cloned()
            .unwrap_or_else(|| "dashboard".to_string()),
        timeout_ms: args
            .get(4)
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(8_000),
    })
}

fn studio_preview_allowed() -> bool {
    !std_core::std_test_mode_enabled()
        && env::var("STD_ALLOW_UI_PREVIEW")
            .map(|value| value == "1")
            .unwrap_or(false)
}

fn studio_preview_blocked_reason() -> String {
    if std_core::std_test_mode_enabled() {
        "STD_TEST_MODE blocked Studio UI preview; use explicit UI preview opt-in outside tests"
            .to_string()
    } else {
        "Studio UI preview requires STD_ALLOW_UI_PREVIEW=1 explicit opt-in".to_string()
    }
}

pub(crate) fn blocked_studio_preview_summary(reason: &str) -> String {
    format!("studio_ui_preview SKIP\nreason={reason}")
}

pub(crate) fn run_studio_preview(config: StudioPreviewConfig) -> eframe::Result<()> {
    eframe::run_native(
        studio_preview_window_title(),
        studio_native_options(),
        Box::new(|_cc| Ok(Box::new(StudioPreviewApp::new(config)))),
    )
}

fn studio_preview_window_title() -> &'static str {
    "std-cli Studio"
}

fn apply_studio_preview_scenario(app: &mut StudioEguiApp, scenario: &str) {
    match scenario {
        "workflow" => seed_workflow_preview(app),
        "analysis" => seed_analysis_preview(app),
        "plugins" => seed_plugin_preview(app),
        "operations" => app.app.switch_pane(StudioPane::Operations),
        "settings" => app.app.switch_pane(StudioPane::Settings),
        "panes" | "windows" | "viewports" => {
            app.app.open_plugin_manager_pane();
            app.app.open_memory_browser_pane();
            app.app.open_execution_history_pane();
        }
        _ => app.app.switch_pane(StudioPane::Dashboard),
    }
}

fn seeded_preview_app(theme: &str, scenario: &str) -> StudioEguiApp {
    let core = StdCore::with_config(StdConfig {
        data_dir: preview_data_dir(),
        theme: theme.to_string(),
        ..StdConfig::default()
    });
    let mut app = StudioEguiApp {
        app: std_studio::StudioApp::with_core(core),
        ..StudioEguiApp::default()
    };
    app.sync_settings_from_app();
    apply_studio_preview_scenario(&mut app, scenario);
    app
}

fn preview_matrix() -> Vec<String> {
    [
        "dark-dashboard",
        "light-dashboard",
        "dark-workflow",
        "light-analysis",
        "dark-plugins",
        "light-operations",
        "dark-settings",
        "light-panes",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

fn preview_state_summary(scenario: &str) -> String {
    let Some((theme, name)) = scenario.split_once('-') else {
        return format!("{scenario}=FAIL");
    };
    let app = seeded_preview_app(theme, name);
    let valid = matches!(theme, "dark" | "light") && preview_state_passes(&app, name);
    format!(
        "{scenario}={}:pane={:?},workspace={},status={}",
        if valid { "PASS" } else { "FAIL" },
        app.app.active_pane,
        app.app.open_workspace_panes().count(),
        app.status
    )
}

fn preview_state_passes(app: &StudioEguiApp, scenario: &str) -> bool {
    match scenario {
        "dashboard" => app.app.active_pane == StudioPane::Dashboard,
        "workflow" => {
            app.app.active_pane == StudioPane::Workflows
                && app.app.workflow_debug.is_some()
                && app.app.last_workflow_execution.is_some()
        }
        "analysis" => {
            app.app.active_pane == StudioPane::Analysis && !app.analysis.coverage_output.is_empty()
        }
        "plugins" => {
            app.app.active_pane == StudioPane::Plugins
                && app.app.open_workspace_panes().count() >= 1
        }
        "operations" => app.app.active_pane == StudioPane::Operations,
        "settings" => app.app.active_pane == StudioPane::Settings,
        "panes" => app.app.open_workspace_panes().count() >= 3,
        _ => false,
    }
}

fn seed_workflow_preview(app: &mut StudioEguiApp) {
    app.app.switch_pane(StudioPane::Workflows);
    app.workflow_name = "Preview Release".to_string();
    app.workflow_description = "Preview workflow for Studio UI evidence".to_string();
    let path = match app
        .app
        .create_workflow(&app.workflow_name, &app.workflow_description)
    {
        Ok(path) => path,
        Err(error) => {
            app.status = error.to_string();
            return;
        }
    };
    app.workflow_selected_path = Some(path.clone());
    app.workflow_step_name = "Collect context".to_string();
    app.workflow_step_parameters = serde_json::json!({"source": "preview"}).to_string();
    let _ = app.app.add_workflow_step(
        &path,
        "Collect context",
        serde_json::json!({"source": "preview"}),
    );
    let _ = app.app.add_workflow_step(
        &path,
        "Summarize result",
        serde_json::json!({"format": "brief"}),
    );
    let _ = app.app.preview_workflow_path(&path);
    let _ = app.app.run_workflow_path(&path);
    app.app.open_workflow_builder(path);
    app.status = "workflow preview seeded".to_string();
}

fn seed_analysis_preview(app: &mut StudioEguiApp) {
    app.app.switch_pane(StudioPane::Analysis);
    let project_dir = app.app.core.config.data_dir.join("preview-project");
    let src_dir = project_dir.join("src");
    if std::fs::create_dir_all(&src_dir).is_err() {
        return;
    }
    let source = "pub struct StudioPreviewAnalysis;\nfn workflow_preview() {}\n";
    if std::fs::write(src_dir.join("lib.rs"), source).is_err() {
        return;
    }
    app.analysis.path = project_dir.display().to_string();
    app.analysis.query = "StudioPreviewAnalysis".to_string();
    if app.app.analyze_entity(&project_dir).is_ok() {
        app.analysis.coverage_output = app
            .app
            .analysis_coverage_report()
            .map(|report| format!("coverage complete={}", report.complete))
            .unwrap_or_else(|error| error.to_string());
        app.app.open_analysis_workbench(project_dir);
        app.status = "analysis preview seeded".to_string();
    }
}

fn seed_plugin_preview(app: &mut StudioEguiApp) {
    app.app.switch_pane(StudioPane::Plugins);
    app.app.open_plugin_manager_pane();
    let plugin_dir = app.app.core.config.plugins_dir().join("preview-plugin");
    if std::fs::create_dir_all(&plugin_dir).is_err() {
        return;
    }
    let _ = std::fs::write(plugin_dir.join("main.js"), r#"std.emit({ ok: true });"#);
    let _ = std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "preview-plugin",
            "description": "Studio preview plugin",
            "permissions": ["code"],
            "actions": [{
                "name": "Preview Plugin Action",
                "description": "Preview plugin action",
                "when_to_use": "When previewing Studio plugin UI",
                "kind": "javascript",
                "script": "main.js",
                "tags": ["preview-plugin"]
            }]
        })
        .to_string(),
    );
    let _ = app.app.reload_plugins();
    app.status = "plugin preview seeded".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::smoke::smoke_from_args;

    #[test]
    fn ui_preview_args_are_explicit_opt_in() {
        let args = vec![
            "std-studio".to_string(),
            "--ui-preview".to_string(),
            "light".to_string(),
            "panes".to_string(),
            "900".to_string(),
        ];
        let config = studio_preview_config_from_args(&args).unwrap();

        assert_eq!(config.theme, "light");
        assert_eq!(config.scenario, "panes");
        assert_eq!(config.timeout_ms, 900);
        assert!(smoke_from_args(args).is_none());
    }

    #[test]
    fn ui_preview_uses_product_window_title() {
        assert_eq!(studio_preview_window_title(), "std-cli Studio");
    }

    #[test]
    fn ui_preview_args_are_blocked_without_opt_in() {
        std::env::remove_var("STD_ALLOW_UI_PREVIEW");
        let args = vec![
            "std-studio".to_string(),
            "--ui-preview".to_string(),
            "light".to_string(),
            "panes".to_string(),
            "900".to_string(),
        ];

        let Some(StudioPreviewRequest::Blocked(reason)) = studio_preview_request_from_args(&args)
        else {
            panic!("expected blocked Studio UI preview request");
        };
        assert!(reason.contains("STD_TEST_MODE blocked Studio UI preview"));
        assert!(blocked_studio_preview_summary(&reason).contains("studio_ui_preview SKIP"));
    }

    #[test]
    fn workflow_preview_seeds_builder_runtime_state() {
        let core = StdCore::with_config(StdConfig {
            data_dir: preview_data_dir(),
            ..StdConfig::default()
        });
        let mut app = StudioEguiApp {
            app: std_studio::StudioApp::with_core(core),
            ..StudioEguiApp::default()
        };

        seed_workflow_preview(&mut app);

        assert_eq!(app.app.active_pane, StudioPane::Workflows);
        assert!(app.workflow_selected_path.is_some());
        assert!(app.app.workflow_debug.is_some());
        assert!(app.app.last_workflow_execution.is_some());
        assert_eq!(app.app.open_workspace_panes().count(), 1);
    }

    #[test]
    fn preview_smoke_reports_required_studio_screenshot_matrix() {
        let report = StudioPreviewSmokeReport::new();
        let summary = report.summary();

        assert!(report.pass(), "{summary}");
        assert!(summary.contains("studio_preview_smoke PASS"));
        assert!(summary.contains("dark-workflow"));
        assert!(summary.contains("light-analysis"));
        assert!(summary.contains("dark-settings"));
        assert!(summary.contains("STD_ALLOW_UI_PREVIEW=1"));
    }
}
