use crate::smoke::run_studio_smoke;
use std_egui::input;

#[test]
fn studio_smoke_reports_internal_workspace_pane_management() {
    let report = run_studio_smoke().unwrap();
    let summary = report.summary();

    assert!(report.pass());
    assert_workspace_policy_summary(&summary);
    assert_shell_layout_summary(&summary);
    assert_workflow_builder_summary(&summary);
    assert_analysis_workbench_summary(&summary);
    assert_keyboard_summary(&summary);
    assert_operations_summary(&summary);
}

fn assert_workspace_policy_summary(summary: &str) {
    assert!(summary.contains("pane_opened=true"));
    assert!(summary.contains("pane_focus_switched=true"));
    assert!(summary.contains("pane_closed=true"));
    assert!(summary.contains("pane_focus_restored=true"));
    assert!(summary.contains("pane_deduplicated=true"));
    assert!(summary.contains("pane_content_keys=analysis,apps,dashboard"));
    assert!(summary.contains("history,memory,operations,plugins,settings,workflows"));
    assert!(summary.contains("pane_focused_title=Plugin Manager"));
    assert!(summary.contains("pane_restored_title=Plugin Manager"));
    assert!(summary.contains("pane_closed_removed=true"));
    assert!(summary.contains("pane_state_preserved=true"));
    assert!(summary.contains("pane_focus_label=strategy=internal-egui-workspace-panes"));
    assert!(summary.contains("host=single-borderless-egui-viewport"));
    assert!(summary.contains("sequence=open>focus>switch>close>reopen>restore"));
    assert!(summary.contains("state_preserved=true"));
    assert!(summary.contains("focused="));
    assert!(summary.contains("title=Plugin Manager"));
    assert!(summary.contains("strategy=internal-egui-workspace-panes"));
    assert!(summary.contains("reopened_memory="));
    assert!(summary.contains("reopened_restored=true"));
    assert_workspace_forbidden_policy_summary(summary);
    assert_workspace_pane_lifecycle_summary(summary);
    assert_studio_open_intent_summary(summary);
}

fn assert_workspace_forbidden_policy_summary(summary: &str) {
    assert!(summary.contains("forbidden=native-child-windows:false|detached-panels:false"));
    assert!(summary.contains("native_child_windows=false"));
    assert!(summary.contains("detached_panels=false"));
}

fn assert_workspace_pane_lifecycle_summary(summary: &str) {
    assert!(summary.contains(
        "pane_host_policy=host=single-borderless-egui-viewport;pane_system=internal-egui-workspace-panes;native_child_windows=false;detached_panels=false;docs=docs/22 + docs/24"
    ));
    assert!(
        summary.contains("pane_management_sequence=open>dedupe>focus>switch>close>reopen>restore")
    );
    assert!(summary.contains("pane_focus_switch_path=settings>plugins>plugins"));
    assert!(summary.contains("pane_close_restore_path=close:"));
    assert!(summary.contains("hotkey_source=default-or-user"));
    assert!(summary.contains("hotkey_reset=reset-to-default"));
    assert!(summary.contains("hotkey_control=token-binding-row"));
    assert!(summary.contains("theme_modes=system|dark|light"));
    assert!(summary.contains("theme_control=segmented-control"));
    assert!(summary.contains("ai_control=token-toggle-row"));
    assert!(summary.contains("storage_control=token-path-row"));
    assert!(summary.contains("workspace_main_path_contract=host=single-borderless-egui-viewport"));
    assert!(summary.contains("panes=internal-egui-workspace-panes"));
    assert!(summary.contains("extra_viewports=forbidden"));
    assert!(summary.contains("egui_window=forbidden"));
    assert!(summary.contains("settings_overlay=forbidden"));
}

