use super::StudioSmokeReport;
use std_egui::input;

impl StudioSmokeReport {
    pub(crate) fn contract_diagnostics(&self) -> String {
        [
            ("workspace", self.workspace_diagnostic_pass()),
            ("layout", self.layout_diagnostic_pass()),
            ("workflow", self.workflow_diagnostic_pass()),
            ("analysis", self.analysis_diagnostic_pass()),
            ("plugin", self.plugin_diagnostic_pass()),
            ("keyboard", self.keyboard_diagnostic_pass()),
            ("operations", self.operations_diagnostic_pass()),
            ("open-intent", self.open_intent_diagnostic_pass()),
        ]
        .into_iter()
        .map(|(name, pass)| format!("{name}={}", if pass { "PASS" } else { "FAIL" }))
        .collect::<Vec<_>>()
        .join(",")
    }

    fn workspace_diagnostic_pass(&self) -> bool {
        self.workspace_panes >= 7
            && self.pane_opened
            && self.pane_focus_switched
            && self.pane_closed
            && self.pane_focus_restored
            && self.pane_deduplicated
            && !self.native_child_windows
            && !self.detached_panels
    }

    fn layout_diagnostic_pass(&self) -> bool {
        self.host_window_size == "1280x800"
            && self.min_window_size == "1080x640"
            && self.host_viewport_contract.contains("decorations=false")
            && self.host_chrome_contract.contains("native-controls=false")
            && self.host_chrome_height == 52
            && self.status_bar_height == 24
            && self.bottom_panel_tabs.contains("role=bottom-panel-tabs")
    }

    fn workflow_diagnostic_pass(&self) -> bool {
        self.workflow_contract_diagnostics()
            .split(',')
            .all(|item| item.ends_with("=PASS"))
    }

    fn analysis_diagnostic_pass(&self) -> bool {
        self.analysis_coverage_complete >= 1
            && self.analysis_search_hits >= 1
            && self.analysis_answer_sources >= 1
            && self.analysis_answer_has_evidence
    }

    fn plugin_diagnostic_pass(&self) -> bool {
        self.plugin_js_status == "Completed"
            && self.plugin_ts_status == "Completed"
            && self.plugin_action_count >= 2
            && self
                .plugin_visual_contract
                .contains("runtime=js:deno_core|ts:deno_core")
    }

    fn keyboard_diagnostic_pass(&self) -> bool {
        self.keyboard_summary.contains("studio_keyboard_smoke=PASS")
    }

    fn operations_diagnostic_pass(&self) -> bool {
        self.operations_summary.contains("operations_smoke=PASS")
    }

    fn open_intent_diagnostic_pass(&self) -> bool {
        self.open_intent_summary.contains("studio_open_smoke PASS")
    }

    pub(crate) fn workflow_contract_diagnostics(&self) -> String {
        [
            ("workflow_status", self.workflow_status == "Completed"),
            ("builder_created", self.builder_created),
            ("builder_added_step", self.builder_added_step),
            ("builder_updated_step", self.builder_updated_step),
            ("builder_moved_step", self.builder_moved_step),
            ("builder_simulated", self.builder_simulated),
            ("builder_run_status", self.builder_run_status == "Completed"),
            ("builder_planned_run_status", self.planned_run_pass()),
            ("builder_trace_steps", self.builder_trace_steps >= 2),
            ("builder_trace_events", self.builder_trace_events >= 3),
            ("builder_toolbar", self.toolbar_contract_pass()),
            ("builder_properties", self.properties_contract_pass()),
            ("builder_side_effect_model", self.side_effect_model_pass()),
            (
                "builder_next_action",
                self.builder_next_action == "complete",
            ),
            (
                "builder_bottom_panel",
                self.builder_bottom_panel_contract
                    .contains("batch-debug=simulate:open|run:open|planned-run:open|history:open")
                    && self.builder_bottom_panel_contract.contains("helper=open")
                    && self
                        .builder_bottom_panel_contract
                        .contains("role=bottom-panel-tabs"),
            ),
            ("builder_debug_panel_prefix", self.debug_panel_prefix_pass()),
            (
                "builder_debug_panel_statuses",
                self.debug_panel_status_pass(),
            ),
            ("builder_visual_workbench", self.visual_workbench_pass()),
            ("builder_visual_flow", self.visual_flow_pass()),
            ("builder_visual_steps", self.visual_steps_pass()),
            ("builder_visual_properties", self.visual_properties_pass()),
            ("builder_visual_debug", self.visual_debug_pass()),
            ("builder_visual_bottom", self.visual_bottom_pass()),
            ("builder_keyboard", self.builder_keyboard_pass()),
            ("builder_selected_step", self.selected_step_pass()),
            (
                "builder_trace_status",
                self.builder_trace_status == "Completed",
            ),
            ("batch_status", self.batch_status == "NeedsExternalRunner"),
            ("memory_count", self.memory_count >= 1),
            ("history_count", self.history_count >= 1),
            ("history_trace_steps", self.history_trace_steps >= 1),
            ("history_payload_visible", self.history_payload_visible),
        ]
        .into_iter()
        .map(|(name, pass)| format!("{name}={}", if pass { "PASS" } else { "FAIL" }))
        .collect::<Vec<_>>()
        .join(",")
    }

