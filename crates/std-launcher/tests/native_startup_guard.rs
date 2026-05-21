use std::process::Command;

#[test]
fn launcher_binary_test_mode_blocks_native_window_startup() {
    let output = run_launcher(&[]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("launcher_native_app SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked native app startup"));
}

#[test]
fn launcher_binary_test_mode_blocks_ui_preview_window_startup() {
    let output = run_launcher(&["--ui-preview", "light", "results", "10"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("launcher_ui_preview SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked UI preview"));
}

fn run_launcher(args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_std-launcher"));
    command
        .args(args)
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
