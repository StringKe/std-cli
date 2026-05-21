use super::*;
use std_core::StdConfig;

#[test]
fn launcher_exists() {
    assert_eq!(super::launcher_version(), "0.1.0");
}

#[test]
fn launcher_hotkey_parses_config_string() {
    let hotkey = LauncherHotkey::parse("Cmd+Shift+K").unwrap();

    assert_eq!(hotkey.modifiers, vec!["Command", "Shift"]);
    assert_eq!(hotkey.key, "K");
    assert_eq!(hotkey.display(), "Command+Shift+K");
    assert!(LauncherHotkey::parse("Bad+K").is_none());
}

#[test]
fn launcher_controller_toggles_visibility() {
    let config = StdConfig {
        launcher_hotkey: "Ctrl+Space".to_string(),
        ..StdConfig::default()
    };
    let mut controller = LauncherController::new(&config);

    assert!(!controller.visible);
    assert!(!controller.focused);

    controller.toggle();
    assert!(controller.visible);
    assert!(controller.focused);
    assert_eq!(
        LauncherController::window_commands(false, controller.visible),
        vec![
            LauncherWindowCommand::ResizeToPanel,
            LauncherWindowCommand::PositionForPanel,
            LauncherWindowCommand::SetVisible(true),
            LauncherWindowCommand::Focus
        ]
    );

    controller.hide();
    assert!(!controller.visible);
    assert!(!controller.focused);
    assert_eq!(controller.hotkey.display(), "Control+Space");
    assert_eq!(
        LauncherController::window_commands(true, controller.visible),
        vec![LauncherWindowCommand::SetVisible(false)]
    );

    controller.start_voice_input();
    assert!(controller.voice_active);
    controller.hide();
    assert!(!controller.voice_active);
}

#[test]
fn launcher_controller_produces_hotkey_registration_plan() {
    let config = StdConfig {
        launcher_hotkey: "Alt+Space".to_string(),
        ..StdConfig::default()
    };
    let controller = LauncherController::new(&config);
    let plan = controller.registration_plan();
    let runtime = GlobalHotkeyRuntime::disabled(plan.clone());

    assert_eq!(plan.accelerator, "Alt+Space");
    assert!(plan.enabled);
    assert!(!runtime.is_registered());
}

#[test]
fn hotkey_smoke_is_skipped_in_test_mode() {
    let report = hotkey_smoke("Alt+Space");

    assert_eq!(report.status, "SKIP");
    assert!(!report.registered);
    assert!(report
        .error
        .as_deref()
        .unwrap()
        .contains("STD_TEST_MODE blocked global hotkey registration"));
    assert!(report.summary().contains("launcher_hotkey_smoke SKIP"));
}

#[test]
fn hotkey_runtime_register_is_blocked_in_tests() {
    let result = GlobalHotkeyRuntime::register(HotkeyRegistrationPlan {
        accelerator: "Alt+Space".to_string(),
        enabled: true,
    });

    let Err(error) = result else {
        panic!("test mode must block global hotkey registration");
    };
    assert!(error.contains("STD_TEST_MODE blocked global hotkey registration"));
}

#[test]
fn hotkey_runtime_matches_registered_event_id() {
    let plan = HotkeyRegistrationPlan {
        accelerator: "Alt+Space".to_string(),
        enabled: true,
    };
    let mut runtime = GlobalHotkeyRuntime::disabled(plan);
    runtime.set_hotkey_id_for_test(42);
    let pressed = global_hotkey::GlobalHotKeyEvent {
        id: 42,
        state: global_hotkey::HotKeyState::Pressed,
    };
    let released = global_hotkey::GlobalHotKeyEvent {
        id: 42,
        state: global_hotkey::HotKeyState::Released,
    };

    assert!(runtime.should_toggle_for_event(pressed));
    assert!(!runtime.should_toggle_for_event(released));
}

#[test]
fn launcher_state_previews_and_triggers_selected_action() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        launcher_hotkey: "Cmd+Space".to_string(),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    let preview = state.update_query("index").unwrap();
    let execution = state.trigger_selected().unwrap();

    assert_eq!(state.controller.hotkey.display(), "Command+Space");
    assert_eq!(preview.title, "Rebuild Index");
    assert_eq!(execution.action_name, "Rebuild Index");
    let feedback = state.view.feedback.as_ref().unwrap();
    assert_eq!(
        feedback.title,
        std_egui::i18n::t("launcher.feedback.completed")
    );
    assert_eq!(feedback.status, ActionExecutionStatus::Completed);
    assert!(!feedback.deferred);
    assert!(feedback.summary().contains("Completed"));
    assert_eq!(
        state.view.telemetry.last_result_count,
        state.view.results.len()
    );
    assert!(state.view.telemetry.last_result_count >= 1);
    let report = state.performance_report();
    assert!(report.pass(), "{}", report.summary());
    assert_eq!(report.search_budget_ms, 16);
    assert_eq!(report.hotkey_budget_ms, 80);

    state.toggle_visibility();
    assert!(state.controller.visible);
}

