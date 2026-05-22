use crate::workspace_panes::StudioWorkspaceSpec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum WorkspacePaneLayoutKind {
    Dashboard,
    WorkflowBuilder,
    AnalysisWorkbench,
    PluginManager,
    MemoryBrowser,
    ExecutionHistory,
    Operations,
    Settings,
    AppManager,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct WorkspacePaneLayoutSpec {
    pub(crate) kind: WorkspacePaneLayoutKind,
    pub(crate) toolbar: &'static str,
    pub(crate) primary: &'static str,
    pub(crate) secondary: &'static str,
    pub(crate) inspector: &'static str,
    pub(crate) bottom: &'static str,
}

pub(crate) fn layout_for_spec(spec: &StudioWorkspaceSpec) -> WorkspacePaneLayoutSpec {
    if spec.workflow_path.is_some() {
        return WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::WorkflowBuilder,
            toolbar: "Save | Test | Simulate | History | AI",
            primary: "Steps list, keyboard reorder, Add Step",
            secondary: "Step Properties schema form",
            inspector: "Trace, payload, selected step metadata",
            bottom: "Batch Debug opens for preview and run",
        };
    }
    if spec.analysis_path.is_some() {
        return WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::AnalysisWorkbench,
            toolbar: "Target | Re-Index | Q&A",
            primary: "Overview, Components, Symbols, Relations, Q&A",
            secondary: "Search hits and quoted sources",
            inspector: "Coverage, selected component, relation detail",
            bottom: "Index logs and problems",
        };
    }
    match spec.content_key {
        "dashboard" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::Dashboard,
            toolbar: "Refresh | Open Current Pane",
            primary: "Workspace overview and recent activity",
            secondary: "Workflow, Plugin, Index, Release signals",
            inspector: "Runtime paths and policy evidence",
            bottom: "Problems and performance summary",
        },
        "plugins" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::PluginManager,
            toolbar: "Install from path | Reload Plugins",
            primary: "Plugin manifest list and JS/TS runtime status",
            secondary: "Commands, audit log, enable switch",
            inspector: "Permissions and manifest checks",
            bottom: "Plugin runner output",
        },
        "memory" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::MemoryBrowser,
            toolbar: "Search | Scope | Time Range",
            primary: "Memory list grouped by scope and tags",
            secondary: "Selected memory detail",
            inspector: "Related events and Pin to Workflow",
            bottom: "Recall evidence",
        },
        "history" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::ExecutionHistory,
            toolbar: "Filter | Status | Workflow",
            primary: "Execution table",
            secondary: "Timeline and payload",
            inspector: "Failure detail and retry context",
            bottom: "Trace events",
        },
        "operations" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::Operations,
            toolbar: "Quality | Release | Install | Doctor",
            primary: "Gate matrix with command evidence",
            secondary: "Current result and artifact path",
            inspector: "Manual opt-in gates",
            bottom: "Command output",
        },
        "settings" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::Settings,
            toolbar: "Save Settings",
            primary: "Appearance, Hotkeys, AI Provider, Index, Plugins, Privacy, About",
            secondary: "Selected settings form",
            inspector: "Validation and config path",
            bottom: "Pending changes",
        },
        "apps" => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::AppManager,
            toolbar: "Register | Search | Preview",
            primary: "Application registry and localized aliases",
            secondary: "Preview and launch contract",
            inspector: "External runner guard",
            bottom: "Discovery log",
        },
        _ => WorkspacePaneLayoutSpec {
            kind: WorkspacePaneLayoutKind::Dashboard,
            toolbar: "Refresh",
            primary: "Workspace content",
            secondary: "Details",
            inspector: "Context",
            bottom: "Status",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_studio::{StudioPane, WorkspacePaneId};

    #[test]
    fn workspace_layout_maps_product_panes_to_docs22_regions() {
        let workflow = spec("workflows", Some("daily.json"), None, StudioPane::Workflows);
        let analysis = spec("analysis", None, Some("."), StudioPane::Analysis);
        let plugins = spec("plugins", None, None, StudioPane::Plugins);
        let settings = spec("settings", None, None, StudioPane::Settings);

        assert_eq!(
            layout_for_spec(&workflow).kind,
            WorkspacePaneLayoutKind::WorkflowBuilder
        );
        assert_eq!(
            layout_for_spec(&analysis).kind,
            WorkspacePaneLayoutKind::AnalysisWorkbench
        );
        assert_eq!(
            layout_for_spec(&plugins).kind,
            WorkspacePaneLayoutKind::PluginManager
        );
        assert_eq!(
            layout_for_spec(&settings).kind,
            WorkspacePaneLayoutKind::Settings
        );
        assert!(layout_for_spec(&workflow).bottom.contains("Batch Debug"));
        assert!(layout_for_spec(&analysis).inspector.contains("Coverage"));
        assert!(layout_for_spec(&plugins)
            .primary
            .contains("JS/TS runtime status"));
    }

    fn spec(
        content_key: &'static str,
        workflow: Option<&str>,
        analysis: Option<&str>,
        pane: StudioPane,
    ) -> StudioWorkspaceSpec {
        StudioWorkspaceSpec {
            id: WorkspacePaneId::new(1),
            title: "Pane".to_string(),
            content_key,
            heading: "Pane".to_string(),
            lines: Vec::new(),
            pane,
            workflow_path: workflow.map(Into::into),
            analysis_path: analysis.map(Into::into),
        }
    }
}
