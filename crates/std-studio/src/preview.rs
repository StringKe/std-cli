use crate::{
    preview_evidence::{preview_matrix, preview_size_summary, preview_state_summary},
    viewport::studio_native_options,
    StudioEguiApp, StudioPane,
};
use eframe::egui;
use std::env;
use std_core::{StdConfig, StdCore};
use std_egui::tokens::ThemeMode;
use std_orchestration::{ExecutionStatus, StepResult, WorkflowExecution};

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
    pub(crate) scenarios: Vec<String>,
    pub(crate) commands: Vec<String>,
    pub(crate) states: Vec<String>,
    pub(crate) sizes: Vec<String>,
    pub(crate) required_capture_states: Vec<String>,
    pub(crate) capture_contract: &'static str,
}

impl StudioPreviewSmokeReport {
    pub(crate) fn new() -> Self {
        let scenarios = preview_matrix();
        Self {
            commands: scenarios
                .iter()
                .map(|scenario| {
                    let (theme, name) = scenario.split_once('-').unwrap_or(("dark", "dashboard"));
                    format!(
                        "STD_ALLOW_UI_PREVIEW=1 cargo run -p std-studio -- --ui-preview {theme} {name} 8000"
                    )
                })
                .collect(),
            states: scenarios
                .iter()
                .map(|scenario| preview_state_summary(scenario))
                .collect(),
            sizes: scenarios
                .iter()
                .map(|scenario| preview_size_summary(scenario))
                .collect(),
            required_capture_states: required_capture_states(&scenarios),
            scenarios,
            capture_contract: preview_capture_contract(),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.scenarios == preview_matrix()
            && self.commands.len() == self.scenarios.len()
            && self.states.iter().all(|state| state.contains("PASS"))
            && self.sizes.iter().all(|size| size.contains("PASS"))
            && self.required_capture_states == required_capture_states(&self.scenarios)
            && required_capture_states_pass(&self.required_capture_states)
            && self.capture_contract == preview_capture_contract()
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_preview_smoke {}\npreview_scenarios={}\npreview_commands={}\npreview_states={}\npreview_sizes={}\nrequired_capture_states={}\npreview_capture_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.scenarios.join(","),
            self.commands.join(";"),
            self.states.join(";"),
            self.sizes.join(";"),
            self.required_capture_states.join(","),
            self.capture_contract
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

pub(crate) fn preview_data_dir() -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "std-cli-studio-ui-preview-{}-{}",
        std::process::id(),
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default()
    ))
}

impl eframe::App for StudioPreviewApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.started_at.elapsed() >= std::time::Duration::from_millis(self.timeout_ms) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
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

pub(crate) fn studio_preview_config_from_args(args: &[String]) -> Option<StudioPreviewConfig> {
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

fn preview_capture_contract() -> &'static str {
    "explicit-opt-in-only,checkout-binary-only,blocked-in-STD_TEST_MODE,no-default-window,normal-viewport-close"
}

fn required_capture_states(scenarios: &[String]) -> Vec<String> {
    [
        "light-dashboard",
        "dark-dashboard",
        "light-workflow",
        "dark-workflow",
        "light-workflow-error",
        "dark-workflow-error",
        "light-analysis",
        "dark-analysis",
        "light-plugins",
        "dark-plugins",
        "light-plugin-permission",
        "dark-plugin-permission",
        "light-operations",
        "dark-operations",
        "light-settings",
        "dark-settings",
        "light-panes",
        "dark-panes",
    ]
    .into_iter()
    .filter(|required| scenarios.iter().any(|scenario| scenario == *required))
    .map(str::to_string)
    .collect()
}

fn required_capture_states_pass(states: &[String]) -> bool {
    states
        == [
            "light-dashboard",
            "dark-dashboard",
            "light-workflow",
            "dark-workflow",
            "light-workflow-error",
            "dark-workflow-error",
            "light-analysis",
            "dark-analysis",
            "light-plugins",
            "dark-plugins",
            "light-plugin-permission",
            "dark-plugin-permission",
            "light-operations",
            "dark-operations",
            "light-settings",
            "dark-settings",
            "light-panes",
            "dark-panes",
        ]
}

pub(crate) fn studio_preview_window_title() -> &'static str {
    "std-cli Studio"
}

