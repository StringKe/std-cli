pub(crate) const UI_COMPLETION_BOUNDARY: &str = "headless-preview-is-not-ui-completion";

pub(crate) const MANUAL_UI_EVIDENCE_GATES: &[&str] = &[
    "launcher-light-dark-screenshots",
    "launcher-results-no-results-defer-error-screenshots",
    "launcher-keyboard-navigation-ime",
    "launcher-installed-hotkey-toggle",
];

pub(crate) fn launcher_ui_completion_boundary_summary() -> String {
    format!(
        "ui_completion_boundary={};manual_ui_evidence_gates={}",
        UI_COMPLETION_BOUNDARY,
        MANUAL_UI_EVIDENCE_GATES.join("|")
    )
}

pub(crate) fn launcher_ui_completion_boundary_passes(summary: &str) -> bool {
    summary.contains("ui_completion_boundary=headless-preview-is-not-ui-completion")
        && MANUAL_UI_EVIDENCE_GATES
            .iter()
            .all(|gate| summary.contains(gate))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_completion_boundary_requires_manual_ui_evidence() {
        let summary = launcher_ui_completion_boundary_summary();

        assert!(launcher_ui_completion_boundary_passes(&summary));
        assert!(summary.contains("launcher-light-dark-screenshots"));
        assert!(summary.contains("launcher-results-no-results-defer-error-screenshots"));
        assert!(summary.contains("launcher-keyboard-navigation-ime"));
        assert!(summary.contains("launcher-installed-hotkey-toggle"));
    }
}
