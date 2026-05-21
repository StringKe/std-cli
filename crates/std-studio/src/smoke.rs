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
use crate::views::history_timeline;
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

struct SmokeInputs {
    layout: StudioLayoutSmoke,
    workflow_path: std::path::PathBuf,
    workflow_status: String,
    history_count: usize,
    history_timeline_contract: String,
    history_trace_steps: usize,
    history_payload_visible: bool,
    builder: workflow_builder_smoke::WorkflowBuilderSmoke,
    batch_status: String,
    memory_count: usize,
    project_dir: std::path::PathBuf,
    analysis_name: String,
    coverage: std_index::IndexCoverageReport,
    analysis: analysis_smoke::AnalysisWorkbenchSmoke,
    keyboard: StudioKeyboardSmoke,
    plugin: plugin_smoke::PluginManagerSmoke,
    operations: OperationsSmoke,
    open_smoke: StudioOpenSmokeReport,
}

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
    pane_settings_contract: String,
    workspace_main_path_contract: String,
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
    inspector_context_route: String,
    bottom_panel_height: u32,
    bottom_panel_default_open: bool,
    bottom_panel_tabs: String,
    canvas_surface: String,
    canvas_content_route: String,
    status_bar_right: String,
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
    builder_toolbar_contract: String,
    builder_properties_contract: String,
    builder_keyboard_move_path: String,
    builder_selected_step: String,
    builder_trace_status: String,
    builder_side_effect_model: String,
    builder_next_action: String,
    builder_bottom_panel_contract: String,
    builder_debug_panel_contract: String,
    builder_visual_contract: String,
    batch_status: String,
    analysis_name: String,
    analysis_coverage_complete: usize,
    analysis_coverage_layers: String,
    analysis_search_hits: usize,
    analysis_answer_sources: usize,
    analysis_visual_contract: String,
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
    plugin_status_bar_contract: String,
    plugin_permission_visual_contract: String,
    plugin_inspector_contract: String,
    plugin_visual_contract: String,
    history_count: usize,
    history_timeline_contract: String,
    history_trace_steps: usize,
    history_payload_visible: bool,
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
            pane_settings_contract: "FAIL".to_string(),
            workspace_main_path_contract: "FAIL".to_string(),
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
            inspector_context_route: "FAIL".to_string(),
            bottom_panel_height: 0,
            bottom_panel_default_open: true,
            bottom_panel_tabs: "FAIL".to_string(),
            canvas_surface: "FAIL".to_string(),
            canvas_content_route: "FAIL".to_string(),
            status_bar_right: "FAIL".to_string(),
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
            builder_toolbar_contract: "FAIL".to_string(),
            builder_properties_contract: "FAIL".to_string(),
            builder_keyboard_move_path: "FAIL".to_string(),
            builder_selected_step: "FAIL".to_string(),
            builder_trace_status: "FAIL".to_string(),
            builder_side_effect_model: "FAIL".to_string(),
            builder_next_action: "FAIL".to_string(),
            builder_bottom_panel_contract: "FAIL".to_string(),
            builder_debug_panel_contract: "FAIL".to_string(),
            builder_visual_contract: "FAIL".to_string(),
            batch_status: "FAIL".to_string(),
            analysis_name: "FAIL".to_string(),
            analysis_coverage_complete: 0,
            analysis_coverage_layers: "FAIL".to_string(),
            analysis_search_hits: 0,
            analysis_answer_sources: 0,
            analysis_visual_contract: "FAIL".to_string(),
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
            plugin_status_bar_contract: "FAIL".to_string(),
            plugin_permission_visual_contract: "FAIL".to_string(),
            plugin_inspector_contract: "FAIL".to_string(),
            plugin_visual_contract: "FAIL".to_string(),
            history_count: 0,
            history_timeline_contract: "FAIL".to_string(),
            history_trace_steps: 0,
            history_payload_visible: false,
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
    let inputs = collect_smoke_inputs(&mut studio, temp.path())?;

    studio.open_workspace_pane(StudioPane::Dashboard);
    studio.open_workflow_builder(inputs.workflow_path.clone());
    studio.open_analysis_workbench(inputs.project_dir.clone());
    studio.open_plugin_manager_pane();
    studio.open_app_manager_pane();
    let memory = studio.open_memory_browser_pane();
    studio.open_execution_history_pane();
    let pane_smoke = run_workspace_pane_smoke(&mut studio, memory);
    studio.open_workspace_pane(StudioPane::Operations);

    Ok(report_from_inputs(&studio, pane_smoke, inputs))
}

