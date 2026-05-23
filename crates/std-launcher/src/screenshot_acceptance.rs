use crate::preview_contract::LauncherPreviewScenario;
use std_egui::ui_capture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherScreenshotAcceptanceMatrix {
    pub(crate) delivery_states: Vec<String>,
    pub(crate) diagnostic_states: Vec<String>,
    pub(crate) evidence_rule: &'static str,
    pub(crate) opt_in_rule: &'static str,
    pub(crate) acceptance_rule: &'static str,
}

impl LauncherScreenshotAcceptanceMatrix {
    pub(crate) fn for_scenarios(scenarios: &[LauncherPreviewScenario]) -> Self {
        let labels = scenario_labels(scenarios);
        Self {
            delivery_states: filter_labels(&labels, delivery_state_labels()),
            diagnostic_states: filter_labels(&labels, diagnostic_state_labels()),
            evidence_rule:
                "docs21-delivery=theme-baseline+results+no-results+defer+error;theme-pairs=light|dark",
            opt_in_rule: "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless",
            acceptance_rule: ui_capture::UI_CAPTURE_ACCEPTANCE_RULE,
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.delivery_states == delivery_state_labels()
            && self.diagnostic_states == diagnostic_state_labels()
            && self.evidence_rule
                == "docs21-delivery=theme-baseline+results+no-results+defer+error;theme-pairs=light|dark"
            && self.opt_in_rule == "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless"
            && self.acceptance_rule == ui_capture::UI_CAPTURE_ACCEPTANCE_RULE
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "launcher_screenshot_acceptance {}\ndelivery_capture_states={}\ndiagnostic_capture_states={}\nevidence_rule={}\nopt_in_rule={}\ncapture_verify_rule={}\ncapture_source_rule={}\nacceptance_rule={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.delivery_states.join(","),
            self.diagnostic_states.join(","),
            self.evidence_rule,
            self.opt_in_rule,
            ui_capture::UI_CAPTURE_VERIFY_RULE,
            ui_capture::UI_CAPTURE_SOURCE_RULE,
            self.acceptance_rule
        )
    }
}

fn scenario_labels(scenarios: &[LauncherPreviewScenario]) -> Vec<String> {
    scenarios
        .iter()
        .map(LauncherPreviewScenario::label)
        .collect()
}

fn filter_labels(labels: &[String], required: Vec<String>) -> Vec<String> {
    required
        .into_iter()
        .filter(|label| labels.iter().any(|actual| actual == label))
        .collect()
}

fn delivery_state_labels() -> Vec<String> {
    [
        "light-empty",
        "dark-empty",
        "light-results",
        "dark-results",
        "light-no-results",
        "dark-no-results",
        "light-defer",
        "dark-defer",
        "light-error",
        "dark-error",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn diagnostic_state_labels() -> Vec<String> {
    [
        "light-collapsed",
        "dark-collapsed",
        "light-searching",
        "dark-searching",
        "light-loading",
        "dark-loading",
        "light-executing",
        "dark-executing",
        "light-ime",
        "dark-ime",
        "light-action-panel",
        "dark-action-panel",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preview_contract::preview_matrix;

    #[test]
    fn screenshot_acceptance_matrix_separates_delivery_from_diagnostics() {
        let matrix = LauncherScreenshotAcceptanceMatrix::for_scenarios(&preview_matrix());
        let summary = matrix.summary();

        assert!(matrix.pass(), "{summary}");
        assert!(summary.contains("launcher_screenshot_acceptance PASS"));
        assert!(summary.contains("delivery_capture_states=light-empty,dark-empty"));
        assert!(summary.contains("light-results,dark-results"));
        assert!(summary.contains("light-no-results,dark-no-results"));
        assert!(summary.contains("light-defer,dark-defer"));
        assert!(summary.contains("light-error,dark-error"));
        assert!(summary.contains("diagnostic_capture_states=light-collapsed,dark-collapsed"));
        assert!(summary.contains("light-ime,dark-ime"));
        assert!(summary.contains("STD_ALLOW_UI_PREVIEW=1 only"));
        assert!(summary.contains(ui_capture::UI_CAPTURE_SOURCE_RULE));
        assert!(summary.contains(ui_capture::UI_CAPTURE_ACCEPTANCE_RULE));
    }
}
