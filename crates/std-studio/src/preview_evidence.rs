use crate::{
    layout::{
        BOTTOM_PANEL_DEFAULT_HEIGHT, HOST_CHROME_HEIGHT, INSPECTOR_DEFAULT_WIDTH,
        SIDEBAR_DEFAULT_WIDTH, STATUS_BAR_HEIGHT,
    },
    preview::seeded_preview_app,
    viewport::{STUDIO_MIN_WINDOW_SIZE, STUDIO_WINDOW_SIZE},
    StudioEguiApp, StudioPane,
};
use eframe::egui;
use std_egui::tokens::{apply_theme, Color, ThemeMode};

pub(crate) fn required_capture_states_summary() -> &'static str {
    "required_capture_states=light-dashboard,dark-dashboard,light-workflow,dark-workflow,light-workflow-error,dark-workflow-error,light-analysis,dark-analysis,light-plugins,dark-plugins,light-plugin-permission,dark-plugin-permission,light-operations,dark-operations,light-settings,dark-settings,light-panes,dark-panes"
}

pub(crate) fn preview_matrix() -> Vec<String> {
    [
        "light-dashboard",
        "dark-dashboard",
        "light-workflow",
        "dark-workflow",
        "light-workflow-error",
        "dark-workflow-error",
        "light-analysis",
        "dark-analysis",
        "light-plugins",
        "dark-plugins",
        "light-plugin-permission",
        "dark-plugin-permission",
        "light-operations",
        "dark-operations",
        "light-settings",
        "dark-settings",
        "light-panes",
        "dark-panes",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

pub(crate) fn preview_state_summary(scenario: &str) -> String {
    let Some((theme, name)) = scenario.split_once('-') else {
        return format!("{scenario}=FAIL");
    };
    let app = seeded_preview_app(theme, name);
    let surface = preview_surface_summary(theme);
    let valid = matches!(theme, "dark" | "light")
        && preview_state_passes(&app, name)
        && preview_surface_passes(&surface, theme);
    format!(
        "{scenario}={}:pane={:?},workspace={},status={},workflow_e2e={},workflow_error={},pane_management={},plugin_permission={},{}",
        if valid { "PASS" } else { "FAIL" },
        app.app.active_pane,
        app.app.open_workspace_panes().count(),
        app.status,
        workflow_e2e_contract(&app, name),
        workflow_error_contract(&app, name),
        pane_management_contract(&app, name),
        plugin_permission_contract(&app),
        surface.summary()
    )
}

pub(crate) fn preview_size_summary(scenario: &str) -> String {
    let Some((theme, name)) = scenario.split_once('-') else {
        return format!("{scenario}=FAIL");
    };
    let app = seeded_preview_app(theme, name);
    let host = format_window_size(STUDIO_WINDOW_SIZE);
    let min = format_window_size(STUDIO_MIN_WINDOW_SIZE);
    let host_layout = StudioPreviewGeometry::for_window(STUDIO_WINDOW_SIZE, &app);
    let min_layout = StudioPreviewGeometry::for_window(STUDIO_MIN_WINDOW_SIZE, &app);
    let valid = matches!(theme, "dark" | "light")
        && !app.app.workspace_policy.allows_native_child_windows()
        && !app.app.workspace_policy.allows_detached_panels()
        && host == "1280x800"
        && min == "1080x640"
        && host_layout.passes()
        && min_layout.passes();
    format!(
        "{scenario}={}:host={},min={},workspace={},native_child_windows={},detached_panels={},settings_surface={},host_layout={},min_layout={}",
        if valid { "PASS" } else { "FAIL" },
        host,
        min,
        app.app.open_workspace_panes().count(),
        app.app.workspace_policy.allows_native_child_windows(),
        app.app.workspace_policy.allows_detached_panels(),
        settings_surface_policy(&app, name),
        host_layout.summary(),
        min_layout.summary()
    )
}

fn settings_surface_policy(app: &StudioEguiApp, scenario: &str) -> &'static str {
    if scenario == "settings"
        && app.app.active_pane == StudioPane::Settings
        && !app.app.workspace_policy.allows_native_child_windows()
        && !app.app.workspace_policy.allows_detached_panels()
    {
        "internal-workspace-pane"
    } else {
        "not-settings"
    }
}

fn preview_state_passes(app: &StudioEguiApp, scenario: &str) -> bool {
    match scenario {
        "dashboard" => app.app.active_pane == StudioPane::Dashboard,
        "workflow" => {
            app.app.active_pane == StudioPane::Workflows
                && app.app.workflow_debug.is_some()
                && app.app.last_workflow_execution.is_some()
        }
        "workflow-error" => {
            app.app.active_pane == StudioPane::Workflows
                && app.app.last_workflow_execution.is_some()
                && app.bottom_panel_tab == crate::bottom_panel_model::BottomPanelTab::Problems
                && app
                    .app
                    .last_workflow_execution
                    .as_ref()
                    .map(|execution| execution.status == std_orchestration::ExecutionStatus::Failed)
                    .unwrap_or(false)
        }
        "analysis" => {
            app.app.active_pane == StudioPane::Analysis && !app.analysis.coverage_output.is_empty()
        }
        "plugins" => {
            app.app.active_pane == StudioPane::Plugins
                && app.app.open_workspace_panes().count() >= 1
        }
        "plugin-permission" => {
            app.app.active_pane == StudioPane::Plugins
                && plugin_permission_contract(app) == "permissions|fs|network|review-prompt"
        }
        "operations" => app.app.active_pane == StudioPane::Operations,
        "settings" => app.app.active_pane == StudioPane::Settings,
        "panes" => {
            app.app.open_workspace_panes().count() >= 3
                && !app.app.workspace_policy.allows_native_child_windows()
                && !app.app.workspace_policy.allows_detached_panels()
        }
        _ => false,
    }
}

fn workflow_e2e_contract(app: &StudioEguiApp, scenario: &str) -> &'static str {
    if scenario == "workflow"
        && app.app.active_pane == StudioPane::Workflows
        && app.app.workflow_debug.is_some()
        && app.app.last_workflow_execution.is_some()
        && app.app.open_workspace_panes().count() >= 2
        && app.layout.bottom_panel_open
    {
        "builder|dry-run|execution|trace|history-pane|bottom-panel"
    } else {
        "not-workflow"
    }
}

