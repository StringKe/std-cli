use std::{fs, path::Path, process::Command};

#[test]
fn std_binary_test_mode_blocks_child_process_external_runner() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());

    let define = run_std_in_desktop_safe_test_mode(
        &config_path,
        &[
            "command",
            "define",
            "Inherited Opt In Guard",
            "External runner guard",
            "printf inherited-opt-in-guard",
        ],
    );
    assert!(define.status.success(), "{}", stderr(&define));

    let trigger = run_std_in_desktop_safe_test_mode(
        &config_path,
        &["trigger", "Inherited Opt In Guard", "--allow-external"],
    );

    assert!(trigger.status.success(), "{}", stderr(&trigger));
    let stdout = stdout(&trigger);
    assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(stdout.contains("printf inherited-opt-in-guard"));
    assert!(!stdout.contains("\"status\": \"Completed\""));
}

#[test]
fn std_registered_app_test_mode_blocks_child_process_app_open() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());
    let app_path = temp.path().join("StdNeverLaunchFixture.app");
    fs::create_dir_all(app_path.join("Contents").join("MacOS")).unwrap();
    fs::write(
        app_path.join("Contents").join("MacOS").join("fixture"),
        "bin",
    )
    .unwrap();

    let register = run_std_in_desktop_safe_test_mode(
        &config_path,
        &["app", "register", app_path.to_str().unwrap()],
    );
    assert!(register.status.success(), "{}", stderr(&register));

    let trigger = run_std_in_desktop_safe_test_mode(
        &config_path,
        &["trigger", "StdNeverLaunchFixture", "--allow-external"],
    );

    assert!(trigger.status.success(), "{}", stderr(&trigger));
    let stdout = stdout(&trigger);
    assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(stdout.contains("StdNeverLaunchFixture.app"));
    assert!(!stdout.contains("\"status\": \"Completed\""));
}

#[test]
fn std_binary_test_mode_ignores_ambient_user_data_dir() {
    let temp = tempfile::tempdir().unwrap();
    let ambient_data = temp.path().join("ambient-data");
    let ambient_app = ambient_data
        .join("Applications")
        .join("StdNeverExposeAmbient.app");
    fs::create_dir_all(ambient_app.join("Contents").join("MacOS")).unwrap();
    fs::write(
        ambient_app.join("Contents").join("MacOS").join("fixture"),
        "bin",
    )
    .unwrap();

    let mut command = Command::new(env!("CARGO_BIN_EXE_std"));
    let output = command
        .args(["search", "StdNeverExposeAmbient"])
        .env("STD_TEST_MODE", "1")
        .env("STDCLI_DATA_DIR", &ambient_data)
        .env_remove("STDCLI_CONFIG")
        .env_remove("STD_ALLOW_DESKTOP_AUTOMATION")
        .env_remove("STD_ALLOW_UI_PREVIEW")
        .output()
        .unwrap();

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(!stdout.contains("StdNeverExposeAmbient"));
    assert!(!stdout.contains(&ambient_data.display().to_string()));
}

fn write_config(root: &Path) -> std::path::PathBuf {
    let config_path = root.join("std-cli.json");
    fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": root.join("data"),
        })
        .to_string(),
    )
    .unwrap();
    config_path
}

fn run_std_in_desktop_safe_test_mode(config_path: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_std"));
    command
        .args(args)
        .env("STDCLI_CONFIG", config_path)
        .env("STD_TEST_MODE", "1")
        .env_remove("STD_ALLOW_DESKTOP_AUTOMATION")
        .env_remove("STD_ALLOW_UI_PREVIEW");
    command.output().unwrap()
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
