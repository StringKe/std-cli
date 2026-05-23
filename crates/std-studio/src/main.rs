mod analysis;
mod analysis_format;
mod analysis_query_panel;
mod analysis_rows;
mod analysis_state;
mod analysis_tab_content;
mod app_rows;
#[cfg(test)]
mod app_shell_tests;
#[cfg(test)]
mod app_tests;
mod app_view;
#[cfg(test)]
mod app_workspace_tests;
mod bottom_panel;
#[cfg(test)]
mod bottom_panel_keyboard_tests;
mod bottom_panel_model;
mod commands;
mod context_help;
mod host_chrome;
mod host_chrome_drag;
mod host_window;
mod layout;
mod native_app;
mod operations;
mod operations_rows;
mod preview;
mod preview_evidence;
mod preview_smoke;
#[cfg(test)]
mod preview_tests;
mod screenshot_acceptance;
#[cfg(test)]
mod settings_tests;
mod shell;
mod shell_icons;
mod shell_nav_model;
mod shell_navigation;
mod shell_overlays;
mod shell_parts;
mod smoke;
#[cfg(test)]
mod smoke_tests;
mod status_bar;
mod studio_metrics;
mod studio_open;
#[cfg(test)]
mod studio_open_tests;
mod studio_smoke_cli;
mod ui;
mod viewport;
mod views;
#[cfg(test)]
mod workflow_keyboard_tests;
mod workspace_context;
mod workspace_lifecycle;
mod workspace_pane_content;
mod workspace_panes;
mod workspace_policy_evidence;
mod workspace_tabs;
mod zoom;

use analysis_state::AnalysisUiState;
use bottom_panel_model::BottomPanelTab;
use layout::StudioLayoutState;
use native_app::{native_app_blocked_by_test_mode, run_studio_native_app};
use preview::{
    blocked_studio_preview_summary, run_studio_preview, studio_preview_request_from_args,
    StudioPreviewRequest,
};
use preview_smoke::StudioPreviewSmokeReport;
use smoke::smoke_from_args;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std_egui::tokens::ThemeProfile;
use std_studio::{StudioApp, StudioPane, WorkspacePaneCloseGuard, WorkspacePaneId};
use studio_open::{
    run_studio_open_request, studio_open_blocked_summary, studio_open_request_from_args,
    studio_open_smoke_from_args,
};
use studio_smoke_cli::{
    surface_smoke_from_args, theme_smoke_from_args, workspace_policy_smoke_from_args,
};
use workspace_panes::{StudioWorkspaceCommand, WorkspaceCommandQueue};
use zoom::StudioZoomAction;

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
    pub(crate) history_filter: String,
    pub(crate) analysis: AnalysisUiState,
    pub(crate) settings_hotkey: String,
    pub(crate) settings_data_dir: String,
    pub(crate) settings_enable_ai: bool,
    pub(crate) settings_reduce_motion: bool,
    pub(crate) settings_high_contrast: bool,
    pub(crate) settings_reduce_transparency: bool,
    pub(crate) settings_ui_scale: String,
    pub(crate) settings_theme: String,
    pub(crate) settings_category: crate::views::settings_model::SettingsCategory,
    pub(crate) batch_json: String,
    pub(crate) status: String,
    pub(crate) host_maximized: bool,
    pub(crate) theme_profile: Option<ThemeProfile>,
    pub(crate) layout: StudioLayoutState,
    pub(crate) bottom_panel_tab: BottomPanelTab,
    pub(crate) workspace_commands: WorkspaceCommandQueue,
    pub(crate) pending_workspace_focus: Option<WorkspacePaneId>,
    pub(crate) pending_closeguard: Option<WorkspacePaneCloseGuard>,
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
            history_filter: String::new(),
            analysis: AnalysisUiState::initial(),
            settings_hotkey: String::new(),
            settings_data_dir: String::new(),
            settings_enable_ai: false,
            settings_reduce_motion: false,
            settings_high_contrast: false,
            settings_reduce_transparency: false,
            settings_ui_scale: String::new(),
            settings_theme: String::new(),
            settings_category: crate::views::settings_model::SettingsCategory::Appearance,
            batch_json: default_batch_json(),
            status: String::new(),
            host_maximized: false,
            theme_profile: None,
            layout: StudioLayoutState::default(),
            bottom_panel_tab: BottomPanelTab::BatchDebug,
            workspace_commands: Arc::new(Mutex::new(Vec::new())),
            pending_workspace_focus: None,
            pending_closeguard: None,
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
        self.settings_reduce_motion = self.app.core.config.reduce_motion();
        self.settings_high_contrast = self.app.core.config.high_contrast();
        self.settings_reduce_transparency = self.app.core.config.reduce_transparency();
        self.settings_ui_scale = format!("{:.2}", self.app.core.config.ui_scale());
        self.settings_theme = self.app.core.config.theme.clone();
    }
}

