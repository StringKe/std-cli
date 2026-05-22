use crate::{
    doctor::workspace::{check_text, read_required},
    CliError,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

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

struct CaptureManifestEntry<'a> {
    path: &'a str,
    declared_bytes: usize,
    declared_width: u32,
    declared_height: u32,
    surface: &'a str,
    theme: &'a str,
    scenario: &'a str,
}

pub(crate) fn check_ui_capture_scripts(root: &std::path::Path) -> Result<(), CliError> {
    check_window_capture_script(root)?;
    check_matrix_capture_script(root)?;
    check_optional_capture_manifest()
}

fn verify_ui_capture_manifest_with_root(
    body: &str,
    root: Option<&Path>,
) -> Result<usize, CliError> {
    check_text(body, "capture-ui-matrix manifest")?;
    check_text(body, "opt_in=STD_ALLOW_UI_PREVIEW=1")?;
    check_text(body, "capture_rule=pid+process-name+window-title")?;
    check_text(body, "completion_rule=current-run-png-only")?;
    for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
        verify_capture_line(body, root, "launcher", theme, scenario)?;
    }
    for (theme, scenario) in STUDIO_CAPTURE_STATES {
        verify_capture_line(body, root, "studio", theme, scenario)?;
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
        "/usr/bin/sips -g pixelWidth",
        "/usr/bin/sips -g pixelHeight",
        "width=$width height=$height",
        "record_capture launcher",
        "record_capture studio",
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

fn check_optional_capture_manifest() -> Result<(), CliError> {
    let Ok(path) = env::var("STD_UI_CAPTURE_MANIFEST") else {
        return Ok(());
    };
    let manifest_path = PathBuf::from(&path);
    let body = fs::read_to_string(&manifest_path).map_err(|error| {
        CliError::Doctor(format!("unable to read UI capture manifest: {error}"))
    })?;
    let root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    verify_ui_capture_manifest_with_root(&body, Some(root))?;
    Ok(())
}

fn verify_capture_line(
    body: &str,
    root: Option<&Path>,
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
    let width = capture_dimension(line, "width=", surface, theme, scenario)?;
    let height = capture_dimension(line, "height=", surface, theme, scenario)?;
    let expected_name = format!("{surface}-{theme}-{scenario}.png");
    if !path.ends_with(&expected_name) {
        return Err(CliError::Doctor(format!(
            "capture manifest path mismatch for {surface} {theme} {scenario}: {path}"
        )));
    }
    let declared_bytes = bytes
        .parse::<usize>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| {
            CliError::Doctor(format!(
                "capture manifest bytes must be positive for {surface} {theme} {scenario}"
            ))
        })?;
    if let Some(root) = root {
        let entry = CaptureManifestEntry {
            path,
            declared_bytes,
            declared_width: width,
            declared_height: height,
            surface,
            theme,
            scenario,
        };
        verify_capture_png(root, &entry)?;
    }
    Ok(())
}

fn verify_capture_png(root: &Path, entry: &CaptureManifestEntry<'_>) -> Result<(), CliError> {
    let png_path = capture_path(root, entry.path);
    let bytes = fs::read(&png_path).map_err(|error| {
        CliError::Doctor(format!(
            "unable to read capture png for {} {} {}: {error}",
            entry.surface, entry.theme, entry.scenario
        ))
    })?;
    if bytes.len() != entry.declared_bytes {
        return Err(CliError::Doctor(format!(
            "capture manifest byte count mismatch for {} {} {}: declared={} actual={}",
            entry.surface,
            entry.theme,
            entry.scenario,
            entry.declared_bytes,
            bytes.len()
        )));
    }
    if !bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        return Err(CliError::Doctor(format!(
            "capture file must be PNG for {} {} {}",
            entry.surface, entry.theme, entry.scenario
        )));
    }
    let (actual_width, actual_height) = png_dimensions(&bytes)?;
    if actual_width != entry.declared_width || actual_height != entry.declared_height {
        return Err(CliError::Doctor(format!(
            "capture manifest dimensions mismatch for {} {} {}: declared={}x{} actual={}x{}",
            entry.surface,
            entry.theme,
            entry.scenario,
            entry.declared_width,
            entry.declared_height,
            actual_width,
            actual_height
        )));
    }
    Ok(())
}

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), CliError> {
    if bytes.len() < 24 || !bytes.starts_with(b"\x89PNG\r\n\x1a\n") || &bytes[12..16] != b"IHDR" {
        return Err(CliError::Doctor(
            "capture file must contain PNG IHDR dimensions".to_string(),
        ));
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
    let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
    if width == 0 || height == 0 {
        return Err(CliError::Doctor(
            "capture PNG dimensions must be positive".to_string(),
        ));
    }
    Ok((width, height))
}

fn capture_path(root: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else if let Some(name) = path.file_name() {
        root.join(name)
    } else {
        root.join(path)
    }
}

fn capture_field<'a>(line: &'a str, key: &str) -> Result<&'a str, CliError> {
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(key))
        .ok_or_else(|| CliError::Doctor(format!("capture manifest field missing: {key}")))
}

