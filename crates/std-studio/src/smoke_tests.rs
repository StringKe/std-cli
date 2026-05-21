use crate::smoke::run_studio_smoke;

#[test]
fn studio_smoke_reports_internal_workspace_pane_management() {
    let report = run_studio_smoke().unwrap();
    let summary = report.summary();

    assert!(report.pass());
    assert_workspace_policy_summary(&summary);
    assert_shell_layout_summary(&summary);
    assert_workflow_builder_summary(&summary);
    assert_analysis_workbench_summary(&summary);
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
    assert!(summary.contains("forbidden=native-child-windows:false|detached-panels:false"));
    assert!(summary.contains("focused="));
    assert!(summary.contains("title=Plugin Manager"));
    assert!(summary.contains("strategy=internal-egui-workspace-panes"));
    assert!(summary.contains("reopened_memory="));
    assert!(summary.contains("reopened_internal=true"));
    assert!(summary.contains("native_child_windows=false"));
    assert!(summary.contains("detached_panels=false"));
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
    assert!(summary.contains("bottom_panel_height=240"));
    assert!(summary.contains("bottom_panel_default_open=false"));
    assert!(summary.contains("canvas_surface=surface=bg/surface-0"));
    assert!(summary.contains("standard_launcher_enter_ms=320"));
    assert!(summary.contains("reduced_launcher_enter_ms=0"));
    assert!(summary.contains("reduced_focus_ring_ms=0"));
    assert!(summary.contains("reduced_modal_enter_ms=0"));
    assert!(summary.contains("reduce_motion_env=STD_REDUCE_MOTION"));
}

fn assert_workflow_builder_summary(summary: &str) {
    assert!(summary.contains("builder_created=true"));
    assert!(summary.contains("builder_added_step=true"));
    assert!(summary.contains("builder_updated_step=true"));
    assert!(summary.contains("builder_moved_step=true"));
    assert!(summary.contains("builder_simulated=true"));
    assert!(summary.contains("builder_run_status=Completed"));
    assert!(summary.contains("builder_trace_steps=2"));
    assert!(
        summary.contains("builder_interaction_sequence=create>add>edit>move>simulate>run>trace")
    );
    assert!(summary.contains("builder_selected_step=Validate edited output"));
    assert!(summary.contains("builder_trace_status=Completed"));
    assert!(summary.contains("builder_side_effect_model=simulate=dry-run,run=audit-log"));
    assert!(summary.contains("plugin_js_status=Completed"));
    assert!(summary.contains("plugin_ts_status=Completed"));
    assert!(summary.contains("plugin_manifest_checks=1"));
    assert!(summary.contains("plugin_permissions=Code"));
    assert!(summary.contains("plugin_action_count="));
    assert!(summary.contains("plugin_preview_kind=Command"));
    assert!(summary.contains("plugin_js_runtime=deno_core"));
    assert!(summary.contains("plugin_ts_runtime=deno_core"));
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
