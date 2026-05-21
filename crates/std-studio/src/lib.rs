//! std-studio - Professional workspace pane environment.
//!
//! This is the main place for Workflow building and project analysis.
//! Shared Studio application state.

mod analysis_model;
mod analysis_workbench;
mod apps;
mod ops_evidence;
mod ops_runbook;
mod planned_workflow;
pub mod plugin_security;
pub mod plugin_status;
mod trace;
mod workflow;
mod workspace_pane;
mod workspace_policy;

pub use analysis_workbench::{
    AnalysisAnswerSource, AnalysisCoverageLayer, AnalysisInspectionSummary, AnalysisOverviewCard,
    AnalysisSearchHit, AnalysisTab, AnalysisWorkbenchTab, AnalysisWorkbenchViewModel,
};
pub use ops_evidence::{OpsEvidence, OpsGate, OpsStatus};
use std_core::{StdConfig, StdCore};
use std_egui::{MemoryBrowserViewModel, PluginManagerViewModel, StudioDashboardViewModel};
use std_index::{
    IndexAnswer, IndexCoverageReport, IndexDocument, IndexError, IndexInspection, IndexSearchResult,
};
use std_orchestration::{
    BatchExecutor, BatchPlan, BatchReport, Workflow, WorkflowDryRun, WorkflowExecution,
};
pub use std_orchestration::{WorkflowExecutionTrace, WorkflowTraceStep};
use std_types::{ActionExecution, SearchResult};
pub use workflow::built_in_studio_preview_workflow;
pub use workspace_pane::{WorkspacePane, WorkspacePaneContent, WorkspacePaneId, WorkspacePaneKind};
pub use workspace_policy::{HostWindowPolicy, PaneSystemPolicy, StudioWorkspacePolicy};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudioPane {
    Dashboard,
    Workflows,
    Apps,
    Memory,
    Plugins,
    Analysis,
    History,
    Operations,
    Settings,
}

impl StudioPane {
    pub fn label(self) -> &'static str {
        match self {
            StudioPane::Dashboard => "Dashboard",
            StudioPane::Workflows => "Workflows",
            StudioPane::Apps => "Apps",
            StudioPane::Memory => "Memory",
            StudioPane::Plugins => "Plugins",
            StudioPane::Analysis => "Analysis",
            StudioPane::History => "History",
            StudioPane::Operations => "Operations",
            StudioPane::Settings => "Settings",
        }
    }

    pub fn all() -> [StudioPane; 9] {
        [
            StudioPane::Dashboard,
            StudioPane::Workflows,
            StudioPane::Apps,
            StudioPane::Memory,
            StudioPane::Plugins,
            StudioPane::Analysis,
            StudioPane::History,
            StudioPane::Operations,
            StudioPane::Settings,
        ]
    }

    pub fn content_key(self) -> &'static str {
        match self {
            StudioPane::Dashboard => "dashboard",
            StudioPane::Workflows => "workflows",
            StudioPane::Apps => "apps",
            StudioPane::Memory => "memory",
            StudioPane::Plugins => "plugins",
            StudioPane::Analysis => "analysis",
            StudioPane::History => "history",
            StudioPane::Operations => "operations",
            StudioPane::Settings => "settings",
        }
    }
}

pub struct StudioApp {
    pub name: String,
    pub core: StdCore,
    pub dashboard: StudioDashboardViewModel,
    pub memory_browser: MemoryBrowserViewModel,
    pub plugin_manager: PluginManagerViewModel,
    pub active_pane: StudioPane,
    pub workflow_debug: Option<WorkflowDryRun>,
    pub last_workflow_execution: Option<WorkflowExecution>,
    pub last_batch_report: Option<BatchReport>,
    pub active_analysis: Option<IndexDocument>,
    pub planned_workflow: Option<Workflow>,
    pub workspace_panes: Vec<WorkspacePane>,
    pub focused_pane: Option<WorkspacePaneId>,
    pub workspace_policy: StudioWorkspacePolicy,
    next_pane_serial: u64,
    next_focus_serial: u64,
}

impl Default for StudioApp {
    fn default() -> Self {
        let core = StdCore::with_config(StdConfig::load());
        core.seed_builtin_actions().ok();
        let dashboard = StudioDashboardViewModel::load(&core);
        let memory_browser = MemoryBrowserViewModel::load(&core);
        let plugin_manager = PluginManagerViewModel::load(&core);
        Self {
            name: "std-cli Studio".to_string(),
            core,
            dashboard,
            memory_browser,
            plugin_manager,
            active_pane: StudioPane::Dashboard,
            workflow_debug: None,
            last_workflow_execution: None,
            last_batch_report: None,
            active_analysis: None,
            planned_workflow: None,
            workspace_panes: Vec::new(),
            focused_pane: None,
            workspace_policy: StudioWorkspacePolicy::studio_v1(),
            next_pane_serial: 1,
            next_focus_serial: 1,
        }
    }
}

