use crate::ops_evidence::{OpsEvidence, OpsStatus};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionAuditRow {
    pub area: &'static str,
    pub status: OpsStatus,
    pub evidence: String,
}

pub fn completion_audit_rows(evidence: &OpsEvidence) -> Vec<CompletionAuditRow> {
    let ui_detail = "requires current screenshots, focus, Enter, IME, light and dark evidence";
    vec![
        row(
            "UI Docs 18-24",
            OpsStatus::Manual,
            "docs define contract; runtime proof still required",
        ),
        row("Launcher", OpsStatus::Manual, ui_detail),
        row("Studio", OpsStatus::Manual, ui_detail),
        row("Core", evidence.doctor.status, &evidence.doctor.result),
        row(
            "Terminal",
            OpsStatus::Manual,
            "terminal automation remains manual completion evidence",
        ),
        row(
            "Plugin",
            evidence.doctor.status,
            "plugin command and manifest gates are source verified",
        ),
        row(
            "Index",
            evidence.doctor.status,
            "index coverage is part of doctor and release smoke",
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

pub fn completion_audit_contract() -> &'static str {
    "completion=area|status|evidence;ui_areas=manual_until_runtime_proof"
}

fn row(area: &'static str, status: OpsStatus, evidence: &str) -> CompletionAuditRow {
    CompletionAuditRow {
        area,
        status,
        evidence: evidence.to_string(),
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
        assert!(summary.contains("Quality:PASS"));
        assert!(completion_manual_areas(&rows).contains("UI Docs 18-24"));
        assert_eq!(rows.len(), 11);
    }
}
