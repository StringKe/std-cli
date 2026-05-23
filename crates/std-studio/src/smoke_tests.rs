use crate::smoke::run_studio_smoke;
use std_egui::input;

#[test]
fn studio_smoke_reports_internal_workspace_pane_management() {
    let report = run_studio_smoke().unwrap();
    let summary = report.summary();

    assert!(
        report.pass(),
        "{}\n{}\n{}",
        report.contract_diagnostics(),
        report.workflow_contract_diagnostics(),
        summary
    );
    assert_workspace_policy_summary(&summary);
    assert_shell_layout_summary(&summary);
    assert_workflow_builder_summary(&summary);
    assert_analysis_workbench_summary(&summary);
    assert_keyboard_summary(&summary);
    assert_operations_summary(&summary);
}

fn assert_workspace_policy_summary(summary: &str) {
    assert_workspace_pane_state_summary(summary);
    assert_workspace_tab_summary(summary);
    assert_workspace_forbidden_policy_summary(summary);
    assert_workspace_pane_lifecycle_summary(summary);
    assert_workspace_ui_completion_boundary(summary);
    assert_studio_open_intent_summary(summary);
}

fn assert_workspace_pane_state_summary(summary: &str) {
    assert!(summary.contains("pane_opened=true"));
    assert!(summary.contains("pane_focus_switched=true"));
    assert!(summary.contains("pane_closed=true"));
    assert!(summary.contains("pane_focus_restored=true"));
    assert!(summary.contains("pane_deduplicated=true"));
    assert!(summary.contains("pane_content_keys=analysis,apps,dashboard"));
    assert!(summary.contains("history,memory,operations,plugins,settings,workflows"));
    assert!(summary.contains("pane_focused_title=插件管理"));
    assert!(summary.contains("pane_restored_title=插件管理"));
    assert!(summary.contains("pane_closed_removed=true"));
    assert!(summary.contains("pane_state_preserved=true"));
    assert!(summary.contains("reopened_memory="));
    assert!(summary.contains("reopened_restored=true"));
}

fn assert_workspace_tab_summary(summary: &str) {
    assert!(summary.contains("pane_focus_label=strategy=internal-egui-workspace-panes"));
    assert!(summary.contains("host=single-borderless-egui-viewport"));
    assert!(summary.contains("sequence=open>focus>switch>close>reopen>restore"));
    assert!(summary.contains("counts=before-close:"));
    assert!(summary.contains("after-close:"));
    assert!(summary.contains("after-reopen:"));
    assert!(summary.contains("state_preserved=true"));
    assert!(summary.contains("focused="));
    assert!(summary.contains("title=插件管理"));
    assert!(summary.contains("tabs=tabs="));
    assert!(summary.contains("focused=插件管理"));
    assert!(summary.contains("cycle=previous|next"));
    assert!(summary.contains("close_hit=28x28"));
    assert!(summary.contains("keyboard_close=true"));
    assert!(summary.contains("工作区面板标签，Dashboard"));
    assert!(summary.contains("关闭工作区面板，插件管理"));
    assert!(summary.contains("closeguard=disk_roundtrip=true"));
    assert!(summary.contains("saved=true"));
    assert!(summary.contains("restored_count=3"));
    assert!(summary.contains("native_terms=false"));
    assert!(summary.contains("strategy=internal-egui-workspace-panes"));
}

fn assert_workspace_forbidden_policy_summary(summary: &str) {
    assert!(summary.contains("forbidden=native-child-windows:false|detached-panels:false"));
    assert!(summary.contains("native_child_windows=false"));
    assert!(summary.contains("detached_panels=false"));
    assert!(summary.contains("extra_viewports=false"));
    assert!(summary.contains("show_viewport_api=false"));
    assert!(summary.contains("egui_window_api=false"));
    assert!(summary.contains("settings_overlay=false"));
}

fn assert_workspace_pane_lifecycle_summary(summary: &str) {
    assert!(summary.contains("pane_host_policy=host=single-borderless-egui-viewport"));
    assert!(summary.contains("pane_system=internal-egui-workspace-panes"));
    assert!(summary.contains("docs=docs/22 + docs/24"));
    assert!(
        summary.contains("pane_management_sequence=open>dedupe>focus>switch>close>reopen>restore")
    );
    assert!(summary.contains("pane_focus_switch_path=settings>plugins>plugins"));
    assert!(summary.contains("pane_close_restore_path=close:"));
    assert!(summary.contains("hotkey_source=default-or-user"));
    assert!(summary.contains("hotkey_reset=reset-to-default"));
    assert!(summary.contains("hotkey_control=token-binding-row"));
    assert_workspace_settings_policy_summary(summary);
    assert_workspace_main_path_summary(summary);
}

