mod analysis_smoke;
mod keyboard_smoke;
mod layout_smoke;
mod operations_smoke;
mod plugin_smoke;
mod report;
pub(crate) mod surface_smoke;
mod workflow_builder_smoke;
pub(crate) mod workspace_policy_smoke;
mod workspace_smoke;

use crate::studio_open::StudioOpenSmokeReport;
use crate::{default_batch_json, StudioPane};
use analysis_smoke::run_analysis_workbench_smoke;
use keyboard_smoke::StudioKeyboardSmoke;
use layout_smoke::StudioLayoutSmoke;
use operations_smoke::OperationsSmoke;
use plugin_smoke::run_plugin_manager_smoke;
use std_core::{StdConfig, StdCore};
use std_studio::StudioApp;
use workflow_builder_smoke::run_workflow_builder_smoke;
use workspace_smoke::run_workspace_pane_smoke;

pub(crate) struct StudioSmokeReport {
    workspace_panes: usize,
    focused_pane: u64,
    pane_opened: bool,
    pane_focus_switched: bool,
    pane_closed: bool,
    pane_focus_restored: bool,
    pane_deduplicated: bool,
    pane_content_keys: String,
    pane_focused_title: String,
    pane_restored_title: String,
    pane_settings_kind: String,
    pane_closed_removed: bool,
    pane_state_preserved: bool,
    pane_focus_label: String,
    pane_host_policy: String,
    pane_management_sequence: String,
    pane_focus_switch_path: String,
    pane_close_restore_path: String,
    native_child_windows: bool,
    detached_panels: bool,
    host_window_size: String,
    min_window_size: String,
    host_chrome_height: u32,
    status_bar_height: u32,
    sidebar_width: u32,
    collapsed_sidebar_width: u32,
    inspector_width: u32,
    inspector_default_open: bool,
    bottom_panel_height: u32,
    bottom_panel_default_open: bool,
    canvas_surface: String,
    canvas_content_route: String,
    workflow_status: String,
    builder_created: bool,
    builder_added_step: bool,
    builder_updated_step: bool,
    builder_moved_step: bool,
    builder_simulated: bool,
    builder_run_status: String,
    builder_planned_run_status: String,
    builder_trace_steps: usize,
    builder_trace_events: usize,
    builder_interaction_sequence: String,
    builder_keyboard_move_path: String,
    builder_selected_step: String,
    builder_trace_status: String,
    builder_side_effect_model: String,
    builder_next_action: String,
    builder_bottom_panel_contract: String,
    batch_status: String,
    analysis_name: String,
    analysis_coverage_complete: usize,
    analysis_coverage_layers: String,
    analysis_search_hits: usize,
    analysis_answer_sources: usize,
    analysis_inspect_components: usize,
    analysis_inspect_relations: usize,
    analysis_inspect_history: usize,
    analysis_answer_has_evidence: bool,
    memory_count: usize,
    plugin_js_status: String,
    plugin_ts_status: String,
    plugin_manifest_checks: usize,
    plugin_permissions: String,
    plugin_action_count: usize,
    plugin_preview_kind: String,
    plugin_js_runtime: String,
    plugin_ts_runtime: String,
    history_count: usize,
    keyboard_summary: String,
    operations_summary: String,
    open_intent_summary: String,
}

