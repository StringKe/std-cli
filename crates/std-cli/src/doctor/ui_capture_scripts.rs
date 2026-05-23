use crate::{
    doctor::workspace::{check_text, read_required},
    CliError,
};

pub(crate) const LAUNCHER_CAPTURE_STATES: &[(&str, &str)] = &[
    ("light", "collapsed"),
    ("dark", "collapsed"),
    ("light", "empty"),
    ("dark", "empty"),
    ("light", "results"),
    ("dark", "results"),
    ("light", "no-results"),
    ("dark", "no-results"),
    ("light", "searching"),
    ("dark", "searching"),
    ("light", "loading"),
    ("dark", "loading"),
    ("light", "executing"),
    ("dark", "executing"),
    ("light", "defer"),
    ("dark", "defer"),
    ("light", "error"),
    ("dark", "error"),
    ("light", "ime"),
    ("dark", "ime"),
    ("light", "action-panel"),
    ("dark", "action-panel"),
];

pub(crate) const STUDIO_CAPTURE_STATES: &[(&str, &str)] = &[
    ("light", "dashboard"),
    ("dark", "dashboard"),
    ("light", "workflow"),
    ("dark", "workflow"),
    ("light", "workflow-error"),
    ("dark", "workflow-error"),
    ("light", "analysis"),
    ("dark", "analysis"),
    ("light", "plugins"),
    ("dark", "plugins"),
    ("light", "plugin-permission"),
    ("dark", "plugin-permission"),
    ("light", "operations"),
    ("dark", "operations"),
    ("light", "settings"),
    ("dark", "settings"),
    ("light", "panes"),
    ("dark", "panes"),
];

pub(crate) fn check_capture_scripts(root: &std::path::Path) -> Result<(), CliError> {
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
        "CGWindowListCreateImage",
        ".optionIncludingWindow",
        "UTType.png.identifier",
    ] {
        check_text(&driver, required)?;
    }
    for forbidden in ["/usr/sbin/screencapture", "-R"] {
        if driver.contains(forbidden) {
            return Err(CliError::Doctor(
                "capture driver must capture by CGWindowID, not screen rectangle".to_string(),
            ));
        }
    }
    let sampler = read_required(&root.join("scripts/cg-sample-pixels.swift"))?;
    for required in [
        "CGImageSourceCreateWithURL",
        "CGContext(",
        "xPercents = [25, 50, 75]",
        "yPercents = [25, 50, 75]",
        "opaqueSamples",
        "unique_colors",
        "black_pixels",
        "white_pixels",
        "transparent_pixels",
        "edge_samples",
        "edge_transparent_pixels",
        "edge_black_pixels",
        "edge_white_pixels",
    ] {
        check_text(&sampler, required)?;
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
        "scripts/capture-window.sh \"$pid\" std-launcher \"std-cli-Launcher\"",
        "scripts/capture-window.sh \"$pid\" std-studio",
        "scripts/capture-window.sh \"$pid\" std-studio \"std-cli-Studio\"",
        "manifest=\"$out_dir/manifest.txt\"",
        "capture-ui-matrix manifest",
        "created_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)",
        "capture_rule=pid+process-name+window-title",
        "completion_rule=current-run-png-only",
        "pid=$pid process=$process window_title=$title",
        "/usr/bin/sips -g pixelWidth",
        "/usr/bin/sips -g pixelHeight",
        "scripts/cg-sample-pixels.swift",
        "width=$width height=$height",
        "pixel_evidence=$(",
        "$pixel_evidence",
        "record_capture launcher",
        "record_capture launcher \"$theme\" \"$scenario\" \"$output\" \"$pid\" std-launcher std-cli-Launcher",
        "record_capture studio",
        "record_capture studio \"$theme\" \"$scenario\" \"$output\" \"$pid\" std-studio std-cli-Studio",
        "manifest=$manifest",
    ] {
        check_text(&body, required)?;
    }
    for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
        check_text(&body, &format!("capture_launcher {theme} {scenario}"))?;
    }
    for (theme, scenario) in STUDIO_CAPTURE_STATES {
        check_text(&body, &format!("capture_studio {theme} {scenario}"))?;
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
