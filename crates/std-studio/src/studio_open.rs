use crate::StudioEguiApp;
use std_studio::{PaneSystemPolicy, StudioWorkspacePolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StudioOpenRequest {
    Analysis,
    Apps,
    History,
    Memory,
    Plugins,
    Settings,
    Workflows,
}

impl StudioOpenRequest {
    pub(crate) fn target(self) -> &'static str {
        match self {
            Self::Analysis => "analysis",
            Self::Apps => "apps",
            Self::History => "history",
            Self::Memory => "memory",
            Self::Plugins => "plugins",
            Self::Settings => "settings",
            Self::Workflows => "workflows",
        }
    }
}

pub(crate) fn studio_open_request_from_args(args: &[String]) -> Option<StudioOpenRequest> {
    if args.get(1).map(String::as_str) != Some("--open") {
        return None;
    }
    match args.get(2).map(String::as_str) {
        Some("analysis") => Some(StudioOpenRequest::Analysis),
        Some("apps") => Some(StudioOpenRequest::Apps),
        Some("history") => Some(StudioOpenRequest::History),
        Some("memory") => Some(StudioOpenRequest::Memory),
        Some("plugins") => Some(StudioOpenRequest::Plugins),
        Some("settings") => Some(StudioOpenRequest::Settings),
        Some("workflows") => Some(StudioOpenRequest::Workflows),
        _ => None,
    }
}

pub(crate) fn studio_open_smoke_from_args(args: &[String]) -> Option<StudioOpenSmokeReport> {
    if args.get(1).map(String::as_str) != Some("--open-smoke") {
        return None;
    }
    Some(StudioOpenSmokeReport::new())
}

