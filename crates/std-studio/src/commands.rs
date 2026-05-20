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

pub(crate) fn move_selection(selected: usize, delta: isize, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    selected.saturating_add_signed(delta).min(len - 1)
}

pub(crate) fn filter_items(items: &[StudioCommandItem], query: &str) -> Vec<StudioCommandItem> {
    let query = query.trim().to_ascii_lowercase();
    if query.is_empty() {
        return items.to_vec();
    }
    items
        .iter()
        .filter(|item| command_matches(item, &query))
        .cloned()
        .collect()
}

pub(crate) fn selected_action(
    items: &[StudioCommandItem],
    selected: usize,
) -> Option<StudioCommandAction> {
    items.get(selected).map(|item| item.action)
}

fn command_matches(item: &StudioCommandItem, query: &str) -> bool {
    item.title.to_ascii_lowercase().contains(query)
        || item.detail.to_ascii_lowercase().contains(query)
        || item.shortcut.to_ascii_lowercase().contains(query)
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

    #[test]
    fn command_selection_clamps_to_edges() {
        assert_eq!(move_selection(0, -1, 3), 0);
        assert_eq!(move_selection(0, 1, 3), 1);
        assert_eq!(move_selection(2, 1, 3), 2);
        assert_eq!(move_selection(2, -1, 3), 1);
        assert_eq!(move_selection(3, 1, 0), 0);
    }

    #[test]
    fn selected_action_returns_matching_command_action() {
        let app = StudioApp::default();
        let items = command_palette_items(&app);

        assert_eq!(
            selected_action(&items, 0),
            Some(StudioCommandAction::SwitchPane(StudioPane::Dashboard))
        );
        assert_eq!(selected_action(&items, usize::MAX), None);
    }

    #[test]
    fn command_filter_matches_title_detail_and_shortcut() {
        let app = StudioApp::default();
        let items = command_palette_items(&app);

        assert!(filter_items(&items, "dashboard")
            .iter()
            .any(|item| item.title == "Show Dashboard"));
        assert!(filter_items(&items, "workspace")
            .iter()
            .any(|item| item.title == "Show Dashboard"));
        assert!(filter_items(&items, "mod+,")
            .iter()
            .any(|item| item.title == "Open Settings"));
        assert!(filter_items(&items, "missing").is_empty());
    }
}
