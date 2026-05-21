use std::process::Command;

#[test]
fn launcher_test_mode_wins_over_inherited_desktop_opt_in() {
    let output = run_launcher_with_inherited_desktop_opt_in(&[]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("launcher_native_app SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked native app startup"));
}

#[test]
fn launcher_hotkey_smoke_test_mode_wins_over_inherited_desktop_opt_in() {
    let output = run_launcher_with_inherited_desktop_opt_in(&["--hotkey-smoke", "Alt+Space"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("launcher_hotkey_smoke SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked global hotkey registration"));
}

#[test]
fn launcher_gui_hotkey_smoke_test_mode_wins_over_inherited_desktop_opt_in() {
    let output =
        run_launcher_with_inherited_desktop_opt_in(&["--gui-hotkey-smoke", "Alt+Space", "10"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("launcher_gui_hotkey_smoke SKIP"));
    assert!(stdout.contains("registered=false"));
    assert!(stdout.contains("input_sent=false"));
    assert!(stdout.contains("STD_TEST_MODE blocked GUI hotkey smoke"));
}

#[test]
fn launcher_preview_test_mode_wins_over_inherited_preview_opt_in() {
    let output =
        run_launcher_with_inherited_desktop_opt_in(&["--ui-preview", "light", "results", "10"]);

    assert!(output.status.success(), "{}", stderr(&output));
    let stdout = stdout(&output);
    assert!(stdout.contains("launcher_ui_preview SKIP"));
    assert!(stdout.contains("STD_TEST_MODE blocked UI preview"));
}

fn run_launcher_with_inherited_desktop_opt_in(args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_std-launcher"));
    command
        .args(args)
        .env("STD_TEST_MODE", "1")
        .env("STD_ALLOW_DESKTOP_AUTOMATION", "1")
        .env("STD_ALLOW_UI_PREVIEW", "1");
    command.output().unwrap()
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}
