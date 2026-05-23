use crate::{
    doctor::background_acceptance::check_background_acceptance_manifest,
    doctor::ui_capture::check_ui_capture_scripts,
    doctor::workspace::{check_text, read_required},
    CliError,
};
use std::path::Path;

pub(crate) const UI_DOCS: [&str; 7] = [
    "docs/18_UI_Philosophy_and_Visual_Language.md",
    "docs/19_Motion_and_Interaction_Rhythm.md",
    "docs/20_Keyboard_Focus_and_Input.md",
    "docs/21_Launcher_UX_Spec.md",
    "docs/22_Studio_UX_Spec.md",
    "docs/23_Accessibility_and_Localization.md",
    "docs/24_egui_Implementation_Constraints.md",
];

pub(crate) fn check_all_ui_evidence(root: &Path) -> Result<(), CliError> {
    check_ui_docs(root)?;
    check_quality_report_gates(root)?;
    check_runtime_theme_profiles(root)?;
    check_studio_keyboard_evidence(root)?;
    check_studio_operations_evidence(root)?;
    check_studio_workspace_policy_contract(root)?;
    check_launcher_panel_viewport(root)?;
    check_preview_matrices(root)?;
    check_launcher_keyboard_ime_evidence(root)?;
    check_launcher_app_localization_evidence(root)?;
    check_desktop_automation_boundary(root)
}

fn check_ui_docs(root: &Path) -> Result<(), CliError> {
    for doc in UI_DOCS {
        let body = read_required(&root.join(doc))?;
        check_text(&body, "# ")?;
    }
    Ok(())
}

