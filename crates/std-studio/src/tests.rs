use super::*;
use std_core::EventBus;
use std_orchestration::load_workflow;
use uuid::Uuid;

fn test_studio() -> StudioApp {
    let temp = tempfile::tempdir().unwrap();
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    StudioApp::with_core(core)
}

#[test]
fn studio_can_be_instantiated() {
    let studio = test_studio();
    assert_eq!(studio.name, "std-cli Studio");
    assert!(studio.dashboard.action_count >= 3);
    assert_eq!(studio.active_pane, StudioPane::Dashboard);
    assert_eq!(
        studio.workspace_policy.host_window,
        HostWindowPolicy::SingleBorderlessEguiViewport
    );
    assert_eq!(
        studio.workspace_policy.pane_system,
        PaneSystemPolicy::InternalEguiWorkspacePanes
    );
    assert!(!studio.workspace_policy.allows_native_child_windows());
    assert!(!studio.workspace_policy.allows_detached_panels());
}

#[test]
fn studio_switches_panes() {
    let mut studio = test_studio();

    studio.switch_pane(StudioPane::Plugins);

    assert_eq!(studio.active_pane, StudioPane::Plugins);
    assert_eq!(
        StudioPane::Plugins.label(),
        std_egui::i18n::t("studio.plugins.title")
    );
    assert_eq!(
        StudioPane::Apps.label(),
        std_egui::i18n::t("studio.apps.title")
    );
    assert_eq!(
        StudioPane::Memory.label(),
        std_egui::i18n::t("studio.memory.title")
    );
    assert_eq!(
        StudioPane::Settings.label(),
        std_egui::i18n::t("studio.settings.title")
    );
    assert_eq!(
        StudioPane::Operations.label(),
        std_egui::i18n::t("studio.operations.title")
    );
    assert_eq!(StudioPane::Operations.content_key(), "operations");
    assert_eq!(StudioPane::all().len(), 9);
}

mod analysis;
mod memory;
mod operations;
mod plugins;
mod trace;
mod workflow_cancel;
mod workspace_panes;
mod workspace_policy_guard;

#[test]
fn studio_plans_workflow_from_goal() {
    let mut studio = test_studio();

    let workflow = studio.plan_workflow("terminal").unwrap();
    let preview_status = studio.preview_workflow(&workflow).unwrap().status.clone();

    assert_eq!(workflow.name, "terminal");
    assert_eq!(workflow.steps[0].name, "StdFixtureTerminal");
    assert!(workflow.steps[0].action_id.is_some());
    assert_eq!(studio.planned_workflow.as_ref().unwrap().name, "terminal");
    assert_eq!(
        preview_status,
        std_orchestration::ExecutionStatus::Completed
    );
}

#[test]
fn studio_saves_planned_workflow() {
    let mut studio = test_studio();
    studio.plan_workflow("terminal").unwrap();

    let path = studio.save_planned_workflow().unwrap();
    let workflow = load_workflow(&path).unwrap();
    let results = studio.core.search("Run Workflow: terminal", 10).unwrap();

    assert!(path.ends_with("terminal/workflow.json"));
    assert_eq!(workflow.name, "terminal");
    assert_eq!(workflow.steps[0].name, "StdFixtureTerminal");
    assert!(results
        .iter()
        .any(|result| result.action.name == "Run Workflow: terminal"));
}

#[test]
fn studio_creates_and_lists_workflows() {
    let mut studio = test_studio();
    let path = studio
        .create_workflow("Release Notes", "Draft release notes")
        .unwrap();
    let workflows = studio.saved_workflows().unwrap();

    assert!(path.ends_with("release-notes/workflow.md"));
    assert!(workflows.contains(&path));
}

#[test]
fn studio_adds_workflow_step_and_refreshes_preview() {
    let mut studio = test_studio();
    let path = studio
        .create_workflow("Daily Check", "Run daily checks")
        .unwrap();
    let step = studio
        .add_workflow_step(
            &path,
            "Run tests",
            serde_json::json!({"command": "cargo test"}),
        )
        .unwrap();

    assert_eq!(step.name, "Run tests");
    assert_eq!(
        studio
            .workflow_debug
            .as_ref()
            .unwrap()
            .steps
            .first()
            .unwrap()
            .step_name,
        "Run tests"
    );
}

#[test]
fn studio_edits_workflow_steps_and_refreshes_preview() {
    let mut studio = test_studio();
    let path = studio
        .create_workflow("Release Notes", "Draft release notes")
        .unwrap();
    studio
        .add_workflow_step(&path, "Collect commits", serde_json::json!({}))
        .unwrap();
    studio
        .add_workflow_step(&path, "Draft notes", serde_json::json!({}))
        .unwrap();

    let updated = studio
        .update_workflow_step(
            &path,
            0,
            Some("Collect merged commits"),
            Some(serde_json::json!({"command": "git log --merges"})),
        )
        .unwrap();
    let moved = studio.move_workflow_step(&path, 1, 0).unwrap();
    let removed = studio.remove_workflow_step(&path, 1).unwrap();

    assert_eq!(updated.name, "Collect merged commits");
    assert_eq!(moved.name, "Draft notes");
    assert_eq!(removed.name, "Collect merged commits");
    assert_eq!(
        studio.workflow_debug.as_ref().unwrap().steps[0].step_name,
        "Draft notes"
    );
}

