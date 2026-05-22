use crate::{
    doctor::ui_capture_pixels::{verify_pixel_evidence, CapturePixelEvidence},
    doctor::ui_capture_png::{verify_capture_png, CaptureManifestEntry},
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
    let sampler = read_required(&root.join("scripts/cg-sample-pixels.swift"))?;
    for required in [
        "CGImageSourceCreateWithURL",
        "CGContext(",
        "xPercents = [25, 50, 75]",
        "yPercents = [25, 50, 75]",
        "unique_colors",
        "black_pixels",
        "white_pixels",
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
        "scripts/capture-window.sh \"$pid\" std-studio",
        "manifest=\"$out_dir/manifest.txt\"",
        "capture-ui-matrix manifest",
        "created_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)",
        "capture_rule=pid+process-name+window-title",
        "completion_rule=current-run-png-only",
        "/usr/bin/sips -g pixelWidth",
        "/usr/bin/sips -g pixelHeight",
        "scripts/cg-sample-pixels.swift",
        "width=$width height=$height",
        "pixel_evidence=$(",
        "$pixel_evidence",
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
    let evidence = CapturePixelEvidence {
        samples: capture_dimension(line, "samples=", surface, theme, scenario)?,
        unique_colors: capture_dimension(line, "unique_colors=", surface, theme, scenario)?,
        black_pixels: capture_count(line, "black_pixels=", surface, theme, scenario)?,
        white_pixels: capture_count(line, "white_pixels=", surface, theme, scenario)?,
    };
    verify_pixel_evidence(surface, theme, scenario, &evidence)?;
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

fn capture_count(
    line: &str,
    key: &str,
    surface: &str,
    theme: &str,
    scenario: &str,
) -> Result<u32, CliError> {
    capture_field(line, key)?.parse::<u32>().map_err(|_| {
        CliError::Doctor(format!(
            "capture manifest {key} must be a non-negative count for {surface} {theme} {scenario}"
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::doctor::workspace::find_workspace_root;

    const SAMPLE_EVIDENCE: &str = "samples=9 unique_colors=3 black_pixels=0 white_pixels=0";

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
        let png_bytes = sample_png_bytes(720, 64).len();
        let manifest = sample_manifest().replace(
            &format!(
                "launcher theme=dark scenario=error path=artifacts/ui/manual-acceptance/launcher-dark-error.png bytes={png_bytes} width=720 height=64 {SAMPLE_EVIDENCE}\n"
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
        let png_bytes = sample_png_bytes(1080, 640).len();
        let manifest = sample_manifest().replace(
            &format!(
                "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes={png_bytes} width=1080 height=640 {SAMPLE_EVIDENCE}"
            ),
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=0 width=1080 height=640 {SAMPLE_EVIDENCE}"),
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
        let png_bytes = sample_png_bytes(1080, 640).len();
        let manifest = sample_manifest().replace(
            &format!(
                "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes={png_bytes} width=1080 height=640 {SAMPLE_EVIDENCE}"
            ),
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=7 width=1080 height=640 {SAMPLE_EVIDENCE}"),
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
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=24 width=1080 height=640 {SAMPLE_EVIDENCE}"),
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=24 width=1081 height=640 {SAMPLE_EVIDENCE}"),
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(temp.path())).unwrap_err();
        assert!(error.to_string().contains(
            "capture manifest dimensions mismatch for studio light panes: declared=1081x640 actual=1080x640"
        ));
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_too_small_png() {
        let temp = tempfile::tempdir().unwrap();
        write_sample_pngs(temp.path());
        let bytes = sample_png_bytes(500, 64);
        fs::write(temp.path().join("launcher-light-collapsed.png"), bytes).unwrap();
        let manifest = sample_manifest().replace(
            &format!("launcher theme=light scenario=collapsed path=artifacts/ui/manual-acceptance/launcher-light-collapsed.png bytes=24 width=720 height=64 {SAMPLE_EVIDENCE}"),
            &format!("launcher theme=light scenario=collapsed path=artifacts/ui/manual-acceptance/launcher-light-collapsed.png bytes=24 width=500 height=64 {SAMPLE_EVIDENCE}"),
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(temp.path())).unwrap_err();
        assert!(error
            .to_string()
            .contains("capture PNG too small for launcher light collapsed"));
    }

    #[test]
    fn ui_capture_manifest_rejects_all_black_or_white_carrier_evidence() {
        let black = sample_manifest().replace(
            SAMPLE_EVIDENCE,
            "samples=9 unique_colors=3 black_pixels=9 white_pixels=0",
        );
        let white = sample_manifest().replace(
            SAMPLE_EVIDENCE,
            "samples=9 unique_colors=3 black_pixels=0 white_pixels=9",
        );

        assert!(verify_ui_capture_manifest_with_root(&black, None)
            .unwrap_err()
            .to_string()
            .contains("all black host background"));
        assert!(verify_ui_capture_manifest_with_root(&white, None)
            .unwrap_err()
            .to_string()
            .contains("all white host background"));
    }

    fn sample_manifest() -> String {
        let launcher_bytes = sample_png_bytes(720, 64).len();
        let studio_bytes = sample_png_bytes(1080, 640).len();
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
                "launcher theme={theme} scenario={scenario} path=artifacts/ui/manual-acceptance/launcher-{theme}-{scenario}.png bytes={launcher_bytes} width=720 height=64 {SAMPLE_EVIDENCE}"
            ));
        }
        for (theme, scenario) in STUDIO_CAPTURE_STATES {
            lines.push(format!(
                "studio theme={theme} scenario={scenario} path=artifacts/ui/manual-acceptance/studio-{theme}-{scenario}.png bytes={studio_bytes} width=1080 height=640 {SAMPLE_EVIDENCE}"
            ));
        }
        lines.join("\n")
    }

    fn write_sample_pngs(root: &Path) {
        let launcher_bytes = sample_png_bytes(720, 64);
        for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
            fs::write(
                root.join(format!("launcher-{theme}-{scenario}.png")),
                &launcher_bytes,
            )
            .unwrap();
        }
        let studio_bytes = sample_png_bytes(1080, 640);
        for (theme, scenario) in STUDIO_CAPTURE_STATES {
            fs::write(
                root.join(format!("studio-{theme}-{scenario}.png")),
                &studio_bytes,
            )
            .unwrap();
        }
    }

    fn sample_png_bytes(width: u32, height: u32) -> Vec<u8> {
        let mut bytes = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".to_vec();
        bytes.extend(width.to_be_bytes());
        bytes.extend(height.to_be_bytes());
        bytes
    }
}
