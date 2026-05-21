mod analysis;
mod analysis_rows;
mod analysis_state;
mod analysis_tab_content;
mod app_rows;
mod app_view;
mod bottom_panel;
mod commands;
mod host_chrome;
mod layout;
mod operations;
mod operations_rows;
mod preview;
#[cfg(test)]
mod preview_tests;
mod shell;
mod shell_icons;
mod shell_navigation;
mod shell_overlays;
mod shell_parts;
mod smoke;
mod studio_open;
mod studio_smoke_cli;
mod ui;
mod viewport;
mod views;
mod workspace_panes;
mod workspace_tabs;

use analysis_state::AnalysisUiState;
use layout::StudioLayoutState;
use preview::{
    blocked_studio_preview_summary, run_studio_preview, studio_preview_request_from_args,
    StudioPreviewRequest, StudioPreviewSmokeReport,
};
use smoke::smoke_from_args;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std_egui::tokens::ThemeProfile;
use std_studio::{StudioApp, StudioPane, WorkspacePaneId};
use studio_open::{
    apply_studio_open_request, run_studio_open_request, studio_open_blocked_summary,
    studio_open_request_from_args, StudioOpenRequest,
};
use studio_smoke_cli::{theme_smoke_from_args, workspace_policy_smoke_from_args};
use viewport::studio_native_options;
use workspace_panes::WorkspaceCommandQueue;

pub(crate) struct StudioEguiApp {
    pub(crate) app: StudioApp,
    pub(crate) workflow_name: String,
    pub(crate) workflow_description: String,
    pub(crate) workflow_goal: String,
    pub(crate) workflow_selected_path: Option<PathBuf>,
    pub(crate) workflow_step_name: String,
    pub(crate) workflow_step_parameters: String,
    pub(crate) workflow_edit_index: String,
    pub(crate) plugin_query: String,
    pub(crate) app_query: String,
    pub(crate) app_bundle_path: String,
    pub(crate) memory_query: String,
    pub(crate) memory_scope: String,
    pub(crate) memory_title: String,
    pub(crate) memory_body: String,
    pub(crate) memory_tags: String,
    pub(crate) analysis: AnalysisUiState,
    pub(crate) settings_hotkey: String,
    pub(crate) settings_data_dir: String,
    pub(crate) settings_enable_ai: bool,
    pub(crate) settings_theme: String,
    pub(crate) batch_json: String,
    pub(crate) status: String,
    pub(crate) host_maximized: bool,
    pub(crate) theme_profile: Option<ThemeProfile>,
    pub(crate) layout: StudioLayoutState,
    pub(crate) workspace_commands: WorkspaceCommandQueue,
    pub(crate) pending_workspace_focus: Option<WorkspacePaneId>,
}

impl Default for StudioEguiApp {
    fn default() -> Self {
        Self {
            app: StudioApp::default(),
            workflow_name: "Daily automation".to_string(),
            workflow_description: "Local-first workflow draft".to_string(),
            workflow_goal: "terminal".to_string(),
            workflow_selected_path: None,
            workflow_step_name: "Collect context".to_string(),
            workflow_step_parameters: "{}".to_string(),
            workflow_edit_index: "0".to_string(),
            plugin_query: "plugin".to_string(),
            app_query: String::new(),
            app_bundle_path: String::new(),
            memory_query: String::new(),
            memory_scope: "global".to_string(),
            memory_title: String::new(),
            memory_body: String::new(),
            memory_tags: String::new(),
            analysis: AnalysisUiState::initial(),
            settings_hotkey: String::new(),
            settings_data_dir: String::new(),
            settings_enable_ai: false,
            settings_theme: String::new(),
            batch_json: default_batch_json(),
            status: String::new(),
            host_maximized: false,
            theme_profile: None,
            layout: StudioLayoutState::default(),
            workspace_commands: Arc::new(Mutex::new(Vec::new())),
            pending_workspace_focus: None,
        }
        .with_loaded_settings()
    }
}

