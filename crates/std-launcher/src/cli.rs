use crate::gui_smoke::{run_gui_hotkey_smoke, GuiHotkeySmokeConfig};
use crate::preview::LauncherPreviewSmokeReport;
use std_egui::tokens::ThemeSmokeReport;
use std_launcher::{
    hotkey_smoke, HotkeySmokeReport, LauncherActionPanelSmokeReport,
    LauncherAppLocalizationSmokeReport, LauncherCloseSmokeReport, LauncherKeyboardReport,
    LauncherSmokeReport, LauncherState, LauncherSurfaceSmokeReport, LauncherUiSemanticsReport,
    LauncherUserEnterSmokeReport, LauncherWindowSmokeReport,
};

enum LauncherCliSmoke {
    Launcher(LauncherSmokeReport),
    Hotkey(HotkeySmokeReport),
    Window(LauncherWindowSmokeReport),
    Keyboard(Box<LauncherKeyboardReport>),
    ActionPanel(LauncherActionPanelSmokeReport),
    AppLocalization(LauncherAppLocalizationSmokeReport),
    Close(LauncherCloseSmokeReport),
    UserEnter(LauncherUserEnterSmokeReport),
    UiSemantics(Box<LauncherUiSemanticsReport>),
    Surface(Box<LauncherSurfaceSmokeReport>),
    Preview(LauncherPreviewSmokeReport),
    GuiHotkey(GuiHotkeySmokeConfig),
    Theme(ThemeSmokeReport),
}

