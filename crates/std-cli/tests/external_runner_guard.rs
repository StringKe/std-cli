use std::{fs, path::Path, process::Command};

#[test]
fn binary_test_mode_blocks_external_runner() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());

    let define = run_std(
        &config_path,
        &[
            "command",
            "define",
            "Binary External Guard",
            "External runner guard",
            "printf binary-external-guard",
        ],
    );
    assert!(define.status.success(), "{}", command_stderr(&define));

    let trigger = run_std(
        &config_path,
        &["trigger", "Binary External Guard", "--allow-external"],
    );
    assert!(trigger.status.success(), "{}", command_stderr(&trigger));

    let stdout = command_stdout(&trigger);
    assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(stdout.contains("printf binary-external-guard"));
    assert!(!stdout.contains("\"status\": \"Completed\""));
}

#[test]
fn binary_test_mode_blocks_dangerous_command_text() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());

    for (command_text, guard_terms) in [
        (
            "open -a StdNeverLaunchFixture",
            vec!["open", "StdNeverLaunchFixture"],
        ),
        (
            "/usr/bin/open -a StdNeverLaunchFixture",
            vec!["/usr/bin/open", "StdNeverLaunchFixture"],
        ),
        (
            "/usr/bin/osascript -e 'tell application \"StdNeverLaunchFixture\" to activate'",
            vec!["/usr/bin/osascript", "StdNeverLaunchFixture"],
        ),
    ] {
        let define = run_std(
            &config_path,
            &[
                "command",
                "define",
                command_text,
                "Dangerous external runner guard",
                command_text,
            ],
        );
        assert!(define.status.success(), "{}", command_stderr(&define));

        let trigger = run_std(&config_path, &["trigger", command_text, "--allow-external"]);
        assert!(trigger.status.success(), "{}", command_stderr(&trigger));

        let stdout = command_stdout(&trigger);
        assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
        for term in guard_terms {
            assert!(stdout.contains(term), "{stdout}");
        }
        assert!(!stdout.contains("\"status\": \"Completed\""));
    }
}

#[test]
fn binary_test_mode_blocks_registered_app_launch() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());
    let app_path = temp.path().join("FakePassword.app");
    fs::create_dir_all(app_path.join("Contents").join("MacOS")).unwrap();
    fs::write(
        app_path
            .join("Contents")
            .join("MacOS")
            .join("fake-password"),
        "bin",
    )
    .unwrap();

    let register = run_std(
        &config_path,
        &["app", "register", app_path.to_str().unwrap()],
    );
    assert!(register.status.success(), "{}", command_stderr(&register));

    let trigger = run_std(
        &config_path,
        &["trigger", "FakePassword", "--allow-external"],
    );
    assert!(trigger.status.success(), "{}", command_stderr(&trigger));

    let stdout = command_stdout(&trigger);
    assert!(stdout.contains("\"action_name\": \"Open App: FakePassword\""));
    assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(stdout.contains("open "));
    assert!(stdout.contains("FakePassword.app"));
    assert!(!stdout.contains("\"status\": \"Completed\""));
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

fn run_std(config_path: &Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_std"));
    command
        .args(args)
        .env("STDCLI_CONFIG", config_path)
        .env("STD_TEST_MODE", "1");
    command.output().unwrap()
}

fn command_stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn command_stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
