use super::*;
use std_core::StdConfig;

#[test]
fn launcher_exists() {
    assert_eq!(super::launcher_version(), "0.1.0");
}

#[test]
fn launcher_hotkey_parses_config_string() {
    let hotkey = LauncherHotkey::parse("Cmd+Shift+K").unwrap();

    assert_eq!(hotkey.modifiers, vec!["Command", "Shift"]);
    assert_eq!(hotkey.key, "K");
    assert_eq!(hotkey.display(), "Command+Shift+K");
    assert!(LauncherHotkey::parse("Bad+K").is_none());
}

#[test]
fn launcher_controller_toggles_visibility() {
    let config = StdConfig {
        launcher_hotkey: "Ctrl+Space".to_string(),
        ..StdConfig::default()
    };
    let mut controller = LauncherController::new(&config);

    assert!(!controller.visible);
    assert!(!controller.focused);

    controller.toggle();
    assert!(controller.visible);
    assert!(controller.focused);
    assert_eq!(
        LauncherController::window_commands(false, controller.visible),
        vec![
            LauncherWindowCommand::ResizeToPanel,
            LauncherWindowCommand::PositionForPanel,
            LauncherWindowCommand::SetVisible(true),
            LauncherWindowCommand::Focus
        ]
    );

    controller.hide();
    assert!(!controller.visible);
    assert!(!controller.focused);
    assert_eq!(controller.hotkey.display(), "Control+Space");
    assert_eq!(
        LauncherController::window_commands(true, controller.visible),
        vec![
            LauncherWindowCommand::ResizeToHiddenHost,
            LauncherWindowCommand::SetVisible(false)
        ]
    );

    controller.start_voice_input();
    assert!(controller.voice_active);
    controller.hide();
    assert!(!controller.voice_active);
}

#[test]
fn launcher_controller_produces_hotkey_registration_plan() {
    let config = StdConfig {
        launcher_hotkey: "Alt+Space".to_string(),
        ..StdConfig::default()
    };
    let controller = LauncherController::new(&config);
    let plan = controller.registration_plan();
    let runtime = GlobalHotkeyRuntime::disabled(plan.clone());

    assert_eq!(plan.accelerator, "Alt+Space");
    assert!(plan.enabled);
    assert!(!runtime.is_registered());
}

#[test]
fn hotkey_smoke_is_skipped_in_test_mode() {
    let report = hotkey_smoke("Alt+Space");

    assert_eq!(report.status, "SKIP");
    assert!(!report.registered);
    assert!(report
        .error
        .as_deref()
        .unwrap()
        .contains("STD_TEST_MODE blocked global hotkey registration"));
    assert!(report.summary().contains("launcher_hotkey_smoke SKIP"));
}

#[test]
fn hotkey_runtime_register_is_blocked_in_tests() {
    let result = GlobalHotkeyRuntime::register(HotkeyRegistrationPlan {
        accelerator: "Alt+Space".to_string(),
        enabled: true,
    });

    let Err(error) = result else {
        panic!("test mode must block global hotkey registration");
    };
    assert!(error.contains("STD_TEST_MODE blocked global hotkey registration"));
}

#[test]
fn hotkey_runtime_matches_registered_event_id() {
    let plan = HotkeyRegistrationPlan {
        accelerator: "Alt+Space".to_string(),
        enabled: true,
    };
    let mut runtime = GlobalHotkeyRuntime::disabled(plan);
    runtime.set_hotkey_id_for_test(42);
    let pressed = global_hotkey::GlobalHotKeyEvent {
        id: 42,
        state: global_hotkey::HotKeyState::Pressed,
    };
    let released = global_hotkey::GlobalHotKeyEvent {
        id: 42,
        state: global_hotkey::HotKeyState::Released,
    };

    assert!(runtime.should_toggle_for_event(pressed));
    assert!(!runtime.should_toggle_for_event(released));
}

#[test]
fn launcher_window_smoke_validates_hotkey_window_commands() {
    let report = LauncherState::window_smoke();
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_eq!(
        report.hidden_commands,
        vec![
            LauncherWindowCommand::ResizeToHiddenHost,
            LauncherWindowCommand::SetVisible(false)
        ]
    );
    assert_eq!(
        report.shown_commands,
        vec![
            LauncherWindowCommand::ResizeToPanel,
            LauncherWindowCommand::PositionForPanel,
            LauncherWindowCommand::SetVisible(true),
            LauncherWindowCommand::Focus
        ]
    );
    assert!(summary.contains("launcher_window_smoke PASS"));
    assert!(summary.contains("hidden_commands=ResizeToHiddenHost,Visible(false)"));
    assert!(summary.contains("shown_commands=ResizeToPanel,PositionForPanel,Visible(true),Focus"));
    assert!(summary.contains(
        "host_positioning=show:resize-to-panel>outer-position-0.28-monitor-anchor>visible>focus"
    ));
    assert!(summary.contains("hide:resize-to-1x1>hidden"));
    assert!(summary.contains("native_host=panel-sized-transparent"));
    assert!(summary.contains("panel_surface=opaque-bg-surface-0"));
    assert!(summary.contains("host_background=none"));
    assert!(summary.contains("host_gutter=0px"));
}

#[test]
fn launcher_cleans_voice_transcript_for_query() {
    let cleaned = clean_voice_transcript("um please just open terminal");

    assert_eq!(cleaned, "open terminal");
}

#[test]
fn launcher_state_uses_voice_transcript_to_preview_and_trigger() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.start_voice_input();
    let preview = state
        .apply_voice_transcript("um please just rebuild index")
        .unwrap();
    let execution = state.trigger_selected().unwrap();

    assert!(!state.controller.voice_active);
    assert_eq!(state.view.query, "rebuild index");
    assert_eq!(preview.title, "Rebuild Index");
    assert_eq!(execution.action_name, "Rebuild Index");
}
