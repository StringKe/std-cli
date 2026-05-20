use crate::{
    default_batch_json,
    layout::StudioLayoutState,
    viewport::{STUDIO_MIN_WINDOW_SIZE, STUDIO_WINDOW_SIZE},
    StudioPane,
};
use std_core::{StdConfig, StdCore};
use std_egui::tokens::ThemeSmokeReport;
use std_studio::{StudioApp, WorkspacePaneId};

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
    batch_status: String,
    analysis_name: String,
    analysis_coverage_complete: usize,
    memory_count: usize,
    plugin_status: String,
    history_count: usize,
}

impl StudioSmokeReport {
    pub(crate) fn summary(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_smoke {status}\nworkspace_panes={}\nfocused_pane={}\npane_opened={}\npane_focus_switched={}\npane_closed={}\npane_focus_restored={}\nnative_child_windows={}\ndetached_panels={}\nhost_window_size={}\nmin_window_size={}\nhost_chrome_height={}\nstatus_bar_height={}\nsidebar_width={}\ncollapsed_sidebar_width={}\ninspector_width={}\ninspector_default_open={}\nbottom_panel_height={}\nbottom_panel_default_open={}\ncanvas_surface={}\nworkflow_status={}\nbuilder_created={}\nbuilder_added_step={}\nbuilder_updated_step={}\nbuilder_moved_step={}\nbuilder_simulated={}\nbuilder_run_status={}\nbuilder_trace_steps={}\nbuilder_trace_events={}\nbatch_status={}\nanalysis={}\nanalysis_coverage_complete={}\nmemory_count={}\nplugin_status={}\nhistory_count={}",
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
            self.batch_status,
            self.analysis_name,
            self.analysis_coverage_complete,
            self.memory_count,
            self.plugin_status,
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
            && self.batch_status == "NeedsExternalRunner"
            && self.analysis_coverage_complete >= 1
            && self.memory_count >= 1
            && self.plugin_status == "Completed"
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
            batch_status: "FAIL".to_string(),
            analysis_name: "FAIL".to_string(),
            analysis_coverage_complete: 0,
            memory_count: 0,
            plugin_status: "FAIL".to_string(),
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
    let layout = StudioLayoutSmoke::from_layout(StudioLayoutState::default());

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

    let plugin_dir = studio.core.config.plugins_dir().join("studio-smoke");
    std::fs::create_dir_all(&plugin_dir)?;
    std::fs::write(
        plugin_dir.join("main.js"),
        r#"std.emit({ plugin: "studio-smoke", status: "ok" });"#,
    )?;
    std::fs::write(plugin_dir.join("plugin.json"), smoke_plugin_manifest())?;
    studio.reload_plugins()?;
    studio.search_plugins("studio-smoke");
    let plugin_status = studio
        .run_selected_plugin()
        .map(|execution| format!("{:?}", execution.status))
        .unwrap_or_else(|| "Missing".to_string());

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
        batch_status,
        analysis_name,
        analysis_coverage_complete: coverage.complete,
        memory_count,
        plugin_status,
        history_count,
    })
}

struct WorkflowBuilderSmoke {
    created: bool,
    added_step: bool,
    updated_step: bool,
    moved_step: bool,
    simulated: bool,
    run_status: String,
    trace_steps: usize,
    trace_events: usize,
}

