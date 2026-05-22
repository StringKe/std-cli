use crate::preview::LauncherPreviewScenario;

pub(crate) const LAUNCHER_CAPTURE_DIR: &str = "artifacts/ui/manual-acceptance";
pub(crate) const LAUNCHER_CAPTURE_MANIFEST: &str = "artifacts/ui/manual-acceptance/manifest.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherCaptureManifest {
    pub(crate) out_dir: &'static str,
    pub(crate) manifest_path: &'static str,
    pub(crate) expected_files: Vec<String>,
    pub(crate) capture_command: &'static str,
    pub(crate) verify_rule: &'static str,
    pub(crate) pixel_evidence_rule: &'static str,
    pub(crate) carrier_reject_rule: &'static str,
}

impl LauncherCaptureManifest {
    pub(crate) fn for_scenarios(scenarios: &[LauncherPreviewScenario]) -> Self {
        Self {
            out_dir: LAUNCHER_CAPTURE_DIR,
            manifest_path: LAUNCHER_CAPTURE_MANIFEST,
            expected_files: scenarios.iter().map(capture_file_name).collect(),
            capture_command: "STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix",
            verify_rule: "manifest-current-run-png-files-by-theme-state",
            pixel_evidence_rule: "samples+unique_colors+black_pixels+white_pixels",
            carrier_reject_rule: "reject-single-color+dominant-black+dominant-white-carrier",
        }
    }

    pub(crate) fn pass(&self, scenarios: &[LauncherPreviewScenario]) -> bool {
        self.out_dir == LAUNCHER_CAPTURE_DIR
            && self.manifest_path == LAUNCHER_CAPTURE_MANIFEST
            && self.expected_files == scenarios.iter().map(capture_file_name).collect::<Vec<_>>()
            && self
                .expected_files
                .iter()
                .all(|file| file.starts_with("launcher-") && file.ends_with(".png"))
            && self.capture_command == "STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix"
            && self.verify_rule == "manifest-current-run-png-files-by-theme-state"
            && self.pixel_evidence_rule == "samples+unique_colors+black_pixels+white_pixels"
            && self.carrier_reject_rule
                == "reject-single-color+dominant-black+dominant-white-carrier"
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "expected_capture_manifest={},capture_out_dir={},expected_capture_files={},capture_command={},verify_rule={},pixel_evidence_rule={},carrier_reject_rule={}",
            self.manifest_path,
            self.out_dir,
            self.expected_files.join(","),
            self.capture_command,
            self.verify_rule,
            self.pixel_evidence_rule,
            self.carrier_reject_rule
        )
    }
}

fn capture_file_name(scenario: &LauncherPreviewScenario) -> String {
    format!("launcher-{}-{}.png", scenario.theme, scenario.state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_capture_manifest_maps_each_preview_state_to_png() {
        let scenarios = [
            LauncherPreviewScenario {
                theme: "light",
                state: "results",
            },
            LauncherPreviewScenario {
                theme: "dark",
                state: "defer",
            },
        ];
        let manifest = LauncherCaptureManifest::for_scenarios(&scenarios);

        assert!(manifest.pass(&scenarios));
        assert_eq!(
            manifest.expected_files,
            ["launcher-light-results.png", "launcher-dark-defer.png"]
        );
        assert!(manifest.summary().contains("expected_capture_manifest="));
        assert!(manifest.summary().contains("STD_ALLOW_UI_PREVIEW=1"));
        assert!(manifest.summary().contains("samples+unique_colors"));
        assert!(manifest.summary().contains("reject-single-color"));
    }
}