fn check_quality_report_gates(root: &Path) -> Result<(), CliError> {
    let body = read_required(&root.join("crates/std-cli/src/release/quality.rs"))?;
    for required in [
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --theme-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --surface-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --ui-semantics-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --keyboard-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --app-localization-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --user-enter-smoke",
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

fn check_runtime_theme_profiles(root: &Path) -> Result<(), CliError> {
    let egui_tokens = read_required(&root.join("crates/std-egui/src/tokens/style.rs"))?;
    for required in [
        "pub struct ThemeProfile",
        "pub fn apply(ctx: &egui::Context, mode: ThemeMode) -> Self",
        "pub requested: ThemeMode",
        "pub effective: EffectiveTheme",
        "pub high_contrast: bool",
        "pub reduce_motion: bool",
        "pub bold_text: bool",
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

fn check_studio_operations_evidence(root: &Path) -> Result<(), CliError> {
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

fn check_studio_workspace_policy_contract(root: &Path) -> Result<(), CliError> {
    let policy = read_required(&root.join("crates/std-studio/src/workspace_policy.rs"))?;
    let smoke = read_required(&root.join("crates/std-studio/src/smoke/workspace_policy_smoke.rs"))?;
    let guard = read_required(&root.join("crates/std-studio/src/tests/workspace_policy_guard.rs"))?;
    let main_smoke = read_required(&root.join("crates/std-studio/src/smoke/workspace_smoke.rs"))?;
    let evidence = format!("{policy}\n{smoke}\n{guard}\n{main_smoke}");
    for required in [
        "pub extra_viewports: bool",
        "pub show_viewport_api: bool",
        "pub egui_window_api: bool",
        "pub settings_overlay: bool",
        "extra_viewports: false",
        "show_viewport_api: false",
        "egui_window_api: false",
        "settings_overlay: false",
        "pub const fn allows_extra_viewports",
        "pub const fn allows_show_viewport_api",
        "pub const fn allows_egui_window_api",
        "pub const fn allows_settings_overlay",
        "extra_viewports=false",
        "show_viewport_api=false",
        "egui_window_api=false",
        "settings_overlay=false",
        "source_guard=workspace_policy_guard.rs",
        "extra_viewports=forbidden",
        "show_viewport=forbidden",
        "egui_window=forbidden",
        "settings_overlay=forbidden",
    ] {
        check_text(&evidence, required)?;
    }
    Ok(())
}

fn check_studio_keyboard_evidence(root: &Path) -> Result<(), CliError> {
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

fn check_launcher_panel_viewport(root: &Path) -> Result<(), CliError> {
    let launcher_ui = read_required(&root.join("crates/std-launcher/src/ui.rs"))?;
    for required in ["Color::bg_surface_0(&ctx)", "render_launcher_panel"] {
        check_text(&launcher_ui, required)?;
    }
    let launcher_app = read_required(&root.join("crates/std-launcher/src/app.rs"))?;
    check_text(&launcher_app, "ui::render_launcher_viewport")?;
    let launcher_metrics = read_required(&root.join("crates/std-launcher/src/ui_metrics.rs"))?;
    let launcher_layout =
        read_required(&root.join("crates/std-launcher/src/ui_metrics_layout.rs"))?;
    check_text(&launcher_layout, "LauncherSize::host_size")?;
    let launcher_metrics_tests =
        read_required(&root.join("crates/std-launcher/src/ui_metrics_tests.rs"))?;
    check_text(
        &launcher_metrics_tests,
        "panel_rect_owns_entire_native_host",
    )?;
    check_text(
        &launcher_metrics_tests,
        "native_host_is_panel_sized_without_visible_carrier",
    )?;
    let launcher_surface = read_required(&root.join("crates/std-launcher/src/surface_smoke.rs"))?;
    for required in [
        "native_host_window=panel_sized_transparent_host,panel_surface=opaque,host_gutter=0px,no_host_background",
        "capture_window=panel_sized_transparent_host,opt_in_only,panel_surface=opaque,host_gutter=0px,no_host_background",
        "capture_surface=opaque_panel_surface,panel_sized_transparent_host,host_gutter=0px,no_host_background,no_shadow_clip",
        "host-carrier=absent",
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
    for forbidden in [
        "const CARRIER_MARGIN",
        "carrier_margin_for_scale",
        "panel_only=true",
    ] {
        if launcher_metrics.contains(forbidden) || launcher_surface.contains(forbidden) {
            return Err(CliError::Config(
                "launcher must use panel-sized host without visible carrier gutter".to_string(),
            ));
        }
    }
    let forbidden_host_container = concat!("preview", "_viewport");
    if launcher_surface.contains(&format!("{forbidden_host_container}=")) {
        return Err(CliError::Config(
            "launcher screenshot tooling must be capture-only, not an extra host container"
                .to_string(),
        ));
    }
    check_ui_capture_scripts(root)?;
    Ok(())
}

fn check_preview_matrices(root: &Path) -> Result<(), CliError> {
    let launcher_preview = launcher_preview_evidence(root)?;
    let studio_preview = studio_preview_evidence(root)?;
    check_launcher_preview_matrix(&launcher_preview)?;
    check_studio_preview_matrix(&studio_preview)?;
    check_preview_matrix_forbidden_commands(&launcher_preview, &studio_preview)
}

fn launcher_preview_evidence(root: &Path) -> Result<String, CliError> {
    let launcher = read_required(&root.join("crates/std-launcher/src/preview.rs"))?;
    let launcher_evidence =
        read_required(&root.join("crates/std-launcher/src/preview_evidence.rs"))?;
    let launcher_contract =
        read_required(&root.join("crates/std-launcher/src/preview_contract.rs"))?;
    let launcher_surface =
        read_required(&root.join("crates/std-launcher/src/preview_surface_evidence.rs"))?;
    let launcher_acceptance =
        read_required(&root.join("crates/std-launcher/src/screenshot_acceptance.rs"))?;
    let launcher_preview = format!(
        "{launcher}\n{launcher_evidence}\n{launcher_contract}\n{launcher_surface}\n{launcher_acceptance}"
    );
    Ok(launcher_preview)
}

fn studio_preview_evidence(root: &Path) -> Result<String, CliError> {
    let studio = read_required(&root.join("crates/std-studio/src/preview.rs"))?;
    let studio_evidence = read_required(&root.join("crates/std-studio/src/preview_evidence.rs"))?;
    let studio_smoke = read_required(&root.join("crates/std-studio/src/preview_smoke.rs"))?;
    let studio_acceptance =
        read_required(&root.join("crates/std-studio/src/screenshot_acceptance.rs"))?;
    Ok(format!(
        "{studio}\n{studio_evidence}\n{studio_smoke}\n{studio_acceptance}"
    ))
}

fn check_launcher_preview_matrix(launcher_preview: &str) -> Result<(), CliError> {
    for required in [
        "STD_ALLOW_UI_PREVIEW=1 target/ui-capture/debug/std-launcher --ui-preview",
        "state: \"results\"",
        "state: \"no-results\"",
        "state: \"defer\"",
        "state: \"error\"",
        "theme: \"light\"",
        "theme: \"dark\"",
        "fn preview_matrix() -> Vec<LauncherPreviewScenario>",
        "state: \"action-panel\"",
        "self.scenarios == preview_matrix()",
        "panel-sized-transparent-host,opaque-panel-surface,opt-in-only",
        "no-default-window",
        "host-gutter-0px",
        "no-shadow-clip",
        "preview_surface_summary",
        "preview_size_summary",
        "panel_token=bg/surface-0",
        "search_token=bg/surface-1",
        "result_token=bg/surface-1",
        "selected_token=accent/weak",
        "bottom_clearance",
        "budget_source=panel",
        "required_capture_states",
        "light-results",
        "dark-results",
        "light-no-results",
        "dark-no-results",
        "light-defer",
        "dark-defer",
        "light-error",
        "dark-error",
        "light-ime",
        "dark-ime",
        "launcher_screenshot_acceptance",
        "delivery_capture_states",
        "diagnostic_capture_states",
    ] {
        check_text(launcher_preview, required)?;
    }
    if launcher_preview.contains("host-gutter-64px")
        || launcher_preview.contains("transparent-native-host")
        || launcher_preview.contains("host_gap=128x128")
        || launcher_preview.contains("panel_origin=64x64")
    {
        return Err(CliError::Doctor(
            "launcher preview capture contract must use panel-sized transparent host".to_string(),
        ));
    }
    let launcher_preview_without_tests = launcher_preview
        .replace("host-gutter-64px", "")
        .replace("host_gap=128x128", "")
        .replace("panel_origin=64x64", "");
    if launcher_preview_without_tests.contains("transparent_host,panel_surface") {
        return Err(CliError::Doctor(
            "launcher preview evidence must not use transparent host carrier".to_string(),
        ));
    }
    Ok(())
}

fn check_studio_preview_matrix(studio_preview: &str) -> Result<(), CliError> {
    for required in [
        "STD_ALLOW_UI_PREVIEW=1 target/ui-capture/debug/std-studio --ui-preview",
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
        "analysis_preview=coverage=overview:PASS|components:PASS|relations:PASS|history:PASS|complete:PASS",
        "plugin_runtime=runtime=js:Completed:deno_core|ts:Completed:deno_core",
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
        "studio_screenshot_acceptance",
        "delivery_capture_states",
        "workflow_capture_states",
        "diagnostic_capture_states",
    ] {
        check_text(studio_preview, required)?;
    }
    Ok(())
}

fn check_preview_matrix_forbidden_commands(
    launcher_preview: &str,
    studio_preview: &str,
) -> Result<(), CliError> {
    for forbidden in [
        "cargo run -p std-launcher -- --ui-preview",
        "cargo run -p std-studio -- --ui-preview",
    ] {
        if studio_preview.contains(forbidden) || launcher_preview.contains(forbidden) {
            return Err(CliError::Doctor(
                "preview smoke must report target/ui-capture binaries, not cargo run".to_string(),
            ));
        }
    }
    Ok(())
}

fn check_launcher_keyboard_ime_evidence(root: &Path) -> Result<(), CliError> {
    let keyboard = read_required(&root.join("crates/std-launcher/src/keyboard.rs"))?;
    let smoke = read_required(&root.join("crates/std-launcher/src/keyboard_smoke.rs"))?;
    let user_enter = read_required(&root.join("crates/std-launcher/src/user_enter_smoke.rs"))?;
    let evidence = format!("{keyboard}\n{smoke}\n{user_enter}");
    for required in [
        "if ime_composing",
        "ime_composition_path",
        "zh-preedit({query_before_preedit})>blocked>commit({commit_query})>enter",
        "ime_commit_trigger_status",
        "user_enter_status",
        "user_enter_route",
        "Enter>handle_keyboard_input_by_user>ReviewFirst",
        "user_enter_deferred",
        "user_enter_open_contract",
        "ui_enter=handle_keyboard_input_by_user",
        "mode=ReviewFirst",
        "default=review-command",
        "run=ActionPanel>Run",
        "desktop_open=default_review_first",
        "explicit_run_status=NeedsExternalRunner",
        "explicit_run_reason=STD_TEST_MODE blocked desktop open",
        "hide_policy=Completed->hide,NeedsExternalRunner->keep-open",
    ] {
        check_text(&evidence, required)?;
    }
    Ok(())
}

fn check_launcher_app_localization_evidence(root: &Path) -> Result<(), CliError> {
    let smoke = read_required(&root.join("crates/std-launcher/src/app_localization_smoke.rs"))?;
    let cli = read_required(&root.join("crates/std-launcher/src/cli.rs"))?;
    let core = read_required(&root.join("crates/std-core/src/app_bundle.rs"))?;
    let core_profile = read_required(&root.join("crates/std-core/src/app_bundle_profile.rs"))?;
    let tests = read_required(&root.join("crates/std-launcher/src/app_tests.rs"))?;
    let evidence = format!("{smoke}\n{cli}\n{core}\n{core_profile}\n{tests}");
    for required in [
        "--app-localization-smoke",
        "launcher_app_localization_smoke",
        "queries=wechat|weixin|",
        "fixture_scope=local_apps_dir_only",
        "system_apps_scanned=false",
        "CFBundleDisplayName",
        "InfoPlist.strings",
        "read_localized_info_plist_names",
        "derived_aliases",
        "wechat",
        "weixin",
        "\\\\U5fae\\\\U4fe1",
        "ActionExecutionStatus::NeedsExternalRunner",
        "launcher_searches_wechat_by_macos_multilingual_names_without_launching",
    ] {
        check_text(&evidence, required)?;
    }
    Ok(())
}

fn check_desktop_automation_boundary(root: &Path) -> Result<(), CliError> {
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
    check_text(&guard, "binary_test_mode_blocks_registered_app_launch")?;
    check_background_acceptance_manifest()
}
