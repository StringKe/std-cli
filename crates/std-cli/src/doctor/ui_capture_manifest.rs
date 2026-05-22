use crate::{doctor::workspace::check_text, CliError};
use std::path::Path;
use std_egui::ui_capture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CaptureManifestHeader {
    pub(crate) created_at: String,
    pub(crate) out_dir: String,
}

pub(crate) fn verify_capture_manifest_header(
    body: &str,
    root: Option<&Path>,
) -> Result<CaptureManifestHeader, CliError> {
    check_text(body, "capture-ui-matrix manifest")?;
    check_text(body, "opt_in=STD_ALLOW_UI_PREVIEW=1")?;
    check_text(body, "test_mode=STD_TEST_MODE must not be 1")?;
    check_text(body, "capture_rule=pid+process-name+window-title")?;
    check_text(body, "completion_rule=current-run-png-only")?;

    let created_at = manifest_value(body, "created_at=")?;
    verify_created_at(created_at)?;
    let out_dir = manifest_value(body, "out_dir=")?;
    verify_out_dir(out_dir, root)?;

    Ok(CaptureManifestHeader {
        created_at: created_at.to_string(),
        out_dir: out_dir.to_string(),
    })
}

fn verify_created_at(value: &str) -> Result<(), CliError> {
    let valid = value.len() == 20
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value.as_bytes().get(10) == Some(&b'T')
        && value.as_bytes().get(13) == Some(&b':')
        && value.as_bytes().get(16) == Some(&b':')
        && value.ends_with('Z')
        && value
            .bytes()
            .enumerate()
            .filter(|(index, _)| !matches!(index, 4 | 7 | 10 | 13 | 16 | 19))
            .all(|(_, byte)| byte.is_ascii_digit());
    if valid {
        return Ok(());
    }
    Err(CliError::Doctor(format!(
        "capture manifest created_at must be UTC RFC3339 seconds: {value}"
    )))
}

fn verify_out_dir(value: &str, root: Option<&Path>) -> Result<(), CliError> {
    if value != ui_capture::UI_CAPTURE_DIR {
        return Err(CliError::Doctor(format!(
            "capture manifest out_dir must be {}: {value}",
            ui_capture::UI_CAPTURE_DIR
        )));
    }
    if let Some(root) = root {
        let manifest_dir = root.to_string_lossy();
        if !manifest_dir.ends_with(value) {
            return Err(CliError::Doctor(format!(
                "capture manifest path must live under declared out_dir: declared={value} actual={manifest_dir}"
            )));
        }
    }
    Ok(())
}

fn manifest_value<'a>(body: &'a str, key: &str) -> Result<&'a str, CliError> {
    body.lines()
        .find_map(|line| line.strip_prefix(key))
        .ok_or_else(|| CliError::Doctor(format!("capture manifest header missing: {key}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_manifest_header_accepts_current_run_contract() {
        let header_body = header();
        let header = verify_capture_manifest_header(&header_body, None).unwrap();

        assert_eq!(header.created_at, "2026-05-22T00:00:00Z");
        assert_eq!(header.out_dir, ui_capture::UI_CAPTURE_DIR);
    }

    #[test]
    fn capture_manifest_header_rejects_non_utc_timestamp() {
        let body = header().replace("2026-05-22T00:00:00Z", "2026-05-22 00:00:00");

        let error = verify_capture_manifest_header(&body, None).unwrap_err();

        assert!(error.to_string().contains("created_at must be UTC"));
    }

    #[test]
    fn capture_manifest_header_rejects_stale_output_dir() {
        let body = header().replace(ui_capture::UI_CAPTURE_DIR, "/tmp/old-ui-evidence");

        let error = verify_capture_manifest_header(&body, None).unwrap_err();

        assert!(error.to_string().contains("out_dir must be"));
    }

    fn header() -> String {
        format!(
            "capture-ui-matrix manifest\ncreated_at=2026-05-22T00:00:00Z\nout_dir={}\nopt_in=STD_ALLOW_UI_PREVIEW=1\ntest_mode=STD_TEST_MODE must not be 1\ncapture_rule=pid+process-name+window-title\ncompletion_rule=current-run-png-only\n",
            ui_capture::UI_CAPTURE_DIR
        )
    }
}
