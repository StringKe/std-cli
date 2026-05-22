use std_studio::{StudioApp, StudioWorkspacePolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspacePolicySmoke {
    pub(crate) host_window: &'static str,
    pub(crate) pane_system: &'static str,
    pub(crate) native_child_windows: bool,
    pub(crate) detached_panels: bool,
    pub(crate) doc_reference: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) viewport_touchpoints: String,
    pub(crate) native_entrypoints: String,
    pub(crate) forbidden_apis: String,
    pub(crate) source_guard: &'static str,
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
            viewport_touchpoints: StudioWorkspacePolicy::VIEWPORT_TOUCHPOINTS.join("|"),
            native_entrypoints: StudioWorkspacePolicy::NATIVE_ENTRYPOINTS.join("|"),
            forbidden_apis: StudioWorkspacePolicy::FORBIDDEN_WORKBENCH_APIS.join("|"),
            source_guard: "workspace_policy_guard.rs",
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.host_window == "single-borderless-egui-viewport"
            && self.pane_system == "internal-egui-workspace-panes"
            && !self.native_child_windows
            && !self.detached_panels
            && self.doc_reference == "docs/22 + docs/24"
            && self.summary == "single egui host viewport, internal workspace panes"
            && self.viewport_touchpoints
                == "src/viewport.rs|src/host_chrome.rs|src/host_chrome_drag.rs|src/preview.rs|src/preview_tests.rs"
            && self.native_entrypoints == "src/native_app.rs|src/preview.rs"
            && self.forbidden_apis.contains("egui::Window::new")
            && self.forbidden_apis.contains("ViewportBuilder::default")
            && self.forbidden_apis.contains("send_viewport_cmd")
            && self.source_guard == "workspace_policy_guard.rs"
    }

    pub(crate) fn output(&self) -> String {
        format!(
            "studio_workspace_policy_smoke {}\nhost_window={}\npane_system={}\nnative_child_windows={}\ndetached_panels={}\ndoc_reference={}\nsummary={}\nviewport_touchpoints={}\nnative_entrypoints={}\nforbidden_apis={}\nsource_guard={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.host_window,
            self.pane_system,
            self.native_child_windows,
            self.detached_panels,
            self.doc_reference,
            self.summary,
            self.viewport_touchpoints,
            self.native_entrypoints,
            self.forbidden_apis,
            self.source_guard
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
        assert!(report
            .output()
            .contains("viewport_touchpoints=src/viewport.rs"));
        assert!(report
            .output()
            .contains("native_entrypoints=src/native_app.rs"));
        assert!(report.output().contains("forbidden_apis=egui::Window::new"));
        assert!(report
            .output()
            .contains("source_guard=workspace_policy_guard.rs"));
    }
}
