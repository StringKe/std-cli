//! std-studio - Professional environment (full product)

mod analysis;
mod app_view;
mod commands;
mod host_chrome;
mod layout;
mod operations;
mod preview;
mod shell;
mod shell_parts;
mod smoke;
mod ui;
mod views;
mod windows;

use eframe::egui;
use layout::StudioLayoutState;
use preview::{run_studio_preview, studio_preview_from_args};
use smoke::smoke_from_args;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std_studio::{StudioApp, StudioPane};
use windows::WorkspaceCommandQueue;

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
    pub(crate) analysis_path: String,
    pub(crate) analysis_query: String,
    pub(crate) analysis_answer: String,
    pub(crate) analysis_search_output: String,
    pub(crate) analysis_coverage_output: String,
    pub(crate) settings_hotkey: String,
    pub(crate) settings_data_dir: String,
    pub(crate) settings_enable_ai: bool,
    pub(crate) settings_theme: String,
    pub(crate) batch_json: String,
    pub(crate) status: String,
    pub(crate) host_maximized: bool,
    pub(crate) layout: StudioLayoutState,
    pub(crate) workspace_commands: WorkspaceCommandQueue,
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
            analysis_path: ".".to_string(),
            analysis_query: "workflow".to_string(),
            analysis_answer: String::new(),
            analysis_search_output: String::new(),
            analysis_coverage_output: String::new(),
            settings_hotkey: String::new(),
            settings_data_dir: String::new(),
            settings_enable_ai: false,
            settings_theme: String::new(),
            batch_json: default_batch_json(),
            status: String::new(),
            host_maximized: false,
            layout: StudioLayoutState::default(),
            workspace_commands: Arc::new(Mutex::new(Vec::new())),
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
        ui::install_visuals(ctx, &self.app.core.config.theme);
        self.layout.handle_keyboard(ctx);
        self.consume_workspace_commands();
        self.render_shell(ctx);
    }
}

impl StudioEguiApp {
    fn render_workspace_pane_manager(&mut self, ui: &mut egui::Ui) {
        let panes = self
            .app
            .workspace_panes
            .iter()
            .map(|pane| (pane.id, pane.title.clone(), pane.open))
            .collect::<Vec<_>>();
        if panes.is_empty() {
            return;
        }

        ui.vertical(|ui| {
            ui::section_header(ui, "Workspace Panes", "open work");
            for (id, title, open) in panes {
                ui.horizontal(|ui| {
                    let label = if open {
                        title.clone()
                    } else {
                        format!("{title} inactive")
                    };
                    if ui
                        .selectable_label(self.app.focused_pane == Some(id), label)
                        .clicked()
                    {
                        self.app.focus_workspace_pane(id);
                    }
                    if ui::quiet_button(ui, "Close Pane").clicked() {
                        self.app.close_workspace_pane(id);
                    }
                });
            }
        });
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
    if let Some(config) = studio_preview_from_args(&args) {
        return run_studio_preview(config);
    }
    if let Some(report) = smoke_from_args(args) {
        println!("{}", report.summary());
        return Ok(());
    }

    eframe::run_native(
        "std-cli Studio",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 820.0])
                .with_min_inner_size([1080.0, 640.0])
                .with_decorations(false),
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::new(StudioEguiApp::default()))),
    )
}

#[cfg(test)]
mod app_tests {
    use super::*;
    use crate::windows::StudioWorkspaceCommand;
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
            .push(StudioWorkspaceCommand::PreviewWorkflow(workflow_path));

        app.consume_workspace_commands();

        assert_eq!(app.app.active_pane, StudioPane::Workflows);
        assert!(app.app.workflow_debug.is_some());
        assert!(app.status.contains("workspace preview"));
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
        assert!(!app.layout.quick_open_open);

        app.layout.open_settings();
        assert!(app.layout.settings_open);
        assert!(!app.layout.command_palette_open);
        assert!(!app.layout.quick_open_open);

        app.layout.close_overlays();
        assert!(!app.layout.settings_open);
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
}