pub(crate) fn smoke_from_args(args: Vec<String>) -> Option<StudioSmokeReport> {
    if args.get(1).map(String::as_str) != Some("--smoke") {
        return None;
    }
    match run_studio_smoke() {
        Ok(report) => Some(report),
        Err(error) => Some(StudioSmokeReport {
            workspace_panes: 0,
            focused_pane: 0,
            pane_opened: false,
            pane_focus_switched: false,
            pane_closed: false,
            pane_focus_restored: false,
            pane_deduplicated: false,
            pane_content_keys: "FAIL".to_string(),
            pane_focused_title: "FAIL".to_string(),
            pane_restored_title: "FAIL".to_string(),
            pane_settings_kind: "FAIL".to_string(),
            pane_closed_removed: false,
            pane_state_preserved: false,
            pane_focus_label: "FAIL".to_string(),
            pane_host_policy: "FAIL".to_string(),
            pane_management_sequence: "FAIL".to_string(),
            pane_focus_switch_path: "FAIL".to_string(),
            pane_close_restore_path: "FAIL".to_string(),
            native_child_windows: true,
            detached_panels: true,
            host_window_size: "FAIL".to_string(),
            min_window_size: "FAIL".to_string(),
            host_chrome_height: 0,
            status_bar_height: 0,
            sidebar_width: 0,
            collapsed_sidebar_width: 0,
            inspector_width: 0,
            inspector_default_open: true,
            bottom_panel_height: 0,
            bottom_panel_default_open: true,
            canvas_surface: "FAIL".to_string(),
            canvas_content_route: "FAIL".to_string(),
            workflow_status: format!("FAIL {error}"),
            builder_created: false,
            builder_added_step: false,
            builder_updated_step: false,
            builder_moved_step: false,
            builder_simulated: false,
            builder_run_status: "FAIL".to_string(),
            builder_planned_run_status: "FAIL".to_string(),
            builder_trace_steps: 0,
            builder_trace_events: 0,
            builder_interaction_sequence: "FAIL".to_string(),
            builder_keyboard_move_path: "FAIL".to_string(),
            builder_selected_step: "FAIL".to_string(),
            builder_trace_status: "FAIL".to_string(),
            builder_side_effect_model: "FAIL".to_string(),
            builder_next_action: "FAIL".to_string(),
            builder_bottom_panel_contract: "FAIL".to_string(),
            batch_status: "FAIL".to_string(),
            analysis_name: "FAIL".to_string(),
            analysis_coverage_complete: 0,
            analysis_coverage_layers: "FAIL".to_string(),
            analysis_search_hits: 0,
            analysis_answer_sources: 0,
            analysis_inspect_components: 0,
            analysis_inspect_relations: 0,
            analysis_inspect_history: 0,
            analysis_answer_has_evidence: false,
            memory_count: 0,
            plugin_js_status: "FAIL".to_string(),
            plugin_ts_status: "FAIL".to_string(),
            plugin_manifest_checks: 0,
            plugin_permissions: "FAIL".to_string(),
            plugin_action_count: 0,
            plugin_preview_kind: "FAIL".to_string(),
            plugin_js_runtime: "FAIL".to_string(),
            plugin_ts_runtime: "FAIL".to_string(),
            history_count: 0,
            keyboard_summary: "studio_keyboard_smoke=FAIL".to_string(),
            operations_summary: "operations_smoke=FAIL".to_string(),
            open_intent_summary: "studio_open_smoke FAIL".to_string(),
        }),
    }
}

