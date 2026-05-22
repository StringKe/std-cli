use crate::{preview::*, ui};
use std_egui::tokens::ThemeMode;
use std_types::ActionExecutionStatus;

#[test]
fn ui_preview_args_are_explicit_opt_in() {
    let args = vec![
        "std-launcher".to_string(),
        "--ui-preview".to_string(),
        "light".to_string(),
        "defer".to_string(),
        "1200".to_string(),
    ];
    let config = preview_config_from_args(&args).unwrap();

    assert_eq!(config.theme_mode, ThemeMode::Light);
    assert_eq!(config.scenario, "defer");
    assert_eq!(config.timeout_ms, 1200);
}

#[test]
fn ui_preview_args_are_blocked_without_opt_in() {
    std::env::set_var("STD_ALLOW_UI_PREVIEW", "0");
    let args = vec![
        "std-launcher".to_string(),
        "--ui-preview".to_string(),
        "light".to_string(),
        "defer".to_string(),
        "1200".to_string(),
    ];

    let Some(LauncherPreviewRequest::Blocked(reason)) = preview_request_from_args(&args) else {
        panic!("expected blocked UI preview request");
    };
    assert!(reason.contains("STD_TEST_MODE blocked UI preview"));
    assert!(blocked_preview_summary(&reason).contains("launcher_ui_preview SKIP"));
}

#[test]
fn preview_smoke_commands_match_ui_preview_parser_contract() {
    let report = LauncherPreviewSmokeReport::new();

    assert!(report.pass(), "{}", report.summary());
    assert_eq!(report.scenarios.len(), 22);
    assert!(report
        .commands
        .iter()
        .all(|command| command.starts_with("STD_ALLOW_UI_PREVIEW=1 cargo run -p std-launcher")));
    assert!(report
        .commands
        .iter()
        .all(|command| command.contains(" --ui-preview light ")
            || command.contains(" --ui-preview dark ")));
    assert_eq!(report.sizes.len(), report.scenarios.len());
    assert!(report.sizes.iter().all(|size| size.contains("=PASS")));
    assert_preview_state_matrix(&report);
    assert_preview_theme_tokens(&report);
    assert_preview_affordance_contract(&report);
    assert_required_capture_state_contract(&report);
    assert_preview_capture_contract(&report);
    assert_preview_capture_manifest_contract(&report);
    assert_preview_ui_completion_boundary(&report);
}

fn assert_preview_state_matrix(report: &LauncherPreviewSmokeReport) {
    for state in [
        "light-empty=PASS",
        "light-collapsed=PASS",
        "dark-searching=PASS",
        "light-loading=PASS",
        "dark-loading=PASS",
        "light-executing=PASS",
        "light-no-results=PASS",
        "dark-error=PASS",
        "light-ime=PASS",
        "light-action-panel=PASS",
    ] {
        assert!(report.states.iter().any(|entry| entry.starts_with(state)));
    }
    assert!(report
        .sizes
        .iter()
        .any(|size| size.starts_with("light-defer=PASS")));
    assert!(report
        .sizes
        .iter()
        .any(|size| size.starts_with("dark-error=PASS")));
}

fn assert_preview_theme_tokens(report: &LauncherPreviewSmokeReport) {
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("panel_token=bg/surface-0:#FAFBFD")
            && state.contains("search_token=bg/surface-1:#F2F5F8")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("panel_token=bg/surface-0:#1C1E22")
            && state.contains("search_token=bg/surface-1:#24272C")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("selected_token=accent/weak:#0A6BFF@31")));
    assert!(report
        .states
        .iter()
        .any(|state| state.contains("selected_token=accent/weak:#4E9CFF@46")));
}

