use crate::{viewport::studio_native_options, StudioEguiApp};
use std_studio::StudioPane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StudioOpenRequest {
    Analysis,
    Apps,
    History,
    Memory,
    Plugins,
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
        Some("workflows") => Some(StudioOpenRequest::Workflows),
        _ => None,
    }
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
    eframe::run_native(
        "std-cli Studio",
        studio_native_options(),
        Box::new(move |_cc| Ok(Box::new(crate::app_for_open_request(request)))),
    )
}

fn main_pane_for_request(request: StudioOpenRequest) -> Option<StudioPane> {
    match request {
        StudioOpenRequest::Analysis => Some(StudioPane::Analysis),
        StudioOpenRequest::Apps => Some(StudioPane::Apps),
        StudioOpenRequest::Memory => Some(StudioPane::Memory),
        StudioOpenRequest::Plugins => Some(StudioPane::Plugins),
        StudioOpenRequest::Workflows => Some(StudioPane::Workflows),
        StudioOpenRequest::History => None,
    }
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
            ("workflows", StudioOpenRequest::Workflows),
        ] {
            let args = open_args(target);
            assert_eq!(studio_open_request_from_args(&args), Some(request));
        }
    }

    #[test]
    fn applies_open_requests_to_internal_workspace_panes() {
        for request in [
            StudioOpenRequest::Analysis,
            StudioOpenRequest::Apps,
            StudioOpenRequest::History,
            StudioOpenRequest::Memory,
            StudioOpenRequest::Plugins,
            StudioOpenRequest::Workflows,
        ] {
            let app = crate::app_for_open_request(request);

            assert_eq!(app.app.open_workspace_panes().count(), 1);
            assert!(app.app.focused_pane.is_some());
            assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
            assert!(app.status.contains(request.target()));
        }
    }

    #[test]
    fn blocked_summary_keeps_test_mode_from_opening_native_window() {
        let summary = studio_open_blocked_summary(
            StudioOpenRequest::History,
            "studio_native_app SKIP reason=STD_TEST_MODE blocked native app startup",
        );

        assert!(summary.contains("studio_open SKIP"));
        assert!(summary.contains("target=history"));
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
