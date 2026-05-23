use crate::ops_evidence::{OpsEvidence, OpsStatus};
use std_egui::ui_capture;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompletionAuditRow {
    pub area: &'static str,
    pub status: OpsStatus,
    pub evidence: String,
    pub manual_gates: Vec<String>,
}

pub fn completion_audit_rows(evidence: &OpsEvidence) -> Vec<CompletionAuditRow> {
    vec![
        manual_row(
            "UI Docs 18-24",
            "docs define contract; runtime proof still required",
            [
                "docs-18-24-requirement-audit",
                "light-dark-token-proof",
                "keyboard-focus-ime-proof",
                "a11y-localization-proof",
                "reduce-motion-proof",
                "egui-constraints-proof",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        ),
        manual_row(
            "Launcher",
            "requires current screenshots, focus, Enter, IME, light and dark evidence",
            launcher_manual_gates(),
        ),
        manual_row(
            "Studio",
            "requires current screenshots, pane lifecycle, keyboard focus, runtime evidence",
            studio_manual_gates(),
        ),
        row("Core", evidence.doctor.status, &evidence.doctor.result),
        row(
            "Terminal",
            OpsStatus::Manual,
            "terminal automation remains manual completion evidence",
        ),
        row("Plugin", evidence.plugin.status, &evidence.plugin.result),
        row("Index", evidence.index.status, &evidence.index.result),
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

fn launcher_manual_gates() -> Vec<String> {
    let mut gates = [
        "launcher-light-dark-screenshots",
        "launcher-results-no-results-defer-error-screenshots",
        "launcher-keyboard-navigation-ime",
        "launcher-installed-hotkey-toggle",
        "launcher-background-harness-enter",
        "launcher-search-open-app-enter",
        "launcher-close-hides-and-hotkey-restores",
        "launcher-external-runner-default-defer",
        "launcher-error-feedback-copy-retry-open-studio",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();
    gates.extend(launcher_capture_state_gates());
    gates.extend(ui_capture_manual_gates());
    gates
}

fn studio_manual_gates() -> Vec<String> {
    let mut gates = [
        "studio-light-dark-screenshots",
        "studio-workspace-pane-open-focus-close-restore",
        "studio-keyboard-a11y-focus",
        "studio-operations-runtime-evidence",
        "studio-installed-smoke",
        "studio-workflow-create-edit-simulate-run-trace",
        "studio-plugin-manager-manifest-runtime-permissions-audit",
        "studio-analysis-overview-components-symbols-relations-qa-coverage",
        "studio-qa-doctor-release-install-command-results",
    ]
    .into_iter()
    .map(str::to_string)
    .collect::<Vec<_>>();
    gates.extend(studio_capture_state_gates());
    gates.extend(ui_capture_manual_gates());
    gates
}

fn launcher_capture_state_gates() -> Vec<String> {
    [
        "launcher-capture-state=light-empty",
        "launcher-capture-state=dark-empty",
        "launcher-capture-state=light-results",
        "launcher-capture-state=dark-results",
        "launcher-capture-state=light-no-results",
        "launcher-capture-state=dark-no-results",
        "launcher-capture-state=light-defer",
        "launcher-capture-state=dark-defer",
        "launcher-capture-state=light-error",
        "launcher-capture-state=dark-error",
        "launcher-capture-state=light-ime",
        "launcher-capture-state=dark-ime",
        "launcher-capture-state=light-action-panel",
        "launcher-capture-state=dark-action-panel",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn studio_capture_state_gates() -> Vec<String> {
    [
        "studio-capture-state=light-dashboard",
        "studio-capture-state=dark-dashboard",
        "studio-capture-state=light-workflow",
        "studio-capture-state=dark-workflow",
        "studio-capture-state=light-workflow-error",
        "studio-capture-state=dark-workflow-error",
        "studio-capture-state=light-analysis",
        "studio-capture-state=dark-analysis",
        "studio-capture-state=light-plugins",
        "studio-capture-state=dark-plugins",
        "studio-capture-state=light-plugin-permission",
        "studio-capture-state=dark-plugin-permission",
        "studio-capture-state=light-operations",
        "studio-capture-state=dark-operations",
        "studio-capture-state=light-settings",
        "studio-capture-state=dark-settings",
        "studio-capture-state=light-panes",
        "studio-capture-state=dark-panes",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn ui_capture_manual_gates() -> Vec<String> {
    let mut gates = vec![
        format!("ui-capture-manifest={}", ui_capture::UI_CAPTURE_MANIFEST),
        format!("ui-capture-command={}", ui_capture::UI_CAPTURE_COMMAND),
        format!(
            "ui-capture-doctor=STD_UI_CAPTURE_MANIFEST={} std doctor",
            ui_capture::UI_CAPTURE_MANIFEST
        ),
        format!(
            "ui-capture-pixels={}",
            ui_capture::UI_CAPTURE_PIXEL_EVIDENCE_RULE
        ),
        format!(
            "ui-capture-rejects={}",
            ui_capture::UI_CAPTURE_CARRIER_REJECT_RULE
        ),
        format!(
            "ui-capture-acceptance={}",
            ui_capture::UI_CAPTURE_ACCEPTANCE_RULE
        ),
    ];
    gates.extend(background_ui_manual_gates());
    gates
}

fn background_ui_manual_gates() -> Vec<String> {
    [
        "background-ui-manifest=artifacts/ui/background-acceptance/manifest.txt",
        "background-ui-command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance",
        "background-ui-doctor=STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST=artifacts/ui/background-acceptance/manifest.txt std doctor",
        "background-ui-rule=isolated-harness-only+frontmost-preserved+doctor-validated",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
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
        .flat_map(|row| row.manual_gates.iter().map(String::as_str))
        .collect::<Vec<_>>()
        .join("|")
}

pub fn completion_audit_contract() -> &'static str {
    "completion=area|status|evidence|manual_gates;ui_areas=manual_until_runtime_proof;gates=release-build-package-verify|install-run-verify|launcher-hotkey-ime|studio-pane-workflow-plugin-index|quality"
}

fn row(area: &'static str, status: OpsStatus, evidence: &str) -> CompletionAuditRow {
    CompletionAuditRow {
        area,
        status,
        evidence: evidence.to_string(),
        manual_gates: Vec::new(),
    }
}

fn manual_row(area: &'static str, evidence: &str, manual_gates: Vec<String>) -> CompletionAuditRow {
    CompletionAuditRow {
        area,
        status: OpsStatus::Manual,
        evidence: evidence.to_string(),
        manual_gates,
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
        assert!(summary.contains("Plugin:PASS") || summary.contains("Plugin:MANUAL"));
        assert!(summary.contains("Index:PASS") || summary.contains("Index:MANUAL"));
        assert!(summary.contains("Quality:PASS") || summary.contains("Quality:MISSING"));
        assert!(completion_manual_areas(&rows).contains("UI Docs 18-24"));
        assert_manual_gate_groups(&rows);
        assert!(!completion_manual_areas(&rows).contains("Plugin"));
        assert!(!completion_manual_areas(&rows).contains("Index"));
        assert_eq!(rows.len(), 11);
    }

    fn assert_manual_gate_groups(rows: &[CompletionAuditRow]) {
        let gates = completion_manual_gates(rows);
        for required in [
            "launcher-background-harness-enter",
            "launcher-capture-state=light-results",
            "launcher-capture-state=dark-error",
            "launcher-search-open-app-enter",
            "launcher-close-hides-and-hotkey-restores",
            "launcher-external-runner-default-defer",
            "studio-keyboard-a11y-focus",
            "studio-installed-smoke",
            "studio-workflow-create-edit-simulate-run-trace",
            "studio-plugin-manager-manifest-runtime-permissions-audit",
            "studio-analysis-overview-components-symbols-relations-qa-coverage",
            "studio-capture-state=light-dashboard",
            "studio-capture-state=dark-panes",
            "ui-capture-manifest=",
            "ui-capture-command=",
            "ui-capture-doctor=",
            "ui-capture-pixels=",
            "ui-capture-rejects=",
            "ui-capture-acceptance=",
            "background-ui-manifest=artifacts/ui/background-acceptance/manifest.txt",
            "background-ui-command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance",
            "background-ui-doctor=STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST=artifacts/ui/background-acceptance/manifest.txt std doctor",
            "background-ui-rule=isolated-harness-only+frontmost-preserved+doctor-validated",
        ] {
            assert!(gates.contains(required), "missing manual gate {required}");
        }
    }
}