fn capture_dimension(
    line: &str,
    key: &str,
    surface: &str,
    theme: &str,
    scenario: &str,
) -> Result<u32, CliError> {
    capture_field(line, key)?
        .parse::<u32>()
        .ok()
        .filter(|value| *value > 0)
        .ok_or_else(|| {
            CliError::Doctor(format!(
                "capture manifest {key} must be positive for {surface} {theme} {scenario}"
            ))
        })
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

        assert_eq!(
            verify_ui_capture_manifest_with_root(&manifest, None).unwrap(),
            38
        );
    }

    #[test]
    fn ui_capture_manifest_rejects_missing_state() {
        let png_bytes = sample_png_bytes().len();
        let manifest = sample_manifest().replace(
            &format!(
                "launcher theme=dark scenario=error path=artifacts/ui/manual-acceptance/launcher-dark-error.png bytes={png_bytes} width=1 height=1\n"
            ),
            "",
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, None).unwrap_err();
        assert!(error
            .to_string()
            .contains("capture manifest missing launcher theme=dark scenario=error"));
    }

    #[test]
    fn ui_capture_manifest_rejects_empty_png() {
        let png_bytes = sample_png_bytes().len();
        let manifest = sample_manifest().replace(
            &format!(
                "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes={png_bytes} width=1 height=1"
            ),
            "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=0 width=1 height=1",
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, None).unwrap_err();
        assert!(error.to_string().contains("bytes must be positive"));
    }

    #[test]
    fn ui_capture_manifest_with_root_requires_real_png_files() {
        let temp = tempfile::tempdir().unwrap();
        write_sample_pngs(temp.path());
        let manifest = sample_manifest();

        assert_eq!(
            verify_ui_capture_manifest_with_root(&manifest, Some(temp.path())).unwrap(),
            38
        );
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_missing_png_file() {
        let temp = tempfile::tempdir().unwrap();
        write_sample_pngs(temp.path());
        fs::remove_file(temp.path().join("launcher-dark-error.png")).unwrap();
        let manifest = sample_manifest();

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(temp.path())).unwrap_err();
        assert!(error
            .to_string()
            .contains("unable to read capture png for launcher dark error"));
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_non_png_file() {
        let temp = tempfile::tempdir().unwrap();
        write_sample_pngs(temp.path());
        fs::write(temp.path().join("studio-light-panes.png"), b"not-png").unwrap();
        let png_bytes = sample_png_bytes().len();
        let manifest = sample_manifest().replace(
            &format!(
                "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes={png_bytes} width=1 height=1"
            ),
            "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=7 width=1 height=1",
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(temp.path())).unwrap_err();
        assert!(error
            .to_string()
            .contains("capture file must be PNG for studio light panes"));
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_dimension_mismatch() {
        let temp = tempfile::tempdir().unwrap();
        write_sample_pngs(temp.path());
        let manifest = sample_manifest().replace(
            "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=24 width=1 height=1",
            "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=24 width=2 height=1",
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(temp.path())).unwrap_err();
        assert!(error.to_string().contains(
            "capture manifest dimensions mismatch for studio light panes: declared=2x1 actual=1x1"
        ));
    }

    fn sample_manifest() -> String {
        let png_bytes = sample_png_bytes().len();
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
                "launcher theme={theme} scenario={scenario} path=artifacts/ui/manual-acceptance/launcher-{theme}-{scenario}.png bytes={png_bytes} width=1 height=1"
            ));
        }
        for (theme, scenario) in STUDIO_CAPTURE_STATES {
            lines.push(format!(
                "studio theme={theme} scenario={scenario} path=artifacts/ui/manual-acceptance/studio-{theme}-{scenario}.png bytes={png_bytes} width=1 height=1"
            ));
        }
        lines.join("\n")
    }

    fn write_sample_pngs(root: &Path) {
        let bytes = sample_png_bytes();
        for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
            fs::write(root.join(format!("launcher-{theme}-{scenario}.png")), bytes).unwrap();
        }
        for (theme, scenario) in STUDIO_CAPTURE_STATES {
            fs::write(root.join(format!("studio-{theme}-{scenario}.png")), bytes).unwrap();
        }
    }

    fn sample_png_bytes() -> &'static [u8] {
        b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR\0\0\0\x01\0\0\0\x01"
    }
}
