use std_studio::{StudioApp, StudioWorkspacePolicy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspacePolicySmoke {
    pub(crate) host_window: &'static str,
    pub(crate) pane_system: &'static str,
    pub(crate) native_child_windows: bool,
    pub(crate) detached_panels: bool,
    pub(crate) extra_viewports: bool,
    pub(crate) show_viewport_api: bool,
    pub(crate) egui_window_api: bool,
    pub(crate) settings_overlay: bool,
    pub(crate) doc_reference: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) viewport_touchpoints: String,
    pub(crate) native_entrypoints: String,
    pub(crate) host_viewport_contract: String,
    pub(crate) forbidden_apis: String,
    pub(crate) ui_completion_boundary: &'static str,
    pub(crate) source_guard_contract: &'static str,
    pub(crate) manual_ui_evidence_gates: String,
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
            extra_viewports: policy.allows_extra_viewports(),
            show_viewport_api: policy.allows_show_viewport_api(),
            egui_window_api: policy.allows_egui_window_api(),
            settings_overlay: policy.allows_settings_overlay(),
            doc_reference: StudioWorkspacePolicy::DOC_REFERENCE,
            summary: policy.summary(),
            viewport_touchpoints: StudioWorkspacePolicy::VIEWPORT_TOUCHPOINTS.join("|"),
            native_entrypoints: StudioWorkspacePolicy::NATIVE_ENTRYPOINTS.join("|"),
            host_viewport_contract: policy.host_viewport_contract().summary(),
            forbidden_apis: StudioWorkspacePolicy::FORBIDDEN_WORKBENCH_APIS.join("|"),
            ui_completion_boundary: StudioWorkspacePolicy::UI_COMPLETION_BOUNDARY,
            source_guard_contract: StudioWorkspacePolicy::SOURCE_GUARD_CONTRACT,
            manual_ui_evidence_gates: StudioWorkspacePolicy::MANUAL_UI_EVIDENCE_GATES.join("|"),
            source_guard: "workspace_policy_guard.rs",
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.host_window == "single-borderless-egui-viewport"
            && self.pane_system == "internal-egui-workspace-panes"
            && !self.native_child_windows
            && !self.detached_panels
            && !self.extra_viewports
            && !self.show_viewport_api
            && !self.egui_window_api
            && !self.settings_overlay
            && self.doc_reference == "docs/22 + docs/24"
            && self.summary == "single egui host viewport, internal workspace panes"
            && self.viewport_touchpoints
                == "src/viewport.rs|src/host_window.rs|src/host_chrome.rs|src/host_chrome_drag.rs|src/preview.rs|src/preview_tests.rs"
            && self.native_entrypoints == "src/native_app.rs|src/preview.rs"
            && self.host_viewport_contract.contains("size=1280x800")
            && self.host_viewport_contract.contains("min=1080x640")
            && self.host_viewport_contract.contains("decorations=false")
            && self.host_viewport_contract.contains("detached_panels=false")
            && self.forbidden_apis.contains("egui::Window::new")
            && self.forbidden_apis.contains("ViewportBuilder::default")
            && self.forbidden_apis.contains("send_viewport_cmd")
            && self.ui_completion_boundary == "headless-smoke-is-not-ui-completion"
            && self
                .source_guard_contract
                .contains("allowed_viewport_api=host-boundary-only")
            && self
                .source_guard_contract
                .contains("forbidden_workbench_api=egui-window|show-viewport|extra-viewport")
            && self
                .source_guard_contract
                .contains("open=internal-pane-intent")
            && self
                .manual_ui_evidence_gates
                .contains("light-dark-screenshots")
            && self
                .manual_ui_evidence_gates
                .contains("workspace-pane-open-focus-close-restore")
            && self
                .manual_ui_evidence_gates
                .contains("keyboard-a11y-focus")
            && self
                .manual_ui_evidence_gates
                .contains("operations-runtime-evidence")
            && self.source_guard == "workspace_policy_guard.rs"
    }

    pub(crate) fn output(&self) -> String {
        format!(
            "studio_workspace_policy_smoke {}\nhost_window={}\npane_system={}\nnative_child_windows={}\ndetached_panels={}\nextra_viewports={}\nshow_viewport_api={}\negui_window_api={}\nsettings_overlay={}\ndoc_reference={}\nsummary={}\nviewport_touchpoints={}\nnative_entrypoints={}\nhost_viewport_contract={}\nforbidden_apis={}\nui_completion_boundary={}\nsource_guard_contract={}\nmanual_ui_evidence_gates={}\nsource_guard={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.host_window,
            self.pane_system,
            self.native_child_windows,
            self.detached_panels,
            self.extra_viewports,
            self.show_viewport_api,
            self.egui_window_api,
            self.settings_overlay,
            self.doc_reference,
            self.summary,
            self.viewport_touchpoints,
            self.native_entrypoints,
            self.host_viewport_contract,
            self.forbidden_apis,
            self.ui_completion_boundary,
            self.source_guard_contract,
            self.manual_ui_evidence_gates,
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
        assert!(report.output().contains("extra_viewports=false"));
        assert!(report.output().contains("show_viewport_api=false"));
        assert!(report.output().contains("egui_window_api=false"));
        assert!(report.output().contains("settings_overlay=false"));
        assert!(report
            .output()
            .contains("viewport_touchpoints=src/viewport.rs"));
        assert!(report
            .output()
            .contains("native_entrypoints=src/native_app.rs"));
        assert!(report
            .output()
            .contains("host_viewport_contract=host_viewport=single-borderless-egui-viewport"));
        assert!(report.output().contains("size=1280x800"));
        assert!(report.output().contains("detached_panels=false"));
        assert!(report.output().contains("forbidden_apis=egui::Window::new"));
        assert!(report
            .output()
            .contains("source_guard=workspace_policy_guard.rs"));
        assert!(report
            .output()
            .contains("ui_completion_boundary=headless-smoke-is-not-ui-completion"));
        assert!(report
            .output()
            .contains("source_guard_contract=source_guard=full-src-scan"));
        assert!(report
            .output()
            .contains("allowed_viewport_api=host-boundary-only"));
        assert!(report
            .output()
            .contains("forbidden_workbench_api=egui-window|show-viewport|extra-viewport"));
        assert!(report.output().contains("open=internal-pane-intent"));
        assert!(report
            .output()
            .contains("manual_ui_evidence_gates=light-dark-screenshots"));
    }
}
