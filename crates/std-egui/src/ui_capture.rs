pub const UI_CAPTURE_DIR: &str = "artifacts/ui/manual-acceptance";
pub const UI_CAPTURE_MANIFEST: &str = "artifacts/ui/manual-acceptance/manifest.txt";
pub const UI_CAPTURE_COMMAND: &str = "STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix";
pub const UI_CAPTURE_VERIFY_RULE: &str = "manifest-current-run-png-files-by-theme-state";
pub const UI_CAPTURE_PIXEL_EVIDENCE_RULE: &str =
    "samples+opaque_samples+unique_colors+black_pixels+white_pixels+transparent_pixels";
pub const UI_CAPTURE_CARRIER_REJECT_RULE: &str =
    "reject-single-color+dominant-black+dominant-white-carrier";

pub fn capture_manifest_summary(expected_files: &[String]) -> String {
    format!(
        "expected_capture_manifest={},capture_out_dir={},expected_capture_files={},capture_command={},verify_rule={},pixel_evidence_rule={},carrier_reject_rule={}",
        UI_CAPTURE_MANIFEST,
        UI_CAPTURE_DIR,
        expected_files.join(","),
        UI_CAPTURE_COMMAND,
        UI_CAPTURE_VERIFY_RULE,
        UI_CAPTURE_PIXEL_EVIDENCE_RULE,
        UI_CAPTURE_CARRIER_REJECT_RULE
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_capture_contract_matches_manual_acceptance_gate() {
        let files = vec![
            "launcher-light-results.png".to_string(),
            "studio-dark-dashboard.png".to_string(),
        ];
        let summary = capture_manifest_summary(&files);

        assert_eq!(UI_CAPTURE_DIR, "artifacts/ui/manual-acceptance");
        assert_eq!(
            UI_CAPTURE_MANIFEST,
            "artifacts/ui/manual-acceptance/manifest.txt"
        );
        assert_eq!(
            UI_CAPTURE_COMMAND,
            "STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix"
        );
        assert!(summary.contains("expected_capture_files=launcher-light-results.png"));
        assert!(summary.contains(UI_CAPTURE_PIXEL_EVIDENCE_RULE));
        assert!(summary.contains(UI_CAPTURE_CARRIER_REJECT_RULE));
    }
}