fn assert_studio_open_intent_summary(summary: &str) {
    assert!(summary.contains("studio_open_smoke PASS"));
    assert!(summary.contains("route=internal-egui-workspace-pane-intent"));
    assert!(summary.contains("targets=7"));
    assert!(summary.contains("internal_panes=7"));
    assert!(summary.contains("focus_restored=true"));
}

fn assert_shell_layout_summary(summary: &str) {
    assert!(summary.contains("host_window_size=1280x800"));
    assert!(summary.contains("min_window_size=1080x640"));
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
    assert!(
        summary.contains("canvas_content_route=focused-workspace-pane-primary,main-pane-fallback")
    );
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
    assert!(summary.contains("builder_toolbar_contract=toolbar=goal-input>plan>save>simulate>test>history-action>ai>zoom"));
    assert!(summary.contains("control=token-toolbar-buttons"));
    assert!(summary.contains("primary=plan|test"));
    assert!(summary.contains("shortcuts=save|simulate|test|history"));
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
    assert!(summary.contains("builder_bottom_panel_contract=batch-debug-open"));
    assert!(summary.contains("history_timeline_contract=timeline=expanded"));
    assert!(summary.contains("columns=step,status,started,finished,payload"));
    assert!(summary.contains("history_trace_steps=1"));
    assert!(summary.contains("history_payload_visible=true"));
    assert!(summary.contains(
        "builder_debug_panel_contract=debug_panel=true,dry_run=true,execution=true,statuses="
    ));
    assert!(summary.contains("success>success>success>success"));
}

fn assert_plugin_manager_summary(summary: &str) {
    assert!(summary.contains("plugin_js_status=Completed"));
    assert!(summary.contains("plugin_ts_status=Completed"));
    assert!(summary.contains("plugin_manifest_checks=1"));
    assert!(summary.contains("plugin_permissions=Code"));
    assert!(summary.contains("plugin_action_count="));
    assert!(summary.contains("plugin_preview_kind=Command"));
    assert!(summary.contains("plugin_js_runtime=deno_core"));
    assert!(summary.contains("plugin_ts_runtime=deno_core"));
    assert!(summary.contains("plugin_visual_contract=list=name|version|status|source|enable"));
    assert!(summary.contains("inspector=description|permissions|commands|audit-log"));
    assert!(summary.contains("commands=2"));
    assert!(summary.contains("runtime=js:deno_core|ts:deno_core"));
}

fn assert_analysis_workbench_summary(summary: &str) {
    assert!(summary.contains(
        "analysis_coverage_layers=overview=true,components=true,relations=true,history=true"
    ));
    assert!(summary.contains("analysis_search_hits=2"));
    assert!(summary.contains("analysis_answer_sources=2"));
    assert!(summary.contains("analysis_inspect_components=1"));
    assert!(summary.contains("analysis_inspect_relations=3"));
    assert!(summary.contains("analysis_inspect_history=1"));
    assert!(summary.contains("analysis_answer_has_evidence=true"));
}

fn assert_operations_summary(summary: &str) {
    assert!(summary.contains("operations_smoke=PASS"));
    assert!(summary.contains("operations_qa_command=mise run quality"));
    assert!(summary.contains("operations_qa_output=rustfmt=PASS"));
    assert!(summary.contains("operations_doctor_command=std doctor"));
    assert!(summary.contains("operations_doctor_result=doctor source gates"));
    assert!(summary.contains("operations_release_command=std release verify"));
    assert!(summary.contains("operations_release_output=manifest="));
    assert!(summary.contains("operations_install_command=std install verify"));
    assert!(summary.contains("operations_install_output=std="));
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
    assert!(summary.contains("studio_workspace_focus_path=dashboard>plugins>settings>dashboard"));
    assert!(
        summary.contains("studio_analysis_focus_path=target>tabs>content>query>coverage>target")
    );
    assert!(summary.contains("studio_analysis_qa_focus=?:coverage>query"));
    assert!(summary.contains("studio_keyboard_contract=docs/20#studio-shortcuts"));
}
