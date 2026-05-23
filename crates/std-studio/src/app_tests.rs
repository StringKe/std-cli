use crate::{
    bottom_panel_model::BottomPanelTab, views::workflow_builder_ai::WorkflowAiAction, StudioEguiApp,
};
use std_core::{StdConfig, StdCore};
use std_studio::StudioApp;

#[test]
fn workflow_builder_keyboard_shortcuts_move_selected_loaded_step() {
    let mut app = test_app();
    let path = app
        .app
        .create_workflow("Keyboard Move", "Move selected step")
        .unwrap();
    app.app
        .add_workflow_step(&path, "First", serde_json::json!({"order": 1}))
        .unwrap();
    app.app
        .add_workflow_step(&path, "Second", serde_json::json!({"order": 2}))
        .unwrap();
    app.workflow_selected_path = Some(path);
    app.workflow_edit_index = "0".to_string();

    app.move_workflow_builder_step_by_keyboard(1);

    assert_eq!(app.workflow_edit_index, "1");
    assert!(app.status.contains("moved step"));

    app.move_workflow_builder_step_by_keyboard(-1);

    assert_eq!(app.workflow_edit_index, "0");
    assert!(app.status.contains("moved step"));
}

#[test]
fn workflow_builder_keyboard_selects_steps_only_when_builder_is_focused() {
    let mut app = test_app();
    app.workflow_goal = "Keyboard select".to_string();
    app.plan_workflow_from_goal();
    app.workflow_edit_index = "0".to_string();
    let builder = app
        .app
        .open_workflow_builder(std::path::PathBuf::from("workflow.json"));
    app.app.focus_workspace_pane(builder);

    app.select_workflow_builder_step_by_keyboard(1);

    assert_eq!(app.workflow_edit_index, "1");

    app.app.open_settings_pane();
    assert!(!app.focused_workspace_is_workflow_builder());
    app.select_workflow_builder_step_by_keyboard(1);

    assert_eq!(app.workflow_edit_index, "1");
}

#[test]
fn workflow_builder_arrow_down_selects_next_step_from_raw_input() {
    let ctx = eframe::egui::Context::default();
    let mut app = test_app();
    app.workflow_goal = "Keyboard select".to_string();
    app.plan_workflow_from_goal();
    app.workflow_edit_index = "0".to_string();
    let builder = app
        .app
        .open_workflow_builder(std::path::PathBuf::from("workflow.json"));
    app.app.focus_workspace_pane(builder);

    let _ = ctx.run(key_input(eframe::egui::Key::ArrowDown), |ctx| {
        app.handle_workflow_builder_keyboard(ctx);
    });

    assert_eq!(app.workflow_edit_index, "1");
}

#[test]
fn workflow_ai_assist_apply_insert_and_replace_edit_planned_steps() {
    let mut app = test_app();
    app.workflow_goal = "release".to_string();
    app.apply_workflow_ai_action(WorkflowAiAction::Apply(0));

    let workflow = app.app.planned_workflow.as_ref().unwrap();
    assert_eq!(workflow.steps.len(), 1);
    assert_eq!(workflow.steps[0].name, "Collect context");
    assert_eq!(workflow.steps[0].parameters["source"], "release");
    assert!(app.status.contains("AI applied step"));

    app.workflow_edit_index = "0".to_string();
    app.apply_workflow_ai_action(WorkflowAiAction::Insert(1));

    let workflow = app.app.planned_workflow.as_ref().unwrap();
    assert_eq!(workflow.steps.len(), 2);
    assert_eq!(workflow.steps[0].name, "Validate result");
    assert_eq!(workflow.steps[1].name, "Collect context");
    assert!(app.status.contains("AI inserted step"));

    app.workflow_edit_index = "1".to_string();
    app.apply_workflow_ai_action(WorkflowAiAction::Replace(2));

    let workflow = app.app.planned_workflow.as_ref().unwrap();
    assert_eq!(workflow.steps.len(), 2);
    assert_eq!(workflow.steps[1].name, "Record trace");
    assert_eq!(workflow.steps[1].parameters["target"], "execution-history");
    assert!(app.status.contains("AI replaced step"));
}

