use crate::{viewport::studio_native_options, StudioEguiApp, StudioPane};
use eframe::egui;
use std_core::{StdConfig, StdCore};
use std_egui::tokens::ThemeMode;

pub(crate) struct StudioPreviewConfig {
    pub theme: String,
    pub scenario: String,
    pub timeout_ms: u64,
}

struct StudioPreviewApp {
    app: StudioEguiApp,
    started_at: std::time::Instant,
    timeout_ms: u64,
}

impl StudioPreviewApp {
    fn new(config: StudioPreviewConfig) -> Self {
        let mut app = StudioEguiApp::default();
        app.app.core.config.theme = config.theme;
        if config.scenario == "workflow" {
            let core = StdCore::with_config(StdConfig {
                data_dir: preview_data_dir(),
                theme: app.app.core.config.theme.clone(),
                ..StdConfig::default()
            });
            app.app = std_studio::StudioApp::with_core(core);
            app.sync_settings_from_app();
        }
        apply_studio_preview_scenario(&mut app, &config.scenario);
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

pub(crate) fn studio_preview_from_args(args: &[String]) -> Option<StudioPreviewConfig> {
    if args.get(1).map(String::as_str) != Some("--ui-preview") {
        return None;
    }
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

pub(crate) fn run_studio_preview(config: StudioPreviewConfig) -> eframe::Result<()> {
    eframe::run_native(
        "std-cli Studio UI Preview",
        studio_native_options(),
        Box::new(|_cc| Ok(Box::new(StudioPreviewApp::new(config)))),
    )
}

fn apply_studio_preview_scenario(app: &mut StudioEguiApp, scenario: &str) {
    match scenario {
        "workflow" => seed_workflow_preview(app),
        "analysis" => app.app.switch_pane(StudioPane::Analysis),
        "plugins" => {
            app.app.switch_pane(StudioPane::Plugins);
            app.app.open_plugin_manager_pane();
        }
        "panes" | "windows" | "viewports" => {
            app.app.open_plugin_manager_pane();
            app.app.open_memory_browser_pane();
            app.app.open_execution_history_pane();
        }
        _ => app.app.switch_pane(StudioPane::Dashboard),
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
        let config = studio_preview_from_args(&args).unwrap();

        assert_eq!(config.theme, "light");
        assert_eq!(config.scenario, "panes");
        assert_eq!(config.timeout_ms, 900);
        assert!(smoke_from_args(args).is_none());
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
}
