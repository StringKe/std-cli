use std::process::Command;

#[test]
fn studio_test_mode_removes_desktop_opt_in_for_child_process() {
    let output = run_studio_in_desktop_safe_test_mode(&[]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("studio_native_app SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked native app startup"));
}

#[test]
fn studio_preview_test_mode_removes_ui_preview_opt_in_for_child_process() {
    let output =
        run_studio_in_desktop_safe_test_mode(&["--ui-preview", "light", "dashboard", "10"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("studio_ui_preview SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked Studio UI preview"));
}

fn run_studio_in_desktop_safe_test_mode(args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_std-studio"));
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