impl StudioEguiApp {
    fn with_loaded_settings(mut self) -> Self {
        self.sync_settings_from_app();
        self
    }

    pub(crate) fn sync_settings_from_app(&mut self) {
        self.settings_hotkey = self.app.core.config.launcher_hotkey.clone();
        self.settings_data_dir = self.app.core.config.data_dir.display().to_string();
        self.settings_enable_ai = self.app.core.config.enable_ai;
        self.settings_theme = self.app.core.config.theme.clone();
    }
}

impl eframe::App for StudioEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme_profile = Some(ui::install_visuals(ctx, &self.app.core.config.theme));
        self.layout.handle_keyboard(ctx);
        self.handle_settings_keyboard(ctx);
        self.handle_workspace_tab_keyboard(ctx);
        self.handle_analysis_workbench_keyboard(ctx);
        self.consume_workspace_commands();
        self.render_shell(ctx);
    }
}

impl StudioEguiApp {
    fn handle_settings_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_composing(ctx) {
            return;
        }
        if std_egui::input::studio_settings().pressed(ctx) {
            self.open_settings_workspace_pane();
            self.layout.close_overlays();
        }
    }

    fn handle_workspace_tab_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_composing(ctx) {
            return;
        }
        if std_egui::input::studio_close_tab().pressed(ctx) {
            if let Some(command) =
                crate::workspace_tabs::workspace_tab_keyboard_command(self.app.focused_pane)
            {
                if let Ok(mut queue) = self.workspace_commands.lock() {
                    queue.push(command);
                }
            }
        }
    }
}

fn default_batch_json() -> String {
    r#"{
  "steps": [
    {
      "name": "rebuild",
      "kind": "action",
      "target": "index"
    },
    {
      "name": "terminal",
      "kind": "action",
      "target": "terminal"
    }
  ]
}"#
    .to_string()
}

fn main() -> eframe::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(request) = studio_preview_request_from_args(&args) {
        match request {
            StudioPreviewRequest::Run(config) => return run_studio_preview(config),
            StudioPreviewRequest::Blocked(reason) => {
                println!("{}", blocked_studio_preview_summary(&reason));
                return Ok(());
            }
        }
    }
    if args.get(1).map(String::as_str) == Some("--preview-smoke") {
        println!("{}", StudioPreviewSmokeReport::new().summary());
        return Ok(());
    }
    if let Some(report) = theme_smoke_from_args(&args) {
        println!("{}", report.summary("studio"));
        return Ok(());
    }
    if let Some(report) = workspace_policy_smoke_from_args(&args) {
        println!("{}", report.output());
        return Ok(());
    }
    if let Some(request) = studio_open_request_from_args(&args) {
        if let Some(reason) = native_app_blocked_by_test_mode() {
            println!("{}", studio_open_blocked_summary(request, reason));
            return Ok(());
        }
        return run_studio_open_request(request);
    }
    if let Some(report) = smoke_from_args(args) {
        println!("{}", report.summary());
        return Ok(());
    }
    if let Some(reason) = native_app_blocked_by_test_mode() {
        println!("{reason}");
        return Ok(());
    }

    eframe::run_native(
        "std-cli Studio",
        studio_native_options(),
        Box::new(|_cc| Ok(Box::new(StudioEguiApp::default()))),
    )
}

fn app_for_open_request(request: StudioOpenRequest) -> StudioEguiApp {
    let mut app = StudioEguiApp::default();
    apply_studio_open_request(&mut app, request);
    app
}

fn native_app_blocked_by_test_mode() -> Option<&'static str> {
    std_core::std_test_mode_enabled()
        .then_some("studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup")
}

#[cfg(test)]
mod app_tests {
    use super::*;
    use crate::workspace_panes::{focused_workspace_spec, StudioWorkspaceCommand};
    use std_core::{StdConfig, StdCore};

    #[test]
    fn workspace_commands_are_consumed_by_main_app_state() {
        let mut app = StudioEguiApp::default();
        let id = app.app.open_plugin_manager_pane();
        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::Close(id));

        app.consume_workspace_commands();

