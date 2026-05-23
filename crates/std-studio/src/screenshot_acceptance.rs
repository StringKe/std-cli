use std_egui::ui_capture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioScreenshotAcceptanceMatrix {
    pub(crate) delivery_states: Vec<String>,
    pub(crate) workflow_states: Vec<String>,
    pub(crate) diagnostic_states: Vec<String>,
    pub(crate) evidence_rule: &'static str,
    pub(crate) opt_in_rule: &'static str,
}

impl StudioScreenshotAcceptanceMatrix {
    pub(crate) fn for_scenarios(scenarios: &[String]) -> Self {
        Self {
            delivery_states: filter_labels(scenarios, delivery_state_labels()),
            workflow_states: filter_labels(scenarios, workflow_state_labels()),
            diagnostic_states: filter_labels(scenarios, diagnostic_state_labels()),
            evidence_rule:
                "docs22-delivery=theme-baseline+core-workbenches+operations+settings;theme-pairs=light|dark",
            opt_in_rule: "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless",
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.delivery_states == delivery_state_labels()
            && self.workflow_states == workflow_state_labels()
            && self.diagnostic_states == diagnostic_state_labels()
            && self.evidence_rule
                == "docs22-delivery=theme-baseline+core-workbenches+operations+settings;theme-pairs=light|dark"
            && self.opt_in_rule == "STD_ALLOW_UI_PREVIEW=1 only;default-smoke=headless"
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_screenshot_acceptance {}\ndelivery_capture_states={}\nworkflow_capture_states={}\ndiagnostic_capture_states={}\nevidence_rule={}\nopt_in_rule={}\ncapture_verify_rule={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.delivery_states.join(","),
            self.workflow_states.join(","),
            self.diagnostic_states.join(","),
            self.evidence_rule,
            self.opt_in_rule,
            ui_capture::UI_CAPTURE_VERIFY_RULE
        )
    }
}

fn filter_labels(labels: &[String], required: Vec<String>) -> Vec<String> {
    required
        .into_iter()
        .filter(|label| labels.iter().any(|actual| actual == label))
        .collect()
}

fn delivery_state_labels() -> Vec<String> {
    [
        "light-dashboard",
        "dark-dashboard",
        "light-analysis",
        "dark-analysis",
        "light-plugins",
        "dark-plugins",
        "light-operations",
        "dark-operations",
        "light-settings",
        "dark-settings",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn workflow_state_labels() -> Vec<String> {
    [
        "light-workflow",
        "dark-workflow",
        "light-workflow-error",
        "dark-workflow-error",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn diagnostic_state_labels() -> Vec<String> {
    [
        "light-plugin-permission",
        "dark-plugin-permission",
        "light-panes",
        "dark-panes",
    ]
    .into_iter()
    .map(str::to_string)
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
    }
}
