use crate::preview_smoke::required_capture_state_order;
use std_egui::ui_capture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioScreenshotAcceptanceMatrix {
    pub(crate) required_states: Vec<String>,
    pub(crate) delivery_states: Vec<String>,
    pub(crate) workflow_states: Vec<String>,
    pub(crate) diagnostic_states: Vec<String>,
    pub(crate) evidence_rule: &'static str,
    pub(crate) opt_in_rule: &'static str,
    pub(crate) acceptance_rule: &'static str,
}

impl StudioScreenshotAcceptanceMatrix {
    pub(crate) fn for_scenarios(scenarios: &[String]) -> Self {
        let required_states = filter_labels(scenarios, &required_capture_state_order());
        Self {
            delivery_states: delivery_states_from_required(&required_states),
            workflow_states: workflow_states_from_required(&required_states),
            diagnostic_states: diagnostic_states_from_required(&required_states),
            required_states,
            evidence_rule:
                "docs22-delivery=theme-baseline+core-workbenches+memory+history+operations+settings;theme-pairs=light|dark",
            opt_in_rule: "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless",
            acceptance_rule: ui_capture::UI_CAPTURE_ACCEPTANCE_RULE,
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.required_states == state_labels(&required_capture_state_order())
            && self.delivery_states == state_labels(delivery_state_labels())
            && self.workflow_states == state_labels(workflow_state_labels())
            && self.diagnostic_states == diagnostic_state_labels()
            && self.evidence_rule
            == "docs22-delivery=theme-baseline+core-workbenches+memory+history+operations+settings;theme-pairs=light|dark"
            && self.opt_in_rule == "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless"
            && self.acceptance_rule == ui_capture::UI_CAPTURE_ACCEPTANCE_RULE
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_screenshot_acceptance {}\nrequired_capture_states={}\ndelivery_capture_states={}\nworkflow_capture_states={}\ndiagnostic_capture_states={}\nevidence_rule={}\nopt_in_rule={}\ncapture_verify_rule={}\ncapture_source_rule={}\nacceptance_rule={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.required_states.join(","),
            self.delivery_states.join(","),
            self.workflow_states.join(","),
            self.diagnostic_states.join(","),
            self.evidence_rule,
            self.opt_in_rule,
            ui_capture::UI_CAPTURE_VERIFY_RULE,
            ui_capture::UI_CAPTURE_SOURCE_RULE,
            self.acceptance_rule
        )
    }
}

fn filter_labels(labels: &[String], required: &[&str]) -> Vec<String> {
    required
        .iter()
        .filter(|label| labels.iter().any(|actual| actual == *label))
        .map(|label| (*label).to_string())
        .collect()
}

fn state_labels(labels: &[&str]) -> Vec<String> {
    labels.iter().map(|label| (*label).to_string()).collect()
}

fn delivery_state_labels() -> &'static [&'static str] {
    &[
        "light-dashboard",
        "dark-dashboard",
        "light-analysis",
        "dark-analysis",
        "light-plugins",
        "dark-plugins",
        "light-operations",
        "dark-operations",
        "light-memory",
        "dark-memory",
        "light-history",
        "dark-history",
        "light-settings",
        "dark-settings",
    ]
}

fn workflow_state_labels() -> &'static [&'static str] {
    &[
        "light-workflow",
        "dark-workflow",
        "light-workflow-error",
        "dark-workflow-error",
    ]
}

fn diagnostic_state_labels() -> Vec<String> {
    diagnostic_states_from_required(&state_labels(&required_capture_state_order()))
}

fn delivery_states_from_required(required: &[String]) -> Vec<String> {
    delivery_state_labels()
        .iter()
        .filter(|label| required.iter().any(|state| state == **label))
        .map(|label| (*label).to_string())
        .collect()
}

fn workflow_states_from_required(required: &[String]) -> Vec<String> {
    workflow_state_labels()
        .iter()
        .filter(|label| required.iter().any(|state| state == **label))
        .map(|label| (*label).to_string())
        .collect()
}

fn diagnostic_states_from_required(required: &[String]) -> Vec<String> {
    required
        .iter()
        .filter(|label| {
            !delivery_state_labels().contains(&label.as_str())
                && !workflow_state_labels().contains(&label.as_str())
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preview_evidence::preview_matrix;

    #[test]
    fn screenshot_acceptance_matrix_separates_studio_delivery_groups() {
        let matrix = StudioScreenshotAcceptanceMatrix::for_scenarios(&preview_matrix());
        let summary = matrix.summary();

        assert!(matrix.pass(), "{summary}");
        assert!(summary.contains("studio_screenshot_acceptance PASS"));
        assert!(summary.contains(
            "required_capture_states=light-dashboard,dark-dashboard,light-workflow,dark-workflow"
        ));
        assert!(summary.contains("delivery_capture_states=light-dashboard,dark-dashboard"));
        assert!(summary.contains("light-analysis,dark-analysis"));
        assert!(summary.contains("light-plugins,dark-plugins"));
        assert!(summary.contains("light-operations,dark-operations"));
        assert!(summary.contains("light-settings,dark-settings"));
        assert!(summary.contains("workflow_capture_states=light-workflow,dark-workflow"));
        assert!(summary.contains("light-workflow-error,dark-workflow-error"));
        assert!(summary.contains("diagnostic_capture_states=light-plugin-permission"));
        assert!(summary.contains("light-panes,dark-panes"));
        assert!(summary.contains("STD_ALLOW_UI_PREVIEW=1 only"));
        assert!(summary.contains(ui_capture::UI_CAPTURE_SOURCE_RULE));
        assert!(summary.contains(ui_capture::UI_CAPTURE_ACCEPTANCE_RULE));
        assert_eq!(
            matrix.required_states,
            state_labels(&required_capture_state_order())
        );
    }
}
