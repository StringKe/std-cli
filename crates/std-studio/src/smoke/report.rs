use super::StudioSmokeReport;
use std_egui::input;

impl StudioSmokeReport {
    pub(crate) fn summary(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_smoke {status}\nworkspace_panes={}\nfocused_pane={}\npane_opened={}\npane_focus_switched={}\npane_closed={}\npane_focus_restored={}\npane_deduplicated={}\npane_content_keys={}\npane_focused_title={}\npane_restored_title={}\npane_settings_kind={}\npane_closed_removed={}\npane_state_preserved={}\npane_focus_label={}\npane_host_policy={}\npane_management_sequence={}\npane_focus_switch_path={}\npane_close_restore_path={}\npane_settings_contract={}\nworkspace_main_path_contract={}\nnative_child_windows={}\ndetached_panels={}\nhost_window_size={}\nmin_window_size={}\nhost_chrome_height={}\nstatus_bar_height={}\nsidebar_width={}\ncollapsed_sidebar_width={}\ninspector_width={}\ninspector_default_open={}\ninspector_context_route={}\nbottom_panel_height={}\nbottom_panel_default_open={}\nbottom_panel_tabs={}\ncanvas_surface={}\ncanvas_content_route={}\nstatus_bar_right={}\nworkflow_status={}\nbuilder_created={}\nbuilder_added_step={}\nbuilder_updated_step={}\nbuilder_moved_step={}\nbuilder_simulated={}\nbuilder_run_status={}\nbuilder_planned_run_status={}\nbuilder_trace_steps={}\nbuilder_trace_events={}\nbuilder_interaction_sequence={}\nbuilder_toolbar_contract={}\nbuilder_properties_contract={}\nbuilder_keyboard_move_path={}\nbuilder_selected_step={}\nbuilder_trace_status={}\nbuilder_side_effect_model={}\nbuilder_next_action={}\nbuilder_bottom_panel_contract={}\nbuilder_debug_panel_contract={}\nbatch_status={}\nanalysis={}\nanalysis_coverage_complete={}\nanalysis_coverage_layers={}\nanalysis_search_hits={}\nanalysis_answer_sources={}\nanalysis_visual_contract={}\nanalysis_inspect_components={}\nanalysis_inspect_relations={}\nanalysis_inspect_history={}\nanalysis_answer_has_evidence={}\nmemory_count={}\nplugin_js_status={}\nplugin_ts_status={}\nplugin_manifest_checks={}\nplugin_permissions={}\nplugin_action_count={}\nplugin_preview_kind={}\nplugin_js_runtime={}\nplugin_ts_runtime={}\nplugin_visual_contract={}\nhistory_count={}\nhistory_timeline_contract={}\nhistory_trace_steps={}\nhistory_payload_visible={}\n{}\n{}\n{}",
            self.workspace_panes,
            self.focused_pane,
            self.pane_opened,
            self.pane_focus_switched,
            self.pane_closed,
            self.pane_focus_restored,
            self.pane_deduplicated,
            self.pane_content_keys,
            self.pane_focused_title,
            self.pane_restored_title,
            self.pane_settings_kind,
            self.pane_closed_removed,
            self.pane_state_preserved,
            self.pane_focus_label,
            self.pane_host_policy,
            self.pane_management_sequence,
            self.pane_focus_switch_path,
            self.pane_close_restore_path,
            self.pane_settings_contract,
            self.workspace_main_path_contract,
            self.native_child_windows,
            self.detached_panels,
            self.host_window_size,
            self.min_window_size,
            self.host_chrome_height,
            self.status_bar_height,
            self.sidebar_width,
            self.collapsed_sidebar_width,
            self.inspector_width,
            self.inspector_default_open,
            self.inspector_context_route,
            self.bottom_panel_height,
            self.bottom_panel_default_open,
            self.bottom_panel_tabs,
            self.canvas_surface,
            self.canvas_content_route,
            self.status_bar_right,
            self.workflow_status,
            self.builder_created,
            self.builder_added_step,
            self.builder_updated_step,
            self.builder_moved_step,
            self.builder_simulated,
            self.builder_run_status,
            self.builder_planned_run_status,
            self.builder_trace_steps,
            self.builder_trace_events,
            self.builder_interaction_sequence,
            self.builder_toolbar_contract,
            self.builder_properties_contract,
            self.builder_keyboard_move_path,
            self.builder_selected_step,
            self.builder_trace_status,
            self.builder_side_effect_model,
            self.builder_next_action,
            self.builder_bottom_panel_contract,
            self.builder_debug_panel_contract,
            self.batch_status,
            self.analysis_name,
            self.analysis_coverage_complete,
            self.analysis_coverage_layers,
            self.analysis_search_hits,
            self.analysis_answer_sources,
            self.analysis_visual_contract,
            self.analysis_inspect_components,
            self.analysis_inspect_relations,
            self.analysis_inspect_history,
            self.analysis_answer_has_evidence,
            self.memory_count,
            self.plugin_js_status,
            self.plugin_ts_status,
            self.plugin_manifest_checks,
            self.plugin_permissions,
            self.plugin_action_count,
            self.plugin_preview_kind,
            self.plugin_js_runtime,
            self.plugin_ts_runtime,
            self.plugin_visual_contract,
            self.history_count,
            self.history_timeline_contract,
            self.history_trace_steps,
            self.history_payload_visible,
            self.keyboard_summary,
            self.operations_summary,
            self.open_intent_summary
        )
    }

    pub(crate) fn pass(&self) -> bool {
        self.workspace_contract_pass()
            && self.layout_contract_pass()
            && self.workflow_contract_pass()
            && self.analysis_contract_pass()
            && self.plugin_contract_pass()
            && self.keyboard_contract_pass()
            && self.operations_contract_pass()
            && self.open_intent_contract_pass()
    }

    fn workspace_contract_pass(&self) -> bool {
        self.workspace_panes >= 7
            && self.pane_opened
            && self.pane_focus_switched
            && self.pane_closed
            && self.pane_focus_restored
            && self.pane_deduplicated
            && self.pane_content_keys.contains("dashboard")
            && self.pane_content_keys.contains("settings")
            && self.pane_focused_title == "Plugin Manager"
            && self.pane_restored_title == "Plugin Manager"
            && self.pane_settings_kind == "Settings"
            && self.pane_closed_removed
            && self.pane_state_preserved
            && self
                .pane_focus_label
                .contains("host=single-borderless-egui-viewport")
            && self
                .pane_focus_label
                .contains("sequence=open>focus>switch>close>reopen>restore")
            && self.pane_focus_label.contains("state_preserved=true")
            && self
                .pane_focus_label
                .contains("forbidden=native-child-windows:false|detached-panels:false")
            && self.pane_focus_label.contains("title=Plugin Manager")
            && self
                .pane_host_policy
                .contains("single-borderless-egui-viewport")
            && self
                .pane_host_policy
                .contains("pane_system=internal-egui-workspace-panes")
            && self.pane_host_policy.contains("native_child_windows=false")
            && self.pane_host_policy.contains("detached_panels=false")
            && self.pane_host_policy.contains("docs=docs/22 + docs/24")
            && self.pane_management_sequence == "open>dedupe>focus>switch>close>reopen>restore"
            && self.pane_focus_switch_path == "settings>plugins>plugins"
            && self.pane_close_restore_path.starts_with("close:")
            && self
                .pane_settings_contract
                .contains("surface=internal-workspace-pane")
            && self
                .pane_settings_contract
                .contains("navigation=left-category-rail")
            && self
                .pane_settings_contract
                .contains("appearance|hotkeys|ai-provider|index|plugins|privacy|about")
            && self
                .pane_settings_contract
                .contains("hotkey_source=default-or-user")
            && self
                .pane_settings_contract
                .contains("hotkey_reset=reset-to-default")
            && self
                .pane_settings_contract
                .contains("hotkey_control=token-binding-row")
            && self
                .pane_settings_contract
                .contains("theme_modes=system|dark|light")
            && self
                .pane_settings_contract
                .contains("theme_control=segmented-control")
            && self
                .pane_settings_contract
                .contains("ai_control=token-toggle-row")
            && self
                .pane_settings_contract
                .contains("storage_control=token-path-row")
            && self
                .workspace_main_path_contract
                .contains("host=single-borderless-egui-viewport")
            && self
                .workspace_main_path_contract
                .contains("panes=internal-egui-workspace-panes")
            && self
                .workspace_main_path_contract
                .contains("extra_viewports=forbidden")
            && self
                .workspace_main_path_contract
                .contains("egui_window=forbidden")
            && self
                .workspace_main_path_contract
                .contains("settings_overlay=forbidden")
            && !self.native_child_windows
            && !self.detached_panels
    }

    fn layout_contract_pass(&self) -> bool {
        self.host_window_size == "1280x800"
            && self.min_window_size == "1080x640"
            && self.host_chrome_height == 52
            && self.status_bar_height == 24
            && self.sidebar_width == 240
            && self.collapsed_sidebar_width == 48
            && self.inspector_width == 320
            && !self.inspector_default_open
            && self
                .inspector_context_route
                .contains("focused-workspace-pane-context")
            && self.inspector_context_route.contains("global-fallback")
            && self.bottom_panel_height == 240
            && !self.bottom_panel_default_open
            && self
                .bottom_panel_tabs
                .contains("tabs=Batch Debug|Logs|Problems|Performance")
            && self.bottom_panel_tabs.contains("selected=Batch Debug")
            && self.bottom_panel_tabs.contains("role=bottom-panel-tabs")
            && self.canvas_surface.contains("surface=bg/surface-0")
            && self
                .canvas_content_route
                .contains("focused-workspace-pane-primary")
            && self.canvas_content_route.contains("main-pane-fallback")
            && self
                .canvas_surface
                .contains("standard_launcher_enter_ms=320")
            && self.canvas_surface.contains("reduced_launcher_enter_ms=0")
            && self.canvas_surface.contains("reduced_focus_ring_ms=0")
            && self.canvas_surface.contains("reduced_modal_enter_ms=0")
            && self.status_bar_right.contains("analysis-progress")
            && self.status_bar_right.contains("ai-provider")
            && self.status_bar_right.contains("version")
    }

    fn workflow_contract_pass(&self) -> bool {
        self.workflow_status == "Completed"
            && self.builder_created
            && self.builder_added_step
            && self.builder_updated_step
            && self.builder_moved_step
            && self.builder_simulated
            && self.builder_run_status == "Completed"
            && self.builder_planned_run_status == "Completed:terminal"
            && self.builder_trace_steps >= 2
            && self.builder_trace_events >= 3
            && self.builder_interaction_sequence
                == "create>add>edit>move>simulate>run-planned>run-saved>trace"
            && self
                .builder_toolbar_contract
                .contains("control=token-toolbar-buttons")
            && self.builder_toolbar_contract.contains("primary=plan|test")
            && self
                .builder_toolbar_contract
                .contains("shortcuts=save|simulate|test|history")
            && self
                .builder_properties_contract
                .contains("properties=token-field-rows")
            && self
                .builder_properties_contract
                .contains("inputs=step-name|parameters-json|index")
            && self
                .builder_properties_contract
                .contains("primary=add|update")
            && self.builder_keyboard_move_path
                == format!(
                    "{}:0>1;{}:1>0",
                    input::studio_workflow_step_move_down().label(),
                    input::studio_workflow_step_move_up().label()
                )
            && self.builder_selected_step == "Validate edited output"
            && self.builder_trace_status == "Completed"
            && self.builder_side_effect_model == "simulate=dry-run,run=audit-log"
            && self.builder_next_action == "complete"
            && self.builder_bottom_panel_contract == "batch-debug-open"
            && self
                .builder_debug_panel_contract
                .contains("debug_panel=true,dry_run=true,execution=true,statuses=")
            && self
                .builder_debug_panel_contract
                .contains("success>success>success>success")
            && self.batch_status == "NeedsExternalRunner"
            && self.memory_count >= 1
            && self.history_count >= 1
            && self.history_timeline_contract.contains("timeline=expanded")
            && self
                .history_timeline_contract
                .contains("columns=step,status,started,finished,payload")
            && self.history_trace_steps >= 1
            && self.history_payload_visible
    }

    fn analysis_contract_pass(&self) -> bool {
        self.analysis_coverage_complete >= 1
            && self
                .analysis_coverage_layers
                .contains("overview=true,components=true,relations=true,history=true")
            && self.analysis_search_hits >= 1
            && self.analysis_answer_sources >= 1
            && self
                .analysis_visual_contract
                .contains("toolbar=target-path|re-index|qa-input")
            && self
                .analysis_visual_contract
                .contains("tabs=Overview|Components|Symbols|Relations|Q&A")
            && self
                .analysis_visual_contract
                .contains("overview=target|index|activity")
            && self
                .analysis_visual_contract
                .contains("coverage=overview|components|relations|history")
            && self.analysis_visual_contract.contains("qa=sources:")
            && self.analysis_inspect_components >= 1
            && self.analysis_inspect_relations >= 1
            && self.analysis_inspect_history >= 1
            && self.analysis_answer_has_evidence
    }

    fn plugin_contract_pass(&self) -> bool {
        self.plugin_js_status == "Completed"
            && self.plugin_ts_status == "Completed"
            && self.plugin_manifest_checks >= 1
            && self.plugin_permissions.contains("Code")
            && self.plugin_action_count >= 2
            && self.plugin_preview_kind == "Command"
            && self.plugin_js_runtime == "deno_core"
            && self.plugin_ts_runtime == "deno_core"
            && self
                .plugin_visual_contract
                .contains("list=name|version|status|source|enable")
            && self
                .plugin_visual_contract
                .contains("inspector=description|permissions|commands|audit-log")
            && self.plugin_visual_contract.contains("commands=2")
            && self
                .plugin_visual_contract
                .contains("runtime=js:deno_core|ts:deno_core")
    }

    fn keyboard_contract_pass(&self) -> bool {
        self.keyboard_summary.contains("studio_keyboard_smoke=PASS")
            && self
                .keyboard_summary
                .contains("studio_workspace_focus_path=dashboard>plugins>settings>dashboard")
            && self
                .keyboard_summary
                .contains("studio_analysis_focus_path=target>tabs>content>query>coverage>target")
            && self
                .keyboard_summary
                .contains("studio_keyboard_contract=docs/20#studio-shortcuts")
    }

    fn operations_contract_pass(&self) -> bool {
        self.operations_summary.contains("operations_smoke=PASS")
            && self
                .operations_summary
                .contains("operations_qa_command=mise run quality")
            && self
                .operations_summary
                .contains("operations_doctor_command=std doctor")
            && self
                .operations_summary
                .contains("operations_release_command=std release verify")
            && self
                .operations_summary
                .contains("operations_install_command=std install verify")
            && self
                .operations_summary
                .contains("release-package:std release package")
            && self
                .operations_summary
                .contains("install-run:std install run")
    }

    fn open_intent_contract_pass(&self) -> bool {
        self.open_intent_summary.contains("studio_open_smoke PASS")
            && self
                .open_intent_summary
                .contains("route=internal-egui-workspace-pane-intent")
            && self
                .open_intent_summary
                .contains("native_child_windows=false")
            && self.open_intent_summary.contains("detached_panels=false")
            && self.open_intent_summary.contains("focus_restored=true")
    }
}