#[test]
fn studio_reads_recent_workflow_executions() {
    let studio = test_studio();
    let execution = WorkflowExecution {
        workflow_id: Uuid::new_v4(),
        workflow_name: "Recent Workflow".to_string(),
        status: std_orchestration::ExecutionStatus::Completed,
        current_step: 0,
        started_at: chrono::Utc::now(),
        finished_at: Some(chrono::Utc::now()),
        results: vec![],
    };
    std_orchestration::append_workflow_execution(&studio.core.config.history_dir(), &execution)
        .unwrap();

    let executions = studio.recent_workflow_executions(5).unwrap();

    assert_eq!(executions.len(), 1);
    assert_eq!(executions[0].workflow_id, execution.workflow_id);
}

#[test]
fn studio_runs_workflow_and_persists_execution_history() {
    let mut studio = test_studio();
    let path = studio
        .create_workflow("Studio Run", "Run workflow from Studio")
        .unwrap();
    studio
        .add_workflow_step(&path, "Collect context", serde_json::json!({"ok": true}))
        .unwrap();

    let execution = studio.run_workflow_path(&path).unwrap().clone();
    let history = studio.recent_workflow_executions(5).unwrap();

    assert_eq!(
        execution.status,
        std_orchestration::ExecutionStatus::Completed
    );
    assert_eq!(execution.results.len(), 1);
    assert_eq!(execution.results[0].step_name, "Collect context");
    assert_eq!(
        studio.last_workflow_execution.as_ref().unwrap().workflow_id,
        execution.workflow_id
    );
    assert_eq!(history[0].workflow_id, execution.workflow_id);
}

#[test]
fn studio_runs_batch_plan_through_shared_orchestration() {
    let mut studio = test_studio();
    let report = studio
        .run_batch_json(
            r#"{
  "steps": [
    {"name": "rebuild", "kind": "action", "target": "index"},
    {"name": "smoke", "kind": "workflow", "target": "smoke"},
    {"name": "terminal", "kind": "action", "target": "terminal"}
  ]
}"#,
        )
        .unwrap()
        .clone();

    assert_eq!(
        report.status,
        std_types::ActionExecutionStatus::NeedsExternalRunner
    );
    assert_eq!(report.steps.len(), 3);
    assert_eq!(
        report.steps[0].status,
        std_types::ActionExecutionStatus::Completed
    );
    assert_eq!(
        report.steps[1]
            .execution
            .as_ref()
            .unwrap()
            .action_name
            .as_str(),
        "Run Workflow: smoke"
    );
    assert_eq!(
        report.steps[2].status,
        std_types::ActionExecutionStatus::NeedsExternalRunner
    );
    assert!(studio.last_batch_report.is_some());
}

#[test]
fn studio_browses_and_writes_memory() {
    let mut studio = test_studio();
    studio
        .core
        .remember(
            "project",
            "Workflow storage",
            "Workflow definitions live under workflows",
            vec!["workflow".to_string()],
        )
        .unwrap();

    let results = studio.search_memory("workflow");
    let selected = studio.select_memory(0).unwrap();
    let written = studio
        .remember_from_studio(
            "studio",
            "Studio memory",
            "Memory Browser is a first-class Studio pane",
            vec!["studio".to_string()],
        )
        .unwrap();
    let action_results = studio.core.search("Studio memory", 10).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(selected.title, "Workflow storage");
    assert_eq!(written.scope, "studio");
    assert_eq!(studio.dashboard.memory_count, 2);
    assert!(action_results
        .iter()
        .any(|result| result.action.name.contains("Memory: Studio memory")));
}