fn workflow_error_contract(app: &StudioEguiApp, scenario: &str) -> &'static str {
    if scenario == "workflow-error"
        && app.app.active_pane == StudioPane::Workflows
        && app.layout.bottom_panel_open
        && app.bottom_panel_tab == crate::bottom_panel_model::BottomPanelTab::Problems
        && app
            .app
            .last_workflow_execution
            .as_ref()
            .map(|execution| {
                execution.status == std_orchestration::ExecutionStatus::Failed
                    && execution
                        .results
                        .iter()
                        .any(|step| step.status == std_orchestration::ExecutionStatus::Failed)
            })
            .unwrap_or(false)
    {
        "failed-execution|problems-panel|error-row"
    } else {
        "not-workflow-error"
    }
}

fn pane_management_contract(app: &StudioEguiApp, scenario: &str) -> &'static str {
    if scenario == "panes"
        && app.status.contains("open=true")
        && app.status.contains("focus=true")
        && app.status.contains("switch=true")
        && app.status.contains("close=true")
        && app.status.contains("reopen=true")
        && app.status.contains("restore=true")
        && app.status.contains("state_preserved=true")
        && app.status.contains("history_visible=true")
        && app.app.open_workspace_panes().count() >= 3
        && !app.app.workspace_policy.allows_native_child_windows()
        && !app.app.workspace_policy.allows_detached_panels()
    {
        "open|focus|switch|close|reopen|restore|state-preserved|single-egui-viewport"
    } else {
        "not-panes"
    }
}

fn plugin_permission_contract(app: &StudioEguiApp) -> &'static str {
    let Some(report) = app
        .app
        .plugin_manager
        .check_reports
        .iter()
        .find(|report| report.plugin_name == "permission-preview-plugin")
    else {
        return "not-plugin-permission";
    };
    let permissions = report
        .permissions
        .iter()
        .map(|permission| format!("{permission:?}"))
        .collect::<Vec<_>>();
    if permissions.contains(&"Code".to_string())
        && permissions.contains(&"FsScoped".to_string())
        && permissions.contains(&"Network".to_string())
        && !report.fs_scopes.is_empty()
        && report.network_hosts == ["api.preview.local"]
    {
        "permissions|fs|network|review-prompt"
    } else {
        "not-plugin-permission"
    }
}

