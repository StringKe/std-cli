mod layout_smoke;
mod plugin_smoke;
mod workflow_builder_smoke;
mod workspace_smoke;

use crate::{default_batch_json, StudioPane};
use layout_smoke::StudioLayoutSmoke;
use plugin_smoke::run_plugin_manager_smoke;
use std_core::{StdConfig, StdCore};
use std_egui::tokens::ThemeSmokeReport;
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
    workflow_status: String,
    builder_created: bool,
    builder_added_step: bool,
    builder_updated_step: bool,
    builder_moved_step: bool,
    builder_simulated: bool,
    builder_run_status: String,
    builder_trace_steps: usize,
    builder_trace_events: usize,
    builder_interaction_sequence: String,
    builder_selected_step: String,
    builder_trace_status: String,
    builder_side_effect_model: String,
    batch_status: String,
    analysis_name: String,
    analysis_coverage_complete: usize,
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
}

impl StudioSmokeReport {
    pub(crate) fn summary(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_smoke {status}\nworkspace_panes={}\nfocused_pane={}\npane_opened={}\npane_focus_switched={}\npane_closed={}\npane_focus_restored={}\nnative_child_windows={}\ndetached_panels={}\nhost_window_size={}\nmin_window_size={}\nhost_chrome_height={}\nstatus_bar_height={}\nsidebar_width={}\ncollapsed_sidebar_width={}\ninspector_width={}\ninspector_default_open={}\nbottom_panel_height={}\nbottom_panel_default_open={}\ncanvas_surface={}\nworkflow_status={}\nbuilder_created={}\nbuilder_added_step={}\nbuilder_updated_step={}\nbuilder_moved_step={}\nbuilder_simulated={}\nbuilder_run_status={}\nbuilder_trace_steps={}\nbuilder_trace_events={}\nbuilder_interaction_sequence={}\nbuilder_selected_step={}\nbuilder_trace_status={}\nbuilder_side_effect_model={}\nbatch_status={}\nanalysis={}\nanalysis_coverage_complete={}\nmemory_count={}\nplugin_js_status={}\nplugin_ts_status={}\nplugin_manifest_checks={}\nplugin_permissions={}\nplugin_action_count={}\nplugin_preview_kind={}\nplugin_js_runtime={}\nplugin_ts_runtime={}\nhistory_count={}",
            self.workspace_panes,
            self.focused_pane,
            self.pane_opened,
            self.pane_focus_switched,
            self.pane_closed,
            self.pane_focus_restored,
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
            self.bottom_panel_height,
            self.bottom_panel_default_open,
            self.canvas_surface,
            self.workflow_status,
            self.builder_created,
            self.builder_added_step,
            self.builder_updated_step,
            self.builder_moved_step,
            self.builder_simulated,
            self.builder_run_status,
            self.builder_trace_steps,
            self.builder_trace_events,
            self.builder_interaction_sequence,
            self.builder_selected_step,
            self.builder_trace_status,
            self.builder_side_effect_model,
            self.batch_status,
            self.analysis_name,
            self.analysis_coverage_complete,
            self.memory_count,
            self.plugin_js_status,
            self.plugin_ts_status,
            self.plugin_manifest_checks,
            self.plugin_permissions,
            self.plugin_action_count,
            self.plugin_preview_kind,
            self.plugin_js_runtime,
            self.plugin_ts_runtime,
            self.history_count
        )
    }

    fn pass(&self) -> bool {
        self.workspace_panes >= 7
            && self.pane_opened
            && self.pane_focus_switched
            && self.pane_closed
            && self.pane_focus_restored
            && !self.native_child_windows
            && !self.detached_panels
            && self.host_window_size == "1280x800"
            && self.min_window_size == "1080x640"
            && self.host_chrome_height == 52
            && self.status_bar_height == 24
            && self.sidebar_width == 240
            && self.collapsed_sidebar_width == 48
            && self.inspector_width == 320
            && !self.inspector_default_open
            && self.bottom_panel_height == 240
            && !self.bottom_panel_default_open
            && self.canvas_surface == "bg/surface-0"
            && self.workflow_status == "Completed"
            && self.builder_created
            && self.builder_added_step
            && self.builder_updated_step
            && self.builder_moved_step
            && self.builder_simulated
            && self.builder_run_status == "Completed"
            && self.builder_trace_steps >= 2
            && self.builder_trace_events >= 3
            && self.builder_interaction_sequence == "create>add>edit>move>simulate>run>trace"
            && self.builder_selected_step == "Validate edited output"
            && self.builder_trace_status == "Completed"
            && self.builder_side_effect_model == "simulate=dry-run,run=audit-log"
            && self.batch_status == "NeedsExternalRunner"
            && self.analysis_coverage_complete >= 1
            && self.memory_count >= 1
            && self.plugin_js_status == "Completed"
            && self.plugin_ts_status == "Completed"
            && self.plugin_manifest_checks >= 1
            && self.plugin_permissions.contains("Code")
            && self.plugin_action_count >= 2
            && self.plugin_preview_kind == "Command"
            && self.plugin_js_runtime == "deno_core"
            && self.plugin_ts_runtime == "deno_core"
            && self.history_count >= 1
    }
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
            workflow_status: format!("FAIL {error}"),
            builder_created: false,
            builder_added_step: false,
            builder_updated_step: false,
            builder_moved_step: false,
            builder_simulated: false,
            builder_run_status: "FAIL".to_string(),
            builder_trace_steps: 0,
            builder_trace_events: 0,
            builder_interaction_sequence: "FAIL".to_string(),
            builder_selected_step: "FAIL".to_string(),
            builder_trace_status: "FAIL".to_string(),
            builder_side_effect_model: "FAIL".to_string(),
            batch_status: "FAIL".to_string(),
            analysis_name: "FAIL".to_string(),
            analysis_coverage_complete: 0,
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
        }),
    }
}

