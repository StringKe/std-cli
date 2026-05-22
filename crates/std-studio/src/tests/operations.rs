use super::*;

#[test]
fn studio_operations_evidence_reports_current_quality_release_and_install_state() {
    let evidence = OpsEvidence::load();
    let lines = evidence.lines();

    assert_gate_statuses(&evidence);
    assert_line_contract(&lines);
    assert_gate_outputs(&evidence);
    assert_gate_steps(&evidence);
    assert_completion_audit_rows(&evidence);
}

fn assert_gate_statuses(evidence: &OpsEvidence) {
    assert_eq!(evidence.qa.status, OpsStatus::Pass);
    assert_eq!(evidence.doctor.status, OpsStatus::Pass);
    assert!(matches!(
        evidence.release.status,
        OpsStatus::Pass | OpsStatus::Missing
    ));
    assert!(matches!(
        evidence.install.status,
        OpsStatus::Pass | OpsStatus::Missing
    ));
    assert_eq!(evidence.runtime.status, OpsStatus::Manual);
}

fn assert_line_contract(lines: &[String]) {
    assert!(lines.iter().any(|line| line.contains("qa=PASS")));
    assert!(lines.iter().any(|line| line.contains("result=")));
    assert!(lines.iter().any(|line| line.contains("runbook=")));
    assert!(lines.iter().any(|line| line.contains("std doctor")));
    assert!(lines.iter().any(|line| line.contains("release verify")));
    assert!(lines.iter().any(|line| line.contains("install verify")));
    assert!(lines.iter().any(|line| line.contains("artifact=")));
    assert!(lines.iter().any(|line| line.contains("output=")));
}

fn assert_gate_outputs(evidence: &OpsEvidence) {
    assert!(!evidence.qa.result.is_empty());
    assert!(evidence.qa.output.contains("rustfmt=PASS"));
    assert_runbook_contains(
        &evidence.qa.runbook,
        &[
            "mise run fmt",
            "mise run clippy",
            "mise run dylint",
            "mise run dylint-test",
            "mise run file-limits",
            "mise run test",
            "mise run deny",
            "mise run machete",
            "mise run quality",
        ],
    );
    assert!(evidence.doctor.result.contains("doctor source gates"));
    assert!(evidence.doctor.output.contains("quality=PASS"));
    assert_runbook_contains(
        &evidence.doctor.runbook,
        &["std doctor", "std release plan", "std install plan"],
    );
    assert!(!evidence.release.result.is_empty());
    assert!(evidence.release.result.contains("release verify"));
    assert!(evidence.release.output.contains("manifest="));
    assert_runbook_contains(
        &evidence.release.runbook,
        &[
            "cargo build --release --workspace",
            "std release package",
            "std release verify",
        ],
    );
    assert!(!evidence.install.result.is_empty());
    assert!(evidence.install.result.contains("install verify"));
    assert!(evidence.install.output.contains("launcher="));
    assert_runbook_contains(
        &evidence.install.runbook,
        &["std install run", "std install verify"],
    );
    assert_eq!(
        evidence.runtime.command,
        "mise run ui-background-acceptance"
    );
    assert!(evidence
        .runtime
        .result
        .contains("manual background UI opt-in required"));
    assert!(evidence.runtime.output.contains("SKIP"));
    assert_runbook_contains(
        &evidence.runtime.runbook,
        &[
            "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1",
            "mise run ui-background-acceptance",
            "STD_ALLOW_DESKTOP_AUTOMATION=1",
            "STD_ALLOW_UI_PREVIEW=1",
        ],
    );
}

fn assert_gate_steps(evidence: &OpsEvidence) {
    assert!(evidence
        .qa
        .steps
        .iter()
        .any(|step| step.name == "quality" && step.command == "mise run quality"));
    assert!(evidence
        .release
        .steps
        .iter()
        .any(|step| step.name == "release-build"
            && step.command == "cargo build --release --workspace"));
    assert!(
        evidence
            .release
            .steps
            .iter()
            .any(|step| step.name == "release-package"
                && step.command.contains("std release package"))
    );
    assert!(evidence
        .release
        .steps
        .iter()
        .any(|step| step.name == "release-verify" && step.command.contains("std release verify")));
    assert!(evidence
        .install
        .steps
        .iter()
        .any(|step| step.name == "install-run" && step.command.contains("std install run")));
    assert!(evidence
        .install
        .steps
        .iter()
        .any(|step| step.name == "install-verify" && step.command.contains("std install verify")));
}

fn assert_completion_audit_rows(evidence: &OpsEvidence) {
    let rows = operations_completion::completion_audit_rows(evidence);
    let summary = operations_completion::completion_audit_summary(&rows);
    let manual = operations_completion::completion_manual_areas(&rows);

    assert!(summary.contains("UI Docs 18-24:MANUAL"));
    assert!(summary.contains("Launcher:MANUAL"));
    assert!(summary.contains("Studio:MANUAL"));
    assert!(summary.contains("Quality:PASS"));
    assert!(manual.contains("UI Docs 18-24"));
    assert!(manual.contains("Launcher"));
    assert_eq!(rows.len(), 11);
}

fn assert_runbook_contains(runbook: &str, commands: &[&str]) {
    for command in commands {
        assert!(
            runbook.contains(command),
            "runbook missing {command}: {runbook}"
        );
    }
}