fn assert_workspace_settings_policy_summary(summary: &str) {
    assert!(summary.contains("theme_modes=system|dark|light"));
    assert!(summary.contains("theme_control=segmented-control"));
    assert!(summary.contains("appearance_profile=theme-profile=requested|effective"));
    assert!(summary.contains("focus-ring|ui-scale"));
    assert!(summary.contains("ai_control=token-toggle-row"));
    assert!(summary.contains("storage_control=token-path-row"));
}

fn assert_workspace_main_path_summary(summary: &str) {
    assert!(summary.contains("workspace_main_path_contract=host=single-borderless-egui-viewport"));
    assert!(summary.contains("panes=internal-egui-workspace-panes"));
    assert!(summary.contains("extra_viewports=forbidden"));
    assert!(summary.contains("show_viewport=forbidden"));
    assert!(summary.contains("show_viewport_api=false"));
    assert!(summary.contains("viewport_id=forbidden"));
    assert!(summary.contains("egui_window=forbidden"));
    assert!(summary.contains("egui_window_api=false"));
    assert!(summary.contains("settings_overlay=forbidden"));
    assert!(summary.contains("settings_overlay=false"));
}

fn assert_workspace_ui_completion_boundary(summary: &str) {
    assert!(summary.contains("ui_completion_boundary=headless-smoke-is-not-ui-completion"));
    assert!(summary.contains("manual_ui_evidence_gates=light-dark-screenshots"));
    assert!(summary.contains("workspace-pane-open-focus-close-restore"));
    assert!(summary.contains("keyboard-a11y-focus"));
    assert!(summary.contains("operations-runtime-evidence"));
}

fn assert_studio_open_intent_summary(summary: &str) {
    assert!(summary.contains("studio_open_smoke PASS"));
    assert!(summary.contains("route=internal-egui-workspace-pane-intent"));
    assert!(summary.contains("host_policy=single-borderless-egui-viewport"));
    assert!(summary.contains("pane_system=internal-egui-workspace-panes"));
    assert!(summary.contains("docs=docs/22 + docs/24"));
    assert!(summary.contains("targets=7"));
    assert!(summary.contains("internal_panes=7"));
    assert!(summary.contains("focus_restored=true"));
}

fn assert_shell_layout_summary(summary: &str) {
    assert!(summary.contains("host_window_size=1280x800"));
    assert!(summary.contains("min_window_size=1080x640"));
    assert!(summary.contains(
        "host_viewport_contract=host_viewport=single-borderless-egui-viewport,panes=internal-egui-workspace-panes,size=1280x800,min=1080x640,decorations=false,resizable=true,native_child_windows=false,detached_panels=false,extra_viewports=false"
    ));
    assert!(summary
        .contains("host_chrome_contract=host_chrome=egui-owned,borderless,native-controls=false"));
    assert!(summary.contains(
        "host_chrome_input_contract=drag_region=background-only,left-identity-area;controls_reserved=true"
    ));
    assert!(summary.contains("host_chrome_height=52"));
    assert!(summary.contains("status_bar_height=24"));
    assert!(summary.contains("sidebar_width=240"));
    assert!(summary.contains("collapsed_sidebar_width=48"));
    assert!(summary.contains("inspector_width=320"));
    assert!(summary.contains("inspector_default_open=false"));
    assert!(
        summary.contains("inspector_context_route=focused-workspace-pane-context,global-fallback")
    );
    assert!(summary.contains("bottom_panel_height=240"));
    assert!(summary.contains("bottom_panel_default_open=false"));
    assert!(summary.contains("canvas_surface=surface=bg/surface-0"));
    assert!(summary
        .contains("canvas_content_route=focused-workspace-pane-primary,dashboard-pane-recovery"));
    assert!(summary.contains("status_bar_right=analysis-progress,ai-provider,version"));
    assert!(summary.contains("standard_launcher_enter_ms=320"));
    assert!(summary.contains("reduced_launcher_enter_ms=0"));
    assert!(summary.contains("reduced_focus_ring_ms=0"));
    assert!(summary.contains("reduced_modal_enter_ms=0"));
    assert!(summary.contains("reduce_motion_env=STD_REDUCE_MOTION"));
}

fn assert_workflow_builder_summary(summary: &str) {
    assert_workflow_builder_lifecycle_summary(summary);
    assert_workflow_builder_controls_summary(summary);
    assert_workflow_builder_trace_summary(summary);
    assert_plugin_manager_summary(summary);
}

