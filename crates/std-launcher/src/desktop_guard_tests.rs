use super::*;
use std_core::StdConfig;
use std_types::ActionExecutionStatus;

#[test]
fn launcher_cargo_test_enter_cannot_open_registered_app() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("StdNeverLaunchFixture.app");
    std::fs::create_dir_all(app.join("Contents").join("MacOS")).unwrap();
    std::fs::write(app.join("Contents").join("MacOS").join("fixture"), "bin").unwrap();
    let core = StdCore::with_config(config);
    core.register_local_content_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("StdNeverLaunchFixture");
    let execution = state.trigger_selected_by_user().unwrap();

    assert_eq!(execution.action_name, "Open App: StdNeverLaunchFixture");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(execution.message, format!("open {}", app.display()));
}
