use super::StudioSmokeReport;
use std_egui::input;

impl StudioSmokeReport {
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
            && self.pane_focus_label.contains("counts=before-close:")
            && self.pane_focus_label.contains("after-close:")
            && self.pane_focus_label.contains("after-reopen:")
            && self.pane_focus_label.contains("state_preserved=true")
            && self
                .pane_focus_label
                .contains("forbidden=native-child-windows:false|detached-panels:false")
            && self.pane_focus_label.contains("title=Plugin Manager")
            && self.pane_focus_label.contains("tabs=tabs=")
            && self.pane_focus_label.contains("focused=Plugin Manager")
            && self.pane_focus_label.contains("cycle=previous|next")
            && self.pane_focus_label.contains("close_hit=28x28")
            && self.pane_focus_label.contains("keyboard_close=true")
            && self
                .pane_focus_label
                .contains("Workspace pane tab, Dashboard")
            && self
                .pane_focus_label
                .contains("Close workspace pane, Plugin Manager")
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
            && settings_contract_pass(&self.pane_settings_contract)
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
                .contains("show_viewport=forbidden")
            && self
                .workspace_main_path_contract
                .contains("show_viewport_api=false")
            && self
                .workspace_main_path_contract
                .contains("viewport_id=forbidden")
            && self
                .workspace_main_path_contract
                .contains("egui_window=forbidden")
            && self
                .workspace_main_path_contract
                .contains("egui_window_api=false")
            && self
                .workspace_main_path_contract
                .contains("settings_overlay=forbidden")
            && self
                .workspace_main_path_contract
                .contains("settings_overlay=false")
            && !self.native_child_windows
            && !self.detached_panels
    }

    fn layout_contract_pass(&self) -> bool {
        self.host_window_size == "1280x800"
            && self.min_window_size == "1080x640"
            && self
                .host_viewport_contract
                .contains("single-borderless-egui")
            && self.host_viewport_contract.contains("decorations=false")
            && self
                .host_viewport_contract
                .contains("native-child-windows=false")
            && self.host_chrome_contract.contains("egui-owned")
            && self.host_chrome_contract.contains("native-controls=false")
            && self.host_chrome_input_contract.contains("background-only")
            && self
                .host_chrome_input_contract
                .contains("controls_reserved=true")
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
                .contains("tabs=批量调试|日志|问题|性能")
            && self.bottom_panel_tabs.contains("selected=批量调试")
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
                .builder_toolbar_contract
                .contains("button-label-shortcut-purpose")
            && self.builder_toolbar_contract.contains("textbox-goal-value")
            && self
                .builder_toolbar_contract
                .contains("focus-default=steps-list")
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
            && builder_bottom_panel_contract_pass(&self.builder_bottom_panel_contract)
            && self
                .builder_debug_panel_contract
                .contains("debug_panel=true,dry_run=true,execution=true,statuses=")
            && self
                .builder_debug_panel_contract
                .contains("success>success>success>success")
            && self
                .builder_visual_contract
                .contains("builder_visual=single-pane-workbench")
            && self
                .builder_visual_contract
                .contains("flow=goal-input|plan|save|simulate|test|trace")
            && self
                .builder_visual_contract
                .contains("steps=list|selected-row|keyboard-reorder")
            && self
                .builder_visual_contract
                .contains("properties=step-name|parameters-json|index|add|update|move|remove")
            && self
                .builder_visual_contract
                .contains("debug=dry-run|execution|trace")
            && self
                .builder_visual_contract
                .contains("bottom-panel=batch-debug|logs|problems|performance")
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
                .contains("overview:PASS,components:PASS,relations:PASS,history:PASS")
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
                .contains("coverage=overview:PASS|components:PASS|relations:PASS|history:PASS")
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
                .plugin_status_bar_contract
                .contains("manifest=1/1 PASS")
            && self
                .plugin_status_bar_contract
                .contains("actions=1 actions")
            && self.plugin_status_bar_contract.contains("preview=Command")
            && self.plugin_status_bar_contract.contains("runtime=")
            && self.plugin_status_bar_contract.contains("permissions=Code")
            && self
                .plugin_status_bar_contract
                .contains("boundary=fs=0 network=0")
            && self
                .plugin_permission_visual_contract
                .contains("manifest_checks=PASS")
            && self
                .plugin_permission_visual_contract
                .contains("permissions=Code")
            && self
                .plugin_permission_visual_contract
                .contains("boundary_panel=permissions|fs|network|actions")
            && self
                .plugin_permission_visual_contract
                .contains("runtime_panel=status|runtime|exit|duration|boundary")
            && self
                .plugin_inspector_contract
                .contains("description=visible;permissions=1;commands=1")
            && self
                .plugin_inspector_contract
                .contains("enable_state=enabled")
            && self
                .plugin_inspector_contract
                .contains("review_prompt=none")
            && self.plugin_inspector_contract.contains("audit_log=visible")
            && self
                .plugin_visual_contract
                .contains("list=name|version|status|source|enable")
            && self
                .plugin_visual_contract
                .contains("list_chip_tracks=metadata|match")
            && self
                .plugin_visual_contract
                .contains("status_bar=manifest=1/1 PASS")
            && self
                .plugin_visual_contract
                .contains("inspector=description|permissions|commands|audit-log")
            && self
                .plugin_visual_contract
                .contains("permission_boundary=manifest_checks=PASS")
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
            && self
                .keyboard_summary
                .contains("docs/23#studio-screen-reader")
            && self
                .keyboard_summary
                .contains("studio_sidebar_tree_label=Workflow Builder, group 2, level 1, 3 of 8")
            && self
                .keyboard_summary
                .contains("studio_dnd_pickup_announcement=Picked up Collect context")
            && self
                .keyboard_summary
                .contains("studio_batch_progress_announcements=0%,5%,10%,15%")
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
                .contains("operations_runtime_command=mise run ui-background-acceptance")
            && self
                .operations_summary
                .contains("operations_completion_summary=UI Docs 18-24:MANUAL|Launcher:MANUAL")
            && self.operations_summary.contains("Studio:MANUAL")
            && self.operations_summary.contains("Quality:PASS")
            && self
                .operations_summary
                .contains("operations_completion_manual=UI Docs 18-24")
            && self
                .operations_summary
                .contains("operations_completion_manual_gates=")
            && self
                .operations_summary
                .contains("launcher-background-harness-enter")
            && self
                .operations_summary
                .contains("studio-keyboard-a11y-focus")
            && self
                .operations_summary
                .contains("release-package:std release package")
            && self
                .operations_summary
                .contains("install-run:std install run")
            && self.operations_summary.contains("operations_visual_contract=")
            && self.operations_summary.contains(
                "gate=title|status-icon|status-text|command|step-name|step-command|step-result|runbook|evidence|result|artifact|output|record-evidence",
            )
            && self
                .operations_summary
                .contains("completion=area|status|evidence|manual_gates")
            && self
                .operations_summary
                .contains("ui_areas=manual_until_runtime_proof")
            && self
                .operations_summary
                .contains("gates=QA|Doctor|Release|Install")
            && self.operations_summary.contains("Runtime")
            && self.operations_summary.contains("manual_gates=Runtime")
            && self.operations_summary.contains("commands=5")
            && self.operations_summary.contains("results=5")
            && self.operations_summary.contains("outputs=5")
            && self.operations_summary.contains("operations_a11y_contract=")
            && self
                .operations_summary
                .contains("a11y=row-label-includes-label-value-detail")
    }

    fn open_intent_contract_pass(&self) -> bool {
        self.open_intent_summary.contains("studio_open_smoke PASS")
            && self
                .open_intent_summary
                .contains("route=internal-egui-workspace-pane-intent")
            && self
                .open_intent_summary
                .contains("host_policy=single-borderless-egui-viewport")
            && self
                .open_intent_summary
                .contains("pane_system=internal-egui-workspace-panes")
            && self.open_intent_summary.contains("docs=docs/22 + docs/24")
            && self
                .open_intent_summary
                .contains("native_child_windows=false")
            && self.open_intent_summary.contains("detached_panels=false")
            && self.open_intent_summary.contains("focus_restored=true")
    }
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

fn builder_bottom_panel_contract_pass(contract: &str) -> bool {
    [
        "batch-debug=simulate:open|run:open|planned-run:open|history:open",
        "helper=open",
        "tabs=批量调试|日志|问题|性能",
        "selected=批量调试",
        "role=bottom-panel-tabs",
    ]
    .into_iter()
    .all(|term| contract.contains(term))
}