fn assert_preview_affordance_contract(report: &LauncherPreviewSmokeReport) {
    let summary = report.summary();

    assert!(summary.contains("suggested=3"));
    assert!(summary.contains("ask_ai=true"));
    assert!(summary.contains("no_match_fallback=ask_ai_row,visible=true"));
    assert!(summary.contains("no_match_fallback=ask_ai_row,visible=true,selected=true"));
    assert!(summary.contains("enter_keycap=true,button_semantics=true"));
    assert!(summary.contains("feedback_actions=Copy,Retry"));
    assert!(summary.contains("feedback_actions=Copy,Retry,OpenStudio"));
    assert!(summary.contains(&format!(
        "feedback_action_shortcuts=Copy:{enter},Retry:{enter}",
        enter = std_egui::input::enter().label()
    )));
    assert!(summary.contains(&format!(
        "feedback_action_shortcuts=Copy:{enter},Retry:{enter},OpenStudio:{enter}",
        enter = std_egui::input::enter().label()
    )));
    assert!(summary.contains(&format!(
        "action_panel_actions={},{},{},{}",
        std_egui::i18n::t("launcher.action.review_first"),
        std_egui::i18n::t("launcher.action.defer"),
        std_egui::i18n::t("launcher.action.open_in_studio"),
        std_egui::i18n::t("launcher.action.copy_command")
    )));
    assert!(summary.contains("dark-searching=PASS:phase=Searching,results=0,feedback=none"));
    assert!(summary.contains(
        "state_behavior=search_indicator:search,loading_progress:2px-accent-indeterminate,empty_progress:not-rendered,input:editable"
    ));
    assert!(summary.contains("dark-loading=PASS:phase=Searching,results=0,feedback=none"));
    assert!(summary.contains(
        "state_behavior=search_indicator:spinner,loading_progress:not-rendered,empty_progress:visible,input:editable"
    ));
    assert!(summary.contains("state_behavior=search_indicator:executing"));
    assert!(summary.contains("input:locked"));
    assert!(summary.contains("action_bar:cancel-and-background-hints"));
    assert!(summary.contains("action_bar:feedback-actions"));
    assert!(summary.contains("action_bar:action-panel-open"));
    assert!(summary.contains("preview=action-bar-summary|result-row-action-hint"));
    assert!(!summary.contains("preview_panel_contract"));
}

fn assert_required_capture_state_contract(report: &LauncherPreviewSmokeReport) {
    assert_eq!(
        report.required_capture_states,
        [
            "light-collapsed",
            "dark-collapsed",
            "light-empty",
            "dark-empty",
            "light-results",
            "dark-results",
            "light-no-results",
            "dark-no-results",
            "light-searching",
            "dark-searching",
            "light-loading",
            "dark-loading",
            "light-executing",
            "dark-executing",
            "light-defer",
            "dark-defer",
            "light-error",
            "dark-error",
            "light-ime",
            "dark-ime",
            "light-action-panel",
            "dark-action-panel",
        ]
    );

    let summary = report.summary();
    assert!(summary.contains(
        "required_capture_states=light-collapsed,dark-collapsed,light-empty,dark-empty,light-results,dark-results,light-no-results,dark-no-results,light-searching,dark-searching,light-loading,dark-loading,light-executing,dark-executing,light-defer,dark-defer,light-error,dark-error,light-ime,dark-ime,light-action-panel,dark-action-panel"
    ));
}

fn assert_preview_capture_contract(report: &LauncherPreviewSmokeReport) {
    let summary = report.summary();

    assert!(summary.contains(
        "preview_capture_contract=transparent-native-host,opaque-panel-surface,opt-in-only"
    ));
    assert!(summary.contains("checkout-binary-only"));
    assert!(summary.contains("blocked-in-STD_TEST_MODE"));
    assert!(summary.contains("no-default-window"));
    assert!(summary.contains("no-host-background"));
}

fn assert_preview_capture_manifest_contract(report: &LauncherPreviewSmokeReport) {
    let summary = report.summary();

    assert!(report.capture_manifest.pass(&report.scenarios));
    assert!(
        summary.contains("expected_capture_manifest=artifacts/ui/manual-acceptance/manifest.txt")
    );
    assert!(summary.contains("capture_out_dir=artifacts/ui/manual-acceptance"));
    assert!(summary.contains("expected_capture_files=launcher-light-collapsed.png"));
    assert!(summary.contains("launcher-dark-action-panel.png"));
    assert!(summary.contains("capture_command=STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix"));
    assert!(summary.contains("verify_rule=manifest-current-run-png-files-by-theme-state"));
}

fn assert_preview_ui_completion_boundary(report: &LauncherPreviewSmokeReport) {
    let summary = report.summary();

    assert!(summary.contains("ui_completion_boundary=headless-preview-is-not-ui-completion"));
    assert!(summary.contains("manual_ui_evidence_gates=launcher-light-dark-screenshots"));
    assert!(summary.contains("launcher-results-no-results-defer-error-screenshots"));
    assert!(summary.contains("launcher-keyboard-navigation-ime"));
    assert!(summary.contains("launcher-installed-hotkey-toggle"));
}

#[test]
fn preview_smoke_sizes_prove_capture_window_has_transparent_host() {
    let report = LauncherPreviewSmokeReport::new();

    assert!(report.pass(), "{}", report.summary());
    assert!(report
        .summary()
        .contains("preview_sizes=light-collapsed=PASS"));
    assert!(report.summary().contains("light-empty=PASS"));
    assert!(report
        .summary()
        .contains("panel_frame=transparent_host_with_opaque_panel_surface"));
    assert!(report
        .summary()
        .contains("search_surface=panel_as_search_surface"));
    assert!(report
        .summary()
        .contains("search_surface=nested_search_surface"));
}

