use super::*;

#[test]
fn studio_operations_evidence_reports_current_quality_release_and_install_state() {
    let evidence = OpsEvidence::load();
    let lines = evidence.lines();

    assert_gate_statuses(&evidence);
    assert_line_contract(&lines);
    assert_gate_outputs(&evidence);
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
    assert!(lines.iter().any(|line| line.contains("std doctor")));
    assert!(lines.iter().any(|line| line.contains("release verify")));
    assert!(lines.iter().any(|line| line.contains("install verify")));
    assert!(lines.iter().any(|line| line.contains("artifact=")));
    assert!(lines.iter().any(|line| line.contains("output=")));
}

fn assert_gate_outputs(evidence: &OpsEvidence) {
    assert!(!evidence.qa.result.is_empty());
    assert!(evidence.qa.output.contains("rustfmt=PASS"));
    assert!(evidence.doctor.result.contains("doctor source gates"));
    assert!(evidence.doctor.output.contains("quality=PASS"));
    assert!(!evidence.release.result.is_empty());
    assert!(evidence.release.result.contains("release verify"));
    assert!(evidence.release.output.contains("manifest="));
    assert!(!evidence.install.result.is_empty());
    assert!(evidence.install.result.contains("install verify"));
    assert!(evidence.install.output.contains("launcher="));
    assert!(evidence.runtime.output.contains("SKIP"));
}