pub(crate) fn theme_smoke_from_args(args: &[String]) -> Option<ThemeSmokeReport> {
    if args.get(1).map(String::as_str) == Some("--theme-smoke") {
        Some(ThemeSmokeReport::new())
    } else {
        None
    }
}

fn run_studio_smoke() -> Result<StudioSmokeReport, Box<dyn std::error::Error>> {
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

    let plugin_smoke = run_plugin_manager_smoke(&mut studio)?;

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
        workflow_status,
        builder_created: builder_smoke.created,
        builder_added_step: builder_smoke.added_step,
        builder_updated_step: builder_smoke.updated_step,
        builder_moved_step: builder_smoke.moved_step,
        builder_simulated: builder_smoke.simulated,
        builder_run_status: builder_smoke.run_status,
        builder_trace_steps: builder_smoke.trace_steps,
        builder_trace_events: builder_smoke.trace_events,
        builder_interaction_sequence: builder_smoke.interaction_sequence,
        builder_selected_step: builder_smoke.selected_step_title,
        builder_trace_status: builder_smoke.trace_status,
        builder_side_effect_model: builder_smoke.side_effect_model,
        batch_status,
        analysis_name,
        analysis_coverage_complete: coverage.complete,
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
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_smoke_reports_internal_workspace_pane_management() {
        let report = run_studio_smoke().unwrap();
        let summary = report.summary();

        assert!(report.pass());
        assert_workspace_policy_summary(&summary);
        assert_shell_layout_summary(&summary);
        assert_workflow_builder_summary(&summary);
    }

    fn assert_workspace_policy_summary(summary: &str) {
        assert!(summary.contains("pane_opened=true"));
        assert!(summary.contains("pane_focus_switched=true"));
        assert!(summary.contains("pane_closed=true"));
        assert!(summary.contains("pane_focus_restored=true"));
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
        assert!(summary.contains("canvas_surface=bg/surface-0"));
    }

    fn assert_workflow_builder_summary(summary: &str) {
        assert!(summary.contains("builder_created=true"));
        assert!(summary.contains("builder_added_step=true"));
        assert!(summary.contains("builder_updated_step=true"));
        assert!(summary.contains("builder_moved_step=true"));
        assert!(summary.contains("builder_simulated=true"));
        assert!(summary.contains("builder_run_status=Completed"));
        assert!(summary.contains("builder_trace_steps=2"));
        assert!(summary
            .contains("builder_interaction_sequence=create>add>edit>move>simulate>run>trace"));
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

    #[test]
    fn studio_theme_smoke_reports_light_and_dark_tokens() {
        let args = vec!["std-studio".to_string(), "--theme-smoke".to_string()];
        let report = theme_smoke_from_args(&args).unwrap();

        assert!(report.pass());
        assert!(report.summary("studio").contains("studio_theme_smoke PASS"));
    }
}
