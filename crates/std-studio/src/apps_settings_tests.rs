use super::*;

fn test_studio() -> StudioApp {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    StudioApp::with_core(core)
}

#[test]
fn studio_manages_registered_apps() {
    let mut studio = test_studio();
    let source_app = studio
        .core
        .config
        .data_dir
        .join("source")
        .join("Workbench.app");
    std::fs::create_dir_all(source_app.join("Contents").join("MacOS")).unwrap();
    std::fs::write(
        source_app.join("Contents").join("MacOS").join("workbench"),
        "bin",
    )
    .unwrap();

    let registered = studio.register_app_bundle(&source_app).unwrap();
    let apps = studio.registered_apps().unwrap();
    let results = studio.search_apps("Workbench", 10).unwrap();
    let preview = studio.preview_app("Workbench").unwrap().unwrap();
    let execution = studio.trigger_app("Workbench").unwrap().unwrap();

    assert!(registered.ends_with("Applications/Workbench.app"));
    assert_eq!(apps, vec![registered]);
    assert_eq!(results[0].action.name, "Open App: Workbench");
    assert!(preview.primary_command.contains("Workbench.app"));
    assert_eq!(
        execution.status,
        std_types::ActionExecutionStatus::NeedsExternalRunner
    );
}

#[test]
fn studio_saves_settings_through_shared_config_model() {
    let mut studio = test_studio();
    let config_path = studio
        .core
        .config
        .data_dir
        .join("settings")
        .join("std-cli.json");

    let written = studio
        .save_config_field_to(&config_path, "launcher_hotkey", "Cmd+Space")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "enable_ai", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "theme", "dark")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.reduce_motion", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.high_contrast", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.reduce_transparency", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.ui_scale", "1.25")
        .unwrap();

    let saved: StdConfig =
        serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();

    assert_eq!(written, config_path);
    assert_eq!(
        studio.config_value("launcher_hotkey").as_deref(),
        Some("Cmd+Space")
    );
    assert!(studio.core.config.enable_ai);
    assert_eq!(studio.core.config.theme, "dark");
    assert!(studio.core.config.reduce_motion());
    assert!(studio.core.config.high_contrast());
    assert!(studio.core.config.reduce_transparency());
    assert_eq!(studio.core.config.ui_scale(), 1.25);
    assert_eq!(saved.launcher_hotkey, "Cmd+Space");
    assert!(saved.enable_ai);
    assert_eq!(saved.theme, "dark");
    assert!(saved.reduce_motion());
    assert!(saved.high_contrast());
    assert!(saved.reduce_transparency());
    assert_eq!(saved.ui_scale(), 1.25);
    assert!(studio.dashboard.action_count >= 3);
    assert!(studio
        .save_config_field_to(&config_path, "missing", "x")
        .is_err());
}
