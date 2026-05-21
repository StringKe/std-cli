use std_studio::{StudioApp, StudioWorkspacePolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspacePolicySmoke {
    pub(crate) host_window: &'static str,
    pub(crate) pane_system: &'static str,
    pub(crate) native_child_windows: bool,
    pub(crate) detached_panels: bool,
    pub(crate) doc_reference: &'static str,
    pub(crate) summary: &'static str,
}

impl WorkspacePolicySmoke {
    pub(crate) fn new() -> Self {
        Self::from_policy(StudioApp::default().workspace_policy)
    }

    fn from_policy(policy: StudioWorkspacePolicy) -> Self {
        Self {
            host_window: policy.host_window.label(),
            pane_system: policy.pane_system.label(),
            native_child_windows: policy.allows_native_child_windows(),
            detached_panels: policy.allows_detached_panels(),
            doc_reference: StudioWorkspacePolicy::DOC_REFERENCE,
            summary: policy.summary(),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.host_window == "single-borderless-egui-viewport"
            && self.pane_system == "internal-egui-workspace-panes"
            && !self.native_child_windows
            && !self.detached_panels
            && self.doc_reference == "docs/22 + docs/24"
            && self.summary == "single egui host viewport, internal workspace panes"
    }

    pub(crate) fn output(&self) -> String {
        format!(
            "studio_workspace_policy_smoke {}\nhost_window={}\npane_system={}\nnative_child_windows={}\ndetached_panels={}\ndoc_reference={}\nsummary={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.host_window,
            self.pane_system,
            self.native_child_windows,
            self.detached_panels,
            self.doc_reference,
            self.summary
        )
    }
}

impl Default for WorkspacePolicySmoke {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_policy_smoke_rejects_native_child_window_strategy() {
        let report = WorkspacePolicySmoke::new();

        assert!(report.pass(), "{}", report.output());
        assert!(report.output().contains("native_child_windows=false"));
        assert!(report.output().contains("detached_panels=false"));
    }
}
