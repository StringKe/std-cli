use super::*;
use std_core::StdConfig;

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

    assert_eq!(
        hidden,
        vec![
            LauncherWindowCommand::ResizeToHiddenHost,
            LauncherWindowCommand::SetVisible(false)
        ]
    );
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
    assert!(summary.contains("launcher_motion_budget PASS"));
    assert!(summary.contains("frame_budget_ms=8"));
    assert!(summary.contains("active_animation_limit=8"));
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
    assert!(report.selected_label.contains("Rebuild Index"));
    assert!(!report.selected_label.contains("按 Enter 运行"));
    assert_eq!(report.selected_keycap, first_keycap);
    assert!(report
        .selected_action_hint
        .starts_with(&format!("{enter} ")));
    assert_eq!(report.action_bar_hint, format!("Actions {actions}"));
    assert!(report
        .action_panel_actions
        .contains(std_egui::i18n::t("launcher.action.open_in_studio")));
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
    assert_eq!(
        report.executing_background_shortcut,
        format!("Move to background {}", std_egui::input::enter().label())
    );
    assert!(summary.contains("loading_progress=2px Searching indeterminate"));
    assert!(summary.contains("executing_input_enabled=false"));
    assert!(summary.contains(&format!(
        "executing_background_shortcut=Move to background {}",
        std_egui::input::enter().label()
    )));
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
    assert_eq!(
        report.feedback_contract,
        "defer=Copy>Retry,error=Copy>Retry>OpenStudio,keyboard=copy>retry>open-studio"
    );
    assert_eq!(
        report.feedback_a11y_contract,
        "panel=status>target>actions,actions=action>target>status>enter"
    );
    assert_eq!(report.error_open_studio_target, "ExecutionHistory");
    assert_eq!(report.error_open_studio_command, "studio-pane://history");
    assert!(summary.contains(&format!(
        "failed_feedback_label={}",
        std_egui::i18n::t("launcher.feedback.failed")
    )));
    assert!(summary.contains("error_open_studio_target=ExecutionHistory"));
    assert!(summary.contains("feedback_a11y_contract=panel=status>target>actions"));
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
