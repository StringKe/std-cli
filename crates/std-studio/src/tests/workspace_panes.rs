use super::*;

#[test]
fn studio_opens_focuses_and_closes_workspace_panes() {
    let mut studio = test_studio();
    let workflow_path = studio
        .core
        .config
        .workflows_dir()
        .join("daily/workflow.json");

    let dashboard = studio.open_workspace_pane(StudioPane::Dashboard);
    let workflow = studio.open_workflow_builder(workflow_path.clone());
    let duplicate = studio.open_workflow_builder(workflow_path);
    let analysis = studio.open_analysis_workbench(std::path::PathBuf::from("."));
    let apps = studio.open_app_manager_pane();
    let plugin = studio.open_plugin_manager_pane();
    let memory = studio.open_memory_browser_pane();
    studio.open_execution_history_pane();
    let settings = studio.open_settings_pane();
    let duplicate_settings = studio.open_settings_pane();

    assert_eq!(dashboard.value(), 1);
    assert_eq!(workflow, duplicate);
    assert_eq!(settings, duplicate_settings);
    assert_eq!(studio.workspace_panes.len(), 8);
    assert_eq!(studio.focused_pane, Some(settings));
    assert_eq!(studio.open_workspace_panes().count(), 8);
    assert!(studio.focus_workspace_pane(plugin));
    assert_eq!(studio.focused_pane, Some(plugin));
    assert!(studio.close_workspace_pane(memory));
    assert_eq!(studio.workspace_panes.len(), 7);
    assert!(!studio.close_workspace_pane(memory));
    assert!(studio.focus_workspace_pane(analysis));
    assert!(studio.focus_workspace_pane(apps));
    assert!(studio.close_workspace_pane(apps));
    assert_eq!(studio.focused_pane, Some(analysis));
}

#[test]
fn studio_pane_titles_reflect_pane_kind() {
    let mut studio = test_studio();
    let workflow = studio.open_workflow_builder(std::path::PathBuf::from("release/workflow.json"));
    let analysis = studio.open_analysis_workbench(std::path::PathBuf::from("std-cli"));
    let apps = studio.open_app_manager_pane();
    let memory = studio.open_memory_browser_pane();
    let settings = studio.open_settings_pane();

    let titles = studio
        .workspace_panes
        .iter()
        .map(|pane| (pane.id, pane.title.as_str()))
        .collect::<Vec<_>>();

    assert!(titles.contains(&(workflow, "Workflow Builder: workflow.json")));
    assert!(titles.contains(&(analysis, "Analysis Workbench: std-cli")));
    assert!(titles.contains(&(apps, "App Manager")));
    assert!(titles.contains(&(memory, "Memory Browser")));
    assert!(titles.contains(&(settings, "Settings")));
}

#[test]
fn studio_pane_kinds_map_to_real_pane_content() {
    let mut studio = test_studio();
    let dashboard = studio.open_workspace_pane(StudioPane::Dashboard);
    let workflows = studio.open_workspace_pane(StudioPane::Workflows);
    let workflow_builder = studio.open_workflow_builder(std::path::PathBuf::from("daily"));
    let analysis = studio.open_analysis_workbench(std::path::PathBuf::from("std-cli"));
    let apps = studio.open_app_manager_pane();
    let memory = studio.open_memory_browser_pane();
    let history = studio.open_execution_history_pane();
    let plugins = studio.open_plugin_manager_pane();
    let settings = studio.open_settings_pane();

    let content = studio
        .workspace_panes
        .iter()
        .map(|pane| (pane.id, pane.kind.content_key()))
        .collect::<Vec<_>>();

    assert!(content.contains(&(dashboard, "dashboard")));
    assert!(content.contains(&(workflows, "workflows")));
    assert!(content.contains(&(workflow_builder, "workflows")));
    assert!(content.contains(&(analysis, "analysis")));
    assert!(content.contains(&(apps, "apps")));
    assert!(content.contains(&(memory, "memory")));
    assert!(content.contains(&(history, "history")));
    assert!(content.contains(&(plugins, "plugins")));
    assert!(content.contains(&(settings, "settings")));
}