pub(crate) fn run_studio_smoke() -> Result<StudioSmokeReport, Box<dyn std::error::Error>> {
    let temp = tempfile::tempdir()?;
    let core = StdCore::with_config(StdConfig {
        data_dir: temp.path().join("data"),
        ..StdConfig::default()
    });
    let mut studio = StudioApp::with_core(core);
    let layout = StudioLayoutSmoke::from_default_layout();

    let workflow_path = studio.create_workflow("Studio Smoke", "Headless Studio smoke")?;
    studio.add_workflow_step(
        &workflow_path,
        "Collect context",
        serde_json::json!({"ok": true}),
    )?;
    let workflow_status = format!("{:?}", studio.run_workflow_path(&workflow_path)?.status);
    let history_count = studio.recent_workflow_executions(10)?.len();
    let builder_smoke = run_workflow_builder_smoke(&mut studio)?;
    let batch_status = format!("{:?}", studio.run_batch_json(&default_batch_json())?.status);

    studio.remember_from_studio(
        "studio",
        "Studio smoke memory",
        "Memory Browser writes through shared core",
        vec!["studio".to_string()],
    )?;
    let memory_count = studio.search_memory("smoke").len();

    let project_dir = temp.path().join("project");
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {}\npub struct StudioSmoke {}\n",
    )?;
    let analysis_name = studio.analyze_entity(&project_dir)?.overview.name.clone();
    let coverage = studio.analysis_coverage_report()?;
    let analysis_smoke = run_analysis_workbench_smoke(&studio, "StudioSmoke", "project")?;
    let keyboard_smoke = StudioKeyboardSmoke::run(&mut studio);

    let plugin_smoke = run_plugin_manager_smoke(&mut studio)?;
    let operations_smoke = OperationsSmoke::new();
    let open_smoke = StudioOpenSmokeReport::new();

    studio.open_workspace_pane(StudioPane::Dashboard);
    studio.open_workflow_builder(workflow_path);
    studio.open_analysis_workbench(project_dir);
    studio.open_plugin_manager_pane();
    studio.open_app_manager_pane();
    let memory = studio.open_memory_browser_pane();
    studio.open_execution_history_pane();
    let pane_smoke = run_workspace_pane_smoke(&mut studio, memory);
    studio.open_workspace_pane(StudioPane::Operations);

    Ok(StudioSmokeReport {
        workspace_panes: studio.open_workspace_panes().count(),
        focused_pane: studio.focused_pane.map(|id| id.value()).unwrap_or_default(),
        pane_opened: pane_smoke.opened,
        pane_focus_switched: pane_smoke.focus_switched,
        pane_closed: pane_smoke.closed,
        pane_focus_restored: pane_smoke.focus_restored,
        pane_deduplicated: pane_smoke.deduplicated,
        pane_content_keys: pane_smoke.content_keys,
        pane_focused_title: pane_smoke.focused_title,
        pane_restored_title: pane_smoke.restored_title,
        pane_settings_kind: pane_smoke.settings_kind,
        pane_closed_removed: pane_smoke.closed_removed,
        pane_state_preserved: pane_smoke.state_preserved_after_focus,
        pane_focus_label: pane_smoke.focus_label,
        pane_host_policy: pane_smoke.host_policy,
        pane_management_sequence: pane_smoke.management_sequence,
        pane_focus_switch_path: pane_smoke.focus_switch_path,
        pane_close_restore_path: pane_smoke.close_restore_path,
        native_child_windows: studio.workspace_policy.allows_native_child_windows(),
        detached_panels: studio.workspace_policy.allows_detached_panels(),
        host_window_size: layout.host_window_size,
        min_window_size: layout.min_window_size,
        host_chrome_height: layout.host_chrome_height,
        status_bar_height: layout.status_bar_height,
        sidebar_width: layout.sidebar_width,
        collapsed_sidebar_width: layout.collapsed_sidebar_width,
        inspector_width: layout.inspector_width,
        inspector_default_open: layout.inspector_default_open,
        bottom_panel_height: layout.bottom_panel_height,
        bottom_panel_default_open: layout.bottom_panel_default_open,
        canvas_surface: layout.canvas_surface,
        canvas_content_route: layout.canvas_content_route,
        workflow_status,
        builder_created: builder_smoke.created,
        builder_added_step: builder_smoke.added_step,
        builder_updated_step: builder_smoke.updated_step,
        builder_moved_step: builder_smoke.moved_step,
        builder_simulated: builder_smoke.simulated,
        builder_run_status: builder_smoke.run_status,
        builder_planned_run_status: builder_smoke.planned_run_status,
        builder_trace_steps: builder_smoke.trace_steps,
        builder_trace_events: builder_smoke.trace_events,
        builder_interaction_sequence: builder_smoke.interaction_sequence,
        builder_keyboard_move_path: builder_smoke.keyboard_move_path,
        builder_selected_step: builder_smoke.selected_step_title,
        builder_trace_status: builder_smoke.trace_status,
        builder_side_effect_model: builder_smoke.side_effect_model,
        builder_next_action: builder_smoke.next_action,
        builder_bottom_panel_contract: builder_smoke.bottom_panel_contract,
        batch_status,
        analysis_name,
        analysis_coverage_complete: coverage.complete,
        analysis_coverage_layers: analysis_smoke.coverage_layers,
        analysis_search_hits: analysis_smoke.search_hits,
        analysis_answer_sources: analysis_smoke.answer_sources,
        analysis_inspect_components: analysis_smoke.inspect_components,
        analysis_inspect_relations: analysis_smoke.inspect_relations,
        analysis_inspect_history: analysis_smoke.inspect_history,
        analysis_answer_has_evidence: analysis_smoke.answer_has_evidence,
        memory_count,
        plugin_js_status: plugin_smoke.js_status,
        plugin_ts_status: plugin_smoke.ts_status,
        plugin_manifest_checks: plugin_smoke.manifest_checks,
        plugin_permissions: plugin_smoke.permissions,
        plugin_action_count: plugin_smoke.action_count,
        plugin_preview_kind: plugin_smoke.preview_kind,
        plugin_js_runtime: plugin_smoke.js_runtime,
        plugin_ts_runtime: plugin_smoke.ts_runtime,
        history_count,
        keyboard_summary: keyboard_smoke.summary(),
        operations_summary: operations_smoke.summary(),
        open_intent_summary: open_smoke.summary().replace('\n', ";"),
    })
}
