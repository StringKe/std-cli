use crate::{StudioApp, StudioPane};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WorkspacePaneId(u64);

impl WorkspacePaneId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspacePaneKind {
    Pane(StudioPane),
    WorkflowBuilder { workflow_path: std::path::PathBuf },
    AnalysisWorkbench { entity_path: std::path::PathBuf },
    AppManager,
    MemoryBrowser,
    ExecutionHistory,
    PluginManager,
}

impl WorkspacePaneKind {
    pub fn title(&self) -> String {
        match self {
            WorkspacePaneKind::Pane(pane) => pane.label().to_string(),
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => {
                format!("Workflow Builder: {}", display_name(workflow_path))
            }
            WorkspacePaneKind::AnalysisWorkbench { entity_path } => {
                format!("Analysis: {}", display_name(entity_path))
            }
            WorkspacePaneKind::AppManager => "App Manager".to_string(),
            WorkspacePaneKind::MemoryBrowser => "Memory Browser".to_string(),
            WorkspacePaneKind::ExecutionHistory => "Execution History".to_string(),
            WorkspacePaneKind::PluginManager => "Plugin Manager".to_string(),
        }
    }

    pub fn content_key(&self) -> &'static str {
        match self {
            WorkspacePaneKind::Pane(pane) => pane.content_key(),
            WorkspacePaneKind::WorkflowBuilder { .. } => "workflows",
            WorkspacePaneKind::AnalysisWorkbench { .. } => "analysis",
            WorkspacePaneKind::AppManager => "apps",
            WorkspacePaneKind::MemoryBrowser => "memory",
            WorkspacePaneKind::ExecutionHistory => "history",
            WorkspacePaneKind::PluginManager => "plugins",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePane {
    pub id: WorkspacePaneId,
    pub kind: WorkspacePaneKind,
    pub title: String,
    pub open: bool,
    pub focused_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspacePaneContent {
    pub content_key: &'static str,
    pub heading: String,
    pub lines: Vec<String>,
}

impl WorkspacePane {
    pub fn new(id: WorkspacePaneId, kind: WorkspacePaneKind) -> Self {
        let title = kind.title();
        Self {
            id,
            kind,
            title,
            open: true,
            focused_at: chrono::Utc::now(),
        }
    }

    pub fn focus(&mut self) {
        self.focused_at = chrono::Utc::now();
    }
}

fn display_name(path: &std::path::Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

impl StudioApp {
    pub fn workspace_pane_content(&self, kind: &WorkspacePaneKind) -> WorkspacePaneContent {
        match kind {
            WorkspacePaneKind::Pane(pane) => self.pane_content(*pane),
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => WorkspacePaneContent {
                content_key: kind.content_key(),
                heading: "Workflow Builder".to_string(),
                lines: vec![
                    format!("path={}", workflow_path.display()),
                    format!("planned={}", self.planned_workflow.is_some()),
                    format!(
                        "last_execution={}",
                        self.last_workflow_execution
                            .as_ref()
                            .map(|execution| format!("{:?}", execution.status))
                            .unwrap_or_else(|| "none".to_string())
                    ),
                ],
            },
            WorkspacePaneKind::AnalysisWorkbench { entity_path } => WorkspacePaneContent {
                content_key: kind.content_key(),
                heading: "Analysis Workbench".to_string(),
                lines: vec![
                    format!("path={}", entity_path.display()),
                    format!(
                        "active_analysis={}",
                        self.active_analysis
                            .as_ref()
                            .map(|document| document.overview.name.clone())
                            .unwrap_or_else(|| "none".to_string())
                    ),
                ],
            },
            WorkspacePaneKind::AppManager => self.pane_content(StudioPane::Apps),
            WorkspacePaneKind::MemoryBrowser => self.pane_content(StudioPane::Memory),
            WorkspacePaneKind::ExecutionHistory => self.pane_content(StudioPane::History),
            WorkspacePaneKind::PluginManager => self.pane_content(StudioPane::Plugins),
        }
    }

    pub fn window_content(&self, kind: &WorkspacePaneKind) -> WorkspacePaneContent {
        self.workspace_pane_content(kind)
    }

    fn pane_content(&self, pane: StudioPane) -> WorkspacePaneContent {
        let lines = match pane {
            StudioPane::Dashboard => vec![
                format!("actions={}", self.dashboard.action_count),
                format!("memory={}", self.dashboard.memory_count),
                format!("audit_events={}", self.dashboard.audit_event_count),
                "action=open_workspace_pane".to_string(),
            ],
            StudioPane::Workflows => vec![
                format!("planned={}", self.planned_workflow.is_some()),
                format!("debug={}", self.workflow_debug.is_some()),
                format!("batch={}", self.last_batch_report.is_some()),
                "action=create,edit,preview,run,batch".to_string(),
            ],
            StudioPane::Apps => vec![
                "action=register,search,preview,trigger".to_string(),
                "external_runner=NeedsExternalRunner".to_string(),
                format!("path={}", self.core.config.apps_dir().display()),
            ],
            StudioPane::Memory => vec![
                format!("memories={}", self.memory_browser.memories.len()),
                "action=search,select,remember".to_string(),
            ],
            StudioPane::Plugins => vec![
                format!(
                    "plugin_actions={}",
                    self.plugin_manager.plugin_actions.len()
                ),
                "action=reload,search,manifest_check,preview,run".to_string(),
            ],
            StudioPane::Analysis => vec![
                format!("active={}", self.active_analysis.is_some()),
                "action=analyze,search,ask,inspect,coverage".to_string(),
            ],
            StudioPane::History => vec![
                format!(
                    "last_workflow_execution={}",
                    self.last_workflow_execution.is_some()
                ),
                "trace=workflow,audit,event".to_string(),
            ],
            StudioPane::Operations => crate::OpsEvidence::load().lines(),
            StudioPane::Settings => vec![
                format!("config_path={}", self.config_path().display()),
                "action=save_config_field".to_string(),
            ],
        };
        WorkspacePaneContent {
            content_key: pane.content_key(),
            heading: pane.label().to_string(),
            lines,
        }
    }
}

pub type StudioWindow = WorkspacePane;
pub type StudioWindowContent = WorkspacePaneContent;
pub type StudioWindowId = WorkspacePaneId;
pub type StudioWindowKind = WorkspacePaneKind;