pub(crate) fn apply_studio_preview_scenario(app: &mut StudioEguiApp, scenario: &str) {
    match scenario {
        "workflow" => seed_workflow_preview(app),
        "workflow-error" => seed_workflow_error_preview(app),
        "analysis" => seed_analysis_preview(app),
        "plugins" => seed_plugin_preview(app),
        "plugin-permission" => seed_plugin_permission_preview(app),
        "operations" => app.app.switch_pane(StudioPane::Operations),
        "settings" => app.app.switch_pane(StudioPane::Settings),
        "panes" | "windows" | "viewports" => {
            seed_panes_preview(app);
        }
        _ => app.app.switch_pane(StudioPane::Dashboard),
    }
}

pub(crate) fn seeded_preview_app(theme: &str, scenario: &str) -> StudioEguiApp {
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

pub(crate) fn seed_workflow_preview(app: &mut StudioEguiApp) {
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
    app.app.open_execution_history_pane();
    app.layout.open_bottom_panel();
    app.status = "workflow preview seeded".to_string();
}

pub(crate) fn seed_workflow_error_preview(app: &mut StudioEguiApp) {
    seed_workflow_preview(app);
    let Some(workflow_path) = app.workflow_selected_path.clone() else {
        app.status = "workflow error preview missing workflow".to_string();
        return;
    };
    let now = chrono::Utc::now();
    app.app.last_workflow_execution = Some(WorkflowExecution {
        workflow_id: uuid::Uuid::new_v4(),
        workflow_name: "Preview Failure".to_string(),
        status: ExecutionStatus::Failed,
        current_step: 1,
        started_at: now,
        finished_at: Some(now),
        results: vec![StepResult {
            step_id: uuid::Uuid::new_v4(),
            step_name: "Fail preview step".to_string(),
            status: ExecutionStatus::Failed,
            output: serde_json::json!({
                "error": "Preview workflow failure"
            }),
            started_at: now,
            finished_at: now,
        }],
    });
    app.app.open_workflow_builder(workflow_path);
    app.layout.open_bottom_panel();
    app.bottom_panel_tab = crate::bottom_panel_model::BottomPanelTab::Problems;
    app.status = "workflow error preview seeded".to_string();
}

fn seed_panes_preview(app: &mut StudioEguiApp) {
    let plugin = app.app.open_plugin_manager_pane();
    let memory = app.app.open_memory_browser_pane();
    let history = app.app.open_execution_history_pane();
    let opened = app.app.open_workspace_panes().count() >= 3;
    let focus = app.app.focus_workspace_pane(plugin) && app.app.focused_pane == Some(plugin);
    let state_preserved = app
        .app
        .workspace_panes
        .iter()
        .find(|pane| pane.id == plugin)
        .map(|pane| app.app.workspace_pane_content(&pane.kind))
        .map(|content| {
            content
                .lines
                .iter()
                .any(|line| line.contains("manifest_check"))
        })
        .unwrap_or(false);
    let closed = app.app.close_workspace_pane(memory)
        && !app.app.open_workspace_panes().any(|pane| pane.id == memory);
    let restored = app.app.open_memory_browser_pane() == memory
        && app.app.open_workspace_panes().any(|pane| pane.id == memory);
    let history_visible = app
        .app
        .open_workspace_panes()
        .any(|pane| pane.id == history);
    app.status = format!(
        "panes preview seeded open={opened},focus={focus},close={closed},restore={restored},state_preserved={state_preserved},history_visible={history_visible}"
    );
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

fn seed_plugin_permission_preview(app: &mut StudioEguiApp) {
    app.app.switch_pane(StudioPane::Plugins);
    app.app.open_plugin_manager_pane();
    let plugin_dir = app
        .app
        .core
        .config
        .plugins_dir()
        .join("permission-preview-plugin");
    let fs_scope = plugin_dir.join("allowed");
    if std::fs::create_dir_all(&fs_scope).is_err() {
        return;
    }
    let _ = std::fs::write(
        plugin_dir.join("main.js"),
        r#"std.emit({ allowed: true });"#,
    );
    let _ = std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "permission-preview-plugin",
            "description": "Studio permission boundary preview",
            "permissions": ["code", "fs_scoped", "network"],
            "fs_scopes": ["allowed"],
            "network_hosts": ["api.preview.local"],
            "actions": [{
                "name": "Permission Preview Plugin",
                "description": "Preview plugin permission boundary",
                "when_to_use": "When reviewing plugin permissions before enable",
                "kind": "javascript",
                "script": "main.js",
                "tags": ["permission-preview-plugin"]
            }]
        })
        .to_string(),
    );
    let _ = app.app.reload_plugins();
    app.app.search_plugins("Permission Preview Plugin");
    app.status = "plugin permission preview seeded".to_string();
}