fn assert_workflow_builder_lifecycle_summary(summary: &str) {
    assert!(summary.contains("builder_created=true"));
    assert!(summary.contains("builder_added_step=true"));
    assert!(summary.contains("builder_updated_step=true"));
    assert!(summary.contains("builder_moved_step=true"));
    assert!(summary.contains("builder_simulated=true"));
    assert!(summary.contains("builder_run_status=Completed"));
    assert!(summary.contains("builder_planned_run_status=Completed:terminal"));
    assert!(summary.contains("builder_trace_steps=2"));
    assert!(summary.contains(
        "builder_interaction_sequence=create>add>edit>move>simulate>run-planned>run-saved>trace"
    ));
    assert!(summary.contains("builder_selected_step=Validate edited output"));
}

fn assert_workflow_builder_controls_summary(summary: &str) {
    assert!(summary.contains("builder_toolbar_contract=toolbar=goal-input>plan>save>simulate>test>cancel-when-running>history-action>ai>zoom"));
    assert!(summary.contains("control=token-toolbar-buttons"));
    assert!(summary.contains("primary=plan|test"));
    assert!(summary.contains("shortcuts=save|simulate|test|history"));
    assert!(summary.contains("a11y=textbox-goal-value,button-label-shortcut-purpose"));
    assert!(summary.contains("focus-default=steps-list"));
    assert!(summary.contains("builder_properties_contract=properties=token-field-rows"));
    assert!(summary.contains("inputs=step-name|parameters-json|index"));
    assert!(summary.contains("primary=add|update"));
    assert!(summary.contains(&format!(
        "builder_keyboard_move_path={}:0>1;{}:1>0",
        input::studio_workflow_step_move_down().label(),
        input::studio_workflow_step_move_up().label()
    )));
}

fn assert_workflow_builder_trace_summary(summary: &str) {
    assert!(summary.contains("builder_trace_status=Completed"));
    assert!(summary.contains("builder_side_effect_model=simulate=dry-run,run=audit-log"));
    assert!(summary.contains("builder_next_action=complete"));
    assert_workflow_builder_bottom_panel_summary(summary);
    assert_workflow_builder_history_summary(summary);
    assert_workflow_builder_debug_summary(summary);
    assert_workflow_builder_visual_summary(summary);
}

fn assert_workflow_builder_bottom_panel_summary(summary: &str) {
    assert!(summary.contains(
        "builder_bottom_panel_contract=batch-debug=simulate:open|run:open|planned-run:open|history:open"
    ));
    assert!(summary.contains("helper=open"));
    assert!(summary.contains("tabs=批量调试|日志|问题|性能"));
    assert!(summary.contains("selected=批量调试"));
    assert!(summary.contains("role=bottom-panel-tabs"));
}

fn assert_workflow_builder_history_summary(summary: &str) {
    assert!(summary.contains("history_timeline_contract=timeline=expanded"));
    assert!(summary.contains("columns=step,status,started,finished,payload"));
    assert!(summary.contains("history_trace_steps=1"));
    assert!(summary.contains("history_payload_visible=true"));
    assert!(summary.contains("history=execution-history-pane|timeline|payload"));
}

fn assert_workflow_builder_debug_summary(summary: &str) {
    assert!(summary.contains(
        "builder_debug_panel_contract=debug_panel=true,dry_run=true,execution=true,statuses="
    ));
    assert!(summary.contains("success>success>success>success"));
    assert!(summary.contains("debug_panel=true,dry_run=true,execution=true"));
}

fn assert_workflow_builder_visual_summary(summary: &str) {
    assert!(summary.contains("builder_visual_contract=builder_interaction=single-workbench-flow"));
    assert!(summary.contains("shell=toolbar>status>steps+properties>trace>ai-assist"));
    assert!(summary.contains("flow=goal-input|plan|save|simulate|test|trace"));
    assert_workflow_builder_step_visual_summary(summary);
    assert!(summary.contains("inputs=step-name|parameters-json|index"));
    assert!(summary.contains("ai_assist=collapsed-input|suggestions|apply|insert|replace"));
    assert!(summary.contains("batch-debug=simulate:open|run:open|planned-run:open|history:open"));
    assert!(summary.contains("role=bottom-panel-tabs"));
}

fn assert_workflow_builder_step_visual_summary(summary: &str) {
    assert!(summary.contains("steps=list|row=48"));
    assert!(summary.contains("focus-default=steps-list"));
    assert!(summary.contains("keyboard-select"));
    assert!(summary.contains("keyboard-reorder"));
    assert!(summary.contains("a11y=row-index-name-type-selected"));
    assert!(summary.contains("selected-row"));
    assert!(summary.contains("grabber-6px"));
    assert!(summary.contains("selected=surface-3+accent-left"));
    assert!(summary.contains("selected-accent-rail-4px"));
    assert!(summary.contains("type-chip"));
}