fn collect_smoke_inputs(
    studio: &mut StudioApp,
    temp: &std::path::Path,
) -> Result<SmokeInputs, Box<dyn std::error::Error>> {
    let workflow_path = studio.create_workflow("Studio Smoke", "Headless Studio smoke")?;
    studio.add_workflow_step(
        &workflow_path,
        "Collect context",
        serde_json::json!({"ok": true}),
    )?;
    let workflow_status = format!("{:?}", studio.run_workflow_path(&workflow_path)?.status);
    let traces = studio.recent_workflow_traces(10)?;
    let history_count = traces.len();
    let history_timeline_contract = traces
        .first()
        .map(history_timeline::history_timeline_contract)
        .unwrap_or_else(|| "timeline=missing".to_string());
    let history_trace_steps = traces
        .first()
        .map(|trace| trace.execution.results.len())
        .unwrap_or_default();
    let history_payload_visible = traces
        .first()
        .map(|trace| {
            trace
                .execution
                .results
                .iter()
                .any(|step| !step.output.is_null())
        })
        .unwrap_or(false);
    let builder = run_workflow_builder_smoke(studio)?;
    let batch_status = format!("{:?}", studio.run_batch_json(&default_batch_json())?.status);

    studio.remember_from_studio(
        "studio",
        "Studio smoke memory",
        "Memory Browser writes through shared core",
        vec!["studio".to_string()],
    )?;
    let memory_count = studio.search_memory("smoke").len();

    let project_dir = temp.join("project");
    std::fs::create_dir_all(project_dir.join("src"))?;
    std::fs::write(
        project_dir.join("src").join("main.rs"),
        "fn main() {}\npub struct StudioSmoke {}\n",
    )?;
    let analysis_name = studio.analyze_entity(&project_dir)?.overview.name.clone();
    let coverage = studio.analysis_coverage_report()?;
    let analysis = run_analysis_workbench_smoke(studio, "StudioSmoke", "project")?;
    let keyboard = StudioKeyboardSmoke::run(studio);
    let plugin = run_plugin_manager_smoke(studio)?;

    Ok(SmokeInputs {
        layout: StudioLayoutSmoke::from_default_layout(),
        workflow_path,
        workflow_status,
        history_count,
        history_timeline_contract,
        history_trace_steps,
        history_payload_visible,
        builder,
        batch_status,
        memory_count,
        project_dir,
        analysis_name,
        coverage,
        analysis,
        keyboard,
        plugin,
        operations: OperationsSmoke::new(),
        open_smoke: StudioOpenSmokeReport::new(),
    })
}

