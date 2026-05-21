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
    }

    #[test]
    fn studio_surface_smoke_reports_light_and_dark_workspace_layers() {
        let args = vec!["std-studio".to_string(), "--surface-smoke".to_string()];
        let report = surface_smoke_from_args(&args).unwrap();

        assert!(report.pass(), "{}", report.output());
        assert!(report.output().contains("studio_surface_smoke PASS"));
        assert!(report.output().contains("light_canvas_surface_layer"));
        assert!(report.output().contains("dark_selected_surface_layer"));
    }
}