fn assert_plugin_manager_summary(summary: &str) {
    assert_plugin_manager_runtime_summary(summary);
    assert_plugin_manager_visual_summary(summary);
}

fn assert_plugin_manager_runtime_summary(summary: &str) {
    assert!(summary.contains("plugin_js_status=Completed"));
    assert!(summary.contains("plugin_ts_status=Completed"));
    assert!(summary.contains("plugin_manifest_checks=1"));
    assert!(summary.contains("plugin_permissions=Code"));
    assert!(summary.contains("plugin_action_count="));
    assert!(summary.contains("plugin_preview_kind=Command"));
    assert!(summary.contains("plugin_js_runtime=deno_core"));
    assert!(summary.contains("plugin_ts_runtime=deno_core"));
    assert!(summary.contains("plugin_status_bar_contract=manifest=1/1 PASS"));
    assert!(summary.contains("actions=1 actions"));
    assert!(summary.contains("runtime="));
    assert!(summary.contains("deno_core"));
    assert!(summary.contains("permissions=Code"));
    assert!(summary.contains("boundary=fs=0 network=0"));
}

fn assert_plugin_manager_visual_summary(summary: &str) {
    assert!(summary.contains("plugin_permission_visual_contract=manifest_checks=PASS"));
    assert!(summary.contains("boundary_panel=permissions|fs|network|actions"));
    assert!(summary.contains("runtime_panel=status|runtime|exit|duration|boundary"));
    assert!(summary.contains("plugin_inspector_contract=description=visible"));
    assert!(summary.contains("commands=1;enable_state=enabled"));
    assert!(summary.contains("review_prompt=none;audit_log=visible"));
    assert!(summary.contains("plugin_visual_contract=list=name|version|status|source|enable"));
    assert!(summary.contains("status_bar=manifest=1/1 PASS"));
    assert!(summary.contains("inspector=description|permissions|commands|audit-log"));
    assert!(summary.contains("permission_boundary=manifest_checks=PASS"));
    assert!(summary.contains("commands=2"));
    assert!(summary.contains("runtime=js:deno_core|ts:deno_core"));
}

fn assert_analysis_workbench_summary(summary: &str) {
    assert!(summary.contains(
        "analysis_coverage_layers=overview:PASS,components:PASS,relations:PASS,history:PASS"
    ));
    assert!(summary.contains(
        "analysis_visual_contract=toolbar=target-path|re-index|qa-input;tabs=Overview|Components|Symbols|Relations|Q&A;overview=target|index|activity;coverage=overview:PASS|components:PASS|relations:PASS|history:PASS"
    ));
    assert!(summary.contains("analysis_search_hits=2"));
    assert!(summary.contains("analysis_answer_sources=2"));
    assert!(summary.contains("analysis_visual_contract=toolbar=target-path|re-index|qa-input"));
    assert!(summary.contains("tabs=Overview|Components|Symbols|Relations|Q&A"));
    assert!(summary.contains("overview=target|index|activity"));
    assert!(summary.contains("coverage=overview:PASS|components:PASS|relations:PASS|history:PASS"));
    assert!(summary.contains("symbols=search-hits:2"));
    assert!(summary.contains("qa=sources:2"));
    assert!(summary.contains("analysis_inspect_components=1"));
    assert!(summary.contains("analysis_inspect_relations=3"));
    assert!(summary.contains("analysis_inspect_history=1"));
    assert!(summary.contains("analysis_answer_has_evidence=true"));
}

fn assert_operations_summary(summary: &str) {
    assert!(summary.contains("operations_smoke=PASS"));
    assert_operations_gate_summary(summary);
    assert_operations_completion_summary(summary);
    assert_operations_contract_summary(summary);
}

