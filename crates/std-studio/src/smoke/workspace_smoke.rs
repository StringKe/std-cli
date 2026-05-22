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
    pub(crate) settings_kind: String,
    pub(crate) closed_removed: bool,
    pub(crate) state_preserved_after_focus: bool,
    pub(crate) focus_label: String,
    pub(crate) host_policy: String,
    pub(crate) management_sequence: String,
    pub(crate) focus_switch_path: String,
    pub(crate) close_restore_path: String,
    pub(crate) settings_contract: String,
    pub(crate) main_path_contract: String,
}

pub(crate) fn run_workspace_pane_smoke(
    studio: &mut StudioApp,
    close_target: WorkspacePaneId,
) -> WorkspacePaneSmoke {
    let settings = studio.open_settings_pane();
    let opened = studio.focused_pane == Some(settings);
    let settings_kind = pane_kind_label(studio, settings);
    let settings_key = pane_content_key(studio, settings);
    let duplicate_settings = studio.open_settings_pane();
    let alias_workflow = studio.open_workflow_builder(std::path::PathBuf::from(
        "./workspace-smoke/./workflow.json",
    ));
    let alias_workflow_duplicate =
        studio.open_workflow_builder(std::path::PathBuf::from("workspace-smoke/workflow.json"));
    let alias_analysis =
        studio.open_analysis_workbench(std::path::PathBuf::from("workspace-smoke/src/../src"));
    let alias_analysis_duplicate =
        studio.open_analysis_workbench(std::path::PathBuf::from("workspace-smoke/src"));
    let plugin = studio.open_plugin_manager_pane();
    let focus_switched = studio.focused_pane == Some(plugin);
    let focused_title = pane_title(studio, plugin);
    let focused_key = pane_content_key(studio, plugin);
    let content_keys = workspace_content_keys(studio);
    let tabs =
        crate::workspace_tabs::workspace_tab_specs(&studio.workspace_panes, studio.focused_pane);
    let tabs_contract = crate::workspace_tabs::workspace_tabs_contract(&tabs);
    let state_preserved_after_focus = pane_lines(studio, plugin)
        .iter()
        .any(|line| line.contains("action=reload,search,manifest_check,preview,run"));
    let open_count_before_close = studio.open_workspace_panes().count();
    let closed = studio.close_workspace_pane(close_target);
    let open_count_after_close = studio.open_workspace_panes().count();
    let closed_removed = !studio
        .open_workspace_panes()
        .any(|pane| pane.id == close_target);
    let reopened = studio.open_memory_browser_pane();
    let open_count_after_reopen = studio.open_workspace_panes().count();
    let reopened_restored = reopened == close_target
        && pane_content_key(studio, reopened) == "memory"
        && pane_lines(studio, reopened)
            .iter()
            .any(|line| line.starts_with("memories="));
    let plugin_refocused = studio.focus_workspace_pane(plugin);
    let focus_restored = studio.focus_workspace_pane(settings)
        && studio.close_workspace_pane(settings)
        && studio.focused_pane == Some(plugin)
        && plugin_refocused;
    let restored_title = pane_title(studio, plugin);
    let restored_key = pane_content_key(studio, plugin);
    let focus_switch_path = format!("{}>{}>{}", settings_key, focused_key, restored_key);
    let close_restore_path = format!(
        "close:{}>restore:{}",
        close_target.value(),
        studio.focused_pane.map(|id| id.value()).unwrap_or_default()
    );
    let settings_contract = crate::views::settings_model::settings_contract();
    let settings_contract = format!(
        "surface={},navigation={},categories={},hotkey_source={},hotkey_reset={},hotkey_control={},theme_modes={},theme_control={},zoom_levels={},zoom_control={},motion_control={},contrast_control={},transparency_control={},appearance_profile={},ai_control={},storage_control={}",
        settings_contract.surface,
        settings_contract.navigation,
        settings_contract.categories.join("|"),
        settings_contract.hotkey_source,
        settings_contract.hotkey_reset,
        settings_contract.hotkey_control,
        settings_contract.theme_modes.join("|"),
        settings_contract.theme_control,
        settings_contract.zoom_levels.join("|"),
        settings_contract.zoom_control,
        settings_contract.motion_control,
        settings_contract.contrast_control,
        settings_contract.transparency_control,
        settings_contract.appearance_profile,
        settings_contract.ai_control,
        settings_contract.storage_control
    );
    let host_policy = studio.app_workspace_policy_report();
    let main_path_contract = workspace_main_path_contract();
    let management_sequence = "open>dedupe>focus>switch>close>reopen>restore".to_string();
    let closeguard_roundtrip = closeguard_roundtrip_evidence(studio);
    let focus_label = workspace_management_evidence(WorkspaceManagementEvidence {
        plugin,
        reopened,
        restored_title: &restored_title,
        open_count_before_close,
        open_count_after_close,
        open_count_after_reopen,
        reopened_restored,
        state_preserved_after_focus,
        tabs_contract: &tabs_contract,
        closeguard_roundtrip: &closeguard_roundtrip,
    });
    WorkspacePaneSmoke {
        opened,
        focus_switched,
        closed,
        focus_restored,
        deduplicated: settings == duplicate_settings
            && alias_workflow == alias_workflow_duplicate
            && alias_analysis == alias_analysis_duplicate,
        content_keys,
        focused_title,
        restored_title,
        settings_kind,
        closed_removed,
        state_preserved_after_focus,
        focus_label,
        host_policy,
        management_sequence,
        focus_switch_path,
        close_restore_path,
        settings_contract,
        main_path_contract,
    }
}