pub(crate) fn run_smoke_from_args(args: Vec<String>) -> eframe::Result<bool> {
    match smoke_from_args(args) {
        Some(LauncherCliSmoke::Launcher(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Hotkey(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Window(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Keyboard(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::ActionPanel(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::AppLocalization(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Close(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::UserEnter(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::UiSemantics(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Surface(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Preview(report)) => {
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::GuiHotkey(config)) => {
            let report = run_gui_hotkey_smoke(config)?;
            println!("{}", report.summary());
            Ok(true)
        }
        Some(LauncherCliSmoke::Theme(report)) => {
            println!("{}", report.summary("launcher"));
            Ok(true)
        }
        None => Ok(false),
    }
}

fn smoke_from_args(args: Vec<String>) -> Option<LauncherCliSmoke> {
    match args.get(1).map(String::as_str) {
        Some("--smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("rebuild index");
            LauncherState::smoke(query).map(LauncherCliSmoke::Launcher)
        }
        Some("--hotkey-smoke") => {
            let accelerator = args
                .get(2)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("Alt+Space");
            Some(LauncherCliSmoke::Hotkey(hotkey_smoke(accelerator)))
        }
        Some("--window-smoke") => Some(LauncherCliSmoke::Window(LauncherState::window_smoke())),
        Some("--keyboard-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::Keyboard(Box::new(
                LauncherState::keyboard_smoke(query),
            )))
        }
        Some("--action-panel-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::ActionPanel(
                LauncherState::action_panel_smoke(query),
            ))
        }
        Some("--app-localization-smoke") => Some(LauncherCliSmoke::AppLocalization(
            LauncherAppLocalizationSmokeReport::run(),
        )),
        Some("--close-smoke") => Some(LauncherCliSmoke::Close(LauncherState::close_smoke())),
        Some("--user-enter-smoke") => Some(LauncherCliSmoke::UserEnter(
            LauncherUserEnterSmokeReport::run(),
        )),
        Some("--ui-semantics-smoke") => {
            let query = args
                .get(2)
                .map(String::as_str)
                .filter(|query| !query.trim().is_empty())
                .unwrap_or("index");
            Some(LauncherCliSmoke::UiSemantics(Box::new(
                LauncherState::ui_semantics_smoke(query),
            )))
        }
        Some("--surface-smoke") => Some(LauncherCliSmoke::Surface(Box::new(
            LauncherSurfaceSmokeReport::new(),
        ))),
        Some("--preview-smoke") => {
            Some(LauncherCliSmoke::Preview(LauncherPreviewSmokeReport::new()))
        }
        Some("--theme-smoke") => Some(LauncherCliSmoke::Theme(ThemeSmokeReport::new())),
        Some("--gui-hotkey-smoke") => Some(LauncherCliSmoke::GuiHotkey(GuiHotkeySmokeConfig {
            accelerator: args
                .get(2)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or("Alt+Space")
                .to_string(),
            timeout_ms: args
                .get(3)
                .and_then(|value| value.parse::<u64>().ok())
                .unwrap_or(5_000),
            trigger_delay_ms: 500,
            allow_system_events: std_core::desktop_automation_allowed(),
        })),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ui_preview_args_are_not_smoke_args() {
        let args = vec![
            "std-launcher".to_string(),
            "--ui-preview".to_string(),
            "light".to_string(),
            "defer".to_string(),
            "1200".to_string(),
        ];

        assert!(smoke_from_args(args).is_none());
    }

    #[test]
    fn gui_hotkey_smoke_requires_desktop_automation_opt_in() {
        let args = vec![
            "std-launcher".to_string(),
            "--gui-hotkey-smoke".to_string(),
            "Alt+Space".to_string(),
        ];
        let Some(LauncherCliSmoke::GuiHotkey(config)) = smoke_from_args(args) else {
            panic!("expected GUI hotkey smoke config");
        };

        assert!(!config.allow_system_events);
    }

    #[test]
    fn app_localization_smoke_reports_multilingual_aliases() {
        let args = vec![
            "std-launcher".to_string(),
            "--app-localization-smoke".to_string(),
        ];
        let Some(LauncherCliSmoke::AppLocalization(report)) = smoke_from_args(args) else {
            panic!("expected app localization smoke");
        };

        assert_eq!(report.status, "PASS");
        assert!(report.summary().contains("queries=wechat|weixin|"));
        assert!(report.summary().contains("action_ids_match=true"));
        assert!(report
            .summary()
            .contains("enter_status=NeedsExternalRunner"));
        assert!(report.summary().contains("deferred=true"));
        assert!(report
            .summary()
            .contains("fixture_scope=local_apps_dir_only"));
        assert!(report.summary().contains("system_apps_scanned=false"));
    }

    #[test]
    fn user_enter_smoke_reports_launcher_user_route_without_desktop_open() {
        let args = vec!["std-launcher".to_string(), "--user-enter-smoke".to_string()];
        let Some(LauncherCliSmoke::UserEnter(report)) = smoke_from_args(args) else {
            panic!("expected user enter smoke");
        };

        assert!(report.pass(), "{}", report.summary());
        assert!(report.summary().contains("launcher_user_enter_smoke PASS"));
        assert!(report
            .summary()
            .contains("route=Enter>handle_keyboard_input_by_user"));
        assert!(report.summary().contains("mode=ReviewFirst"));
        assert!(report.summary().contains("status=NeedsExternalRunner"));
        assert!(report
            .summary()
            .contains("real_execution_gate=installed-hotkey-or-background-ui-acceptance"));
    }

    #[test]
    fn gui_hotkey_smoke_skips_when_test_mode_is_active() {
        let report = run_gui_hotkey_smoke(GuiHotkeySmokeConfig {
            accelerator: "Alt+Space".to_string(),
            timeout_ms: 5_000,
            trigger_delay_ms: 500,
            allow_system_events: true,
        })
        .unwrap();

        assert_eq!(report.status, "SKIP");
        assert!(!report.registered);
        assert!(!report.input_sent);
        assert!(report
            .error
            .as_deref()
            .unwrap()
            .contains("STD_TEST_MODE blocked GUI hotkey smoke"));
    }

    #[test]
    fn theme_smoke_reports_launcher_light_and_dark_tokens() {
        let args = vec!["std-launcher".to_string(), "--theme-smoke".to_string()];
        let Some(LauncherCliSmoke::Theme(report)) = smoke_from_args(args) else {
            panic!("expected theme smoke report");
        };

        assert!(report.pass());
        assert!(report
            .summary("launcher")
            .contains("launcher_theme_smoke PASS"));
        assert!(report
            .summary("launcher")
            .contains("dark_accent_weak_alpha=46"));
        assert!(report
            .summary("launcher")
            .contains("light_accent_weak_alpha=31"));
        assert!(report
            .summary("launcher")
            .contains("high_contrast_dark_accent_weak_alpha=82"));
        assert!(report
            .summary("launcher")
            .contains("high_contrast_light_accent_weak_alpha=56"));
        assert!(report.summary("launcher").contains(
            "typography_contract=text=caption:11,footnote:12,body:13,title:15,headline:18,display:24,code:12"
        ));
        assert!(report.summary("launcher").contains(
            "status_contract=status=success:#3DCB7C/#138750,warning:#F5B643/#B27500,danger:#FF6A6A/#C8312B,info:#4E9CFF/#0A6BFF"
        ));
        assert!(report
            .summary("launcher")
            .contains("no_pure_black_white_tokens=true"));
    }

    #[test]
    fn surface_smoke_reports_launcher_visual_state_contract() {
        let args = vec!["std-launcher".to_string(), "--surface-smoke".to_string()];
        let Some(LauncherCliSmoke::Surface(report)) = smoke_from_args(args) else {
            panic!("expected surface smoke report");
        };

        assert!(report.pass(), "{}", report.summary());
        assert!(report.summary().contains("launcher_surface_smoke PASS"));
        assert!(report.summary().contains("panel_opaque=true"));
        assert!(report
            .summary()
            .contains("native_clear_color=transparent_rgba_0_0_0_0"));
        assert!(report
            .summary()
            .contains("viewport_frame_contract=viewport_frame=transparent_fill,no_stroke"));
        assert!(report
            .summary()
            .contains("feedback_icon_contract=status_icons=completed|deferred|failed"));
        assert!(report
            .summary()
            .contains("native_host_window=transparent_host,panel_surface=opaque,host_gutter=64px,no_host_background"));
        assert!(report
            .summary()
            .contains("capture_window=transparent_host,opt_in_only,panel_surface=opaque,host_gutter=64px,no_host_background"));
        assert!(report
            .summary()
            .contains("capture_surface=opaque_panel_surface,transparent_host"));
        assert!(report
            .summary()
            .contains("capture_pixel_contract=capture_pixels="));
        assert!(report.summary().contains("center-panel-opaque-non-carrier"));
        assert!(report.summary().contains("host-carrier-zero"));
        assert!(report.summary().contains("edge-black-white-zero"));
        assert!(report.summary().contains("standard_launcher_enter_ms=320"));
        assert!(report.summary().contains("reduced_launcher_enter_ms=0"));
        assert!(report.summary().contains("reduced_launcher_exit_ms=0"));
        assert!(report.summary().contains("reduced_focus_ring_ms=0"));
        assert!(report.summary().contains(
            "visible_structure_contract=search=input|placeholder|focus-ring|mode-tag|ime-chip"
        ));
        assert!(report
            .summary()
            .contains("results=group-header|row-icon|title|subtitle|keycap|enter-action"));
        assert!(report
            .summary()
            .contains("states=empty|no-results|loading|executing|defer|error"));
    }

    #[test]
    fn preview_smoke_reports_required_screenshot_matrix() {
        let args = vec!["std-launcher".to_string(), "--preview-smoke".to_string()];
        let Some(LauncherCliSmoke::Preview(report)) = smoke_from_args(args) else {
            panic!("expected preview smoke report");
        };

        assert!(report.pass(), "{}", report.summary());
        assert_preview_summary_has_required_states(&report.summary());
        assert_preview_summary_has_theme_tokens(&report.summary());
        assert_preview_summary_has_state_surfaces(&report.summary());
        assert_preview_summary_has_overlay_structure(&report.summary());
    }

    fn assert_preview_summary_has_required_states(summary: &str) {
        for expected in [
            "launcher_preview_smoke PASS",
            "light-empty",
            "dark-results",
            "light-no-results",
            "dark-no-results",
            "dark-searching",
            "light-loading",
            "dark-loading",
            "light-executing",
            "light-defer",
            "dark-defer",
            "light-error",
            "light-ime",
            "dark-action-panel",
            "STD_ALLOW_UI_PREVIEW=1",
            "target/ui-capture/debug/std-launcher --ui-preview",
        ] {
            assert!(summary.contains(expected), "{expected}");
        }
        assert!(!summary.contains("cargo run -p std-launcher -- --ui-preview"));
        for expected in [
            "required_capture_states=",
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
        ] {
            assert!(summary.contains(expected), "{expected}");
        }
    }

    fn assert_preview_summary_has_theme_tokens(summary: &str) {
        for expected in [
            "panel_token=bg/surface-0:#FAFBFD",
            "panel_token=bg/surface-0:#1C1E22",
            "search_token=bg/surface-1:#F2F5F8",
            "search_token=bg/surface-1:#24272C",
            "selected_token=accent/weak:#0A6BFF@31",
            "selected_token=accent/weak:#4E9CFF@46",
        ] {
            assert!(summary.contains(expected), "{expected}");
        }
    }

    fn assert_preview_summary_has_state_surfaces(summary: &str) {
        for expected in [
            "state_surface=panel_only_surface:true,search:panel-as-search-surface",
            "host_contract=native_clear_color=transparent_rgba_0_0_0_0,viewport_frame=transparent_fill,no_stroke;native_host=",
            "host_background=none",
            "panel_surface=opaque",
            "panel_origin=64x64",
            "host_gap=128x128",
            "panel_only_surface=true",
            "forbidden=black_or_white_host_background",
            "body:loading-progress-token-surface",
            "feedback:status-warning-weak",
            "feedback:status-danger-weak",
            "status_icon=deferred",
            "status_icon=failed",
            "popover:bg/surface-1+elev/2",
        ] {
            assert!(summary.contains(expected), "{expected}");
        }
    }

    fn assert_preview_summary_has_overlay_structure(summary: &str) {
        for expected in [
            "structure=overlay:single-overlay",
            "search:visible",
            "results:grouped:",
            "results:suggestions:3",
            "results:empty-state",
            "results:progress",
            "preview:action-bar-summary:selected-row",
            "action_bar:visible",
            "feedback:visible",
            "action_panel:foreground-popover",
            "ownership:egui-panel-no-native-child",
        ] {
            assert!(summary.contains(expected), "{expected}");
        }
    }
}
