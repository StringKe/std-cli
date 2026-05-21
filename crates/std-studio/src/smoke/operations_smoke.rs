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
}

impl OperationsSmoke {
    pub(crate) fn new() -> Self {
        let evidence = OpsEvidence::load();
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
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "operations_smoke={}\noperations_qa_command={}\noperations_qa_result={}\noperations_qa_output={}\noperations_doctor_command={}\noperations_doctor_result={}\noperations_doctor_output={}\noperations_release_command={}\noperations_release_result={}\noperations_release_output={}\noperations_install_command={}\noperations_install_result={}\noperations_install_output={}",
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
        )
    }
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
    }
}
