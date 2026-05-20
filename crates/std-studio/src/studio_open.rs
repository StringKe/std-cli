use crate::{viewport::studio_native_options, StudioEguiApp};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StudioOpenRequest {
    History,
}

impl StudioOpenRequest {
    fn target(self) -> &'static str {
        match self {
            Self::History => "history",
        }
    }
}

pub(crate) fn studio_open_request_from_args(args: &[String]) -> Option<StudioOpenRequest> {
    if args.get(1).map(String::as_str) != Some("--open") {
        return None;
    }
    match args.get(2).map(String::as_str) {
        Some("history") => Some(StudioOpenRequest::History),
        _ => None,
    }
}

pub(crate) fn apply_studio_open_request(app: &mut StudioEguiApp, request: StudioOpenRequest) {
    match request {
        StudioOpenRequest::History => {
            let id = app.app.open_execution_history_pane();
            app.pending_workspace_focus = Some(id);
            app.status = format!("opened studio {}", request.target());
        }
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
    eframe::run_native(
        "std-cli Studio",
        studio_native_options(),
        Box::new(move |_cc| Ok(Box::new(crate::app_for_open_request(request)))),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_history_open_request() {
        let args = vec![
            "std-studio".to_string(),
            "--open".to_string(),
            "history".to_string(),
        ];

        assert_eq!(
            studio_open_request_from_args(&args),
            Some(StudioOpenRequest::History)
        );
    }

    #[test]
    fn applies_history_open_request_to_internal_workspace_pane() {
        let app = crate::app_for_open_request(StudioOpenRequest::History);

        assert_eq!(app.app.open_workspace_panes().count(), 1);
        assert!(app.app.focused_pane.is_some());
        assert_eq!(app.pending_workspace_focus, app.app.focused_pane);
        assert!(app.status.contains("history"));
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
}
