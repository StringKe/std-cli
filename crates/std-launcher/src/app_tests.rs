use crate::{LauncherKey, LauncherState};
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
fn launcher_gui_enter_defers_external_runner_in_tests() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("Weixin.app");
    write_wechat_app_bundle(&app);
    let core = StdCore::with_config(config);
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
    assert_eq!(
        gui_execution.status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(gui_execution.action_name, "Open App: WeChat");
    assert_eq!(
        gui_execution
            .output
            .as_ref()
            .unwrap()
            .get("deferred")
            .and_then(|value| value.as_bool()),
        Some(true)
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
