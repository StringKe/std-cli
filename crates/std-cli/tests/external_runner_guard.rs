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

    let plain_runner = "std-fixture-runner --target StdNeverLaunchFixture";
    let absolute_runner = "/tmp/std-fixture-runner --target StdNeverLaunchFixture";
    let scripted_runner = "std-fixture-script --activate StdNeverLaunchFixture";
    for (command_text, guard_terms) in [
        (
            plain_runner,
            vec![
                "std-fixture-runner".to_string(),
                "StdNeverLaunchFixture".to_string(),
            ],
        ),
        (
            absolute_runner,
            vec![
                "/tmp/std-fixture-runner".to_string(),
                "StdNeverLaunchFixture".to_string(),
            ],
        ),
        (
            scripted_runner,
            vec![
                "std-fixture-script".to_string(),
                "StdNeverLaunchFixture".to_string(),
            ],
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
        for term in &guard_terms {
            assert!(stdout.contains(term), "{stdout}");
        }
        assert!(!stdout.contains("\"status\": \"Completed\""));
    }
}

#[test]
fn binary_test_mode_blocks_macos_activation_command_text() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());
    let commands = [
        ["op", "en -a StdFixtureVault"].concat(),
        [
            "/usr/bin/osa",
            "script -e 'tell app \"StdFixtureVault\" to activate'",
        ]
        .concat(),
    ];

    for command_text in commands {
        let define = run_std(
            &config_path,
            &[
                "command",
                "define",
                &command_text,
                "macOS activation runner guard",
                &command_text,
            ],
        );
        assert!(define.status.success(), "{}", command_stderr(&define));

        let trigger = run_std(
            &config_path,
            &["trigger", &command_text, "--allow-external"],
        );
        assert!(trigger.status.success(), "{}", command_stderr(&trigger));

        let stdout = command_stdout(&trigger);
        assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
        assert!(stdout.contains("StdFixtureVault"));
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

#[test]
fn binary_test_mode_does_not_expose_system_app_results() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());

    for query in [
        "StdFixtureVault".to_string(),
        "StdFixtureChat".to_string(),
        "stdfixturealias".to_string(),
        "测试应用".to_string(),
    ] {
        let search = run_std(&config_path, &["search", &query]);
        assert!(search.status.success(), "{}", command_stderr(&search));

        let stdout = command_stdout(&search);
        assert!(
            !stdout.contains("Open App:"),
            "STD_TEST_MODE must not expose app launch results for {query}: {stdout}"
        );
        assert!(
            !stdout.contains("/Applications")
                && !stdout.contains(&["/System", "/Applications"].join("")),
            "STD_TEST_MODE must not expose system app paths for {query}: {stdout}"
        );
    }
}

#[test]
fn binary_test_mode_blocks_fixture_desktop_app_names() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());

    let app_names = [
        "StdFixtureVault".to_string(),
        "StdFixtureChat".to_string(),
        "stdfixturealias".to_string(),
        "测试应用".to_string(),
        "StdFixtureShell".to_string(),
    ];
    for app_name in app_names {
        let command_text = format!("std-fixture-launch --app {app_name}");
        let define = run_std(
            &config_path,
            &[
                "command",
                "define",
                &format!("Guard {app_name}"),
                "Real desktop app launch guard",
                &command_text,
            ],
        );
        assert!(define.status.success(), "{}", command_stderr(&define));

        let trigger = run_std(
            &config_path,
            &["trigger", &format!("Guard {app_name}"), "--allow-external"],
        );
        assert!(trigger.status.success(), "{}", command_stderr(&trigger));

        let stdout = command_stdout(&trigger);
        assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
        assert!(stdout.contains(&command_text));
        assert!(!stdout.contains("\"status\": \"Completed\""));
    }
}

#[test]
fn binary_blocks_external_runner_without_desktop_opt_in() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());

    let define = run_std(
        &config_path,
        &[
            "command",
            "define",
            "Opt In External Guard",
            "External runner guard",
            "printf opt-in-external-guard",
        ],
    );
    assert!(define.status.success(), "{}", command_stderr(&define));

    let trigger = run_std(
        &config_path,
        &["trigger", "Opt In External Guard", "--allow-external"],
    );
    assert!(trigger.status.success(), "{}", command_stderr(&trigger));

    let stdout = command_stdout(&trigger);
    assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(stdout.contains("printf opt-in-external-guard"));
    assert!(!stdout.contains("\"status\": \"Completed\""));
}

#[test]
fn binary_blocks_registered_app_even_with_allow_external_without_desktop_opt_in() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = write_config(temp.path());
    let app_path = temp.path().join("StdNeverLaunchFixture.app");
    fs::create_dir_all(app_path.join("Contents").join("MacOS")).unwrap();
    fs::write(
        app_path.join("Contents").join("MacOS").join("fixture"),
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
        &["trigger", "StdNeverLaunchFixture", "--allow-external"],
    );
    assert!(trigger.status.success(), "{}", command_stderr(&trigger));

    let stdout = command_stdout(&trigger);
    assert!(stdout.contains("\"status\": \"NeedsExternalRunner\""));
    assert!(stdout.contains("StdNeverLaunchFixture.app"));
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
        .env("STD_TEST_MODE", "1")
        .env_remove("STD_ALLOW_DESKTOP_AUTOMATION")
        .env_remove("STD_ALLOW_UI_PREVIEW");
    command.output().unwrap()
}

fn command_stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn command_stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
