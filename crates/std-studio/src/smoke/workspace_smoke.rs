use std_studio::{StudioApp, StudioPane, WorkspacePaneId};

pub(crate) struct WorkspacePaneSmoke {
    pub(crate) opened: bool,
    pub(crate) focus_switched: bool,
    pub(crate) closed: bool,
    pub(crate) focus_restored: bool,
    pub(crate) deduplicated: bool,
    pub(crate) content_keys: String,
    pub(crate) focused_title: String,
    pub(crate) restored_title: String,
    pub(crate) closed_removed: bool,
    pub(crate) state_preserved_after_focus: bool,
    pub(crate) focus_label: String,
}

pub(crate) fn run_workspace_pane_smoke(
    studio: &mut StudioApp,
    close_target: WorkspacePaneId,
) -> WorkspacePaneSmoke {
    let settings = studio.open_settings_pane();
    let opened = studio.focused_pane == Some(settings);
    let duplicate_settings = studio.open_settings_pane();
    let plugin = studio.open_plugin_manager_pane();
    let focus_switched = studio.focused_pane == Some(plugin);
    let focused_title = pane_title(studio, plugin);
    let content_keys = workspace_content_keys(studio);
    let state_preserved_after_focus = pane_lines(studio, plugin)
        .iter()
        .any(|line| line.contains("action=reload,search,manifest_check,preview,run"));
    let closed = studio.close_workspace_pane(close_target);
    let closed_removed = !studio
        .open_workspace_panes()
        .any(|pane| pane.id == close_target);
    let focus_restored = studio.focus_workspace_pane(settings)
        && studio.close_workspace_pane(settings)
        && studio.focused_pane == Some(plugin);
    let restored_title = pane_title(studio, plugin);
    let focus_label = format!("focused={},title={restored_title}", plugin.value());
    WorkspacePaneSmoke {
        opened,
        focus_switched,
        closed,
        focus_restored,
        deduplicated: settings == duplicate_settings,
        content_keys,
        focused_title,
        restored_title,
        closed_removed,
        state_preserved_after_focus,
        focus_label,
    }
}

fn workspace_content_keys(studio: &StudioApp) -> String {
    StudioPane::all()
        .into_iter()
        .map(|pane| pane.content_key())
        .chain(
            studio
                .open_workspace_panes()
                .map(|pane| pane.kind.content_key()),
        )
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(",")
}

fn pane_title(studio: &StudioApp, id: WorkspacePaneId) -> String {
    studio
        .open_workspace_panes()
        .find(|pane| pane.id == id)
        .map(|pane| pane.title.clone())
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn pane_lines(studio: &StudioApp, id: WorkspacePaneId) -> Vec<String> {
    studio
        .open_workspace_panes()
        .find(|pane| pane.id == id)
        .map(|pane| studio.workspace_pane_content(&pane.kind).lines)
        .unwrap_or_default()
}
