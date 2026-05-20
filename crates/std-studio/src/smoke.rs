use crate::{default_batch_json, StudioPane};
use std_core::{StdConfig, StdCore};
use std_studio::StudioApp;

pub(crate) struct StudioSmokeReport {
    workspace_panes: usize,
    focused_pane: u64,
    workflow_status: String,
    batch_status: String,
    analysis_name: String,
    analysis_coverage_complete: usize,
    memory_count: usize,
    plugin_status: String,
    history_count: usize,
}

impl StudioSmokeReport {
    pub(crate) fn summary(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_smoke {status}\nworkspace_panes={}\nfocused_pane={}\nworkflow_status={}\nbatch_status={}\nanalysis={}\nanalysis_coverage_complete={}\nmemory_count={}\nplugin_status={}\nhistory_count={}",
            self.workspace_panes,
            self.focused_pane,
            self.workflow_status,
            self.batch_status,
            self.analysis_name,
            self.analysis_coverage_complete,
            self.memory_count,
            self.plugin_status,
            self.history_count
        )
    }

    fn pass(&self) -> bool {
        self.workspace_panes >= 7
            && self.workflow_status == "Completed"
            && self.batch_status == "NeedsExternalRunner"
            && self.analysis_coverage_complete >= 1
            && self.memory_count >= 1
            && self.plugin_status == "Completed"
            && self.history_count >= 1
    }
}

pub(crate) fn smoke_from_args(args: Vec<String>) -> Option<StudioSmokeReport> {
    if args.get(1).map(String::as_str) != Some("--smoke") {
        return None;
    }
    match run_studio_smoke() {
        Ok(report) => Some(report),
        Err(error) => Some(StudioSmokeReport {
            workspace_panes: 0,
            focused_pane: 0,
            workflow_status: format!("FAIL {error}"),
            batch_status: "FAIL".to_string(),
            analysis_name: "FAIL".to_string(),
            analysis_coverage_complete: 0,
            memory_count: 0,
            plugin_status: "FAIL".to_string(),
            history_count: 0,
        }),
    }
}

fn run_studio_smoke() -> Result<StudioSmokeReport, Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let mut studio = StudioApp::with_core(core);

    let workflow_path = studio.create_workflow("Studio Smoke", "Headless Studio smoke")?;
    studio.add_workflow_step(
        &workflow_path,
        "Collect context",
        serde_json::json!({"ok": true}),
    )?;
    let workflow_status = format!("{:?}", studio.run_workflow_path(&workflow_path)?.status);
    let history_count = studio.recent_workflow_executions(10)?.len();
    let batch_status = format!("{:?}", studio.run_batch_json(&default_batch_json())?.status);

    studio.remember_from_studio(
        "studio",
        "Studio smoke memory",
        "Memory Browser writes through shared core",
        vec!["studio".to_string()],
    )?;
    let memory_count = studio.search_memory("smoke").len();

    let project_dir = temp.path().join("project");
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {}\npub struct StudioSmoke {}\n",
    )?;
    let analysis_name = studio.analyze_entity(&project_dir)?.overview.name.clone();
    let coverage = studio.analysis_coverage_report()?;

    let plugin_dir = studio.core.config.plugins_dir().join("studio-smoke");
    std::fs::create_dir_all(&plugin_dir)?;
    std::fs::write(plugin_dir.join("plugin.json"), smoke_plugin_manifest())?;
    studio.reload_plugins()?;
    studio.search_plugins("studio-smoke");
    let plugin_status = studio
        .run_selected_plugin()
        .map(|execution| format!("{:?}", execution.status))
        .unwrap_or_else(|| "Missing".to_string());

    studio.open_workspace_pane(StudioPane::Dashboard);
    studio.open_workflow_builder(workflow_path);
    studio.open_analysis_workbench(project_dir);
    studio.open_plugin_manager_pane();
    studio.open_app_manager_pane();
    studio.open_memory_browser_pane();
    studio.open_execution_history_pane();

    Ok(StudioSmokeReport {
        workspace_panes: studio.open_workspace_panes().count(),
        focused_pane: studio.focused_pane.map(|id| id.value()).unwrap_or_default(),
        workflow_status,
        batch_status,
        analysis_name,
        analysis_coverage_complete: coverage.complete,
        memory_count,
        plugin_status,
        history_count,
    })
}

fn smoke_plugin_manifest() -> String {
    serde_json::json!({
        "name": "studio-smoke",
        "description": "Studio smoke plugin",
        "permissions": ["shell"],
        "actions": [{
            "name": "Plugin Studio Smoke",
            "description": "Run Studio smoke plugin",
            "when_to_use": "When validating std-studio smoke",
            "kind": "shell",
            "command": "printf studio-smoke",
            "tags": ["studio-smoke"]
        }]
    })
    .to_string()
}
