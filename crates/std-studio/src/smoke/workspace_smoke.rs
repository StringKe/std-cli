use std_studio::{StudioApp, WorkspacePaneId};

pub(crate) struct WorkspacePaneSmoke {
    pub(crate) opened: bool,
    pub(crate) focus_switched: bool,
    pub(crate) closed: bool,
    pub(crate) focus_restored: bool,
}

pub(crate) fn run_workspace_pane_smoke(
    studio: &mut StudioApp,
    close_target: WorkspacePaneId,
) -> WorkspacePaneSmoke {
    let settings = studio.open_settings_pane();
    let opened = studio.focused_pane == Some(settings);
    let plugin = studio.open_plugin_manager_pane();
    let focus_switched = studio.focused_pane == Some(plugin);
    let closed = studio.close_workspace_pane(close_target);
    let focus_restored = studio.focus_workspace_pane(settings)
        && studio.close_workspace_pane(settings)
        && studio.focused_pane == Some(plugin);
    WorkspacePaneSmoke {
        opened,
        focus_switched,
        closed,
        focus_restored,
    }
}
