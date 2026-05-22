use crate::StudioEguiApp;

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

    assert!(commands.iter().any(|item| item.title == "显示 设置"));
    assert!(commands.iter().any(|item| item.title == "刷新工作区状态"));
    assert!(quick_open.iter().any(|item| item.title == "插件管理"));
    assert_eq!(app.app.focused_pane, Some(pane));
}