        assert_eq!(app.app.focused_pane, None);
        assert_eq!(app.app.open_workspace_panes().count(), 0);
        assert!(app.status.contains("closed workspace pane"));
    }

    #[test]
    fn workspace_focus_command_switches_internal_tab() {
        let mut app = StudioEguiApp::default();
        let plugin = app.app.open_plugin_manager_pane();
        app.app.open_settings_pane();
        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::Focus(plugin));

        app.consume_workspace_commands();

        assert_eq!(app.app.focused_pane, Some(plugin));
        assert_eq!(app.pending_workspace_focus, Some(plugin));
        assert!(app.status.contains("focused workspace pane"));
    }

    #[test]
    fn workspace_focus_cycle_commands_switch_internal_tabs() {
        let mut app = StudioEguiApp::default();
        let dashboard = app.app.open_workspace_pane(StudioPane::Dashboard);
        let plugins = app.app.open_plugin_manager_pane();
        let settings = app.app.open_settings_pane();

        assert_eq!(app.app.focused_pane, Some(settings));
        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::FocusNext);
        app.consume_workspace_commands();
        assert_eq!(app.app.focused_pane, Some(dashboard));
        assert_eq!(app.pending_workspace_focus, Some(dashboard));
        assert!(app.status.contains(&dashboard.value().to_string()));

        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::FocusPrevious);
        app.consume_workspace_commands();
        assert_eq!(app.app.focused_pane, Some(settings));
        assert_eq!(app.pending_workspace_focus, Some(settings));

        assert!(app.app.close_workspace_pane(settings));
        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::FocusPrevious);
        app.consume_workspace_commands();
        assert_eq!(app.app.focused_pane, Some(plugins));
        assert_eq!(app.pending_workspace_focus, Some(plugins));
    }

    #[test]
    fn workspace_focus_ids_are_stable_for_accessibility() {
        assert_eq!(
            crate::workspace_panes::workspace_pane_focus_id(WorkspacePaneId::new(7)),
            crate::workspace_panes::workspace_pane_focus_id(WorkspacePaneId::new(7))
        );
        assert_ne!(
            crate::workspace_panes::workspace_pane_focus_id(WorkspacePaneId::new(7)),
            crate::workspace_panes::workspace_pane_focus_id(WorkspacePaneId::new(8))
        );
    }

    #[test]
    fn workspace_pane_a11y_label_includes_heading_and_kind() {
        let mut app = StudioEguiApp::default();
        let settings = app.app.open_settings_pane();
        let spec = focused_workspace_spec(&app.app).unwrap();

        assert_eq!(spec.id, settings);
        assert_eq!(
            crate::workspace_panes::workspace_pane_a11y_label(&spec),
            "Workspace pane, Settings, settings"
        );
    }

    #[test]
    fn workspace_canvas_renders_only_focused_internal_pane() {
        let mut app = StudioEguiApp::default();
        let plugin = app.app.open_plugin_manager_pane();
        let settings = app.app.open_settings_pane();

        let spec = focused_workspace_spec(&app.app).unwrap();
        assert_eq!(spec.id, settings);
        assert_eq!(spec.content_key, "settings");

        assert!(app.app.focus_workspace_pane(plugin));
        let focused = focused_workspace_spec(&app.app).unwrap();
        assert_eq!(focused.id, plugin);
        assert_eq!(focused.content_key, "plugins");
    }

    #[test]
    fn workspace_commands_drive_main_workspace_and_workflow_preview() {
        let mut app = StudioEguiApp::default();
        let temp = tempfile::tempdir().unwrap();
        app.app = StudioApp::with_core(StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        }));
        let workflow_path = app
            .app
            .create_workflow("Workspace Preview", "Preview from workspace pane")
            .unwrap();
        app.app
            .add_workflow_step(&workflow_path, "Collect", serde_json::json!({}))
            .unwrap();
        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::ShowInMain(StudioPane::Workflows));
        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::PreviewWorkflow(
                workflow_path.clone(),
            ));

        app.consume_workspace_commands();

        assert_eq!(app.app.active_pane, StudioPane::Workflows);
        assert!(app.app.workflow_debug.is_some());
        assert!(app.layout.bottom_panel_open);
        let preview_panel = app.bottom_panel_snapshot();
        assert_eq!(preview_panel.title, "Workspace Preview");
        assert_eq!(preview_panel.rows.len(), 1);
        assert!(app.status.contains("workspace preview"));

        app.workspace_commands
            .lock()
            .unwrap()
            .push(StudioWorkspaceCommand::RunWorkflow(workflow_path));
        app.consume_workspace_commands();

        assert!(app.app.last_workflow_execution.is_some());
        assert!(app.layout.bottom_panel_open);
        let run_panel = app.bottom_panel_snapshot();
        assert_eq!(run_panel.title, "Workspace Preview");
        assert_eq!(run_panel.rows.len(), 1);
        assert!(app.status.contains("workspace run"));
    }

    #[test]
    fn batch_run_opens_bottom_panel_with_report_state() {
        let mut app = StudioEguiApp::default();
        let temp = tempfile::tempdir().unwrap();
        app.app = StudioApp::with_core(StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        }));

        let body = app.batch_json.clone();
        let report = app.app.run_batch_json(&body).unwrap();
        app.layout.open_bottom_panel();
        app.status = format!("batch {:?} steps={}", report.status, report.steps.len());

        assert!(app.app.last_batch_report.is_some());
        assert!(app.layout.bottom_panel_open);
        let panel = app.bottom_panel_snapshot();
        assert_eq!(panel.title, "Batch Debug");
        assert_eq!(panel.rows.len(), 2);
        assert!(panel
            .rows
            .iter()
            .any(|row| row.status == crate::bottom_panel::completed_status()));
        assert!(app.status.contains("batch"));
    }

    #[test]
    fn studio_shell_layout_defaults_to_single_host_workspace() {
        let app = StudioEguiApp::default();

        assert!(app.layout.sidebar_open);
        assert!(!app.layout.inspector_open);
        assert!(!app.layout.bottom_panel_open);
        assert_eq!(app.layout.sidebar_width(), 240.0);
        assert_eq!(app.layout.inspector_width(), 320.0);
        assert_eq!(app.layout.bottom_panel_height(), 240.0);
    }

    #[test]
    fn studio_command_overlays_are_internal_and_exclusive() {
        let mut app = StudioEguiApp::default();

        app.layout.open_quick_open();
        assert!(app.layout.quick_open_open);
        assert!(!app.layout.command_palette_open);

        app.layout.open_command_palette();
        assert!(app.layout.command_palette_open);

        app.layout.close_overlays();
        assert!(!app.layout.command_palette_open);
        assert!(!app.layout.quick_open_open);
    }

    #[test]
    fn studio_overlays_do_not_use_egui_windows() {
        let overlays = include_str!("shell_overlays.rs");

        assert!(!overlays.contains(&["egui::", "Window", "::new"].join("")));
        assert!(!overlays.contains(&["Window", "::new"].join("")));
        assert!(overlays.contains("egui::Area::new"));
        assert!(!overlays.contains(&["studio", "_settings", "_overlay"].join("")));
    }

    #[test]
    fn studio_command_sources_use_real_app_state() {
        let mut app = StudioEguiApp::default();
        let pane = app.app.open_plugin_manager_pane();

        let commands = crate::commands::command_palette_items(&app.app);
        let quick_open = crate::commands::quick_open_items(&app.app);

        assert!(commands.iter().any(|item| item.title == "Show Settings"));
        assert!(commands
            .iter()
            .any(|item| item.title == "Refresh Workspace State"));
        assert!(quick_open.iter().any(|item| item.title == "Plugin Manager"));
        assert_eq!(app.app.focused_pane, Some(pane));
    }

    #[test]
    fn test_mode_blocks_native_studio_startup() {
        assert_eq!(
            native_app_blocked_by_test_mode(),
            Some("studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup")
        );
    }
}
