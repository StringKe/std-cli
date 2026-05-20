use std_core::{StdConfig, StdCore};
use std_launcher::LauncherState;

#[test]
fn launcher_voice_transcript_triggers_shared_core_action() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut launcher = LauncherState::with_core(core);

    launcher.start_voice_input();
    let execution = launcher
        .trigger_voice_transcript("um please just rebuild index")
        .unwrap();

    assert_eq!(launcher.view.query, "rebuild index");
    assert_eq!(execution.action_name, "Rebuild Index");
    assert!(!launcher.controller.voice_active);
}