#[test]
fn launcher_hotkey_toggle_returns_window_show_commands() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let mut state = LauncherState::with_core(core);

    let hidden = state.handle_hotkey_toggle();
    let hidden = {
        assert_eq!(
            hidden,
            vec![
                LauncherWindowCommand::ResizeToPanel,
                LauncherWindowCommand::PositionForPanel,
                LauncherWindowCommand::SetVisible(true),
                LauncherWindowCommand::Focus
            ]
        );
        state.handle_escape_hide()
    };
    let shown = state.handle_hotkey_toggle();

    assert_eq!(hidden, vec![LauncherWindowCommand::SetVisible(false)]);
    assert_eq!(
        shown,
        vec![
            LauncherWindowCommand::ResizeToPanel,
            LauncherWindowCommand::PositionForPanel,
            LauncherWindowCommand::SetVisible(true),
            LauncherWindowCommand::Focus
        ]
    );
    assert!(state.controller.visible);
    assert!(state.controller.focused);
}

#[test]
fn launcher_smoke_report_validates_fast_search_preview_and_feedback() {
    let report = LauncherState::smoke("rebuild index").unwrap();
    let summary = report.summary();

    assert_eq!(report.query, "rebuild index");
    assert_eq!(report.preview_title, "Rebuild Index");
    assert_eq!(report.execution_status, ActionExecutionStatus::Completed);
    assert_eq!(
        report.feedback_title,
        std_egui::i18n::t("launcher.feedback.completed")
    );
    assert!(report.performance.pass(), "{summary}");
    assert!(summary.contains("launcher_smoke PASS"));
    assert!(summary.contains("launcher_perf PASS"));
}

#[test]
fn launcher_ui_semantics_smoke_covers_result_empty_defer_and_error_states() {
    let report = LauncherState::ui_semantics_smoke("index");
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_result_semantics(&report, &summary);
    assert_no_result_semantics(&report);
    assert_loading_and_execution_semantics(&report, &summary);
    assert_feedback_semantics(&report, &summary);
}

fn assert_result_semantics(report: &LauncherUiSemanticsReport, summary: &str) {
    let first_keycap = std_egui::input::launcher_result_keycap(0).unwrap();
    let enter = std_egui::input::enter().label();
    let actions = std_egui::input::launcher_action_panel().label();
    assert_eq!(report.result_phase, "WithResults");
    assert_eq!(report.result_mode, "Matches");
    assert_eq!(report.selected_keycap, first_keycap);
    assert!(report
        .selected_action_hint
        .starts_with(&format!("{enter} ")));
    assert_eq!(report.action_bar_hint, format!("Actions {actions}"));
    assert!(report.action_panel_actions.contains("Open in Studio"));
    assert!(report
        .action_panel_open_studio_command
        .starts_with("studio-pane://"));
    assert!(summary.contains("launcher_ui_semantics_smoke PASS"));
    assert!(summary.contains("result_phase=WithResults"));
    assert!(summary.contains(&format!("selected_keycap={first_keycap}")));
    assert!(summary.contains(&format!("action_bar_hint=Actions {actions}")));
    assert!(summary.contains("action_panel_actions="));
}

fn assert_no_result_semantics(report: &LauncherUiSemanticsReport) {
    assert_eq!(report.no_results_label, "No matches");
    assert!(report.no_results_fallback.contains("Ask AI about"));
    assert_eq!(report.no_results_phase, "NoMatches/NoMatches");
    assert!(report.no_results_ime_enter_blocked);
}

fn assert_loading_and_execution_semantics(report: &LauncherUiSemanticsReport, summary: &str) {
    assert!(report.loading_label.contains("Searching registry"));
    assert_eq!(report.loading_progress, "2px Searching indeterminate");
    assert_eq!(report.loading_spinner_after_ms, 200);
    assert!(report.executing_search_text.starts_with("Running:"));
    assert!(!report.executing_input_enabled);
    assert_eq!(
        report.executing_cancel_shortcut,
        format!("Cancel {}", std_egui::input::launcher_cancel().label())
    );
    assert!(summary.contains("loading_progress=2px Searching indeterminate"));
    assert!(summary.contains("executing_input_enabled=false"));
}

fn assert_feedback_semantics(report: &LauncherUiSemanticsReport, summary: &str) {
    assert!(report
        .defer_feedback_label
        .contains(std_egui::i18n::t("launcher.feedback.deferred")));
    assert_eq!(report.defer_actions, "Copy,Retry");
    assert!(report
        .failed_feedback_label
        .contains(std_egui::i18n::t("launcher.feedback.failed")));
    assert_eq!(report.error_actions, "Copy,Retry,Open Studio");
    assert_eq!(report.error_open_studio_target, "ExecutionHistory");
    assert_eq!(report.error_open_studio_command, "studio-pane://history");
    assert!(summary.contains(&format!(
        "failed_feedback_label={}",
        std_egui::i18n::t("launcher.feedback.failed")
    )));
    assert!(summary.contains("error_open_studio_target=ExecutionHistory"));
}