impl eframe::App for StudioEguiApp {
    fn clear_color(&self, visuals: &egui::Visuals) -> [f32; 4] {
        studio_clear_color(visuals)
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme_profile = Some(ui::install_visuals(
            ctx,
            &self.app.core.config.theme,
            self.app.core.config.reduce_motion(),
            self.app.core.config.high_contrast(),
            self.app.core.config.reduce_transparency(),
            self.app.core.config.ui_scale(),
        ));
        self.layout.handle_keyboard(ctx);
        self.handle_settings_keyboard(ctx);
        self.handle_workflow_creation_keyboard(ctx);
        self.handle_zoom_keyboard(ctx);
        self.handle_workspace_tab_keyboard(ctx);
        self.handle_bottom_panel_keyboard(ctx);
        self.handle_workflow_builder_keyboard(ctx);
        self.handle_analysis_workbench_keyboard(ctx);
        self.consume_workspace_commands();
        self.render_shell(ctx);
    }
}

pub(crate) fn studio_clear_color(visuals: &egui::Visuals) -> [f32; 4] {
    visuals.panel_fill.to_normalized_gamma_f32()
}

pub(crate) fn studio_clear_color_contract() -> &'static str {
    "native_clear_color=bg/surface-0,not-transparent,not-system-black-white"
}

