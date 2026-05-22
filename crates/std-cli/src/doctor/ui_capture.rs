use crate::{
    doctor::ui_capture_manifest::verify_capture_manifest_header,
    doctor::ui_capture_pixels::{verify_pixel_evidence, CapturePixelEvidence},
    doctor::ui_capture_png::{verify_capture_png, CaptureManifestEntry},
    doctor::ui_capture_scripts::{
        check_capture_scripts, LAUNCHER_CAPTURE_STATES, STUDIO_CAPTURE_STATES,
    },
    CliError,
};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub(crate) fn check_ui_capture_scripts(root: &std::path::Path) -> Result<(), CliError> {
    check_capture_scripts(root)?;
    check_optional_capture_manifest()
}

fn verify_ui_capture_manifest_with_root(
    body: &str,
    root: Option<&Path>,
) -> Result<usize, CliError> {
    verify_capture_manifest_header(body, root)?;
    for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
        verify_capture_line(body, root, "launcher", theme, scenario)?;
    }
    for (theme, scenario) in STUDIO_CAPTURE_STATES {
        verify_capture_line(body, root, "studio", theme, scenario)?;
    }
    Ok(LAUNCHER_CAPTURE_STATES.len() + STUDIO_CAPTURE_STATES.len())
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
        opaque_samples: capture_dimension(line, "opaque_samples=", surface, theme, scenario)?,
        unique_colors: capture_dimension(line, "unique_colors=", surface, theme, scenario)?,
        black_pixels: capture_count(line, "black_pixels=", surface, theme, scenario)?,
        white_pixels: capture_count(line, "white_pixels=", surface, theme, scenario)?,
        transparent_pixels: capture_count(line, "transparent_pixels=", surface, theme, scenario)?,
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
    use crate::doctor::ui_capture_tests::{
        capture_root, sample_manifest, sample_png_bytes, write_sample_pngs, SAMPLE_EVIDENCE,
    };
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
            40
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
        let root = capture_root(&temp);
        write_sample_pngs(&root);
        let manifest = sample_manifest();

        assert_eq!(
            verify_ui_capture_manifest_with_root(&manifest, Some(&root)).unwrap(),
            40
        );
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_missing_png_file() {
        let temp = tempfile::tempdir().unwrap();
        let root = capture_root(&temp);
        write_sample_pngs(&root);
        fs::remove_file(root.join("launcher-dark-error.png")).unwrap();
        let manifest = sample_manifest();

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(&root)).unwrap_err();
        assert!(error
            .to_string()
            .contains("unable to read capture png for launcher dark error"));
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_non_png_file() {
        let temp = tempfile::tempdir().unwrap();
        let root = capture_root(&temp);
        write_sample_pngs(&root);
        fs::write(root.join("studio-light-panes.png"), b"not-png").unwrap();
        let png_bytes = sample_png_bytes(1080, 640).len();
        let manifest = sample_manifest().replace(
            &format!(
                "studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes={png_bytes} width=1080 height=640 {SAMPLE_EVIDENCE}"
            ),
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=7 width=1080 height=640 {SAMPLE_EVIDENCE}"),
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(&root)).unwrap_err();
        assert!(error
            .to_string()
            .contains("capture file must be PNG for studio light panes"));
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_dimension_mismatch() {
        let temp = tempfile::tempdir().unwrap();
        let root = capture_root(&temp);
        write_sample_pngs(&root);
        let manifest = sample_manifest().replace(
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=24 width=1080 height=640 {SAMPLE_EVIDENCE}"),
            &format!("studio theme=light scenario=panes path=artifacts/ui/manual-acceptance/studio-light-panes.png bytes=24 width=1081 height=640 {SAMPLE_EVIDENCE}"),
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(&root)).unwrap_err();
        assert!(error.to_string().contains(
            "capture manifest dimensions mismatch for studio light panes: declared=1081x640 actual=1080x640"
        ));
    }

    #[test]
    fn ui_capture_manifest_with_root_rejects_too_small_png() {
        let temp = tempfile::tempdir().unwrap();
        let root = capture_root(&temp);
        write_sample_pngs(&root);
        let bytes = sample_png_bytes(500, 64);
        fs::write(root.join("launcher-light-collapsed.png"), bytes).unwrap();
        let manifest = sample_manifest().replace(
            &format!("launcher theme=light scenario=collapsed path=artifacts/ui/manual-acceptance/launcher-light-collapsed.png bytes=24 width=720 height=64 {SAMPLE_EVIDENCE}"),
            &format!("launcher theme=light scenario=collapsed path=artifacts/ui/manual-acceptance/launcher-light-collapsed.png bytes=24 width=500 height=64 {SAMPLE_EVIDENCE}"),
        );

        let error = verify_ui_capture_manifest_with_root(&manifest, Some(&root)).unwrap_err();
        assert!(error
            .to_string()
            .contains("capture PNG too small for launcher light collapsed"));
    }

    #[test]
    fn ui_capture_manifest_rejects_dominant_black_or_white_carrier_evidence() {
        let black = sample_manifest().replace(
            SAMPLE_EVIDENCE,
            "samples=9 opaque_samples=9 unique_colors=3 black_pixels=7 white_pixels=0 transparent_pixels=0",
        );
        let white = sample_manifest().replace(
            SAMPLE_EVIDENCE,
            "samples=9 opaque_samples=9 unique_colors=3 black_pixels=0 white_pixels=7 transparent_pixels=0",
        );

        assert!(verify_ui_capture_manifest_with_root(&black, None)
            .unwrap_err()
            .to_string()
            .contains("dominant black host background"));
        assert!(verify_ui_capture_manifest_with_root(&white, None)
            .unwrap_err()
            .to_string()
            .contains("dominant white host background"));
    }
}