    fn planned_run_pass(&self) -> bool {
        self.builder_planned_run_status == "Completed:terminal"
    }

    fn toolbar_contract_pass(&self) -> bool {
        self.builder_toolbar_contract
            .contains("focus-default=steps-list")
    }

    fn properties_contract_pass(&self) -> bool {
        self.builder_properties_contract
            .contains("primary=add|update")
    }

    fn side_effect_model_pass(&self) -> bool {
        self.builder_side_effect_model == "simulate=dry-run,run=audit-log"
    }

    fn debug_panel_prefix_pass(&self) -> bool {
        self.builder_debug_panel_contract
            .contains("debug_panel=true,dry_run=true,execution=true,statuses=")
    }

    fn debug_panel_status_pass(&self) -> bool {
        self.builder_debug_panel_contract
            .contains("success>success>success>success")
    }

    fn visual_workbench_pass(&self) -> bool {
        self.builder_visual_contract
            .contains("builder_interaction=single-workbench-flow")
            && self
                .builder_visual_contract
                .contains("shell=toolbar>status>steps+properties>trace>ai-assist")
    }

    fn visual_flow_pass(&self) -> bool {
        self.builder_visual_contract
            .contains("flow=goal-input|plan|save|simulate|run|trace")
    }

    fn visual_steps_pass(&self) -> bool {
        self.builder_visual_contract.contains("steps=list|row=48")
            && self
                .builder_visual_contract
                .contains("focus-default=steps-list")
            && self.builder_visual_contract.contains("keyboard-select")
            && self.builder_visual_contract.contains("keyboard-reorder")
            && self
                .builder_visual_contract
                .contains("a11y=row-index-name-type-selected")
            && self.builder_visual_contract.contains("selected-row")
            && self
                .builder_visual_contract
                .contains("selected=surface-3+accent-left")
    }

    fn visual_properties_pass(&self) -> bool {
        self.builder_visual_contract
            .contains("inputs=step-name|parameters-json|index")
    }

    fn visual_debug_pass(&self) -> bool {
        self.builder_visual_contract
            .contains("debug_panel=true,dry_run=true,execution=true")
            && self
                .builder_visual_contract
                .contains("ai_assist=collapsed-input|suggestions|apply|insert|replace")
    }

    fn visual_bottom_pass(&self) -> bool {
        self.builder_visual_contract
            .contains("batch-debug=simulate:open|run:open|planned-run:open|history:open")
            && self
                .builder_visual_contract
                .contains("role=bottom-panel-tabs")
            && self
                .builder_visual_contract
                .contains("history=execution-history-pane|timeline|payload")
    }

    fn builder_keyboard_pass(&self) -> bool {
        self.builder_keyboard_move_path
            == format!(
                "{}:0>1;{}:1>0",
                input::studio_workflow_step_move_down().label(),
                input::studio_workflow_step_move_up().label()
            )
    }

    fn selected_step_pass(&self) -> bool {
        self.builder_selected_step == "Validate edited output"
    }
}