fn workspace_main_path_contract() -> String {
    [
        "host=single-borderless-egui-viewport",
        "panes=internal-egui-workspace-panes",
        "extra_viewports=forbidden",
        "show_viewport=forbidden",
        "show_viewport_api=false",
        "viewport_id=forbidden",
        "egui_window=forbidden",
        "egui_window_api=false",
        "settings_overlay=forbidden",
        "settings_overlay=false",
        "allowed_viewport_files=viewport|host_chrome|preview",
    ]
    .join(",")
}

struct WorkspaceManagementEvidence<'a> {
    plugin: WorkspacePaneId,
    reopened: WorkspacePaneId,
    restored_title: &'a str,
    open_count_before_close: usize,
    open_count_after_close: usize,
    open_count_after_reopen: usize,
    reopened_restored: bool,
    state_preserved_after_focus: bool,
    tabs_contract: &'a str,
    closeguard_roundtrip: &'a str,
}

fn workspace_management_evidence(evidence: WorkspaceManagementEvidence<'_>) -> String {
    format!(
        "strategy=internal-egui-workspace-panes,host=single-borderless-egui-viewport,sequence=open>focus>switch>close>reopen>restore,counts=before-close:{}|after-close:{}|after-reopen:{},state_preserved={},forbidden=native-child-windows:false|detached-panels:false,focused={},title={},reopened_memory={},reopened_restored={},tabs={},closeguard={}",
        evidence.open_count_before_close,
        evidence.open_count_after_close,
        evidence.open_count_after_reopen,
        evidence.state_preserved_after_focus,
        evidence.plugin.value(),
        evidence.restored_title,
        evidence.reopened.value(),
        evidence.reopened_restored,
        evidence.tabs_contract,
        evidence.closeguard_roundtrip
    )
}

fn closeguard_roundtrip_evidence(studio: &StudioApp) -> String {
    match closeguard_roundtrip_evidence_result(studio) {
        Ok(evidence) => evidence,
        Err(error) => format!("disk_roundtrip=FAIL,error={error}"),
    }
}

fn closeguard_roundtrip_evidence_result(studio: &StudioApp) -> Result<String, std::io::Error> {
    let mut isolated = StudioApp::with_core(studio.core.clone());
    let dashboard = isolated.open_workspace_pane(StudioPane::Dashboard);
    let workflow = isolated.open_workflow_builder(std::path::PathBuf::from(
        "workspace-smoke/closeguard-workflow.json",
    ));
    let plugins = isolated.open_plugin_manager_pane();
    isolated.focus_workspace_pane(workflow);
    let path = isolated.close_workspace_instance_to_disk()?;
    let body = std::fs::read_to_string(&path)?;
    let body_has_native = body.contains("native_window") || body.contains("detached");
    isolated.workspace_panes.clear();
    isolated.restore_workspace_closeguard_from_disk()?;
    let restored_ids = isolated
        .open_workspace_panes()
        .map(|pane| pane.id.value().to_string())
        .collect::<Vec<_>>()
        .join("|");
    let restored = isolated.open_workspace_panes().count() == 3
        && isolated.focused_pane == Some(workflow)
        && isolated
            .open_workspace_panes()
            .any(|pane| pane.id == dashboard)
        && isolated
            .open_workspace_panes()
            .any(|pane| pane.id == plugins)
        && !body_has_native;
    Ok(format!(
        "disk_roundtrip={},saved=true,restored_count={},focused={},ids={},native_terms={}",
        restored,
        isolated.open_workspace_panes().count(),
        isolated
            .focused_pane
            .map(|id| id.value().to_string())
            .unwrap_or_else(|| "none".to_string()),
        restored_ids,
        body_has_native
    ))
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

fn pane_kind_label(studio: &StudioApp, id: WorkspacePaneId) -> String {
    studio
        .open_workspace_panes()
        .find(|pane| pane.id == id)
        .map(|pane| format!("{:?}", pane.kind))
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn pane_content_key(studio: &StudioApp, id: WorkspacePaneId) -> &'static str {
    studio
        .open_workspace_panes()
        .find(|pane| pane.id == id)
        .map(|pane| pane.kind.content_key())
        .unwrap_or("UNKNOWN")
}

fn pane_lines(studio: &StudioApp, id: WorkspacePaneId) -> Vec<String> {
    studio
        .open_workspace_panes()
        .find(|pane| pane.id == id)
        .map(|pane| studio.workspace_pane_content(&pane.kind).lines)
        .unwrap_or_default()
}
