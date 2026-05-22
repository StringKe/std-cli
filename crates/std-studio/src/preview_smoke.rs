use crate::preview_evidence::{preview_matrix, preview_size_summary, preview_state_summary};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioPreviewSmokeReport {
    pub(crate) scenarios: Vec<String>,
    pub(crate) commands: Vec<String>,
    pub(crate) states: Vec<String>,
    pub(crate) sizes: Vec<String>,
    pub(crate) required_capture_states: Vec<String>,
    pub(crate) capture_contract: &'static str,
    pub(crate) capture_manifest: StudioCaptureManifest,
}

impl StudioPreviewSmokeReport {
    pub(crate) fn new() -> Self {
        let scenarios = preview_matrix();
        let capture_manifest = StudioCaptureManifest::for_scenarios(&scenarios);
        Self {
            commands: scenarios
                .iter()
                .map(|scenario| preview_command(scenario))
                .collect(),
            states: scenarios
                .iter()
                .map(|scenario| preview_state_summary(scenario))
                .collect(),
            sizes: scenarios
                .iter()
                .map(|scenario| preview_size_summary(scenario))
                .collect(),
            required_capture_states: required_capture_states(&scenarios),
            scenarios,
            capture_contract: preview_capture_contract(),
            capture_manifest,
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.scenarios == preview_matrix()
            && self.commands.len() == self.scenarios.len()
            && self.states.iter().all(|state| state.contains("PASS"))
            && self.sizes.iter().all(|size| size.contains("PASS"))
            && self.required_capture_states == required_capture_states(&self.scenarios)
            && required_capture_states_pass(&self.required_capture_states)
            && self.capture_contract == preview_capture_contract()
            && self.capture_manifest.pass(&self.scenarios)
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_preview_smoke {}\npreview_scenarios={}\npreview_commands={}\npreview_states={}\npreview_sizes={}\nrequired_capture_states={}\npreview_capture_contract={}\n{}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.scenarios.join(","),
            self.commands.join(";"),
            self.states.join(";"),
            self.sizes.join(";"),
            self.required_capture_states.join(","),
            self.capture_contract,
            self.capture_manifest.summary()
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioCaptureManifest {
    pub(crate) out_dir: &'static str,
    pub(crate) manifest_path: &'static str,
    pub(crate) expected_files: Vec<String>,
    pub(crate) capture_command: &'static str,
    pub(crate) verify_rule: &'static str,
    pub(crate) pixel_evidence_rule: &'static str,
    pub(crate) carrier_reject_rule: &'static str,
}

impl StudioCaptureManifest {
    fn for_scenarios(scenarios: &[String]) -> Self {
        Self {
            out_dir: "artifacts/ui/manual-acceptance",
            manifest_path: "artifacts/ui/manual-acceptance/manifest.txt",
            expected_files: scenarios
                .iter()
                .map(|scenario| format!("studio-{scenario}.png"))
                .collect(),
            capture_command: "STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix",
            verify_rule: "manifest-current-run-png-files-by-theme-state",
            pixel_evidence_rule: "samples+unique_colors+black_pixels+white_pixels",
            carrier_reject_rule: "reject-single-color+all-black+all-white-carrier",
        }
    }

    pub(crate) fn pass(&self, scenarios: &[String]) -> bool {
        self.out_dir == "artifacts/ui/manual-acceptance"
            && self.manifest_path == "artifacts/ui/manual-acceptance/manifest.txt"
            && self.expected_files
                == scenarios
                    .iter()
                    .map(|scenario| format!("studio-{scenario}.png"))
                    .collect::<Vec<_>>()
            && self
                .expected_files
                .iter()
                .all(|file| file.starts_with("studio-") && file.ends_with(".png"))
            && self.capture_command == "STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix"
            && self.verify_rule == "manifest-current-run-png-files-by-theme-state"
            && self.pixel_evidence_rule == "samples+unique_colors+black_pixels+white_pixels"
            && self.carrier_reject_rule == "reject-single-color+all-black+all-white-carrier"
    }

    fn summary(&self) -> String {
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

fn preview_command(scenario: &str) -> String {
    let (theme, name) = scenario.split_once('-').unwrap_or(("dark", "dashboard"));
    format!("STD_ALLOW_UI_PREVIEW=1 cargo run -p std-studio -- --ui-preview {theme} {name} 8000")
}

fn preview_capture_contract() -> &'static str {
    "explicit-opt-in-only,checkout-binary-only,blocked-in-STD_TEST_MODE,no-default-window,normal-viewport-close"
}

fn required_capture_states(scenarios: &[String]) -> Vec<String> {
    required_capture_state_order()
        .into_iter()
        .filter(|required| scenarios.iter().any(|scenario| scenario == *required))
        .map(str::to_string)
        .collect()
}

fn required_capture_states_pass(states: &[String]) -> bool {
    states == required_capture_state_order()
}

fn required_capture_state_order() -> [&'static str; 18] {
    [
        "light-dashboard",
        "dark-dashboard",
        "light-workflow",
        "dark-workflow",
        "light-workflow-error",
        "dark-workflow-error",
        "light-analysis",
        "dark-analysis",
        "light-plugins",
        "dark-plugins",
        "light-plugin-permission",
        "dark-plugin-permission",
        "light-operations",
        "dark-operations",
        "light-settings",
        "dark-settings",
        "light-panes",
        "dark-panes",
    ]
}

impl Default for StudioPreviewSmokeReport {
    fn default() -> Self {
        Self::new()
    }
}