fn preview_surface_summary(theme: &str) -> PreviewSurfaceSummary {
    let ctx = egui::Context::default();
    apply_theme(&ctx, ThemeMode::resolve(theme));
    PreviewSurfaceSummary {
        canvas: color_hex(Color::bg_surface_0(&ctx)),
        panel: color_hex(Color::bg_surface_1(&ctx)),
        inspector: color_hex(Color::bg_surface_1(&ctx)),
        selected: color_hex_alpha(Color::accent_weak(&ctx)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewSurfaceSummary {
    canvas: String,
    panel: String,
    inspector: String,
    selected: String,
}

impl PreviewSurfaceSummary {
    fn summary(&self) -> String {
        format!(
            "canvas_token=bg/surface-0:{},panel_token=bg/surface-1:{},inspector_token=bg/surface-1:{},selected_token=accent/weak:{}",
            self.canvas, self.panel, self.inspector, self.selected
        )
    }
}

fn preview_surface_passes(surface: &PreviewSurfaceSummary, theme: &str) -> bool {
    match theme {
        "dark" => {
            surface.canvas == "#1C1E22"
                && surface.panel == "#24272C"
                && surface.inspector == "#24272C"
                && surface.selected == "#4E9CFF@46"
        }
        "light" => {
            surface.canvas == "#FAFBFD"
                && surface.panel == "#F2F5F8"
                && surface.inspector == "#F2F5F8"
                && surface.selected == "#0A6BFF@31"
        }
        _ => false,
    }
}

fn format_window_size(size: [f32; 2]) -> String {
    format!("{}x{}", size[0] as u32, size[1] as u32)
}

#[derive(Debug, Clone, PartialEq)]
struct StudioPreviewGeometry {
    width: f32,
    height: f32,
    host_chrome: f32,
    sidebar: f32,
    inspector: f32,
    bottom_panel: f32,
    status_bar: f32,
    canvas_width: f32,
    canvas_height: f32,
}

impl StudioPreviewGeometry {
    fn for_window(size: [f32; 2], app: &StudioEguiApp) -> Self {
        let sidebar = app.layout.sidebar_width();
        let inspector = if app.layout.inspector_open {
            app.layout.inspector_width()
        } else {
            0.0
        };
        let bottom_panel = if app.layout.bottom_panel_open {
            app.layout.bottom_panel_height()
        } else {
            0.0
        };
        Self {
            width: size[0],
            height: size[1],
            host_chrome: HOST_CHROME_HEIGHT,
            sidebar,
            inspector,
            bottom_panel,
            status_bar: STATUS_BAR_HEIGHT,
            canvas_width: size[0] - sidebar - inspector,
            canvas_height: size[1] - HOST_CHROME_HEIGHT - bottom_panel - STATUS_BAR_HEIGHT,
        }
    }

    fn passes(&self) -> bool {
        self.host_chrome == 52.0
            && self.status_bar == 24.0
            && self.sidebar == SIDEBAR_DEFAULT_WIDTH
            && self.inspector <= INSPECTOR_DEFAULT_WIDTH
            && self.bottom_panel <= BOTTOM_PANEL_DEFAULT_HEIGHT
            && self.canvas_width >= 520.0
            && self.canvas_height >= 324.0
    }

    fn summary(&self) -> String {
        format!(
            "{}x{}:chrome={},sidebar={},inspector={},bottom={},status={},canvas={}x{},fits={}",
            self.width as u32,
            self.height as u32,
            self.host_chrome as u32,
            self.sidebar as u32,
            self.inspector as u32,
            self.bottom_panel as u32,
            self.status_bar as u32,
            self.canvas_width as u32,
            self.canvas_height as u32,
            self.passes()
        )
    }
}

fn color_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

fn color_hex_alpha(color: egui::Color32) -> String {
    format!(
        "#{:02X}{:02X}{:02X}@{}",
        color.r(),
        color.g(),
        color.b(),
        color.a()
    )
}
