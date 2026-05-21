use crate::{
    doctor::workspace::{check_text, find_workspace_root, read_required},
    CliError,
};

const UI_DOCS: [&str; 7] = [
    "docs/18_UI_Philosophy_and_Visual_Language.md",
    "docs/19_Motion_and_Interaction_Rhythm.md",
    "docs/20_Keyboard_Focus_and_Input.md",
    "docs/21_Launcher_UX_Spec.md",
    "docs/22_Studio_UX_Spec.md",
    "docs/23_Accessibility_and_Localization.md",
    "docs/24_egui_Implementation_Constraints.md",
];

const LAUNCHER_GATES: [&str; 5] = [
    "theme-smoke",
    "surface-smoke",
    "ui-semantics-smoke",
    "keyboard-smoke",
    "preview-smoke",
];

const STUDIO_GATES: [&str; 4] = [
    "smoke",
    "workspace-policy-smoke",
    "theme-smoke",
    "preview-smoke",
];

pub(crate) struct UiDoctor {
    pub(crate) docs: &'static str,
    pub(crate) docs_count: usize,
    pub(crate) launcher_gates: Vec<&'static str>,
    pub(crate) studio_gates: Vec<&'static str>,
    pub(crate) manual_desktop_acceptance: &'static str,
    pub(crate) desktop_automation_default: &'static str,
    pub(crate) completion: &'static str,
}

pub(crate) fn check_ui_completion_evidence() -> Result<UiDoctor, CliError> {
    let root = find_workspace_root()?;
    check_ui_docs(&root)?;
    check_quality_report_gates(&root)?;
    check_runtime_theme_profiles(&root)?;
    check_launcher_panel_viewport(&root)?;
    check_preview_matrices(&root)?;
    check_desktop_automation_boundary(&root)?;
    Ok(UiDoctor {
        docs: "PASS",
        docs_count: UI_DOCS.len(),
        launcher_gates: LAUNCHER_GATES.to_vec(),
        studio_gates: STUDIO_GATES.to_vec(),
        manual_desktop_acceptance: "explicit_opt_in_only",
        desktop_automation_default: "blocked",
        completion: "INCOMPLETE_REAL_GUI_REQUIRED",
    })
}

fn check_ui_docs(root: &std::path::Path) -> Result<(), CliError> {
    for doc in UI_DOCS {
        let body = read_required(&root.join(doc))?;
        check_text(&body, "# ")?;
    }
    Ok(())
}

fn check_quality_report_gates(root: &std::path::Path) -> Result<(), CliError> {
    let body = read_required(&root.join("crates/std-cli/src/release/quality.rs"))?;
    for required in [
        "STD_TEST_MODE=1 std-launcher --theme-smoke",
        "STD_TEST_MODE=1 std-launcher --surface-smoke",
        "STD_TEST_MODE=1 std-launcher --ui-semantics-smoke index",
        "STD_TEST_MODE=1 std-launcher --keyboard-smoke index",
        "STD_TEST_MODE=1 std-launcher --preview-smoke",
        "STD_TEST_MODE=1 std-studio --smoke",
        "STD_TEST_MODE=1 std-studio --workspace-policy-smoke",
        "STD_TEST_MODE=1 std-studio --theme-smoke",
        "STD_TEST_MODE=1 std-studio --preview-smoke",
        "manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space",
    ] {
        check_text(&body, required)?;
    }
    if body.contains("smoke=STD_ALLOW_DESKTOP_AUTOMATION=1") {
        return Err(CliError::Config(
            "desktop automation must not be a default smoke gate".to_string(),
        ));
    }
    Ok(())
}

