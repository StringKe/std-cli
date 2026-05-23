use crate::{StudioApp, StudioPane, WorkspacePaneContent, WorkspacePaneKind};
use std_egui::i18n;

impl StudioApp {
    pub fn workspace_pane_content(&self, kind: &WorkspacePaneKind) -> WorkspacePaneContent {
        match kind {
            WorkspacePaneKind::Pane(pane) => self.pane_content(*pane),
            WorkspacePaneKind::WorkflowBuilder { workflow_path } => WorkspacePaneContent {
                content_key: kind.content_key(),
                heading: i18n::t("studio.shell.open.workflow.title").to_string(),
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
                heading: i18n::t("studio.shell.open.analysis.title").to_string(),
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
            WorkspacePaneKind::Settings => self.pane_content(StudioPane::Settings),
        }
    }

    fn pane_content(&self, pane: StudioPane) -> WorkspacePaneContent {
        let lines = match pane {
            StudioPane::Dashboard => vec![
                format!("actions={}", self.dashboard.action_count),
                format!("memory={}", self.dashboard.memory_count),
                format!("audit_events={}", self.dashboard.audit_event_count),
                format!("workspace_policy={}", self.workspace_policy.summary()),
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