#[test]
fn ui_preview_uses_transparent_host_with_opaque_panel() {
    let config = LauncherPreviewConfig {
        theme_mode: ThemeMode::Light,
        scenario: "empty".to_string(),
        timeout_ms: 8000,
    };
    let options = preview_native_options_for_config(&config);
    let description = format!("{:?}", options.viewport);

    assert_eq!(preview_window_title(), "std-cli Launcher");
    assert!(description.contains("transparent: Some(true)"));
    assert!(description.contains("decorations: Some(false)"));
    assert!(description.contains("resizable: Some(false)"));
    assert!(description.contains("visible: Some(true)"));
    let mut state = std_launcher::LauncherState::new();
    apply_preview_scenario(&mut state, &config.scenario);
    let expected_size = ui::launcher_window_inner_size(&state);
    let expected_contract = std_launcher::transparent_visible_panel_contract(expected_size);

    assert_eq!(preview_window_inner_size(&config), expected_size);
    assert_eq!(preview_capture_window_contract(&config), expected_contract);
}

#[test]
fn launcher_entrypoints_share_panel_native_options() {
    let main = include_str!("main.rs");
    let preview = include_str!("preview.rs");
    let gui_smoke = include_str!("gui_smoke.rs");

    for source in [main, preview, gui_smoke] {
        assert!(source.contains("launcher_panel_native_options"));
        assert!(!source.contains("ViewportBuilder::default()"));
    }
}

#[test]
fn preview_evidence_names_transparent_host_not_preview_viewport() {
    let surface = include_str!("surface_smoke.rs");
    let preview = include_str!("preview.rs");

    assert!(surface.contains("capture_window=transparent_host,opt_in_only"));
    assert!(surface.contains("capture_surface=opaque_panel_surface"));
    assert!(surface.contains("panel_surface=opaque"));
    assert!(!surface.contains("preview_viewport="));
    assert!(preview.contains("transparent-native-host"));
    assert!(preview.contains("opaque-panel-surface"));
    assert!(preview.contains("no-host-background"));
    assert!(!preview.contains("preview_viewport"));
    assert!(!preview.contains("preview_viewport_contract"));
}

#[test]
fn ui_preview_window_size_matches_seeded_panel_state() {
    for scenario in ["results", "defer", "error", "action-panel"] {
        let config = LauncherPreviewConfig {
            theme_mode: ThemeMode::Light,
            scenario: scenario.to_string(),
            timeout_ms: 8000,
        };
        let preview_size = preview_window_inner_size(&config);
        let mut state = std_launcher::LauncherState::new();
        apply_preview_scenario(&mut state, scenario);
        let expected_size = ui::launcher_window_inner_size(&state);

        assert!(preview_size.y > ui::launcher_initial_window_inner_size().y);
        assert_eq!(preview_size, expected_size);
    }
}

#[test]
fn ui_preview_scenarios_seed_visible_launcher_states() {
    let mut state = std_launcher::LauncherState::new();

    apply_preview_scenario(&mut state, "no-results");
    assert!(state.view.results.is_empty());
    assert_eq!(state.view.phase, std_egui::LauncherPhase::NoMatches);

    apply_preview_scenario(&mut state, "searching");
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Searching);
    assert_eq!(
        state.view.loading,
        std_egui::LauncherLoadingState::UpdatingResults
    );

    apply_preview_scenario(&mut state, "loading");
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Searching);
    assert_eq!(
        state.view.loading,
        std_egui::LauncherLoadingState::SlowEmptyResults
    );

    apply_preview_scenario(&mut state, "executing");
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Executing);

    apply_preview_scenario(&mut state, "defer");
    assert_eq!(
        state.view.feedback.as_ref().unwrap().status,
        ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(state.view.phase, std_egui::LauncherPhase::Feedback);

    apply_preview_scenario(&mut state, "action-panel");
    assert!(state.action_panel.open);
    assert_eq!(state.action_panel.action_name, "StdFixtureTerminal");

    apply_preview_scenario(&mut state, "error");
    assert_eq!(
        state.view.feedback.as_ref().unwrap().status,
        ActionExecutionStatus::Failed
    );

    apply_preview_scenario(&mut state, "ime");
    assert_eq!(state.view.query, "index");
    assert_eq!(state.ime_preedit.as_deref(), Some("zhong"));
    assert_eq!(state.view.phase, std_egui::LauncherPhase::WithResults);
}