fn check_runtime_theme_profiles(root: &std::path::Path) -> Result<(), CliError> {
    let egui_tokens = read_required(&root.join("crates/std-egui/src/tokens/style.rs"))?;
    for required in [
        "pub struct ThemeProfile",
        "pub fn apply(ctx: &egui::Context, mode: ThemeMode) -> Self",
        "pub requested: ThemeMode",
        "pub effective: EffectiveTheme",
        "pub high_contrast: bool",
        "pub reduce_motion: bool",
    ] {
        check_text(&egui_tokens, required)?;
    }
    let launcher = read_required(&root.join("crates/std-launcher/src/app.rs"))?;
    check_text(&launcher, "pub(crate) theme_profile: Option<ThemeProfile>")?;
    check_text(&launcher, "ThemeProfile::apply(ctx, self.theme_mode)")?;
    let studio = read_required(&root.join("crates/std-studio/src/main.rs"))?;
    check_text(&studio, "pub(crate) theme_profile: Option<ThemeProfile>")?;
    check_text(&studio, "self.theme_profile = Some(ui::install_visuals")?;
    Ok(())
}

fn check_launcher_panel_viewport(root: &std::path::Path) -> Result<(), CliError> {
    let launcher_ui = read_required(&root.join("crates/std-launcher/src/ui.rs"))?;
    for required in ["Color::bg_surface_0(&ctx)", "render_launcher_panel"] {
        check_text(&launcher_ui, required)?;
    }
    let launcher_app = read_required(&root.join("crates/std-launcher/src/app.rs"))?;
    check_text(&launcher_app, "ui::render_launcher_viewport")?;
    let launcher_metrics = read_required(&root.join("crates/std-launcher/src/ui_metrics.rs"))?;
    for required in [
        "scale.f32(PANEL_WIDTH)",
        "native_viewport_is_the_launcher_panel_not_a_carrier",
    ] {
        check_text(&launcher_metrics, required)?;
    }
    for forbidden in ["const CARRIER_MARGIN", "carrier_margin_for_scale"] {
        if launcher_metrics.contains(forbidden) {
            return Err(CliError::Config(
                "launcher must not depend on a visible viewport carrier".to_string(),
            ));
        }
    }
    Ok(())
}

fn check_preview_matrices(root: &std::path::Path) -> Result<(), CliError> {
    let launcher = read_required(&root.join("crates/std-launcher/src/preview.rs"))?;
    for required in [
        "STD_ALLOW_UI_PREVIEW=1 std-launcher --ui-preview",
        "state: \"results\"",
        "state: \"no-results\"",
        "state: \"defer\"",
        "state: \"error\"",
        "theme: \"light\"",
        "theme: \"dark\"",
        "assert_eq!(report.scenarios.len(), 8)",
    ] {
        check_text(&launcher, required)?;
    }
    let studio = read_required(&root.join("crates/std-studio/src/preview.rs"))?;
    for required in [
        "dark-dashboard",
        "light-dashboard",
        "dark-workflow",
        "light-analysis",
        "dark-plugins",
        "light-operations",
        "dark-settings",
        "light-panes",
    ] {
        check_text(&studio, required)?;
    }
    Ok(())
}

fn check_desktop_automation_boundary(root: &std::path::Path) -> Result<(), CliError> {
    let core = read_required(&root.join("crates/std-core/src/lib.rs"))?;
    check_text(&core, "pub fn desktop_automation_allowed()")?;
    check_text(&core, "cfg!(test) || std_test_mode_enabled()")?;
    check_text(&core, "STD_ALLOW_DESKTOP_AUTOMATION")?;
    let guard = read_required(&root.join("crates/std-cli/tests/external_runner_guard.rs"))?;
    check_text(&guard, "binary_test_mode_blocks_dangerous_command_text")?;
    check_text(&guard, "binary_test_mode_blocks_registered_app_launch")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_completion_evidence_tracks_docs_gates_and_opt_in_boundary() {
        let report = check_ui_completion_evidence().unwrap();

        assert_eq!(report.docs, "PASS");
        assert_eq!(report.docs_count, UI_DOCS.len());
        assert_eq!(report.manual_desktop_acceptance, "explicit_opt_in_only");
        assert_eq!(report.desktop_automation_default, "blocked");
        assert_eq!(report.completion, "INCOMPLETE_REAL_GUI_REQUIRED");
        assert!(report.launcher_gates.contains(&"preview-smoke"));
        assert!(report.studio_gates.contains(&"preview-smoke"));
    }
}
