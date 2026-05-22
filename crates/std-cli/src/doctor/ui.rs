use crate::{
    doctor::ui_capture::check_ui_capture_scripts,
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

const STUDIO_GATES: [&str; 5] = [
    "smoke",
    "workspace-policy-smoke",
    "theme-smoke",
    "surface-smoke",
    "preview-smoke",
];

pub(crate) struct UiDoctor {
    pub(crate) docs: &'static str,
    pub(crate) docs_count: usize,
    pub(crate) launcher_gates: Vec<&'static str>,
    pub(crate) studio_gates: Vec<&'static str>,
    pub(crate) manual_desktop_acceptance: &'static str,
    pub(crate) background_ui_acceptance: &'static str,
    pub(crate) desktop_automation_default: &'static str,
    pub(crate) completion: &'static str,
}

pub(crate) fn check_ui_completion_evidence() -> Result<UiDoctor, CliError> {
    let root = find_workspace_root()?;
    check_ui_docs(&root)?;
    check_quality_report_gates(&root)?;
    check_runtime_theme_profiles(&root)?;
    check_studio_keyboard_evidence(&root)?;
    check_studio_operations_evidence(&root)?;
    check_launcher_panel_viewport(&root)?;
    check_preview_matrices(&root)?;
    check_launcher_keyboard_ime_evidence(&root)?;
    check_desktop_automation_boundary(&root)?;
    Ok(UiDoctor {
        docs: "PASS",
        docs_count: UI_DOCS.len(),
        launcher_gates: LAUNCHER_GATES.to_vec(),
        studio_gates: STUDIO_GATES.to_vec(),
        manual_desktop_acceptance: "explicit_opt_in_only",
        background_ui_acceptance: "explicit_opt_in_only",
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
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --theme-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --surface-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --ui-semantics-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --keyboard-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --preview-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --workspace-policy-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --theme-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --surface-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --preview-smoke",
        "manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space",
        "lines.push(format!(\"background_ui_acceptance={command}\"))",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-acceptance.sh",
        "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke",
        "--harness-pid <pid>",
        "--window-id <window-id>",
        "--bundle-id dev.std-cli.background-ui-harness",
        "--window-title \\\"std-cli Background UI Harness <token>\\\"",
        "--harness-token <token>",
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
    check_text(&launcher, "ThemeProfile::apply_with_accessibility(")?;
    check_text(&launcher, "config.reduce_motion()")?;
    check_text(&launcher, "config.high_contrast()")?;
    check_text(&launcher, "config.reduce_transparency()")?;
    check_text(&launcher, "config.ui_scale()")?;
    let studio = read_required(&root.join("crates/std-studio/src/main.rs"))?;
    check_text(&studio, "pub(crate) theme_profile: Option<ThemeProfile>")?;
    check_text(&studio, "self.theme_profile = Some(ui::install_visuals")?;
    Ok(())
}

fn check_studio_operations_evidence(root: &std::path::Path) -> Result<(), CliError> {
    let operations = read_required(&root.join("crates/std-studio/src/smoke/operations_smoke.rs"))?;
    for required in [
        "operations_qa_command",
        "mise run quality",
        "operations_doctor_command",
        "std doctor",
        "operations_release_command",
        "std release verify",
        "operations_install_command",
        "std install verify",
    ] {
        check_text(&operations, required)?;
    }
    let studio_smoke = read_required(&root.join("crates/std-studio/src/smoke.rs"))?;
    check_text(&operations, "operations_smoke=")?;
    check_text(
        &studio_smoke,
        "operations_summary: inputs.operations.summary()",
    )?;
    Ok(())
}

fn check_studio_keyboard_evidence(root: &std::path::Path) -> Result<(), CliError> {
    let keyboard = read_required(&root.join("crates/std-studio/src/smoke/keyboard_smoke.rs"))?;
    let keyboard_tests = read_required(&root.join("crates/std-studio/src/smoke_tests.rs"))?;
    let evidence = format!("{keyboard}\n{keyboard_tests}");
    for required in [
        "studio_keyboard_smoke=PASS",
        "studio_sidebar_toggle_path={}:open>closed>open",
        "studio_inspector_toggle_path={}:closed>open>closed",
        "studio_bottom_panel_toggle_path={}:closed>open>closed",
        "studio_command_palette_path={}|{}:closed>command",
        "studio_quick_open_path={}:command>quick-open",
        "dashboard>plugins>settings>dashboard",
        "target>tabs>content>query>coverage>target",
        "?:coverage>query",
        "docs/20#studio-shortcuts",
    ] {
        check_text(&evidence, required)?;
    }
    let studio_smoke = read_required(&root.join("crates/std-studio/src/smoke.rs"))?;
    check_text(&studio_smoke, "StudioKeyboardSmoke::run")?;
    check_text(&studio_smoke, "keyboard_summary: inputs.keyboard.summary()")?;
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
    let launcher_layout =
        read_required(&root.join("crates/std-launcher/src/ui_metrics_layout.rs"))?;
    check_text(&launcher_layout, "scale.f32(PANEL_WIDTH)")?;
    let launcher_metrics_tests =
        read_required(&root.join("crates/std-launcher/src/ui_metrics_tests.rs"))?;
    check_text(
        &launcher_metrics_tests,
        "panel_rect_matches_native_host_window_height",
    )?;
    let launcher_surface = read_required(&root.join("crates/std-launcher/src/surface_smoke.rs"))?;
    for required in [
        "native_host_window=transparent_carrier,panel_surface=opaque,no_carrier_background",
        "capture_window=transparent_carrier,opt_in_only,panel_surface=opaque,no_carrier_background",
        "capture_surface=opaque_panel_surface,transparent_carrier,no_carrier_background,no_shadow_clip",
    ] {
        check_text(&launcher_surface, required)?;
    }
    let capture_script = read_required(&root.join("scripts/capture-window.sh"))?;
    for required in [
        "STD_ALLOW_UI_PREVIEW",
        "capture-window SKIP",
        "cg-capture-window.swift",
    ] {
        check_text(&capture_script, required)?;
    }
    for forbidden in ["const CARRIER_MARGIN", "carrier_margin_for_scale"] {
        if launcher_metrics.contains(forbidden) {
            return Err(CliError::Config(
                "launcher must not depend on a visible viewport carrier".to_string(),
            ));
        }
    }
    if launcher_surface.contains("preview_viewport=") {
        return Err(CliError::Config(
            "launcher screenshot tooling must be capture-only, not a preview viewport".to_string(),
        ));
    }
    check_ui_capture_scripts(root)?;
    Ok(())
}

fn check_preview_matrices(root: &std::path::Path) -> Result<(), CliError> {
    let launcher = read_required(&root.join("crates/std-launcher/src/preview.rs"))?;
    for required in [
        "STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher -- --ui-preview",
        "state: \"results\"",
        "state: \"no-results\"",
        "state: \"defer\"",
        "state: \"error\"",
        "theme: \"light\"",
        "theme: \"dark\"",
        "fn preview_matrix() -> Vec<LauncherPreviewScenario>",
        "state: \"action-panel\"",
        "self.scenarios == preview_matrix()",
        "transparent-carrier,opaque-panel-surface,opt-in-only",
        "no-default-window",
        "preview_surface_summary",
        "preview_size_summary",
        "panel_token=bg/surface-0",
        "search_token=bg/surface-1",
        "result_token=bg/surface-1",
        "selected_token=accent/weak",
        "bottom_clearance",
        "selected_token=accent/weak",
        "required_capture_states",
        "light-results",
        "dark-results",
        "light-no-results",
        "dark-no-results",
        "light-defer",
        "dark-defer",
        "light-error",
        "dark-error",
    ] {
        let launcher_evidence =
            read_required(&root.join("crates/std-launcher/src/preview_evidence.rs"))?;
        if launcher.contains(required) || launcher_evidence.contains(required) {
            continue;
        }
        return Err(CliError::Doctor(format!(
            "required text missing: {required}"
        )));
    }
    let studio = read_required(&root.join("crates/std-studio/src/preview.rs"))?;
    let studio_evidence = read_required(&root.join("crates/std-studio/src/preview_evidence.rs"))?;
    let studio_preview = format!("{studio}\n{studio_evidence}");
    for required in [
        "dark-dashboard",
        "light-dashboard",
        "dark-workflow",
        "light-analysis",
        "dark-plugins",
        "light-plugin-permission",
        "dark-plugin-permission",
        "light-operations",
        "dark-settings",
        "light-panes",
        "preview_size_summary",
        "canvas_token=bg/surface-0",
        "panel_token=bg/surface-1",
        "selected_token=accent/weak",
        "host={},min={},workspace={}",
        "native_child_windows={},detached_panels={}",
        "required_capture_states",
        "light-dashboard",
        "dark-dashboard",
        "light-workflow",
        "dark-workflow",
        "light-analysis",
        "dark-analysis",
        "light-plugins",
        "dark-plugins",
        "light-plugin-permission",
        "dark-plugin-permission",
        "light-operations",
        "dark-operations",
        "light-settings",
        "dark-settings",
        "light-panes",
        "dark-panes",
    ] {
        check_text(&studio_preview, required)?;
    }
    Ok(())
}

fn check_launcher_keyboard_ime_evidence(root: &std::path::Path) -> Result<(), CliError> {
    let keyboard = read_required(&root.join("crates/std-launcher/src/keyboard.rs"))?;
    let smoke = read_required(&root.join("crates/std-launcher/src/keyboard_smoke.rs"))?;
    let evidence = format!("{keyboard}\n{smoke}");
    for required in [
        "if ime_composing",
        "ime_composition_path",
        "zh-preedit({query_before_preedit})>blocked>commit({commit_query})>enter",
        "ime_commit_trigger_status",
        "user_enter_status",
        "user_enter_route",
        "Enter>handle_keyboard_input_by_user>LauncherUser",
        "user_enter_deferred",
    ] {
        check_text(&evidence, required)?;
    }
    Ok(())
}

fn check_desktop_automation_boundary(root: &std::path::Path) -> Result<(), CliError> {
    let core = read_required(&root.join("crates/std-core/src/lib.rs"))?;
    check_text(&core, "pub fn desktop_automation_allowed()")?;
    check_text(&core, "pub fn desktop_integration_allowed()")?;
    check_text(&core, "cfg!(test) || std_test_mode_enabled()")?;
    check_text(&core, "STD_ALLOW_DESKTOP_AUTOMATION")?;
    check_text(&core, "STD_ALLOW_BACKGROUND_UI_AUTOMATION")?;
    let cli_ui = read_required(&root.join("crates/std-cli/src/ui/background.rs"))?;
    check_text(&cli_ui, "STD_TEST_MODE blocks background UI automation")?;
    check_text(&cli_ui, "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 required")?;
    check_text(&cli_ui, "isolated_background_ui_harness_only")?;
    check_text(
        &cli_ui,
        "target_identity=fixed_bundle_pid_window_title_quadruple",
    )?;
    check_text(&cli_ui, "event_route=postToPid_target_pid_only")?;
    check_text(
        &cli_ui,
        "event_tap_then_appkit_defined_primer_then_center_primer",
    )?;
    check_text(&cli_ui, "fallback=never_frontmost_desktop_click")?;
    let quality_doc = read_required(&root.join("docs/14_Code_Quality.md"))?;
    check_text(&quality_doc, "per-process event tap")?;
    check_text(&quality_doc, "appKitDefined primer")?;
    check_text(&quality_doc, "center primer")?;
    check_text(&quality_doc, "隔离 harness")?;
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
        assert_eq!(report.background_ui_acceptance, "explicit_opt_in_only");
        assert_eq!(report.desktop_automation_default, "blocked");
        assert_eq!(report.completion, "INCOMPLETE_REAL_GUI_REQUIRED");
        assert!(report.launcher_gates.contains(&"preview-smoke"));
        assert!(report.studio_gates.contains(&"surface-smoke"));
        assert!(report.studio_gates.contains(&"preview-smoke"));
    }
}
