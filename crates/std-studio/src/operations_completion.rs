use crate::ops_evidence::{OpsEvidence, OpsStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionAuditRow {
    pub area: &'static str,
    pub status: OpsStatus,
    pub evidence: String,
    pub manual_gates: Vec<&'static str>,
}

pub fn completion_audit_rows(evidence: &OpsEvidence) -> Vec<CompletionAuditRow> {
    vec![
        manual_row(
            "UI Docs 18-24",
            "docs define contract; runtime proof still required",
            &[
                "docs-18-24-requirement-audit",
                "light-dark-token-proof",
                "keyboard-focus-ime-proof",
                "a11y-localization-proof",
            ],
        ),
        manual_row(
            "Launcher",
            "requires current screenshots, focus, Enter, IME, light and dark evidence",
            &[
                "launcher-light-dark-screenshots",
                "launcher-results-no-results-defer-error-screenshots",
                "launcher-keyboard-navigation-ime",
                "launcher-installed-hotkey-toggle",
                "launcher-background-harness-enter",
            ],
        ),
        manual_row(
            "Studio",
            "requires current screenshots, pane lifecycle, keyboard focus, runtime evidence",
            &[
                "studio-light-dark-screenshots",
                "studio-workspace-pane-open-focus-close-restore",
                "studio-keyboard-a11y-focus",
                "studio-operations-runtime-evidence",
            ],
        ),
        row("Core", evidence.doctor.status, &evidence.doctor.result),
        row(
            "Terminal",
            OpsStatus::Manual,
            "terminal automation remains manual completion evidence",
        ),
        manual_row(
            "Plugin",
            "requires binary JS and TS plugin runtime proof",
            &[
                "plugin-js-binary-runtime",
                "plugin-ts-binary-runtime",
                "plugin-permission-boundary-runtime",
            ],
        ),
        manual_row(
            "Index",
            "requires four-layer index coverage proof",
            &[
                "index-overview-coverage",
                "index-components-coverage",
                "index-symbols-relations-coverage",
                "index-qa-coverage",
            ],
        ),
        row(
            "Workflow",
            evidence.doctor.status,
            "workflow trace is part of release smoke",
        ),
        row("Release", evidence.release.status, &evidence.release.result),
        row("Install", evidence.install.status, &evidence.install.result),
        row("Quality", evidence.qa.status, &evidence.qa.result),
    ]
}

pub fn completion_audit_summary(rows: &[CompletionAuditRow]) -> String {
    rows.iter()
        .map(|row| format!("{}:{}", row.area, row.status.label()))
        .collect::<Vec<_>>()
        .join("|")
}

pub fn completion_manual_areas(rows: &[CompletionAuditRow]) -> String {
    rows.iter()
        .filter(|row| row.status == OpsStatus::Manual)
        .map(|row| row.area)
        .collect::<Vec<_>>()
        .join("|")
}

pub fn completion_manual_gates(rows: &[CompletionAuditRow]) -> String {
    rows.iter()
        .filter(|row| row.status == OpsStatus::Manual)
        .flat_map(|row| row.manual_gates.iter().copied())
        .collect::<Vec<_>>()
        .join("|")
}

pub fn completion_audit_contract() -> &'static str {
    "completion=area|status|evidence|manual_gates;ui_areas=manual_until_runtime_proof"
}

fn row(area: &'static str, status: OpsStatus, evidence: &str) -> CompletionAuditRow {
    CompletionAuditRow {
        area,
        status,
        evidence: evidence.to_string(),
        manual_gates: Vec::new(),
    }
}

fn manual_row(
    area: &'static str,
    evidence: &str,
    manual_gates: &[&'static str],
) -> CompletionAuditRow {
    CompletionAuditRow {
        area,
        status: OpsStatus::Manual,
        evidence: evidence.to_string(),
        manual_gates: manual_gates.to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn completion_audit_keeps_ui_manual_until_runtime_proof() {
        let evidence = OpsEvidence::load();
        let rows = completion_audit_rows(&evidence);
        let summary = completion_audit_summary(&rows);

        assert!(summary.contains("Launcher:MANUAL"));
        assert!(summary.contains("Studio:MANUAL"));
        assert!(summary.contains("Plugin:MANUAL"));
        assert!(summary.contains("Index:MANUAL"));
        assert!(summary.contains("Quality:PASS") || summary.contains("Quality:MISSING"));
        assert!(completion_manual_areas(&rows).contains("UI Docs 18-24"));
        assert!(completion_manual_gates(&rows).contains("launcher-background-harness-enter"));
        assert!(completion_manual_gates(&rows).contains("studio-keyboard-a11y-focus"));
        assert!(completion_manual_gates(&rows).contains("plugin-js-binary-runtime"));
        assert!(completion_manual_gates(&rows).contains("index-qa-coverage"));
        assert_eq!(rows.len(), 11);
    }
}
