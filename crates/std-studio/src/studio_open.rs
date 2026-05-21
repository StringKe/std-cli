use crate::{native_app::run_studio_native_app_with, StudioEguiApp};
use std_studio::StudioPane;

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
    fn target(self) -> &'static str {
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
    if let Some(pane) = main_pane_for_request(request) {
        app.app.switch_pane(pane);
    }
    app.status = format!("opened studio {}", request.target());
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioOpenSmokeReport {
    pub targets: usize,
    pub route: &'static str,
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
            internal_panes: keys.len(),
            native_child_windows: policy.allows_native_child_windows(),
            detached_panels: policy.allows_detached_panels(),
            focus_restored,
            content_keys: keys.join(","),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.targets == 7
            && self.route == "internal-egui-workspace-pane-intent"
            && self.internal_panes == 7
            && !self.native_child_windows
            && !self.detached_panels
            && self.focus_restored
            && self.content_keys == "analysis,apps,history,memory,plugins,settings,workflows"
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "studio_open_smoke {}\nroute={}\ntargets={}\ninternal_panes={}\nnative_child_windows={}\ndetached_panels={}\nfocus_restored={}\ncontent_keys={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.route,
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
    run_studio_native_app_with(app_for_open_request(request))
}

pub(crate) fn app_for_open_request(request: StudioOpenRequest) -> StudioEguiApp {
    let mut app = StudioEguiApp::default();
    apply_studio_open_request(&mut app, request);
    app
}

fn main_pane_for_request(request: StudioOpenRequest) -> Option<StudioPane> {
    match request {
        StudioOpenRequest::Analysis => Some(StudioPane::Analysis),
        StudioOpenRequest::Apps => Some(StudioPane::Apps),
        StudioOpenRequest::Memory => Some(StudioPane::Memory),
        StudioOpenRequest::Plugins => Some(StudioPane::Plugins),
        StudioOpenRequest::Settings => Some(StudioPane::Settings),
        StudioOpenRequest::Workflows => Some(StudioPane::Workflows),
        StudioOpenRequest::History => None,
    }
}

fn all_open_requests() -> [StudioOpenRequest; 7] {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_supported_open_requests() {
        for (target, request) in [
            ("analysis", StudioOpenRequest::Analysis),
            ("apps", StudioOpenRequest::Apps),
            ("history", StudioOpenRequest::History),
            ("memory", StudioOpenRequest::Memory),
            ("plugins", StudioOpenRequest::Plugins),
            ("settings", StudioOpenRequest::Settings),
            ("workflows", StudioOpenRequest::Workflows),
        ] {
            let args = open_args(target);
            assert_eq!(studio_open_request_from_args(&args), Some(request));
        }
    }

    #[test]
    fn applies_open_requests_to_internal_workspace_panes() {
        for request in all_open_requests() {
            let app = app_for_open_request(request);

            assert_eq!(app.app.open_workspace_panes().count(), 1);
            assert!(app.app.focused_pane.is_some());
            assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
            assert!(app.status.contains(request.target()));
        }
    }

    #[test]
    fn open_smoke_reports_internal_pane_intents_without_native_windows() {
        let args = vec!["std-studio".to_string(), "--open-smoke".to_string()];
        let report = studio_open_smoke_from_args(&args).unwrap();

        assert!(report.pass(), "{}", report.summary());
        assert!(report.summary().contains("studio_open_smoke PASS"));
        assert!(report
            .summary()
            .contains("route=internal-egui-workspace-pane-intent"));
        assert!(report.summary().contains("native_child_windows=false"));
        assert!(report.summary().contains("detached_panels=false"));
    }

    #[test]
    fn settings_request_uses_internal_workspace_pane() {
        let app = app_for_open_request(StudioOpenRequest::Settings);
        let spec = crate::workspace_panes::focused_workspace_spec(&app.app).unwrap();

        assert_eq!(app.app.active_pane, StudioPane::Settings);
        assert_eq!(spec.content_key, "settings");
        assert_eq!(spec.heading, "Settings");
        assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
        assert!(app.status.contains("opened studio settings"));
    }

    #[test]
    fn blocked_summary_keeps_test_mode_from_opening_native_window() {
        let summary = studio_open_blocked_summary(
            StudioOpenRequest::Settings,
            "studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup",
        );

        assert!(summary.contains("studio_open SKIP"));
        assert!(summary.contains("target=settings"));
        assert!(summary.contains("workspace_panes=1"));
        assert!(summary.contains("STD_TEST_MODE blocked native app startup"));
    }

    fn open_args(target: &str) -> Vec<String> {
        vec![
            "std-studio".to_string(),
            "--open".to_string(),
            target.to_string(),
        ]
    }
}
