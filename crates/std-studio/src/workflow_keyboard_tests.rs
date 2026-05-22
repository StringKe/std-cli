use crate::{workspace_panes::focused_workspace_spec, StudioEguiApp};
use std_core::{StdConfig, StdCore};
use std_studio::{StudioApp, StudioPane};

#[test]
fn mod_n_creates_workflow_and_opens_builder_from_raw_input() {
    let ctx = eframe::egui::Context::default();
    let mut app = test_app();
    app.workflow_name = "Keyboard Created".to_string();
    app.workflow_description = "Created from Mod+N".to_string();

    let _ = ctx.run(mod_key_input(eframe::egui::Key::N), |ctx| {
        app.handle_workflow_creation_keyboard(ctx);
    });

    assert_eq!(app.app.active_pane, StudioPane::Workflows);
    assert_eq!(app.app.open_workspace_panes().count(), 1);
    assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
    assert!(app.status.contains("created"));
    let spec = focused_workspace_spec(&app.app).unwrap();
    assert_eq!(spec.content_key, "workflows");
    assert!(spec.workflow_path.is_some());
}

#[test]
fn mod_n_respects_ime_composing_guard() {
    let ctx = eframe::egui::Context::default();
    let mut app = test_app();

    let _ = ctx.run(ime_preedit_input(), |ctx| {
        app.handle_workflow_creation_keyboard(ctx);
    });
    let _ = ctx.run(mod_key_input(eframe::egui::Key::N), |ctx| {
        app.handle_workflow_creation_keyboard(ctx);
    });

    assert_eq!(app.app.open_workspace_panes().count(), 0);
    assert!(app.status.is_empty());
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
    let modifiers = eframe::egui::Modifiers {
        command: true,
        mac_cmd: cfg!(target_os = "macos"),
        ctrl: !cfg!(target_os = "macos"),
        ..Default::default()
    };
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

fn ime_preedit_input() -> eframe::egui::RawInput {
    eframe::egui::RawInput {
        events: vec![eframe::egui::Event::Ime(eframe::egui::ImeEvent::Preedit(
            "gongzuoliu".to_string(),
        ))],
        ..Default::default()
    }
}
