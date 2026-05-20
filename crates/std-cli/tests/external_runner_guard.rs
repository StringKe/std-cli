use std::{fs, process::Command};

#[test]
fn binary_test_mode_blocks_external_runner_even_with_explicit_opt_in() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();

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

fn run_std(config_path: &std::path::Path, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_std"));
    command
        .args(args)
        .env("STDCLI_CONFIG", config_path)
        .env("STD_TEST_MODE", "1")
        .env("STD_ALLOW_DESKTOP_AUTOMATION", "1");
    command.output().unwrap()
}

fn command_stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn command_stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