impl StudioApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_core(core: StdCore) -> Self {
        core.seed_builtin_actions().ok();
        let dashboard = StudioDashboardViewModel::load(&core);
        let memory_browser = MemoryBrowserViewModel::load(&core);
        let plugin_manager = PluginManagerViewModel::load(&core);
        Self {
            name: "std-cli Studio".to_string(),
            core,
            dashboard,
            memory_browser,
            plugin_manager,
            active_pane: StudioPane::Dashboard,
            workflow_debug: None,
            last_workflow_execution: None,
            last_batch_report: None,
            active_analysis: None,
            planned_workflow: None,
            workspace_panes: Vec::new(),
            focused_pane: None,
            workspace_policy: StudioWorkspacePolicy::studio_v1(),
            next_pane_serial: 1,
            next_focus_serial: 1,
        }
    }

    pub fn refresh(&mut self) {
        self.dashboard = StudioDashboardViewModel::load(&self.core);
        self.memory_browser = MemoryBrowserViewModel::load(&self.core);
        self.plugin_manager = PluginManagerViewModel::load(&self.core);
    }

    pub fn switch_pane(&mut self, pane: StudioPane) {
        self.active_pane = pane;
    }

    pub fn open_workspace_pane(&mut self, pane: StudioPane) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::Pane(pane))
    }

    pub fn open_workflow_builder(&mut self, workflow_path: std::path::PathBuf) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::WorkflowBuilder { workflow_path })
    }

    pub fn open_analysis_workbench(&mut self, entity_path: std::path::PathBuf) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::AnalysisWorkbench { entity_path })
    }

    pub fn open_plugin_manager_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::PluginManager)
    }

    pub fn open_app_manager_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::AppManager)
    }

    pub fn open_memory_browser_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::MemoryBrowser)
    }

    pub fn open_execution_history_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::ExecutionHistory)
    }

    pub fn open_settings_pane(&mut self) -> WorkspacePaneId {
        self.open_pane(WorkspacePaneKind::Settings)
    }

    pub fn focus_workspace_pane(&mut self, id: WorkspacePaneId) -> bool {
        let Some(index) = self.workspace_panes.iter().position(|pane| pane.id == id) else {
            return false;
        };
        let serial = self.next_focus_serial();
        let pane = &mut self.workspace_panes[index];
        pane.open = true;
        pane.focus(serial);
        self.focused_pane = Some(id);
        true
    }

    pub fn focus_next_workspace_pane(&mut self) -> Option<WorkspacePaneId> {
        self.focus_workspace_pane_by_offset(1)
    }

    pub fn focus_previous_workspace_pane(&mut self) -> Option<WorkspacePaneId> {
        self.focus_workspace_pane_by_offset(-1)
    }

    pub fn close_workspace_pane(&mut self, id: WorkspacePaneId) -> bool {
        let Some(index) = self.workspace_panes.iter().position(|pane| pane.id == id) else {
            return false;
        };
        self.workspace_panes.remove(index);
        if self.focused_pane == Some(id) {
            self.focused_pane = self
                .workspace_panes
                .iter()
                .filter(|pane| pane.open)
                .max_by_key(|pane| pane.focus_serial)
                .map(|pane| pane.id);
        }
        true
    }

    pub fn open_workspace_panes(&self) -> impl Iterator<Item = &WorkspacePane> {
        self.workspace_panes.iter().filter(|pane| pane.open)
    }

    fn focus_workspace_pane_by_offset(&mut self, offset: isize) -> Option<WorkspacePaneId> {
        let open_ids = self
            .workspace_panes
            .iter()
            .filter(|pane| pane.open)
            .map(|pane| pane.id)
            .collect::<Vec<_>>();
        if open_ids.is_empty() {
            self.focused_pane = None;
            return None;
        }
        let current = self
            .focused_pane
            .and_then(|id| open_ids.iter().position(|open_id| *open_id == id))
            .unwrap_or(0);
        let next = (current as isize + offset).rem_euclid(open_ids.len() as isize) as usize;
        let id = open_ids[next];
        self.focus_workspace_pane(id).then_some(id)
    }

    fn open_pane(&mut self, kind: WorkspacePaneKind) -> WorkspacePaneId {
        let identity_key = kind.identity_key();
        if let Some(index) = self
            .workspace_panes
            .iter()
            .position(|pane| pane.kind.identity_key() == identity_key)
        {
            let serial = self.next_focus_serial();
            let existing = &mut self.workspace_panes[index];
            existing.open = true;
            existing.focus(serial);
            self.focused_pane = Some(existing.id);
            return existing.id;
        }

        let id = WorkspacePaneId::new(self.next_pane_serial);
        self.next_pane_serial += 1;
        let focus_serial = self.next_focus_serial();
        let pane = WorkspacePane::new(id, kind, focus_serial);
        self.focused_pane = Some(id);
        self.workspace_panes.push(pane);
        id
    }

    fn next_focus_serial(&mut self) -> u64 {
        let serial = self.next_focus_serial;
        self.next_focus_serial += 1;
        serial
    }

    pub fn run_batch_plan(&mut self, plan: &BatchPlan) -> &BatchReport {
        let report = BatchExecutor::new(self.core.clone()).execute(plan);
        self.last_batch_report = Some(report);
        self.refresh();
        self.last_batch_report
            .as_ref()
            .expect("last_batch_report is set")
    }

    pub fn run_batch_json(&mut self, body: &str) -> Result<&BatchReport, serde_json::Error> {
        let plan: BatchPlan = serde_json::from_str(body)?;
        Ok(self.run_batch_plan(&plan))
    }

    pub fn analyze_entity(&mut self, path: &std::path::Path) -> Result<&IndexDocument, IndexError> {
        let document = analysis_model::analyze_entity(&self.core, path)?;
        self.active_analysis = Some(document);
        Ok(self
            .active_analysis
            .as_ref()
            .expect("active_analysis is set"))
    }

    pub fn saved_analyses(&self) -> Result<Vec<IndexDocument>, IndexError> {
        analysis_model::saved_analyses(&self.core)
    }

    pub fn search_analyses(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<IndexSearchResult>, IndexError> {
        analysis_model::search_analyses(&self.core, query, limit)
    }

    pub fn ask_analyses(&self, query: &str, limit: usize) -> Result<IndexAnswer, IndexError> {
        analysis_model::ask_analyses(&self.core, query, limit)
    }

    pub fn inspect_analysis(
        &self,
        name_or_path: &str,
        limit: usize,
    ) -> Result<Option<IndexInspection>, IndexError> {
        analysis_model::inspect_analysis(&self.core, name_or_path, limit)
    }

    pub fn analysis_coverage_report(&self) -> Result<IndexCoverageReport, IndexError> {
        analysis_model::analysis_coverage_report(&self.core)
    }

    pub fn recent_workflow_traces(
        &self,
        limit: usize,
    ) -> Result<Vec<WorkflowExecutionTrace>, trace::TraceError> {
        trace::recent_workflow_traces(&self.core, limit)
    }

    pub fn reload_plugins(&mut self) -> Result<&PluginManagerViewModel, std_core::CoreError> {
        self.core.register_plugin_tools()?;
        self.plugin_manager = PluginManagerViewModel::load(&self.core);
        Ok(&self.plugin_manager)
    }

    pub fn search_plugins(&mut self, query: &str) -> Vec<SearchResult> {
        self.plugin_manager.search(&self.core, query);
        self.plugin_manager.plugin_actions.clone()
    }

    pub fn run_selected_plugin(&mut self) -> Option<ActionExecution> {
        self.plugin_manager.run_selected(&self.core)
    }

    pub fn search_memory(&mut self, query: &str) -> Vec<std_types::MemoryRecord> {
        self.memory_browser.search(&self.core, query);
        self.memory_browser.memories.clone()
    }

    pub fn select_memory(&mut self, index: usize) -> Option<std_types::MemoryRecord> {
        self.memory_browser.select(index);
        self.memory_browser.selected_memory().cloned()
    }

    pub fn remember_from_studio(
        &mut self,
        scope: &str,
        title: &str,
        body: &str,
        tags: Vec<String>,
    ) -> Result<std_types::MemoryRecord, std_core::CoreError> {
        let memory = self
            .memory_browser
            .remember(&self.core, scope, title, body, tags)?;
        self.refresh();
        Ok(memory)
    }

    pub fn config_path(&self) -> std::path::PathBuf {
        StdConfig::writable_config_path()
    }

    pub fn config_value(&self, key: &str) -> Option<String> {
        self.core.config.get_field(key)
    }

    pub fn save_config_field(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<std::path::PathBuf, String> {
        let path = StdConfig::writable_config_path();
        self.save_config_field_to(&path, key, value)
    }

    pub fn save_config_field_to(
        &mut self,
        path: &std::path::Path,
        key: &str,
        value: &str,
    ) -> Result<std::path::PathBuf, String> {
        let mut config = self.core.config.clone();
        config.set_field(key, value)?;
        config.save_to(path).map_err(|error| error.to_string())?;
        self.core = StdCore::with_config(config);
        self.core
            .seed_builtin_actions()
            .map_err(|error| error.to_string())?;
        self.core
            .register_local_content_actions()
            .map_err(|error| error.to_string())?;
        self.refresh();
        Ok(path.to_path_buf())
    }
}

fn index_error_from_core(error: std_core::CoreError) -> IndexError {
    IndexError::Io(std::io::Error::other(error.to_string()))
}

#[cfg(test)]
mod tests;
