use std_studio::{StudioApp, StudioPane, WorkspacePaneKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StudioCommandAction {
    SwitchPane(StudioPane),
    OpenWorkspace(StudioPane),
    OpenSettings,
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioCommandItem {
    pub title: String,
    pub detail: String,
    pub shortcut: String,
    pub action: StudioCommandAction,
}

pub(crate) fn command_palette_items(app: &StudioApp) -> Vec<StudioCommandItem> {
    let mut items = StudioPane::all()
        .into_iter()
        .map(|pane| StudioCommandItem {
            title: format!("Show {}", pane.label()),
            detail: "Switch main Studio workspace".to_string(),
            shortcut: "Enter".to_string(),
            action: StudioCommandAction::SwitchPane(pane),
        })
        .collect::<Vec<_>>();
    items.push(StudioCommandItem {
        title: "Refresh Workspace State".to_string(),
        detail: format!("{} open panes", app.open_workspace_panes().count()),
        shortcut: "Enter".to_string(),
        action: StudioCommandAction::Refresh,
    });
    items.push(StudioCommandItem {
        title: "Open Settings".to_string(),
        detail: app.config_path().display().to_string(),
        shortcut: "Mod+,".to_string(),
        action: StudioCommandAction::OpenSettings,
    });
    items
}

pub(crate) fn quick_open_items(app: &StudioApp) -> Vec<StudioCommandItem> {
    let mut items = app
        .open_workspace_panes()
        .map(|pane| StudioCommandItem {
            title: pane.title.clone(),
            detail: pane.kind.content_key().to_string(),
            shortcut: "Enter".to_string(),
            action: StudioCommandAction::OpenWorkspace(main_pane(&pane.kind)),
        })
        .collect::<Vec<_>>();
    if items.is_empty() {
        items.push(StudioCommandItem {
            title: "Open Workflow Builder".to_string(),
            detail: app.core.config.workflows_dir().display().to_string(),
            shortcut: "Enter".to_string(),
            action: StudioCommandAction::OpenWorkspace(StudioPane::Workflows),
        });
        items.push(StudioCommandItem {
            title: "Open Analysis Workbench".to_string(),
            detail: app.core.config.data_dir.display().to_string(),
            shortcut: "Enter".to_string(),
            action: StudioCommandAction::OpenWorkspace(StudioPane::Analysis),
        });
    }
    items
}

fn main_pane(kind: &WorkspacePaneKind) -> StudioPane {
    match kind {
        WorkspacePaneKind::Pane(pane) => *pane,
        WorkspacePaneKind::WorkflowBuilder { .. } => StudioPane::Workflows,
        WorkspacePaneKind::AnalysisWorkbench { .. } => StudioPane::Analysis,
        WorkspacePaneKind::AppManager => StudioPane::Apps,
        WorkspacePaneKind::MemoryBrowser => StudioPane::Memory,
        WorkspacePaneKind::ExecutionHistory => StudioPane::History,
        WorkspacePaneKind::PluginManager => StudioPane::Plugins,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_palette_items_cover_studio_panes_and_settings() {
        let app = StudioApp::default();
        let items = command_palette_items(&app);

        assert!(items.iter().any(|item| item.title == "Show Dashboard"));
        assert!(items.iter().any(|item| item.title == "Show Operations"));
        assert!(items.iter().any(|item| item.title == "Open Settings"));
    }

    #[test]
    fn quick_open_falls_back_to_real_workspace_entry_points() {
        let app = StudioApp::default();
        let items = quick_open_items(&app);

        assert!(items
            .iter()
            .any(|item| item.title == "Open Workflow Builder"));
        assert!(items
            .iter()
            .any(|item| item.title == "Open Analysis Workbench"));
    }
}
