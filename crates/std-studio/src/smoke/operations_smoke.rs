use crate::operations_rows;
use std_studio::OpsEvidence;

pub(crate) struct OperationsSmoke {
    pub(crate) qa_command: String,
    pub(crate) qa_result: String,
    pub(crate) qa_output: String,
    pub(crate) doctor_command: String,
    pub(crate) doctor_result: String,
    pub(crate) doctor_output: String,
    pub(crate) release_command: String,
    pub(crate) release_result: String,
    pub(crate) release_output: String,
    pub(crate) install_command: String,
    pub(crate) install_result: String,
    pub(crate) install_output: String,
    pub(crate) runtime_command: String,
    pub(crate) runtime_result: String,
    pub(crate) runtime_output: String,
    pub(crate) step_summary: String,
    pub(crate) visual_contract: String,
    pub(crate) a11y_contract: String,
}

impl OperationsSmoke {
    pub(crate) fn new() -> Self {
        let evidence = OpsEvidence::load();
        let step_summary = [
            evidence.qa.steps.clone(),
            evidence.release.steps.clone(),
            evidence.install.steps.clone(),
        ]
        .concat()
        .into_iter()
        .map(|step| step.summary())
        .collect::<Vec<_>>()
        .join("|");
        let visual_contract = operations_visual_contract(&evidence);
        let a11y_contract = operations_a11y_contract(&evidence);
        Self {
            qa_command: evidence.qa.command,
            qa_result: evidence.qa.result,
            qa_output: evidence.qa.output,
            doctor_command: evidence.doctor.command,
            doctor_result: evidence.doctor.result,
            doctor_output: evidence.doctor.output,
            release_command: evidence.release.command,
            release_result: evidence.release.result,
            release_output: evidence.release.output,
            install_command: evidence.install.command,
            install_result: evidence.install.result,
            install_output: evidence.install.output,
            runtime_command: evidence.runtime.command,
            runtime_result: evidence.runtime.result,
            runtime_output: evidence.runtime.output,
            step_summary,
            visual_contract,
            a11y_contract,
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.qa_command == "mise run quality"
            && self.qa_output.contains("rustfmt=PASS")
            && self.qa_output.contains("clippy=PASS")
            && self.doctor_command == "std doctor"
            && self.doctor_result.contains("doctor source gates")
            && self.doctor_output.contains("quality=PASS")
            && self.release_command.contains("std release verify --dist")
            && self.release_result.contains("release verify")
            && self.release_output.contains("manifest=")
            && self.install_command.contains("std install verify --prefix")
            && self.install_result.contains("install verify")
            && self.install_output.contains("launcher=")
            && self
                .runtime_command
                .contains("std-launcher --gui-hotkey-smoke")
            && self
                .runtime_result
                .contains("manual desktop opt-in required")
            && self.runtime_output.contains("SKIP")
            && self.step_summary.contains("release-build:")
            && self.step_summary.contains("release-package:")
            && self.step_summary.contains("release-verify:")
            && self.step_summary.contains("install-run:")
            && self.step_summary.contains("install-verify:")
            && self
                .visual_contract
                .contains(operations_rows::operations_gate_visual_contract())
            && self
                .visual_contract
                .contains("gates=QA|Doctor|Release|Install|Runtime")
            && self.visual_contract.contains("manual_gates=Runtime")
            && self.visual_contract.contains("commands=5")
            && self.visual_contract.contains("results=5")
            && self.visual_contract.contains("outputs=5")
            && self
                .a11y_contract
                .contains(operations_rows::operations_gate_a11y_contract())
            && self
                .a11y_contract
                .contains("rows=command|step|runbook|evidence|result|artifact|output")
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "operations_smoke={}\noperations_qa_command={}\noperations_qa_result={}\noperations_qa_output={}\noperations_doctor_command={}\noperations_doctor_result={}\noperations_doctor_output={}\noperations_release_command={}\noperations_release_result={}\noperations_release_output={}\noperations_install_command={}\noperations_install_result={}\noperations_install_output={}\noperations_runtime_command={}\noperations_runtime_result={}\noperations_runtime_output={}\noperations_step_summary={}\noperations_visual_contract={}\noperations_a11y_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.qa_command,
            self.qa_result,
            self.qa_output,
            self.doctor_command,
            self.doctor_result,
            self.doctor_output,
            self.release_command,
            self.release_result,
            self.release_output,
            self.install_command,
            self.install_result,
            self.install_output,
            self.runtime_command,
            self.runtime_result,
            self.runtime_output,
            self.step_summary,
            self.visual_contract,
            self.a11y_contract,
        )
    }
}

fn operations_visual_contract(evidence: &OpsEvidence) -> String {
    format!(
        "{};gates={};manual_gates={};commands={};results={};outputs={};steps={}",
        operations_rows::operations_gate_visual_contract(),
        [
            evidence.qa.title,
            evidence.doctor.title,
            evidence.release.title,
            evidence.install.title,
            evidence.runtime.title
        ]
        .join("|"),
        evidence.runtime.title,
        visible_count([
            &evidence.qa.command,
            &evidence.doctor.command,
            &evidence.release.command,
            &evidence.install.command,
            &evidence.runtime.command,
        ]),
        visible_count([
            &evidence.qa.result,
            &evidence.doctor.result,
            &evidence.release.result,
            &evidence.install.result,
            &evidence.runtime.result,
        ]),
        visible_count([
            &evidence.qa.output,
            &evidence.doctor.output,
            &evidence.release.output,
            &evidence.install.output,
            &evidence.runtime.output,
        ]),
        evidence.qa.steps.len() + evidence.release.steps.len() + evidence.install.steps.len()
    )
}

fn operations_a11y_contract(_evidence: &OpsEvidence) -> String {
    format!(
        "{};rows=command|step|runbook|evidence|result|artifact|output",
        operations_rows::operations_gate_a11y_contract()
    )
}

fn visible_count(values: [&String; 5]) -> usize {
    values
        .iter()
        .filter(|value| !value.trim().is_empty())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations_smoke_reports_commands_results_and_outputs() {
        let smoke = OperationsSmoke::new();

        assert!(smoke.pass(), "{}", smoke.summary());
        assert!(smoke
            .summary()
            .contains("operations_qa_command=mise run quality"));
        assert!(smoke
            .summary()
            .contains("operations_doctor_command=std doctor"));
        assert!(smoke
            .summary()
            .contains("operations_release_command=std release verify"));
        assert!(smoke
            .summary()
            .contains("operations_install_command=std install verify"));
        assert!(smoke
            .summary()
            .contains("operations_runtime_command=std-launcher --gui-hotkey-smoke"));
        assert!(smoke
            .summary()
            .contains("operations_runtime_result=manual desktop opt-in required"));
        assert!(smoke.summary().contains("operations_runtime_output=SKIP"));
        assert!(smoke.summary().contains("operations_step_summary="));
        assert!(smoke.summary().contains("operations_visual_contract="));
        assert!(smoke
            .summary()
            .contains("gates=QA|Doctor|Release|Install|Runtime"));
        assert!(smoke.summary().contains("manual_gates=Runtime"));
        assert!(smoke.summary().contains("commands=5"));
        assert!(smoke.summary().contains("results=5"));
        assert!(smoke.summary().contains("outputs=5"));
        assert!(smoke.summary().contains("operations_a11y_contract="));
        assert!(smoke
            .summary()
            .contains("a11y=row-label-includes-label-value-detail"));
        assert!(smoke
            .summary()
            .contains("release-package:std release package"));
        assert!(smoke.summary().contains("install-run:std install run"));
    }
}
