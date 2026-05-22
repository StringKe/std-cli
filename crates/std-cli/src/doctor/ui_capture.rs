use crate::{
    doctor::workspace::{check_text, read_required},
    CliError,
};
use std::{env, fs};

const LAUNCHER_CAPTURE_STATES: &[(&str, &str)] = &[
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
    ("light", "action-panel"),
    ("dark", "action-panel"),
];

const STUDIO_CAPTURE_STATES: &[(&str, &str)] = &[
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

pub(crate) fn check_ui_capture_scripts(root: &std::path::Path) -> Result<(), CliError> {
    check_window_capture_script(root)?;
    check_matrix_capture_script(root)?;
    check_optional_capture_manifest()
}

pub(crate) fn verify_ui_capture_manifest(body: &str) -> Result<usize, CliError> {
    check_text(body, "capture-ui-matrix manifest")?;
    check_text(body, "opt_in=STD_ALLOW_UI_PREVIEW=1")?;
    check_text(body, "capture_rule=pid+process-name+window-title")?;
    check_text(body, "completion_rule=current-run-png-only")?;
    for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
        verify_capture_line(body, "launcher", theme, scenario)?;
    }
    for (theme, scenario) in STUDIO_CAPTURE_STATES {
        verify_capture_line(body, "studio", theme, scenario)?;
    }
    Ok(LAUNCHER_CAPTURE_STATES.len() + STUDIO_CAPTURE_STATES.len())
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
        "manifest=\"$out_dir/manifest.txt\"",
        "capture-ui-matrix manifest",
        "created_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)",
        "capture_rule=pid+process-name+window-title",
        "completion_rule=current-run-png-only",
        "record_capture launcher",
        "record_capture studio",
        "manifest=$manifest",
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

fn check_optional_capture_manifest() -> Result<(), CliError> {
    let Ok(path) = env::var("STD_UI_CAPTURE_MANIFEST") else {
        return Ok(());
    };
    let body = fs::read_to_string(&path).map_err(|error| {
        CliError::Doctor(format!("unable to read UI capture manifest: {error}"))
    })?;
    verify_ui_capture_manifest(&body)?;
    Ok(())
}

fn verify_capture_line(
    body: &str,
    surface: &str,
    theme: &str,
    scenario: &str,
) -> Result<(), CliError> {
    let prefix = format!("{surface} theme={theme} scenario={scenario} ");
    let line = body
        .lines()
        .find(|line| line.starts_with(&prefix))
        .ok_or_else(|| CliError::Doctor(format!("capture manifest missing {prefix}")))?;
    let path = capture_field(line, "path=")?;
    let bytes = capture_field(line, "bytes=")?;
    let expected_name = format!("{surface}-{theme}-{scenario}.png");
    if !path.ends_with(&expected_name) {
        return Err(CliError::Doctor(format!(
            "capture manifest path mismatch for {surface} {theme} {scenario}: {path}"
        )));
    }
    if bytes
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
        .is_none()
    {
        return Err(CliError::Doctor(format!(
            "capture manifest bytes must be positive for {surface} {theme} {scenario}"
        )));
    }
    Ok(())
}

fn capture_field<'a>(line: &'a str, key: &str) -> Result<&'a str, CliError> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(key))
        .ok_or_else(|| CliError::Doctor(format!("capture manifest field missing: {key}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::workspace::find_workspace_root;

    #[test]
    fn ui_capture_scripts_require_explicit_preview_opt_in() {
        check_ui_capture_scripts(&find_workspace_root().unwrap()).unwrap();
    }

    #[test]
    fn ui_capture_manifest_requires_all_launcher_and_studio_pngs() {
        let manifest = sample_manifest();

        assert_eq!(verify_ui_capture_manifest(&manifest).unwrap(), 38);
    }

    #[test]
    fn ui_capture_manifest_rejects_missing_state() {
        let manifest = sample_manifest().replace(
            "launcher theme=dark scenario=error path=artifacts/ui/manual-acceptance/launcher-dark-error.png bytes=1\n",
            "",
        );

        let error = verify_ui_capture_manifest(&manifest).unwrap_err();
        assert!(error
            .to_string()
            .contains("capture manifest missing launcher theme=dark scenario=error"));
    }

    #[test]
    fn ui_capture_manifest_rejects_empty_png() {
        let manifest = sample_manifest().replace(
            "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=1",
            "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=0",
        );

        let error = verify_ui_capture_manifest(&manifest).unwrap_err();
        assert!(error.to_string().contains("bytes must be positive"));
    }

    fn sample_manifest() -> String {
        let mut lines = vec![
            "capture-ui-matrix manifest".to_string(),
            "created_at=2026-05-22T00:00:00Z".to_string(),
            "out_dir=artifacts/ui/manual-acceptance".to_string(),
            "opt_in=STD_ALLOW_UI_PREVIEW=1".to_string(),
            "test_mode=STD_TEST_MODE must not be 1".to_string(),
            "capture_rule=pid+process-name+window-title".to_string(),
            "completion_rule=current-run-png-only".to_string(),
        ];
        for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
            lines.push(format!(
                "launcher theme={theme} scenario={scenario} path=artifacts/ui/manual-acceptance/launcher-{theme}-{scenario}.png bytes=1"
            ));
        }
        for (theme, scenario) in STUDIO_CAPTURE_STATES {
            lines.push(format!(
                "studio theme={theme} scenario={scenario} path=artifacts/ui/manual-acceptance/studio-{theme}-{scenario}.png bytes=1"
            ));
        }
        lines.join("\n")
    }
}