fn report_from_inputs(
    studio: &StudioApp,
    pane_smoke: workspace_smoke::WorkspacePaneSmoke,
    inputs: SmokeInputs,
) -> StudioSmokeReport {
    StudioSmokeReport {
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
        pane_settings_contract: pane_smoke.settings_contract,
        workspace_main_path_contract: pane_smoke.main_path_contract,
        native_child_windows: studio.workspace_policy.allows_native_child_windows(),
        detached_panels: studio.workspace_policy.allows_detached_panels(),
        host_window_size: inputs.layout.host_window_size,
        min_window_size: inputs.layout.min_window_size,
        host_chrome_height: inputs.layout.host_chrome_height,
        status_bar_height: inputs.layout.status_bar_height,
        sidebar_width: inputs.layout.sidebar_width,
        collapsed_sidebar_width: inputs.layout.collapsed_sidebar_width,
        inspector_width: inputs.layout.inspector_width,
        inspector_default_open: inputs.layout.inspector_default_open,
        inspector_context_route: inputs.layout.inspector_context_route,
        bottom_panel_height: inputs.layout.bottom_panel_height,
        bottom_panel_default_open: inputs.layout.bottom_panel_default_open,
        bottom_panel_tabs: inputs.layout.bottom_panel_tabs,
        canvas_surface: inputs.layout.canvas_surface,
        canvas_content_route: inputs.layout.canvas_content_route,
        status_bar_right: inputs.layout.status_bar_right,
        workflow_status: inputs.workflow_status,
        builder_created: inputs.builder.created,
        builder_added_step: inputs.builder.added_step,
        builder_updated_step: inputs.builder.updated_step,
        builder_moved_step: inputs.builder.moved_step,
        builder_simulated: inputs.builder.simulated,
        builder_run_status: inputs.builder.run_status,
        builder_planned_run_status: inputs.builder.planned_run_status,
        builder_trace_steps: inputs.builder.trace_steps,
        builder_trace_events: inputs.builder.trace_events,
        builder_interaction_sequence: inputs.builder.interaction_sequence,
        builder_toolbar_contract: inputs.builder.toolbar_contract,
        builder_properties_contract: inputs.builder.properties_contract,
        builder_keyboard_move_path: inputs.builder.keyboard_move_path,
        builder_selected_step: inputs.builder.selected_step_title,
        builder_trace_status: inputs.builder.trace_status,
        builder_side_effect_model: inputs.builder.side_effect_model,
        builder_next_action: inputs.builder.next_action,
        builder_bottom_panel_contract: inputs.builder.bottom_panel_contract,
        builder_debug_panel_contract: inputs.builder.debug_panel_contract,
        builder_visual_contract: inputs.builder.visual_contract,
        batch_status: inputs.batch_status,
        analysis_name: inputs.analysis_name,
        analysis_coverage_complete: inputs.coverage.complete,
        analysis_coverage_layers: inputs.analysis.coverage_layers,
        analysis_search_hits: inputs.analysis.search_hits,
        analysis_answer_sources: inputs.analysis.answer_sources,
        analysis_visual_contract: inputs.analysis.visual_contract,
        analysis_inspect_components: inputs.analysis.inspect_components,
        analysis_inspect_relations: inputs.analysis.inspect_relations,
        analysis_inspect_history: inputs.analysis.inspect_history,
        analysis_answer_has_evidence: inputs.analysis.answer_has_evidence,
        memory_count: inputs.memory_count,
        plugin_js_status: inputs.plugin.js_status,
        plugin_ts_status: inputs.plugin.ts_status,
        plugin_manifest_checks: inputs.plugin.manifest_checks,
        plugin_permissions: inputs.plugin.permissions,
        plugin_action_count: inputs.plugin.action_count,
        plugin_preview_kind: inputs.plugin.preview_kind,
        plugin_js_runtime: inputs.plugin.js_runtime,
        plugin_ts_runtime: inputs.plugin.ts_runtime,
        plugin_status_bar_contract: inputs.plugin.status_bar_contract,
        plugin_permission_visual_contract: inputs.plugin.permission_visual_contract,
        plugin_inspector_contract: inputs.plugin.inspector_contract,
        plugin_visual_contract: inputs.plugin.visual_contract,
        history_count: inputs.history_count,
        history_timeline_contract: inputs.history_timeline_contract,
        history_trace_steps: inputs.history_trace_steps,
        history_payload_visible: inputs.history_payload_visible,
        keyboard_summary: inputs.keyboard.summary(),
        operations_summary: inputs.operations.summary(),
        open_intent_summary: inputs.open_smoke.summary().replace('\n', ";"),
    }
}
