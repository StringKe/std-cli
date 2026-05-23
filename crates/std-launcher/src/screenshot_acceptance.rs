use crate::preview_contract::{required_capture_state_labels, LauncherPreviewScenario};
use std_egui::ui_capture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherScreenshotAcceptanceMatrix {
    pub(crate) required_states: Vec<String>,
    pub(crate) delivery_states: Vec<String>,
    pub(crate) diagnostic_states: Vec<String>,
    pub(crate) evidence_rule: &'static str,
    pub(crate) opt_in_rule: &'static str,
    pub(crate) acceptance_rule: &'static str,
}

impl LauncherScreenshotAcceptanceMatrix {
    pub(crate) fn for_scenarios(scenarios: &[LauncherPreviewScenario]) -> Self {
        let labels = scenario_labels(scenarios);
        let required_states = filter_labels(&labels, required_capture_state_labels());
        Self {
            delivery_states: delivery_states_from_required(&required_states),
            diagnostic_states: diagnostic_states_from_required(&required_states),
            required_states,
            evidence_rule:
                "docs21-delivery=theme-baseline+results+no-results+defer+error;theme-pairs=light|dark",
            opt_in_rule: "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless",
            acceptance_rule: ui_capture::UI_CAPTURE_ACCEPTANCE_RULE,
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.required_states == state_labels(required_capture_state_labels())
            && self.delivery_states == state_labels(delivery_state_labels())
            && self.diagnostic_states == diagnostic_state_labels()
            && self.evidence_rule
                == "docs21-delivery=theme-baseline+results+no-results+defer+error;theme-pairs=light|dark"
            && self.opt_in_rule == "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless"
            && self.acceptance_rule == ui_capture::UI_CAPTURE_ACCEPTANCE_RULE
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "launcher_screenshot_acceptance {}\nrequired_capture_states={}\ndelivery_capture_states={}\ndiagnostic_capture_states={}\nevidence_rule={}\nopt_in_rule={}\ncapture_verify_rule={}\ncapture_source_rule={}\nacceptance_rule={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.required_states.join(","),
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

fn state_labels(labels: &[&str]) -> Vec<String> {
    labels.iter().map(|label| (*label).to_string()).collect()
}

fn delivery_state_labels() -> &'static [&'static str] {
    &[
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
}

fn diagnostic_state_labels() -> Vec<String> {
    diagnostic_states_from_required(&state_labels(required_capture_state_labels()))
}

fn delivery_states_from_required(required: &[String]) -> Vec<String> {
    delivery_state_labels()
        .iter()
        .filter(|label| required.iter().any(|state| state == **label))
        .map(|label| (*label).to_string())
        .collect()
}

fn diagnostic_states_from_required(required: &[String]) -> Vec<String> {
    required
        .iter()
        .filter(|label| !delivery_state_labels().contains(&label.as_str()))
        .cloned()
        .collect()
}

fn filter_labels(labels: &[String], required: &[&str]) -> Vec<String> {
    required
        .iter()
        .filter(|label| labels.iter().any(|actual| actual == **label))
        .map(|label| (*label).to_string())
        .collect()
}

#[cfg(test)]
fn legacy_diagnostic_state_labels() -> Vec<String> {
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
        assert!(summary.contains(
            "required_capture_states=light-collapsed,dark-collapsed,light-empty,dark-empty"
        ));
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
        assert_eq!(matrix.diagnostic_states, legacy_diagnostic_state_labels());
        assert_eq!(
            matrix.required_states,
            state_labels(required_capture_state_labels())
        );
    }
}