#[test]
fn launcher_window_smoke_validates_hotkey_window_commands() {
    let report = LauncherState::window_smoke();
    let summary = report.summary();

    assert!(report.pass(), "{summary}");
    assert_eq!(
        report.hidden_commands,
        vec![LauncherWindowCommand::SetVisible(false)]
    );
    assert_eq!(
        report.shown_commands,
        vec![
            LauncherWindowCommand::ResizeToPanel,
            LauncherWindowCommand::PositionForPanel,
            LauncherWindowCommand::SetVisible(true),
            LauncherWindowCommand::Focus
        ]
    );
    assert!(summary.contains("launcher_window_smoke PASS"));
    assert!(summary.contains("shown_commands=ResizeToPanel,PositionForPanel,Visible(true),Focus"));
}

#[test]
fn launcher_state_triggers_saved_workflow_action() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    std::fs::create_dir_all(config.workflows_dir().join("daily-smoke")).unwrap();
    let workflow = std_orchestration::Workflow {
        id: uuid::Uuid::new_v4(),
        name: "Daily Smoke".to_string(),
        description: "Run daily smoke".to_string(),
        steps: vec![std_orchestration::WorkflowStep {
            id: uuid::Uuid::new_v4(),
            name: "Collect context".to_string(),
            action_id: None,
            step_type: std_orchestration::StepType::Action,
            parameters: serde_json::json!({"kind": "context"}),
        }],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    std::fs::write(
        config
            .workflows_dir()
            .join("daily-smoke")
            .join("workflow.json"),
        serde_json::to_string_pretty(&workflow).unwrap(),
    )
    .unwrap();
    let core = StdCore::with_config(config);
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.update_query("workflow");
    let execution = state.trigger_selected().unwrap();

    assert_eq!(execution.action_name, "Run Workflow: Daily Smoke");
    assert_eq!(execution.status, ActionExecutionStatus::Completed);
    assert!(execution
        .output
        .as_ref()
        .unwrap()
        .to_string()
        .contains("Collect context"));
}

#[test]
fn launcher_state_registers_local_content_on_init() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.remember(
        "project",
        "Launcher memory",
        "Launcher should search memory without caller seeding",
        vec!["launcher".to_string()],
    )
    .unwrap();
    core.capture_clipboard("cargo test --workspace", "test")
        .unwrap();

    let mut state = LauncherState::with_core(core);
    let memory_preview = state.update_query("Launcher memory").unwrap();
    let clipboard_preview = state.update_query("cargo test").unwrap();

    assert_eq!(memory_preview.title, "Memory: Launcher memory");
    assert_eq!(memory_preview.action_type, ActionType::Memory);
    assert_eq!(clipboard_preview.title, "Clipboard: cargo test --workspace");
}

#[test]
fn launcher_state_searches_indexed_files_without_opening_them() {
    let temp = tempfile::tempdir().unwrap();
    let config = StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    };
    let project_dir = temp.path().join("project");
    std::fs::create_dir_all(project_dir.join("src")).unwrap();
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "launcher file search",
    )
    .unwrap();
    std::fs::create_dir_all(config.index_dir()).unwrap();
    std::fs::write(
        config.index_dir().join("files-project.json"),
        serde_json::json!({
            "root": project_dir,
            "created_at": chrono::Utc::now(),
            "entries": [{
                "path": project_dir.join("src").join("main.rs"),
                "name": "main.rs",
                "size_bytes": 20,
                "modified_at": chrono::Utc::now(),
                "snippet": "launcher file search"
            }]
        })
        .to_string(),
    )
    .unwrap();
    let core = StdCore::with_config(config);
    let mut state = LauncherState::with_core(core);

    let preview = state.update_query("main.rs").unwrap();
    let execution = state.trigger_selected().unwrap();

    assert_eq!(preview.title, "Open File: main.rs");
    assert_eq!(execution.action_name, "Open File: main.rs");
    assert_eq!(execution.status, ActionExecutionStatus::NeedsExternalRunner);
    assert_eq!(
        execution
            .output
            .as_ref()
            .unwrap()
            .get("deferred")
            .and_then(|value| value.as_bool()),
        Some(true)
    );
}

#[test]
fn launcher_cleans_voice_transcript_for_query() {
    let cleaned = clean_voice_transcript("um please just open terminal");

    assert_eq!(cleaned, "open terminal");
}

#[test]
fn launcher_state_uses_voice_transcript_to_preview_and_trigger() {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    core.seed_builtin_actions().unwrap();
    let mut state = LauncherState::with_core(core);

    state.start_voice_input();
    let preview = state
        .apply_voice_transcript("um please just rebuild index")
        .unwrap();
    let execution = state.trigger_selected().unwrap();

    assert!(!state.controller.voice_active);
    assert_eq!(state.view.query, "rebuild index");
    assert_eq!(preview.title, "Rebuild Index");
    assert_eq!(execution.action_name, "Rebuild Index");
}