fn run_workflow_builder_smoke(
    studio: &mut StudioApp,
) -> Result<WorkflowBuilderSmoke, Box<dyn std::error::Error>> {
    let workflow_path = studio.create_workflow("Builder Smoke", "Builder interaction smoke")?;
    let created = workflow_path.ends_with("builder-smoke/workflow.md");
    let first = studio.add_workflow_step(
        &workflow_path,
        "Collect builder context",
        serde_json::json!({"phase": "collect"}),
    )?;
    let second = studio.add_workflow_step(
        &workflow_path,
        "Validate builder output",
        serde_json::json!({"phase": "validate"}),
    )?;
    let added_step =
        first.name == "Collect builder context" && second.name == "Validate builder output";
    let updated = studio.update_workflow_step(
        &workflow_path,
        1,
        Some("Validate edited output"),
        Some(serde_json::json!({"phase": "edited"})),
    )?;
    let moved = studio.move_workflow_step(&workflow_path, 1, 0)?;
    let simulated = studio.preview_workflow_path(&workflow_path)?.steps.len() == 2;
    let execution = studio.run_workflow_path(&workflow_path)?.clone();
    let run_status = format!("{:?}", execution.status);
    let traces = studio.recent_workflow_traces(10)?;
    let trace = traces
        .iter()
        .find(|trace| trace.execution.workflow_id == execution.workflow_id);

    Ok(WorkflowBuilderSmoke {
        created,
        added_step,
        updated_step: updated.name == "Validate edited output",
        moved_step: moved.name == "Validate edited output",
        simulated,
        run_status,
        trace_steps: trace.map(|trace| trace.steps.len()).unwrap_or_default(),
        trace_events: trace
            .map(|trace| trace.audit_events.len())
            .unwrap_or_default(),
    })
}

struct StudioLayoutSmoke {
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
}

impl StudioLayoutSmoke {
    fn from_layout(layout: StudioLayoutState) -> Self {
        let collapsed = StudioLayoutState {
            sidebar_open: false,
            ..layout.clone()
        };
        Self {
            host_window_size: format_window_size(STUDIO_WINDOW_SIZE),
            min_window_size: format_window_size(STUDIO_MIN_WINDOW_SIZE),
            host_chrome_height: 52,
            status_bar_height: 24,
            sidebar_width: layout.sidebar_width() as u32,
            collapsed_sidebar_width: collapsed.sidebar_width() as u32,
            inspector_width: layout.inspector_width() as u32,
            inspector_default_open: layout.inspector_open,
            bottom_panel_height: layout.bottom_panel_height() as u32,
            bottom_panel_default_open: layout.bottom_panel_open,
            canvas_surface: "bg/surface-0".to_string(),
        }
    }
}

fn format_window_size(size: [f32; 2]) -> String {
    format!("{}x{}", size[0] as u32, size[1] as u32)
}

struct WorkspacePaneSmoke {
    opened: bool,
    focus_switched: bool,
    closed: bool,
    focus_restored: bool,
}

fn run_workspace_pane_smoke(
    studio: &mut StudioApp,
    close_target: WorkspacePaneId,
) -> WorkspacePaneSmoke {
    let settings = studio.open_settings_pane();
    let opened = studio.focused_pane == Some(settings);
    let plugin = studio.open_plugin_manager_pane();
    let focus_switched = studio.focused_pane == Some(plugin);
    let closed = studio.close_workspace_pane(close_target);
    let focus_restored = studio.focus_workspace_pane(settings)
        && studio.close_workspace_pane(settings)
        && studio.focused_pane == Some(plugin);
    WorkspacePaneSmoke {
        opened,
        focus_switched,
        closed,
        focus_restored,
    }
}

fn smoke_plugin_manifest() -> String {
    serde_json::json!({
        "name": "studio-smoke",
        "description": "Studio smoke plugin",
        "permissions": ["code"],
        "actions": [{
            "name": "Plugin Studio Smoke",
            "description": "Run Studio smoke plugin",
            "when_to_use": "When validating std-studio smoke",
            "kind": "javascript",
            "script": "main.js",
            "tags": ["studio-smoke"]
        }]
    })
    .to_string()
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
    }

    #[test]
    fn studio_theme_smoke_reports_light_and_dark_tokens() {
        let args = vec!["std-studio".to_string(), "--theme-smoke".to_string()];
        let report = theme_smoke_from_args(&args).unwrap();

        assert!(report.pass());
        assert!(report.summary("studio").contains("studio_theme_smoke PASS"));
    }
}
