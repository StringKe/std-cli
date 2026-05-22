use crate::smoke::{
    surface_smoke::StudioSurfaceSmoke, workspace_policy_smoke::WorkspacePolicySmoke,
};
use std_egui::tokens::ThemeSmokeReport;

pub(crate) fn theme_smoke_from_args(args: &[String]) -> Option<ThemeSmokeReport> {
    if args.get(1).map(String::as_str) == Some("--theme-smoke") {
        Some(ThemeSmokeReport::new())
    } else {
        None
    }
}

pub(crate) fn workspace_policy_smoke_from_args(args: &[String]) -> Option<WorkspacePolicySmoke> {
    if args.get(1).map(String::as_str) == Some("--workspace-policy-smoke") {
        Some(WorkspacePolicySmoke::new())
    } else {
        None
    }
}

pub(crate) fn surface_smoke_from_args(args: &[String]) -> Option<StudioSurfaceSmoke> {
    if args.get(1).map(String::as_str) == Some("--surface-smoke") {
        Some(StudioSurfaceSmoke::new())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_theme_smoke_reports_light_and_dark_tokens() {
        let args = vec!["std-studio".to_string(), "--theme-smoke".to_string()];
        let report = theme_smoke_from_args(&args).unwrap();

        assert!(report.pass());
        assert!(report.summary("studio").contains("studio_theme_smoke PASS"));
        assert!(report
            .summary("studio")
            .contains("dark_accent_weak_alpha=46"));
        assert!(report
            .summary("studio")
            .contains("light_accent_weak_alpha=31"));
        assert!(report
            .summary("studio")
            .contains("high_contrast_dark_accent_weak_alpha=82"));
        assert!(report
            .summary("studio")
            .contains("high_contrast_light_accent_weak_alpha=56"));
        assert!(report.summary("studio").contains(
            "typography_contract=text=caption:11,footnote:12,body:13,title:15,headline:18,display:24,code:12"
        ));
        assert!(report.summary("studio").contains(
            "status_contract=status=success:#3DCB7C/#138750,warning:#F5B643/#B27500,danger:#FF6A6A/#C8312B,info:#4E9CFF/#0A6BFF"
        ));
        assert!(report
            .summary("studio")
            .contains("no_pure_black_white_tokens=true"));
    }

    #[test]
    fn studio_workspace_policy_smoke_reports_internal_panes() {
        let args = vec![
            "std-studio".to_string(),
            "--workspace-policy-smoke".to_string(),
        ];
        let report = workspace_policy_smoke_from_args(&args).unwrap();

        assert!(report.pass());
        assert!(report.output().contains("internal-egui-workspace-panes"));
        assert!(report.output().contains("extra_viewports=false"));
        assert!(report.output().contains("show_viewport_api=false"));
        assert!(report.output().contains("egui_window_api=false"));
        assert!(report.output().contains("settings_overlay=false"));
        assert!(report
            .output()
            .contains("source_guard=workspace_policy_guard.rs"));
        assert!(report.output().contains("forbidden_apis=egui::Window::new"));
        assert!(report
            .output()
            .contains("ui_completion_boundary=headless-smoke-is-not-ui-completion"));
        assert!(report
            .output()
            .contains("manual_ui_evidence_gates=light-dark-screenshots"));
    }

    #[test]
    fn studio_surface_smoke_reports_light_and_dark_workspace_layers() {
        let args = vec!["std-studio".to_string(), "--surface-smoke".to_string()];
        let report = surface_smoke_from_args(&args).unwrap();

        assert!(report.pass(), "{}", report.output());
        assert!(report.output().contains("studio_surface_smoke PASS"));
        assert!(report.output().contains("light_canvas_surface_layer"));
        assert!(report.output().contains("dark_selected_surface_layer"));
        assert!(report.output().contains("standard_modal_enter_ms=220"));
        assert!(report.output().contains("reduced_modal_enter_ms=0"));
        assert!(report.output().contains("reduced_focus_ring_ms=0"));
    }
}
