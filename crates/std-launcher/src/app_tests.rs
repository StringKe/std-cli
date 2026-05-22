use crate::{LauncherFocusSection, LauncherKey, LauncherState};
use std::sync::{Arc, Mutex};
use std_core::{StdConfig, StdCore};
use std_types::{Action, ActionExecutionStatus, ActionType, RegistryEntry};

#[test]
fn launcher_state_searches_local_app_bundles_without_launching() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureTalk.app");
    write_multilingual_app_bundle(&app);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);
    let localized_name = localized_fixture_name();

    let preview = state.update_query(&localized_name).unwrap();
    state.update_query("fixturetalk").unwrap();
    assert!(state
        .view
        .results
        .iter()
        .any(|result| result.action.id == preview.action_id));
    state.view.selected = state
        .view
        .results
        .iter()
        .position(|result| result.action.id == preview.action_id)
        .unwrap();
    let execution = state.trigger_selected().unwrap();

    assert_eq!(preview.title, "Open App: Fixture Talk");
    assert_eq!(execution.action_name, "Open App: Fixture Talk");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        execution
            .output
            .as_ref()
            .unwrap()
            .get("deferred")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
}

#[test]
fn launcher_searches_one_app_by_multilingual_aliases_without_launching() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureTalk.app");
    write_multilingual_app_bundle(&app);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);
    let queries = [
        "Fixture Talk".to_string(),
        "fixturetalk".to_string(),
        "fixture-chat".to_string(),
        localized_fixture_name(),
    ];

    let mut action_ids = Vec::new();
    for query in queries {
        let preview = state.update_query(&query).unwrap();
        assert_eq!(preview.title, "Open App: Fixture Talk");
        action_ids.push(preview.action_id);
    }
    let execution = state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .unwrap();

    assert!(action_ids.windows(2).all(|pair| pair[0] == pair[1]));
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.action_name, "Open App: Fixture Talk");
}

#[test]
fn launcher_searches_wechat_by_macos_multilingual_names_without_launching() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("WeChat.app");
    write_wechat_app_bundle(&app);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);
    let queries = ["wechat", "weixin", wechat_chinese_name()];

    let mut action_ids = Vec::new();
    for query in queries {
        let preview = state.update_query(query).unwrap();
        assert_eq!(preview.title, "Open App: WeChat");
        assert_eq!(
            state.view.selected_result().unwrap().action.name,
            "Open App: WeChat"
        );
        action_ids.push(preview.action_id);
    }
    let execution = state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .unwrap();

    assert!(action_ids.windows(2).all(|pair| pair[0] == pair[1]));
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.action_name, "Open App: WeChat");
}

#[test]
fn launcher_gui_enter_reports_test_mode_desktop_open_block_in_tests() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureTalk.app");
    write_multilingual_app_bundle(&app);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);

    state.update_query(localized_fixture_name());
    let safe_execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();
    let ime_execution = state.handle_keyboard_input_by_user(LauncherKey::Enter, true);
    let gui_execution = state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(
        safe_execution.status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert!(ime_execution.is_none());
    assert_eq!(
        gui_execution.status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(gui_execution.action_name, "Open App: Fixture Talk");
    assert_eq!(
        gui_execution
            .output
            .as_ref()
            .unwrap()
            .get("deferred")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
    assert_eq!(
        gui_execution
            .output
            .as_ref()
            .unwrap()
            .get("reason")
            .and_then(|value| value.as_str()),
        Some("STD_TEST_MODE blocked desktop open")
    );
}

#[test]
fn launcher_main_enter_defers_app_launch_without_user_external_opt_in() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("FixtureTalk.app");
    write_multilingual_app_bundle(&app);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);

    state.update_query(localized_fixture_name());
    let execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.action_name, "Open App: Fixture Talk");
    assert_eq!(
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.deferred),
        Some(true)
    );
    assert_eq!(
        execution
            .output
            .as_ref()
            .unwrap()
            .get("reason")
            .and_then(|value| value.as_str()),
        Some("external runner action requires explicit user trigger")
    );
}

#[test]
fn launcher_user_enter_keeps_runner_blocked_in_tests() {
    let temp = tempfile::tempdir().unwrap();
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded_commands = Arc::clone(&commands);
    let core = StdCore::with_config_and_command_runner(
        StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        },
        move |program, args| {
            recorded_commands
                .lock()
                .unwrap()
                .push((program.to_string(), args.to_vec()));
            Err(std::io::Error::other("runner should stay blocked in tests"))
        },
    );
    let mut action = std_types::Action::new(
        "User Runner Fixture",
        "Records runner path",
        "When verifying Launcher Enter",
        std_types::ActionType::Command,
    );
    action.created_at = chrono::Utc::now();
    action.updated_at = chrono::Utc::now();
    core.register_action(
        std_types::RegistryEntry::from_action(action, vec!["runner".to_string()])
            .with_metadata("command", "printf user-runner-fixture"),
    )
    .unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("user runner fixture");
    let safe_execution = state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .unwrap();
    let user_execution = state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(
        safe_execution.status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(
        user_execution.status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert!(commands.lock().unwrap().is_empty());
}

#[test]
fn launcher_feedback_retry_uses_user_enter_path_without_launching_in_tests() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.register_action(fixture_app_action(temp.path()))
        .unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("StdNeverLaunchFixture");
    let first = state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .unwrap();
    state.focus_section = LauncherFocusSection::Feedback;
    state.view.selected_feedback_action = 1;
    let retry = state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .unwrap();

    assert_eq!(first.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(retry.action_name, "Open App: StdNeverLaunchFixture");
    assert_eq!(retry.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        retry
            .output
            .as_ref()
            .and_then(|output| output.get("deferred"))
            .and_then(|value| value.as_bool()),
        Some(true)
    );
}

fn write_multilingual_app_bundle(app: &std::path::Path) {
    std::fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    std::fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>Fixture Talk</string>
<key>CFBundleName</key><string>FixtureTalk</string>
<key>CFBundleURLTypes</key><array><dict><key>CFBundleURLSchemes</key><array>
<string>fixture-chat</string>
</array></dict></array>
</dict></plist>"#,
    )
    .unwrap();
    let body = format!(
        "\"CFBundleDisplayName\" = \"{}\";",
        localized_fixture_name()
    );
    std::fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        body,
    )
    .unwrap();
}

fn fixture_app_action(root: &std::path::Path) -> RegistryEntry {
    let app = root.join("StdNeverLaunchFixture.app");
    RegistryEntry::from_action(
        Action::new(
            "Open App: StdNeverLaunchFixture",
            format!("Launch fixture app at {}", app.display()),
            "When testing Launcher user retry",
            ActionType::AppLaunch,
        ),
        vec!["app".to_string(), "fixture".to_string()],
    )
    .with_metadata("path", app.display().to_string())
}

fn write_wechat_app_bundle(app: &std::path::Path) {
    std::fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    std::fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>WeChat</string>
<key>CFBundleName</key><string>Weixin</string>
<key>CFBundleIdentifier</key><string>com.tencent.xinWeChat</string>
</dict></plist>"#,
    )
    .unwrap();
    std::fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        "\"CFBundleDisplayName\" = \"\\U5fae\\U4fe1\";",
    )
    .unwrap();
}

fn localized_fixture_name() -> String {
    String::from("\u{6d4b}\u{8bd5}\u{5e94}\u{7528}")
}

fn wechat_chinese_name() -> &'static str {
    "\u{5fae}\u{4fe1}"
}
