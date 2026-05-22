use crate::{
    bottom_panel_model::BottomPanelTab,
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

    assert_eq!(app.app.focused_pane, Some(WorkspacePaneId::new(1)));
    assert_eq!(app.app.open_workspace_panes().count(), 1);
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
fn workspace_close_shortcut_preserves_dashboard_base_pane() {
    let mut app = StudioEguiApp::default();
    let dashboard = app.app.open_workspace_pane(StudioPane::Dashboard);

    let ctx = eframe::egui::Context::default();
    let _ = ctx.run(mod_key_input(eframe::egui::Key::W), |ctx| {
        app.handle_workspace_tab_keyboard(ctx);
    });

    assert_eq!(app.app.focused_pane, Some(dashboard));
    assert_eq!(app.app.open_workspace_panes().count(), 1);
    assert!(app.workspace_commands.lock().unwrap().is_empty());

    let settings = app.app.open_settings_pane();
    let ctx = eframe::egui::Context::default();
    let _ = ctx.run(mod_key_input(eframe::egui::Key::W), |ctx| {
        app.handle_workspace_tab_keyboard(ctx);
    });

    assert_eq!(
        app.workspace_commands.lock().unwrap().as_slice(),
        &[StudioWorkspaceCommand::Close(settings)]
    );
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
        "工作区面板，设置，settings"
    );
}

#[test]
fn workspace_action_labels_include_pane_title_role_and_shortcut() {
    let mut app = StudioEguiApp::default();
    app.app.open_settings_pane();
    let spec = focused_workspace_spec(&app.app).unwrap();

    assert_eq!(
        crate::workspace_panes::workspace_action_a11y_label("Close", &spec, Some("Mod+W")),
        "Close，工作区面板操作，设置，按钮，按 Enter，快捷键 Mod+W"
    );
    assert_eq!(
        crate::workspace_panes::workspace_action_a11y_label("Refresh", &spec, None),
        "Refresh，工作区面板操作，设置，按钮，按 Enter"
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
    app.bottom_panel_tab = BottomPanelTab::Problems;
    app.workspace_commands
        .lock()
        .unwrap()
        .push(StudioWorkspaceCommand::PreviewWorkflow(
            workflow_path.clone(),
        ));

    app.consume_workspace_commands();

    let focused = focused_workspace_spec(&app.app).unwrap();
    assert_eq!(focused.content_key, "workflows");
    assert!(app.app.workflow_debug.is_some());
    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let preview_panel = app.bottom_panel_snapshot();
    assert_eq!(preview_panel.title, "Workspace Preview");
    assert_eq!(preview_panel.rows.len(), 1);
    assert!(app.status.contains("workspace preview"));

    app.bottom_panel_tab = BottomPanelTab::Logs;
    app.workspace_commands
        .lock()
        .unwrap()
        .push(StudioWorkspaceCommand::RunWorkflow(workflow_path));
    app.consume_workspace_commands();

    assert!(app.app.last_workflow_execution.is_some());
    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let run_panel = app.bottom_panel_snapshot();
    assert_eq!(run_panel.title, "Workspace Preview");
    assert_eq!(run_panel.rows.len(), 1);
    assert!(app.status.contains("workspace run"));
}

#[test]
fn workflow_history_action_opens_history_pane_and_bottom_panel() {
    let mut app = StudioEguiApp {
        bottom_panel_tab: BottomPanelTab::Performance,
        ..Default::default()
    };

    app.open_workflow_history();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    assert!(app.status.contains("workflow history opened"));
    let focused = focused_workspace_spec(&app.app).unwrap();
    assert_eq!(focused.content_key, "history");
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

fn mod_key_input(key: eframe::egui::Key) -> eframe::egui::RawInput {
    let mut modifiers = eframe::egui::Modifiers::NONE;
    modifiers.command = true;
    modifiers.mac_cmd = cfg!(target_os = "macos");
    modifiers.ctrl = !cfg!(target_os = "macos");
    eframe::egui::RawInput {
        events: vec![eframe::egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers,
        }],
        modifiers,
        ..Default::default()
    }
}
