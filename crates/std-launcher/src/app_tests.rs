use crate::{LauncherKey, LauncherState};
use std::{
    os::unix::process::ExitStatusExt,
    process::ExitStatus,
    sync::{Arc, Mutex},
};
use std_core::{StdConfig, StdCore};
use std_types::ActionExecutionStatus;

#[test]
fn launcher_state_searches_local_app_bundles_without_launching() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("Weixin.app");
    write_wechat_app_bundle(&app);
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);

    let preview = state.update_query("微信").unwrap();
    state.update_query("weixin").unwrap();
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

    assert_eq!(preview.title, "Open App: WeChat");
    assert_eq!(execution.action_name, "Open App: WeChat");
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
fn launcher_gui_enter_allows_external_runner_without_affecting_safe_default() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("Weixin.app");
    write_wechat_app_bundle(&app);
    let commands = Arc::new(Mutex::new(Vec::<(String, Vec<String>)>::new()));
    let recorded_commands = Arc::clone(&commands);
    let core = StdCore::with_config_and_command_runner(config, move |program, args| {
        recorded_commands
            .lock()
            .unwrap()
            .push((program.to_string(), args.to_vec()));
        Ok(std::process::Output {
            status: ExitStatus::from_raw(0),
            stdout: b"opened".to_vec(),
            stderr: Vec::new(),
        })
    });
    let mut state = LauncherState::with_core(core);

    state.update_query("微信");
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
    assert_eq!(gui_execution.status, ActionExecutionStatus::Completed);
    assert_eq!(gui_execution.action_name, "Open App: WeChat");
    assert_eq!(
        commands.lock().unwrap().as_slice(),
        [("open".to_string(), vec![app.to_string_lossy().to_string()])]
    );
}

fn write_wechat_app_bundle(app: &std::path::Path) {
    std::fs::create_dir_all(app.join("Contents").join("Resources").join("zh_CN.lproj")).unwrap();
    std::fs::write(
        app.join("Contents").join("Info.plist"),
        r#"<plist><dict>
<key>CFBundleDisplayName</key><string>WeChat</string>
<key>CFBundleName</key><string>Weixin</string>
</dict></plist>"#,
    )
    .unwrap();
    std::fs::write(
        app.join("Contents")
            .join("Resources")
            .join("zh_CN.lproj")
            .join("InfoPlist.strings"),
        r#""CFBundleDisplayName" = "微信";"#,
    )
    .unwrap();
}
