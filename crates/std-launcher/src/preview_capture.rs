use crate::preview_contract::LauncherPreviewScenario;
use std_egui::ui_capture;

pub(crate) const LAUNCHER_CAPTURE_DIR: &str = ui_capture::UI_CAPTURE_DIR;
pub(crate) const LAUNCHER_CAPTURE_MANIFEST: &str = ui_capture::UI_CAPTURE_MANIFEST;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherCaptureManifest {
    pub(crate) out_dir: &'static str,
    pub(crate) manifest_path: &'static str,
    pub(crate) expected_files: Vec<String>,
    pub(crate) capture_command: &'static str,
    pub(crate) verify_rule: &'static str,
    pub(crate) source_rule: &'static str,
    pub(crate) pixel_evidence_rule: &'static str,
    pub(crate) carrier_reject_rule: &'static str,
}

impl LauncherCaptureManifest {
    pub(crate) fn for_scenarios(scenarios: &[LauncherPreviewScenario]) -> Self {
        Self {
            out_dir: LAUNCHER_CAPTURE_DIR,
            manifest_path: LAUNCHER_CAPTURE_MANIFEST,
            expected_files: scenarios.iter().map(capture_file_name).collect(),
            capture_command: ui_capture::UI_CAPTURE_COMMAND,
            verify_rule: ui_capture::UI_CAPTURE_VERIFY_RULE,
            source_rule: ui_capture::UI_CAPTURE_SOURCE_RULE,
            pixel_evidence_rule: ui_capture::UI_CAPTURE_PIXEL_EVIDENCE_RULE,
            carrier_reject_rule: ui_capture::UI_CAPTURE_CARRIER_REJECT_RULE,
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
            && self.capture_command == ui_capture::UI_CAPTURE_COMMAND
            && self.verify_rule == ui_capture::UI_CAPTURE_VERIFY_RULE
            && self.source_rule == ui_capture::UI_CAPTURE_SOURCE_RULE
            && self.pixel_evidence_rule == ui_capture::UI_CAPTURE_PIXEL_EVIDENCE_RULE
            && self.carrier_reject_rule == ui_capture::UI_CAPTURE_CARRIER_REJECT_RULE
    }

    pub(crate) fn summary(&self) -> String {
        ui_capture::capture_manifest_summary(&self.expected_files)
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
        assert!(manifest
            .summary()
            .contains("source_rule=pid+process-name+window-title-per-capture"));
        assert!(manifest
            .summary()
            .contains("samples+opaque_samples+unique_colors"));
        assert!(manifest.summary().contains("reject-single-color"));
    }
}