#[test]
fn studio_pane_content_snapshots_include_real_state() {
    let mut studio = test_studio();
    studio
        .core
        .remember("project", "Pane memory", "Workspace pane memory", vec![])
        .unwrap();
    studio.refresh();
    let dashboard = WorkspacePaneKind::Pane(StudioPane::Dashboard);
    let memory = WorkspacePaneKind::MemoryBrowser;
    let workflow = WorkspacePaneKind::WorkflowBuilder {
        workflow_path: std::path::PathBuf::from("daily/workflow.json"),
    };

    let dashboard_content = studio.workspace_pane_content(&dashboard);
    let memory_content = studio.workspace_pane_content(&memory);
    let operations_content =
        studio.workspace_pane_content(&WorkspacePaneKind::Pane(StudioPane::Operations));
    let workflow_content = studio.workspace_pane_content(&workflow);

    assert_eq!(dashboard_content.content_key, "dashboard");
    assert!(dashboard_content
        .lines
        .iter()
        .any(|line| line.starts_with("actions=")));
    assert!(dashboard_content.lines.iter().any(|line| {
        line == "workspace_policy=single egui host viewport, internal workspace panes"
    }));
    assert_eq!(memory_content.content_key, "memory");
    assert!(memory_content.lines.contains(&"memories=1".to_string()));
    assert_eq!(operations_content.content_key, "operations");
    assert!(operations_content
        .lines
        .iter()
        .any(|line| line.contains("mise run quality")));
    assert!(operations_content
        .lines
        .iter()
        .any(|line| line.contains("result=")));
    assert!(operations_content
        .lines
        .iter()
        .any(|line| line.contains("std release verify")));
    assert_eq!(workflow_content.content_key, "workflows");
    assert!(workflow_content
        .lines
        .contains(&"path=daily/workflow.json".to_string()));
}

#[test]
fn workspace_panes_cover_interactive_workbench_surfaces() {
    let mut studio = test_studio();
    let workflow_path = studio
        .create_workflow("Workspace Workflow", "Pane workflow")
        .unwrap();
    studio
        .add_workflow_step(&workflow_path, "Collect context", serde_json::json!({}))
        .unwrap();
    studio
        .core
        .remember("project", "Workspace Memory", "Pane memory", vec![])
        .unwrap();
    studio.refresh();

    let kinds = vec![
        WorkspacePaneKind::Pane(StudioPane::Workflows),
        WorkspacePaneKind::WorkflowBuilder {
            workflow_path: workflow_path.clone(),
        },
        WorkspacePaneKind::AppManager,
        WorkspacePaneKind::MemoryBrowser,
        WorkspacePaneKind::ExecutionHistory,
        WorkspacePaneKind::PluginManager,
        WorkspacePaneKind::Pane(StudioPane::Settings),
        WorkspacePaneKind::Pane(StudioPane::Operations),
    ];

    for kind in kinds {
        let content = studio.workspace_pane_content(&kind);
        assert_eq!(content.content_key, kind.content_key());
        assert!(!content.heading.is_empty());
        assert!(content.lines.iter().any(|line| line.contains("action=")
            || line.contains("path=")
            || line.contains("command=")
            || line.contains("release=")
            || line.contains("install=")
            || line.contains("config_path=")
            || line.contains("memories=")
            || line.contains("plugin_actions=")
            || line.contains("trace=")));
    }
}

#[test]
fn studio_ui_uses_workspace_pane_language_not_window_language() {
    let main_source = include_str!("../main.rs");
    let pane_source = include_str!("../workspace_panes.rs");
    let tabs_source = include_str!("../workspace_tabs.rs");

    for source in [main_source, pane_source, tabs_source] {
        assert!(!source.contains("mod windows"));
        assert!(!source.contains("crate::windows"));
        assert!(!source.contains("studio.windows"));
    }
    assert!(main_source.contains("mod workspace_panes"));
    assert!(pane_source.contains("studio.workspace_panes"));
    assert!(tabs_source.contains("studio.workspace_panes.close"));
}

#[test]
fn studio_core_uses_workspace_pane_module_not_window_module() {
    let lib_source = include_str!("../lib.rs");

    assert!(!lib_source.contains("mod window;"));
    assert!(!lib_source.contains("pub use window::"));
    assert!(lib_source.contains("mod workspace_pane;"));
    assert!(lib_source.contains("pub use workspace_pane::"));
}