fn assert_operations_gate_summary(summary: &str) {
    assert!(summary.contains("operations_qa_command=mise run quality"));
    assert!(summary.contains("operations_qa_output=rustfmt=PASS"));
    assert!(summary.contains("operations_doctor_command=std doctor"));
    assert!(summary.contains("operations_doctor_result=doctor source gates"));
    assert!(summary.contains("operations_release_command=std release verify"));
    assert!(summary.contains("operations_release_output=manifest="));
    assert!(summary.contains("operations_install_command=std install verify"));
    assert!(summary.contains("operations_install_output=std="));
    assert!(summary.contains("operations_plugin_command=mise run install-runtime-evidence"));
    assert!(summary.contains("operations_plugin_output=js_runtime=PASS"));
    assert!(summary.contains("ts_runtime=PASS"));
    assert!(summary.contains("deno_core=PASS"));
    assert!(summary.contains("exit_code=PASS") || summary.contains("permission_boundary=PASS"));
    assert!(summary.contains("operations_index_command=std index coverage"));
    assert!(
        summary.contains("operations_index_output=total=PASS")
            || summary.contains("operations_index_output=cli_coverage=PASS")
    );
    assert!(summary.contains("layers=PASS") || summary.contains("overview=PASS"));
    assert!(summary.contains("operations_runtime_command=mise run ui-background-acceptance"));
}

fn assert_operations_completion_summary(summary: &str) {
    assert!(summary.contains("operations_completion_summary=UI Docs 18-24:MANUAL|Launcher:MANUAL"));
    assert!(summary.contains("Studio:MANUAL"));
    assert!(summary.contains("Quality:PASS"));
    assert!(summary.contains("operations_completion_manual=UI Docs 18-24"));
    assert!(summary.contains("operations_completion_manual_gates="));
    assert!(summary.contains("launcher-background-harness-enter"));
    assert!(summary.contains("studio-keyboard-a11y-focus"));
    assert!(summary.contains("ui-capture-manifest=artifacts/ui/manual-acceptance/manifest.txt"));
    assert!(
        summary.contains("ui-capture-command=STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix")
    );
}

fn assert_operations_contract_summary(summary: &str) {
    assert!(summary.contains("operations_visual_contract="));
    assert!(summary.contains(
        "gate=title|status-icon|status-text|command|step-name|step-command|step-result|runbook|evidence|result|artifact|output|record-evidence"
    ));
    assert!(summary.contains("gates=QA|Doctor|Release|Install|Plugin|Index|Runtime"));
    assert!(summary.contains("manual_gates=Runtime"));
    assert!(summary.contains("commands=7"));
    assert!(summary.contains("results=7"));
    assert!(summary.contains("outputs=7"));
    assert!(summary.contains("completion=area|status|evidence|manual_gates"));
    assert!(summary.contains("ui_areas=manual_until_runtime_proof"));
    assert!(summary.contains("operations_a11y_contract="));
    assert!(summary.contains("a11y=row-label-includes-label-value-detail"));
}

fn assert_keyboard_summary(summary: &str) {
    assert!(summary.contains("studio_keyboard_smoke=PASS"));
    assert!(summary.contains(&format!(
        "studio_sidebar_toggle_path={}:open>closed>open",
        input::studio_sidebar_toggle().label()
    )));
    assert!(summary.contains(&format!(
        "studio_inspector_toggle_path={}:closed>open>closed",
        input::studio_inspector_toggle().label()
    )));
    assert!(summary.contains(&format!(
        "studio_bottom_panel_toggle_path={}:closed>open>closed",
        input::studio_bottom_panel_toggle().label()
    )));
    assert!(summary.contains(&format!(
        "studio_command_palette_path={}|{}:closed>command",
        input::studio_command_palette().label(),
        input::studio_command_palette_slash().label()
    )));
    assert!(summary.contains(&format!(
        "studio_quick_open_path={}:command>quick-open",
        input::studio_quick_open().label()
    )));
    assert!(summary.contains(&format!(
        "studio_new_workflow_path={}:closed>builder",
        input::studio_new_workflow().label()
    )));
    assert!(summary.contains(&format!(
        "studio_zoom_path={}|{}|{}:1.00>1.05>1.00>1.00",
        input::studio_zoom_in().label(),
        input::studio_zoom_out().label(),
        input::studio_zoom_reset().label()
    )));
    assert!(summary.contains("studio_workspace_focus_path=dashboard>plugins>settings>dashboard"));
    assert!(
        summary.contains("studio_analysis_focus_path=target>tabs>content>query>coverage>target")
    );
    assert!(summary.contains("studio_analysis_qa_focus=?:coverage>query"));
    assert!(summary.contains("studio_keyboard_contract=docs/20#studio-shortcuts"));
    assert!(summary.contains("docs/23#studio-screen-reader"));
    assert!(
        summary.contains("studio_sidebar_tree_label=Workflow Builder, group 2, level 1, 3 of 8")
    );
    assert!(summary.contains("studio_dnd_pickup_announcement=Picked up Collect context"));
    assert!(summary.contains("studio_dnd_drop_announcement=Moved Collect context to position 3"));
    assert!(summary.contains("studio_batch_progress_announcements=0%,5%,10%,15%"));
}
