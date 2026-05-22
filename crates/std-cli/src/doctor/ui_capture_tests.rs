use super::ui_capture_scripts::{LAUNCHER_CAPTURE_STATES, STUDIO_CAPTURE_STATES};
use std::{fs, path::Path};
use std_egui::ui_capture;

pub(crate) const SAMPLE_EVIDENCE: &str =
    "samples=9 opaque_samples=9 unique_colors=3 black_pixels=0 white_pixels=0 transparent_pixels=0";

pub(crate) fn sample_manifest() -> String {
    let launcher_bytes = sample_png_bytes(720, 64).len();
    let studio_bytes = sample_png_bytes(1080, 640).len();
    let mut lines = vec![
        "capture-ui-matrix manifest".to_string(),
        "created_at=2026-05-22T00:00:00Z".to_string(),
        format!("out_dir={}", ui_capture::UI_CAPTURE_DIR),
        "opt_in=STD_ALLOW_UI_PREVIEW=1".to_string(),
        "test_mode=STD_TEST_MODE must not be 1".to_string(),
        "capture_rule=pid+process-name+window-title".to_string(),
        "completion_rule=current-run-png-only".to_string(),
    ];
    for (theme, scenario) in LAUNCHER_CAPTURE_STATES {
        lines.push(format!(
            "launcher theme={theme} scenario={scenario} path={}/launcher-{theme}-{scenario}.png bytes={launcher_bytes} width=720 height=64 {SAMPLE_EVIDENCE}",
            ui_capture::UI_CAPTURE_DIR
        ));
    }
    for (theme, scenario) in STUDIO_CAPTURE_STATES {
        lines.push(format!(
            "studio theme={theme} scenario={scenario} path={}/studio-{theme}-{scenario}.png bytes={studio_bytes} width=1080 height=640 {SAMPLE_EVIDENCE}",
            ui_capture::UI_CAPTURE_DIR
        ));
    }
    lines.join("\n")
}

pub(crate) fn write_sample_pngs(root: &Path) {
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

pub(crate) fn capture_root(temp: &tempfile::TempDir) -> std::path::PathBuf {
    let root = temp.path().join(ui_capture::UI_CAPTURE_DIR);
    fs::create_dir_all(&root).unwrap();
    root
}

pub(crate) fn sample_png_bytes(width: u32, height: u32) -> Vec<u8> {
    let mut bytes = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR".to_vec();
    bytes.extend(width.to_be_bytes());
    bytes.extend(height.to_be_bytes());
    bytes
}
