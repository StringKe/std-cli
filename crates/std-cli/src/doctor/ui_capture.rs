use crate::{
    doctor::workspace::{check_text, read_required},
    CliError,
};

pub(crate) fn check_ui_capture_scripts(root: &std::path::Path) -> Result<(), CliError> {
    check_window_capture_script(root)?;
    check_matrix_capture_script(root)
}

fn check_window_capture_script(root: &std::path::Path) -> Result<(), CliError> {
    let body = read_required(&root.join("scripts/capture-window.sh"))?;
    for required in [
        "STD_ALLOW_UI_PREVIEW",
        "capture-window SKIP",
        "cg-capture-window.swift",
        "<process-pid> <process-name>",
    ] {
        check_text(&body, required)?;
    }
    let driver = read_required(&root.join("scripts/cg-capture-window.swift"))?;
    for required in [
        "kCGWindowOwnerPID",
        "pid == ownerPid",
        "title.contains(titleFragment)",
    ] {
        check_text(&driver, required)?;
    }
    Ok(())
}

fn check_matrix_capture_script(root: &std::path::Path) -> Result<(), CliError> {
    let body = read_required(&root.join("scripts/capture-ui-matrix.sh"))?;
    for required in [
        "STD_ALLOW_UI_PREVIEW",
        "STD_TEST_MODE blocks UI preview",
        "cargo run -p std-launcher -- --ui-preview",
        "cargo run -p std-studio -- --ui-preview",
        "scripts/capture-window.sh",
        "scripts/capture-window.sh \"$pid\" std-launcher",
        "scripts/capture-window.sh \"$pid\" std-studio",
        "capture_launcher light collapsed",
        "capture_launcher dark collapsed",
        "capture_launcher light empty",
        "capture_launcher dark empty",
        "capture_launcher light results",
        "capture_launcher dark results",
        "capture_launcher light no-results",
        "capture_launcher dark no-results",
        "capture_launcher light searching",
        "capture_launcher dark searching",
        "capture_launcher light loading",
        "capture_launcher dark loading",
        "capture_launcher light executing",
        "capture_launcher dark executing",
        "capture_launcher light defer",
        "capture_launcher dark defer",
        "capture_launcher light error",
        "capture_launcher dark error",
        "capture_studio light dashboard",
        "capture_studio dark dashboard",
        "capture_studio light workflow",
        "capture_studio dark workflow",
        "capture_studio light workflow-error",
        "capture_studio dark workflow-error",
        "capture_studio light analysis",
        "capture_studio dark analysis",
        "capture_studio light plugins",
        "capture_studio dark plugins",
        "capture_studio light plugin-permission",
        "capture_studio dark plugin-permission",
        "capture_studio light operations",
        "capture_studio dark operations",
        "capture_studio light settings",
        "capture_studio dark settings",
        "capture_studio light panes",
        "capture_studio dark panes",
    ] {
        check_text(&body, required)?;
    }
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "cargo run -p std-launcher")?;
    assert_order(&body, "STD_TEST_MODE", "cargo run -p std-launcher")?;
    assert_order(&body, "STD_ALLOW_UI_PREVIEW", "scripts/capture-window.sh")?;
    Ok(())
}

fn assert_order(body: &str, first: &str, second: &str) -> Result<(), CliError> {
    let first_index = body
        .find(first)
        .ok_or_else(|| CliError::Doctor(format!("required text missing: {first}")))?;
    let second_index = body
        .find(second)
        .ok_or_else(|| CliError::Doctor(format!("required text missing: {second}")))?;
    if first_index < second_index {
        return Ok(());
    }
    Err(CliError::Doctor(format!(
        "{first} must appear before {second}"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::workspace::find_workspace_root;

    #[test]
    fn ui_capture_scripts_require_explicit_preview_opt_in() {
        check_ui_capture_scripts(&find_workspace_root().unwrap()).unwrap();
    }
}
