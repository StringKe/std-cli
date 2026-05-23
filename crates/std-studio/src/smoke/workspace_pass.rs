use super::StudioSmokeReport;

pub(crate) fn workspace_contract_pass(report: &StudioSmokeReport) -> bool {
    report.workspace_panes >= 7
        && report.pane_opened
        && report.pane_focus_switched
        && report.pane_closed
        && report.pane_focus_restored
        && report.pane_deduplicated
        && report.pane_content_keys.contains("dashboard")
        && report.pane_content_keys.contains("settings")
        && report.pane_focused_title == "插件管理"
        && report.pane_restored_title == "插件管理"
        && report.pane_settings_kind == "Settings"
        && report.pane_closed_removed
        && report.pane_state_preserved
        && focus_label_pass(report)
        && lifecycle_contract_pass(report)
        && host_policy_pass(report)
        && report.pane_management_sequence == "open>dedupe>focus>switch>close>reopen>restore"
        && report.pane_focus_switch_path == "settings>plugins>plugins"
        && report.pane_close_restore_path.starts_with("close:")
        && settings_contract_pass(&report.pane_settings_contract)
        && workspace_main_path_pass(report)
        && !report.native_child_windows
        && !report.detached_panels
}

fn lifecycle_contract_pass(report: &StudioSmokeReport) -> bool {
    [
        "workspace_lifecycle=open:",
        "focused:插件管理",
        "key:plugins",
        "closed_restore:",
        "policy:single egui host viewport, internal workspace panes",
        "native_child_windows:false",
        "detached_panels:false",
    ]
    .into_iter()
    .all(|term| report.pane_lifecycle_contract.contains(term))
}

fn focus_label_pass(report: &StudioSmokeReport) -> bool {
    [
        "host=single-borderless-egui-viewport",
        "sequence=open>focus>switch>close>reopen>restore",
        "counts=before-close:",
        "after-close:",
        "after-reopen:",
        "state_preserved=true",
        "forbidden=native-child-windows:false|detached-panels:false",
        "title=插件管理",
        "tabs=tabs=",
        "focused=插件管理",
        "cycle=previous|next",
        "close_hit=28x28",
        "keyboard_close=true",
        "工作区面板标签，Dashboard",
        "关闭工作区面板，插件管理",
        "closeguard=disk_roundtrip=true",
        "saved=true",
        "restored_count=3",
        "native_terms=false",
    ]
    .into_iter()
    .all(|term| report.pane_focus_label.contains(term))
}

fn host_policy_pass(report: &StudioSmokeReport) -> bool {
    [
        "single-borderless-egui-viewport",
        "pane_system=internal-egui-workspace-panes",
        "native_child_windows=false",
        "detached_panels=false",
        "docs=docs/22 + docs/24",
    ]
    .into_iter()
    .all(|term| report.pane_host_policy.contains(term))
}

fn workspace_main_path_pass(report: &StudioSmokeReport) -> bool {
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
    ]
    .into_iter()
    .all(|term| report.workspace_main_path_contract.contains(term))
}

fn settings_contract_pass(contract: &str) -> bool {
    [
        "surface=internal-workspace-pane",
        "navigation=left-category-rail",
        "appearance|hotkeys|ai-provider|index|plugins|privacy|about",
        "hotkey_source=default-or-user",
        "hotkey_reset=reset-to-default",
        "hotkey_control=token-binding-row",
        "theme_modes=system|dark|light",
        "theme_control=segmented-control",
        "zoom_levels=0.85|1.00|1.25|1.50",
        "zoom_control=segmented-control",
        "motion_control=token-toggle-row",
        "contrast_control=token-toggle-row",
        "transparency_control=token-toggle-row",
        "appearance_profile=theme-profile=requested|effective",
        "focus-ring|ui-scale",
        "ai_control=token-toggle-row",
        "storage_control=token-path-row",
    ]
    .into_iter()
    .all(|term| contract.contains(term))
}