#[test]
fn batch_run_opens_bottom_panel_with_report_state() {
    let mut app = test_app();

    let body = app.batch_json.clone();
    let report = app.app.run_batch_json(&body).unwrap();
    let status = format!("batch {:?} steps={}", report.status, report.steps.len());
    app.bottom_panel_tab = BottomPanelTab::Logs;
    app.open_batch_debug_panel();
    app.status = status;

    assert!(app.app.last_batch_report.is_some());
    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let panel = app.bottom_panel_snapshot();
    assert_eq!(panel.title, "批量调试");
    assert_eq!(panel.rows.len(), 2);
    assert!(panel
        .rows
        .iter()
        .any(|row| row.status == crate::bottom_panel::completed_status()));
    assert!(app.status.contains("batch"));
}

#[test]
fn planned_workflow_simulate_and_test_share_batch_debug_panel() {
    let mut app = test_app();
    app.app.plan_workflow("terminal").unwrap();

    app.bottom_panel_tab = BottomPanelTab::Problems;
    app.preview_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let preview_panel = app.bottom_panel_snapshot();
    assert_eq!(preview_panel.title, "terminal");
    assert_eq!(preview_panel.status, "success");
    assert!(preview_panel.rows.iter().all(|row| row.status == "success"));
    assert!(app.status.contains("dry-run terminal"));

    app.bottom_panel_tab = BottomPanelTab::Logs;
    app.run_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let run_panel = app.bottom_panel_snapshot();
    assert_eq!(run_panel.title, "terminal");
    assert_eq!(run_panel.status, "success");
    assert!(run_panel.rows.iter().all(|row| row.status == "success"));
}

#[test]
fn saved_workflow_simulate_and_test_share_batch_debug_panel() {
    let mut app = test_app();
    let path = app
        .app
        .create_workflow("Saved Flow", "Saved workflow")
        .unwrap();
    app.app
        .add_workflow_step(&path, "Collect", serde_json::json!({}))
        .unwrap();
    app.workflow_selected_path = Some(path);

    app.bottom_panel_tab = BottomPanelTab::Performance;
    app.preview_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let preview_panel = app.bottom_panel_snapshot();
    assert_eq!(preview_panel.title, "Saved Flow");
    assert_eq!(preview_panel.rows[0].status, "success");

    app.bottom_panel_tab = BottomPanelTab::Logs;
    app.run_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::BatchDebug);
    let run_panel = app.bottom_panel_snapshot();
    assert_eq!(run_panel.title, "Saved Flow");
    assert_eq!(run_panel.rows[0].status, "success");
}

#[test]
fn workflow_builder_errors_open_bottom_problems_panel() {
    let mut app = test_app();

    app.bottom_panel_tab = BottomPanelTab::Logs;
    app.preview_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::Problems);
    assert!(app.status.contains("missing planned workflow"));

    app.bottom_panel_tab = BottomPanelTab::Performance;
    app.run_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::Problems);
    assert!(app.status.contains("missing planned workflow"));
}

#[test]
fn saved_workflow_errors_open_bottom_problems_panel() {
    let mut app = test_app();
    app.workflow_selected_path = Some(std::path::PathBuf::from("missing-workflow.json"));

    app.bottom_panel_tab = BottomPanelTab::Logs;
    app.preview_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::Problems);
    assert!(!app.status.is_empty());

    app.bottom_panel_tab = BottomPanelTab::Performance;
    app.run_active_workflow();

    assert!(app.layout.bottom_panel_open);
    assert_eq!(app.bottom_panel_tab, BottomPanelTab::Problems);
    assert!(!app.status.is_empty());
}

fn test_app() -> StudioEguiApp {
    let mut app = StudioEguiApp::default();
    let temp = tempfile::tempdir().unwrap();
    app.app = StudioApp::with_core(StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    }));
    app
}

fn key_input(key: eframe::egui::Key) -> eframe::egui::RawInput {
    eframe::egui::RawInput {
        events: vec![eframe::egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: eframe::egui::Modifiers::NONE,
        }],
        ..Default::default()
    }
}
