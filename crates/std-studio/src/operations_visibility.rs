use std_studio::{OpsEvidence, OpsGate};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperationsVisibility {
    pub(crate) plugin: OperationVisibleGate,
    pub(crate) index: OperationVisibleGate,
    pub(crate) runtime: OperationVisibleGate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OperationVisibleGate {
    pub(crate) title: &'static str,
    pub(crate) command_visible: bool,
    pub(crate) result_visible: bool,
    pub(crate) output_visible: bool,
    pub(crate) artifact_visible: bool,
    pub(crate) status_text: &'static str,
}

impl OperationsVisibility {
    pub(crate) fn from_evidence(evidence: &OpsEvidence) -> Self {
        Self {
            plugin: OperationVisibleGate::from_gate(&evidence.plugin),
            index: OperationVisibleGate::from_gate(&evidence.index),
            runtime: OperationVisibleGate::from_gate(&evidence.runtime),
        }
    }

    pub(crate) fn pass_docs22(&self) -> bool {
        self.plugin.visible()
            && self.index.visible()
            && self.runtime.visible()
            && self.plugin.title == "Plugin"
            && self.index.title == "Index"
            && self.runtime.title == "Runtime"
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "operations_visibility {};{};{};{}",
            if self.pass_docs22() { "PASS" } else { "FAIL" },
            self.plugin.summary(),
            self.index.summary(),
            self.runtime.summary()
        )
    }
}

impl OperationVisibleGate {
    fn from_gate(gate: &OpsGate) -> Self {
        Self {
            title: gate.title,
            command_visible: !gate.command.trim().is_empty(),
            result_visible: !gate.result.trim().is_empty(),
            output_visible: !gate.output.trim().is_empty(),
            artifact_visible: !gate.artifact.trim().is_empty(),
            status_text: gate.status.label(),
        }
    }

    fn visible(&self) -> bool {
        self.command_visible
            && self.result_visible
            && self.output_visible
            && self.artifact_visible
            && !self.status_text.trim().is_empty()
    }

    fn summary(&self) -> String {
        format!(
            "{}=status:{};command:{};result:{};output:{};artifact:{}",
            self.title.to_ascii_lowercase(),
            self.status_text,
            bool_word(self.command_visible),
            bool_word(self.result_visible),
            bool_word(self.output_visible),
            bool_word(self.artifact_visible)
        )
    }
}

fn bool_word(value: bool) -> &'static str {
    if value {
        "visible"
    } else {
        "missing"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations_visibility_exposes_plugin_index_runtime_rows() {
        let evidence = OpsEvidence::load();
        let visibility = OperationsVisibility::from_evidence(&evidence);
        let summary = visibility.summary();

        assert!(visibility.pass_docs22(), "{summary}");
        assert!(summary.contains("operations_visibility PASS"));
        assert!(summary.contains("plugin=status:"));
        assert!(summary.contains("index=status:"));
        assert!(summary.contains("runtime=status:MANUAL"));
        assert!(summary.contains("command:visible"));
        assert!(summary.contains("artifact:visible"));
    }
}
