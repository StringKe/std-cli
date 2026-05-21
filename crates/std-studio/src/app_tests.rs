use crate::{
    views::workflow_builder_ai::WorkflowAiAction,
    workspace_panes::{focused_workspace_spec, StudioWorkspaceCommand},
    StudioEguiApp,
};
use std_core::{StdConfig, StdCore};
use std_studio::{StudioApp, StudioPane, WorkspacePaneId};

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
fn workflow_builder_keyboard_shortcuts_move_selected_loaded_step() {
    let mut app = test_app();
    let path = app
        .app
        .create_workflow("Keyboard Move", "Move selected step")
        .unwrap();
    app.app
        .add_workflow_step(&path, "First", serde_json::json!({"order": 1}))
        .unwrap();
    app.app
        .add_workflow_step(&path, "Second", serde_json::json!({"order": 2}))
        .unwrap();
    app.workflow_selected_path = Some(path);
    app.workflow_edit_index = "0".to_string();

    app.move_workflow_builder_step_by_keyboard(1);

    assert_eq!(app.workflow_edit_index, "1");
    assert!(app.status.contains("moved step"));

    app.move_workflow_builder_step_by_keyboard(-1);

    assert_eq!(app.workflow_edit_index, "0");
    assert!(app.status.contains("moved step"));
}

#[test]
fn workflow_ai_assist_apply_insert_and_replace_edit_planned_steps() {
    let mut app = test_app();
    app.workflow_goal = "release".to_string();
    app.apply_workflow_ai_action(WorkflowAiAction::Apply(0));

    let workflow = app.app.planned_workflow.as_ref().unwrap();
    assert_eq!(workflow.steps.len(), 1);
    assert_eq!(workflow.steps[0].name, "Collect context");
    assert_eq!(workflow.steps[0].parameters["source"], "release");
    assert!(app.status.contains("AI applied step"));

    app.workflow_edit_index = "0".to_string();
    app.apply_workflow_ai_action(WorkflowAiAction::Insert(1));

    let workflow = app.app.planned_workflow.as_ref().unwrap();
    assert_eq!(workflow.steps.len(), 2);
    assert_eq!(workflow.steps[0].name, "Validate result");
    assert_eq!(workflow.steps[1].name, "Collect context");
    assert!(app.status.contains("AI inserted step"));

    app.workflow_edit_index = "1".to_string();
    app.apply_workflow_ai_action(WorkflowAiAction::Replace(2));

    let workflow = app.app.planned_workflow.as_ref().unwrap();
    assert_eq!(workflow.steps.len(), 2);
    assert_eq!(workflow.steps[1].name, "Record trace");
    assert_eq!(workflow.steps[1].parameters["target"], "execution-history");
    assert!(app.status.contains("AI replaced step"));
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
    let mut app = test_app();
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
fn workflow_history_action_opens_history_pane_and_bottom_panel() {
    let mut app = StudioEguiApp::default();

    app.open_workflow_history();

    assert!(app.layout.bottom_panel_open);
    assert!(app.status.contains("workflow history opened"));
    let focused = focused_workspace_spec(&app.app).unwrap();
    assert_eq!(focused.content_key, "history");
}

#[test]
fn batch_run_opens_bottom_panel_with_report_state() {
    let mut app = test_app();

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
fn settings_hotkeys_render_registry_source_and_reset() {
    let settings = include_str!("views/settings.rs");
    let rows = include_str!("views/settings_rows.rs");
    let registry = std_core::shortcuts::shortcut_registry(&StdConfig {
        launcher_hotkey: "Cmd+Space".to_string(),
        ..StdConfig::default()
    });
    let launcher = registry
        .iter()
        .find(|shortcut| shortcut.id == "launcher.global.toggle")
        .unwrap();

    assert_eq!(launcher.source.label(), "user");
    assert!(launcher.resettable);
    assert!(settings.contains("std_core::shortcuts::shortcut_registry"));
    assert!(settings.contains("ShortcutRowEvent::Reset(\"launcher.global.toggle\")"));
    assert!(rows.contains("shortcut.source.label()"));
    assert!(rows.contains("shortcut.default_binding"));
    assert!(rows.contains("shortcut_a11y_label"));
}

#[test]
fn settings_theme_control_uses_tokens_and_saves_theme_modes() {
    let settings = include_str!("views/settings.rs");
    let rows = include_str!("views/settings_rows.rs");

    assert!(settings.contains("settings_rows::theme_mode_control"));
    assert!(settings.contains("self.save_setting(\"theme\", self.settings_theme.clone())"));
    assert!(settings.contains("self.render_theme_profile(ui)"));
    assert!(!settings.contains("ui.text_edit_singleline(&mut self.settings_theme)"));
    assert!(rows.contains("[\"system\", \"dark\", \"light\"]"));
    assert!(rows.contains("Color::accent_weak"));
    assert!(rows.contains("Color::accent_base"));
}

#[test]
fn settings_ai_provider_uses_token_toggle_row() {
    let settings = include_str!("views/settings.rs");
    let toggle = include_str!("views/settings_toggle.rs");

    assert!(settings.contains("settings_toggle::toggle_row"));
    assert!(settings.contains("self.save_setting(\"enable_ai\""));
    assert!(!settings.contains("ui.checkbox("));
    assert!(toggle.contains("paint_toggle"));
    assert!(toggle.contains("ToggleRowEvent::Toggle"));
    assert!(toggle.contains("WidgetType::Checkbox"));
}

fn test_app() -> StudioEguiApp {
    let mut app = StudioEguiApp::default();
    let temp = tempfile::tempdir().unwrap();
    app.app = StudioApp::with_core(StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    }));
    app
}