#[test]
fn studio_analyzes_searches_and_answers_from_index() {
    let mut studio = test_studio();
    let project_dir = studio.core.config.data_dir.join("project");
    std::fs::create_dir_all(project_dir.join("src")).unwrap();
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {}\npub struct AnalysisWorkbench {}\n",
    )
    .unwrap();

    let analysis_name = studio
        .analyze_entity(&project_dir)
        .unwrap()
        .overview
        .name
        .clone();
    let saved = studio.saved_analyses().unwrap();
    let searched = studio.search_analyses("AnalysisWorkbench", 5).unwrap();
    let answer = studio.ask_analyses("AnalysisWorkbench", 5).unwrap();
    let inspection = studio.inspect_analysis("project", 2).unwrap().unwrap();
    let events = studio.core.events().unwrap();
    let active_component = studio
        .active_analysis
        .as_ref()
        .unwrap()
        .components
        .iter()
        .find(|component| component.path.ends_with("src/main.rs"))
        .unwrap();

    assert_eq!(analysis_name, "project");
    assert_eq!(saved.len(), 1);
    assert_eq!(searched.len(), 1);
    assert_eq!(inspection.overview.name, "project");
    assert_eq!(inspection.component_count, saved[0].components.len());
    assert_eq!(inspection.relation_count, saved[0].relations.len());
    assert_eq!(inspection.history_count, saved[0].history.len());
    assert!(!inspection.key_components.is_empty());
    assert!(events
        .iter()
        .any(|event| event.event_type == std_types::StdEventType::IndexUpdated));
    assert_eq!(active_component.language, "rust");
    assert!(active_component
        .symbols
        .contains(&"type AnalysisWorkbench".to_string()));
    assert!(answer.answer.contains("AnalysisWorkbench"));
    assert!(answer.sources[0]
        .evidence
        .iter()
        .any(|item| item.contains("AnalysisWorkbench defines_type")));
}

#[test]
fn studio_manages_plugin_actions() {
    let mut studio = test_studio();
    let plugin_dir = studio.core.config.plugins_dir().join("smoke");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("main.js"),
        r#"std.emit({ plugin: "studio-plugin-smoke" });"#,
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "smoke",
            "description": "Smoke plugin",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Smoke",
                "description": "Run plugin smoke",
                "when_to_use": "When validating Studio plugin manager",
                "kind": "javascript",
                "script": "main.js",
                "tags": ["studio-plugin-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let manifest_count = studio.reload_plugins().unwrap().manifest_paths.len();
    let results = studio.search_plugins("studio-plugin-smoke");
    let execution = studio.run_selected_plugin().unwrap();

    assert_eq!(manifest_count, 1);
    assert_eq!(results.len(), 1);
    assert_eq!(execution.action_name, "Plugin Smoke");
    assert_eq!(
        execution.status,
        std_types::ActionExecutionStatus::Completed
    );
    assert!(execution
        .output
        .unwrap()
        .to_string()
        .contains("studio-plugin-smoke"));
}

#[test]
fn studio_manages_registered_apps() {
    let mut studio = test_studio();
    let source_app = studio
        .core
        .config
        .data_dir
        .join("source")
        .join("Workbench.app");
    std::fs::create_dir_all(source_app.join("Contents").join("MacOS")).unwrap();
    std::fs::write(
        source_app.join("Contents").join("MacOS").join("workbench"),
        "bin",
    )
    .unwrap();

    let registered = studio.register_app_bundle(&source_app).unwrap();
    let apps = studio.registered_apps().unwrap();
    let results = studio.search_apps("Workbench", 10).unwrap();
    let preview = studio.preview_app("Workbench").unwrap().unwrap();
    let execution = studio.trigger_app("Workbench").unwrap().unwrap();

    assert!(registered.ends_with("Applications/Workbench.app"));
    assert_eq!(apps, vec![registered]);
    assert_eq!(results[0].action.name, "Open App: Workbench");
    assert!(preview.primary_command.contains("Workbench.app"));
    assert_eq!(
        execution.status,
        std_types::ActionExecutionStatus::NeedsExternalRunner
    );
}

#[test]
fn studio_saves_settings_through_shared_config_model() {
    let mut studio = test_studio();
    let config_path = studio
        .core
        .config
        .data_dir
        .join("settings")
        .join("std-cli.json");

    let written = studio
        .save_config_field_to(&config_path, "launcher_hotkey", "Cmd+Space")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "enable_ai", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "theme", "dark")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.reduce_motion", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.high_contrast", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.reduce_transparency", "true")
        .unwrap();
    studio
        .save_config_field_to(&config_path, "appearance.ui_scale", "1.25")
        .unwrap();

    let saved: StdConfig =
        serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();

    assert_eq!(written, config_path);
    assert_eq!(
        studio.config_value("launcher_hotkey").as_deref(),
        Some("Cmd+Space")
    );
    assert!(studio.core.config.enable_ai);
    assert_eq!(studio.core.config.theme, "dark");
    assert!(studio.core.config.reduce_motion());
    assert!(studio.core.config.high_contrast());
    assert!(studio.core.config.reduce_transparency());
    assert_eq!(studio.core.config.ui_scale(), 1.25);
    assert_eq!(saved.launcher_hotkey, "Cmd+Space");
    assert!(saved.enable_ai);
    assert_eq!(saved.theme, "dark");
    assert!(saved.reduce_motion());
    assert!(saved.high_contrast());
    assert!(saved.reduce_transparency());
    assert_eq!(saved.ui_scale(), 1.25);
    assert!(studio.dashboard.action_count >= 3);
    assert!(studio
        .save_config_field_to(&config_path, "missing", "x")
        .is_err());
}
