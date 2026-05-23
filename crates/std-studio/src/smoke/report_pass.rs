use super::StudioSmokeReport;
use std_egui::input;

impl StudioSmokeReport {
    pub(crate) fn pass(&self) -> bool {
        super::workspace_pass::workspace_contract_pass(self)
            && self.layout_contract_pass()
            && self.workflow_contract_pass()
            && self.analysis_contract_pass()
            && self.plugin_contract_pass()
            && self.keyboard_contract_pass()
            && self.operations_contract_pass()
            && self.open_intent_contract_pass()
            && self.motion_budget_contract_pass()
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
                .contains("native_child_windows=false")
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
            && self
                .canvas_content_route
                .contains("dashboard-pane-recovery")
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
                .contains("builder_interaction=single-workbench-flow")
            && self
                .builder_visual_contract
                .contains("shell=toolbar>status>steps+properties>trace>ai-assist")
            && self
                .builder_visual_contract
                .contains("flow=goal-input|plan|save|simulate|test|trace")
            && self.builder_visual_contract.contains("steps=list|row=48")
            && self.builder_visual_contract.contains("selected-row")
            && self.builder_visual_contract.contains("keyboard-reorder")
            && self
                .builder_visual_contract
                .contains("focus-default=steps-list")
            && self.builder_visual_contract.contains("keyboard-select")
            && self
                .builder_visual_contract
                .contains("a11y=row-index-name-type-selected")
            && self
                .builder_visual_contract
                .contains("selected=surface-3+accent-left")
            && self
                .builder_visual_contract
                .contains("inputs=step-name|parameters-json|index")
            && self
                .builder_visual_contract
                .contains("ai_assist=collapsed-input|suggestions|apply|insert|replace")
            && self
                .builder_visual_contract
                .contains("debug_panel=true,dry_run=true,execution=true")
            && self
                .builder_visual_contract
                .contains("batch-debug=simulate:open|run:open|planned-run:open|history:open")
            && self
                .builder_visual_contract
                .contains("role=bottom-panel-tabs")
            && self
                .builder_visual_contract
                .contains("history=execution-history-pane|timeline|payload")
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
            && self
                .operations_summary
                .contains("operations_plugin_command=mise run install-runtime-evidence")
            && self
                .operations_summary
                .contains("operations_plugin_output=js_runtime=PASS")
            && self.operations_summary.contains("ts_runtime=PASS")
            && self.operations_summary.contains("deno_core=PASS")
            && (self.operations_summary.contains("exit_code=PASS")
                || self.operations_summary.contains("permission_boundary=PASS"))
            && self
                .operations_summary
                .contains("operations_index_command=std index coverage")
            && (self
                .operations_summary
                .contains("operations_index_output=total=PASS")
                || self
                    .operations_summary
                    .contains("operations_index_output=cli_coverage=PASS"))
            && (self.operations_summary.contains("layers=PASS")
                || self.operations_summary.contains("overview=PASS"))
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
                .contains("gates=QA|Doctor|Release|Install|Plugin|Index|Runtime")
            && self.operations_summary.contains("manual_gates=Runtime")
            && self.operations_summary.contains("commands=7")
            && self.operations_summary.contains("results=7")
            && self.operations_summary.contains("outputs=7")
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

    fn motion_budget_contract_pass(&self) -> bool {
        self.motion_budget_summary
            .contains("studio_motion_budget PASS")
            && self.motion_budget_summary.contains("frame_budget_ms=8")
            && self
                .motion_budget_summary
                .contains("active_animation_limit=8")
    }
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