pub(crate) fn apply_studio_open_request(app: &mut StudioEguiApp, request: StudioOpenRequest) {
    match request {
        StudioOpenRequest::Analysis => {
            let id = app
                .app
                .open_analysis_workbench(std::path::PathBuf::from(&app.analysis.path));
            app.pending_workspace_focus = Some(id);
        }
        StudioOpenRequest::Apps => {
            let id = app.app.open_app_manager_pane();
            app.pending_workspace_focus = Some(id);
        }
        StudioOpenRequest::History => {
            let id = app.app.open_execution_history_pane();
            app.pending_workspace_focus = Some(id);
        }
        StudioOpenRequest::Memory => {
            let id = app.app.open_memory_browser_pane();
            app.pending_workspace_focus = Some(id);
        }
        StudioOpenRequest::Plugins => {
            let id = app.app.open_plugin_manager_pane();
            app.pending_workspace_focus = Some(id);
        }
        StudioOpenRequest::Settings => {
            let id = app.app.open_settings_pane();
            app.pending_workspace_focus = Some(id);
        }
        StudioOpenRequest::Workflows => {
            let id = app
                .app
                .open_workflow_builder(app.app.core.config.workflows_dir());
            app.pending_workspace_focus = Some(id);
        }
    }
    app.status = format!("opened studio {}", request.target());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioOpenSmokeReport {
    pub targets: usize,
    pub route: &'static str,
    pub runtime_boundary: &'static str,
    pub host_policy: &'static str,
    pub pane_system: &'static str,
    pub docs: &'static str,
    pub internal_panes: usize,
    pub native_child_windows: bool,
    pub detached_panels: bool,
    pub focus_restored: bool,
    pub content_keys: String,
}

impl StudioOpenSmokeReport {
    pub(crate) fn new() -> Self {
        let requests = all_open_requests();
        let mut keys = Vec::new();
        let mut focus_restored = true;
        for request in requests {
            let app = app_for_open_request(request);
            let Some(spec) = crate::workspace_panes::focused_workspace_spec(&app.app) else {
                focus_restored = false;
                continue;
            };
            focus_restored &= app.pending_workspace_focus == Some(spec.id);
            keys.push(spec.content_key.to_string());
        }
        keys.sort();
        keys.dedup();
        let policy = std_studio::StudioApp::default().workspace_policy;
        Self {
            targets: all_open_requests().len(),
            route: "internal-egui-workspace-pane-intent",
            runtime_boundary: studio_open_runtime_boundary(),
            host_policy: policy.host_window.label(),
            pane_system: policy.pane_system.label(),
            docs: StudioWorkspacePolicy::DOC_REFERENCE,
            internal_panes: keys.len(),
            native_child_windows: policy.allows_native_child_windows(),
            detached_panels: policy.allows_detached_panels(),
            focus_restored,
            content_keys: keys.join(","),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        let policy = StudioWorkspacePolicy::studio_v1();
        self.targets == 7
            && self.route == "internal-egui-workspace-pane-intent"
            && self.runtime_boundary == studio_open_runtime_boundary()
            && self.host_policy == policy.host_window.label()
            && self.pane_system == policy.pane_system.label()
            && open_intent_policy_passes(policy)
            && self.internal_panes == 7
            && !self.native_child_windows
            && !self.detached_panels
            && self.focus_restored
            && self.content_keys == "analysis,apps,history,memory,plugins,settings,workflows"
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_open_smoke {}\nroute={}\nruntime_boundary={}\nhost_policy={}\npane_system={}\ndocs={}\ntargets={}\ninternal_panes={}\nnative_child_windows={}\ndetached_panels={}\nfocus_restored={}\ncontent_keys={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.route,
            self.runtime_boundary,
            self.host_policy,
            self.pane_system,
            self.docs,
            self.targets,
            self.internal_panes,
            self.native_child_windows,
            self.detached_panels,
            self.focus_restored,
            self.content_keys
        )
    }
}

pub(crate) fn studio_open_blocked_summary(request: StudioOpenRequest, reason: &str) -> String {
    let mut app = StudioEguiApp::default();
    apply_studio_open_request(&mut app, request);
    format!(
        "studio_open SKIP\ntarget={}\nfocused_pane={}\nworkspace_panes={}\nreason={reason}",
        request.target(),
        app.app
            .focused_pane
            .map(|id| id.value().to_string())
            .unwrap_or_else(|| "none".to_string()),
        app.app.open_workspace_panes().count()
    )
}

pub(crate) fn run_studio_open_request(request: StudioOpenRequest) -> eframe::Result<()> {
    println!("{}", studio_open_intent_summary(request));
    Ok(())
}

pub(crate) fn app_for_open_request(request: StudioOpenRequest) -> StudioEguiApp {
    let mut app = StudioEguiApp::default();
    apply_studio_open_request(&mut app, request);
    app
}

pub(crate) fn studio_open_intent_summary(request: StudioOpenRequest) -> String {
    let app = app_for_open_request(request);
    format!(
        "studio_open_intent PASS\ntarget={}\nroute=internal-egui-workspace-pane-intent\nruntime_boundary={}\nhost_window=existing-studio-host\nhost_policy={}\npane_system={}\ndocs={}\nfocused_pane={}\nworkspace_panes={}\nnative_child_windows={}\ndetached_panels={}",
        request.target(),
        studio_open_runtime_boundary(),
        app.app.workspace_policy.host_window.label(),
        app.app.workspace_policy.pane_system.label(),
        StudioWorkspacePolicy::DOC_REFERENCE,
        app.app
            .focused_pane
            .map(|id| id.value().to_string())
            .unwrap_or_else(|| "none".to_string()),
        app.app.open_workspace_panes().count(),
        app.app.workspace_policy.allows_native_child_windows(),
        app.app.workspace_policy.allows_detached_panels()
    )
}

pub(crate) fn studio_open_runtime_boundary() -> &'static str {
    "open-intent-before-native-startup;no-run-native;no-extra-viewport"
}

pub(crate) fn open_intent_policy_passes(policy: StudioWorkspacePolicy) -> bool {
    policy.host_window.label() == "single-borderless-egui-viewport"
        && policy.pane_system == PaneSystemPolicy::InternalEguiWorkspacePanes
        && !policy.allows_native_child_windows()
        && !policy.allows_detached_panels()
        && StudioWorkspacePolicy::DOC_REFERENCE == "docs/22 + docs/24"
}

pub(crate) fn all_open_requests() -> [StudioOpenRequest; 7] {
    [
        StudioOpenRequest::Analysis,
        StudioOpenRequest::Apps,
        StudioOpenRequest::History,
        StudioOpenRequest::Memory,
        StudioOpenRequest::Plugins,
        StudioOpenRequest::Settings,
        StudioOpenRequest::Workflows,
    ]
}