impl StudioEguiApp {
    fn handle_settings_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        if std_egui::input::studio_settings().pressed(ctx) {
            self.open_settings_workspace_pane();
            self.layout.close_overlays();
        }
    }

    fn handle_workflow_creation_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        if std_egui::input::studio_new_workflow().pressed(ctx) {
            self.create_workflow_from_form();
            self.layout.close_overlays();
        }
    }

    fn handle_zoom_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        let action = if std_egui::input::studio_zoom_in().pressed(ctx) {
            StudioZoomAction::In
        } else if std_egui::input::studio_zoom_out().pressed(ctx) {
            StudioZoomAction::Out
        } else if std_egui::input::studio_zoom_reset().pressed(ctx) {
            StudioZoomAction::Reset
        } else {
            return;
        };
        self.apply_zoom_shortcut(action);
    }

    fn handle_workspace_tab_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        if std_egui::input::studio_previous_workspace_pane().pressed(ctx) {
            self.queue_workspace_tab_command(StudioWorkspaceCommand::FocusPrevious);
        }
        if std_egui::input::studio_next_workspace_pane().pressed(ctx) {
            self.queue_workspace_tab_command(StudioWorkspaceCommand::FocusNext);
        }
        if std_egui::input::studio_close_tab().pressed(ctx) {
            let tabs = crate::workspace_tabs::workspace_tab_specs(
                &self.app.workspace_panes,
                self.app.focused_pane,
            );
            if let Some(command) = tabs
                .iter()
                .find(|spec| spec.focused)
                .and_then(crate::workspace_tabs::workspace_tab_close_keyboard_command)
            {
                self.queue_workspace_tab_command(command);
            }
        }
    }

    fn handle_bottom_panel_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        if !self.layout.bottom_panel_open {
            return;
        }
        let delta = if std_egui::input::studio_previous_bottom_panel_tab().pressed(ctx) {
            -1
        } else if std_egui::input::studio_next_bottom_panel_tab().pressed(ctx) {
            1
        } else {
            return;
        };
        let mut model =
            crate::bottom_panel_model::BottomPanelTabModel::for_selected(self.bottom_panel_tab);
        model.move_selection(delta);
        self.bottom_panel_tab = model.selected;
    }

    fn queue_workspace_tab_command(&self, command: StudioWorkspaceCommand) {
        if let Ok(mut queue) = self.workspace_commands.lock() {
            queue.push(command);
        }
    }

    fn handle_workflow_builder_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::ime_action_guard(ctx).blocks_actions() {
            return;
        }
        if !self.focused_workspace_is_workflow_builder() {
            return;
        }
        if std_egui::input::arrow_up().pressed(ctx) {
            self.select_workflow_builder_step_by_keyboard(-1);
        }
        if std_egui::input::arrow_down().pressed(ctx) {
            self.select_workflow_builder_step_by_keyboard(1);
        }
        if std_egui::input::studio_workflow_step_move_up().pressed(ctx) {
            self.move_workflow_builder_step_by_keyboard(-1);
        }
        if std_egui::input::studio_workflow_step_move_down().pressed(ctx) {
            self.move_workflow_builder_step_by_keyboard(1);
        }
    }

    fn focused_workspace_is_workflow_builder(&self) -> bool {
        self.app
            .focused_pane
            .and_then(|id| self.app.workspace_panes.iter().find(|pane| pane.id == id))
            .map(|pane| {
                matches!(
                    pane.kind,
                    std_studio::WorkspacePaneKind::WorkflowBuilder { .. }
                )
            })
            .unwrap_or(false)
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
    std_core::sanitize_desktop_opt_ins_for_test_mode();
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
    if let Some(report) = surface_smoke_from_args(&args) {
        println!("{}", report.output());
        return Ok(());
    }
    if let Some(report) = studio_open_smoke_from_args(&args) {
        println!("{}", report.summary());
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

    run_studio_native_app()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_egui::tokens::{apply_theme, ThemeMode};

    #[test]
    fn studio_clear_color_uses_theme_surface_not_native_black_or_white() {
        let ctx = egui::Context::default();
        apply_theme(&ctx, ThemeMode::Dark);
        let dark = ctx.style().visuals.clone();
        apply_theme(&ctx, ThemeMode::Light);
        let light = ctx.style().visuals.clone();

        assert_eq!(
            studio_clear_color_contract(),
            "native_clear_color=bg/surface-0,not-transparent,not-system-black-white"
        );
        assert_eq!(
            studio_clear_color(&dark),
            [0.10980392, 0.11764706, 0.13333334, 1.0]
        );
        assert_eq!(
            studio_clear_color(&light),
            [0.98039216, 0.9843137, 0.99215686, 1.0]
        );
    }

    #[test]
    fn studio_main_routes_open_intent_before_native_startup() {
        let source = include_str!("main.rs");
        let main_start = source.find("fn main()").unwrap();
        let tests_start = source.rfind("#[cfg(test)]").unwrap();
        let main_body = &source[main_start..tests_start];
        let open_dispatch = main_body
            .find("studio_open_request_from_args(&args)")
            .unwrap();
        let smoke_dispatch = main_body.find("smoke_from_args(args)").unwrap();
        let native_block = main_body
            .rfind("native_app_blocked_by_test_mode()")
            .unwrap();
        let native_start = main_body.rfind("run_studio_native_app()").unwrap();

        assert!(open_dispatch < smoke_dispatch);
        assert!(open_dispatch < native_start);
        assert!(smoke_dispatch < native_block);
        assert!(native_block < native_start);
        assert!(source.contains("return run_studio_open_request(request);"));
        assert_eq!(
            crate::studio_open::studio_open_runtime_boundary(),
            "open-intent-before-native-startup;no-run-native;no-extra-viewport"
        );
    }
}
