use crate::{bottom_panel_model::BottomPanelTab, StudioEguiApp};
use std_core::{StdConfig, StdCore};
use std_studio::StudioApp;

#[test]
fn bottom_panel_mod_arrow_keys_switch_tabs_only_when_panel_is_open() {
    let ctx = eframe::egui::Context::default();
    let mut app = test_app();
    app.bottom_panel_tab = BottomPanelTab::BatchDebug;

    let _ = ctx.run(mod_key_input(eframe::egui::Key::ArrowRight), |ctx| {
        app.handle_bottom_panel_keyboard(ctx);
    });

    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);

    app.layout.open_bottom_panel();
    let _ = ctx.run(mod_key_input(eframe::egui::Key::ArrowRight), |ctx| {
        app.handle_bottom_panel_keyboard(ctx);
    });

    assert_eq!(app.bottom_panel_tab, BottomPanelTab::Logs);

    let _ = ctx.run(mod_key_input(eframe::egui::Key::ArrowLeft), |ctx| {
        app.handle_bottom_panel_keyboard(ctx);
    });

    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
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
