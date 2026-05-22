use crate::{
    bottom_panel::BottomPanelTabModel,
    layout::StudioLayoutState,
    viewport::{studio_host_viewport_contract, STUDIO_MIN_WINDOW_SIZE, STUDIO_WINDOW_SIZE},
};
use std_egui::motion::MotionContext;

pub(crate) struct StudioLayoutSmoke {
    pub(crate) host_window_size: String,
    pub(crate) min_window_size: String,
    pub(crate) host_viewport_contract: &'static str,
    pub(crate) host_chrome_contract: &'static str,
    pub(crate) host_chrome_input_contract: &'static str,
    pub(crate) host_chrome_height: u32,
    pub(crate) status_bar_height: u32,
    pub(crate) sidebar_width: u32,
    pub(crate) collapsed_sidebar_width: u32,
    pub(crate) inspector_width: u32,
    pub(crate) inspector_default_open: bool,
    pub(crate) inspector_context_route: String,
    pub(crate) bottom_panel_height: u32,
    pub(crate) bottom_panel_default_open: bool,
    pub(crate) bottom_panel_tabs: String,
    pub(crate) canvas_surface: String,
    pub(crate) canvas_content_route: String,
    pub(crate) status_bar_right: String,
}

impl StudioLayoutSmoke {
    pub(crate) fn from_default_layout() -> Self {
        let layout = StudioLayoutState::default();
        let collapsed = StudioLayoutState {
            sidebar_open: false,
            ..layout.clone()
        };
        Self {
            host_window_size: format_window_size(STUDIO_WINDOW_SIZE),
            min_window_size: format_window_size(STUDIO_MIN_WINDOW_SIZE),
            host_viewport_contract: studio_host_viewport_contract(),
            host_chrome_contract: crate::host_chrome::host_chrome_surface_contract(),
            host_chrome_input_contract: crate::host_chrome::host_chrome_input_contract(),
            host_chrome_height: 52,
            status_bar_height: 24,
            sidebar_width: layout.sidebar_width() as u32,
            collapsed_sidebar_width: collapsed.sidebar_width() as u32,
            inspector_width: layout.inspector_width() as u32,
            inspector_default_open: layout.inspector_open,
            inspector_context_route: inspector_context_route_evidence(),
            bottom_panel_height: layout.bottom_panel_height() as u32,
            bottom_panel_default_open: layout.bottom_panel_open,
            bottom_panel_tabs: BottomPanelTabModel::docs22_default().contract(),
            canvas_surface: canvas_motion_evidence(),
            canvas_content_route: canvas_content_route_evidence(),
            status_bar_right: status_bar_right_evidence(),
        }
    }
}

fn format_window_size(size: [f32; 2]) -> String {
    format!("{}x{}", size[0] as u32, size[1] as u32)
}

fn canvas_motion_evidence() -> String {
    let standard = MotionContext::standard();
    let reduced = MotionContext::reduced();
    format!(
        "surface=bg/surface-0,standard_launcher_enter_ms={},reduced_launcher_enter_ms={},reduced_focus_ring_ms={},reduced_modal_enter_ms={},reduce_motion_env=STD_REDUCE_MOTION",
        standard.launcher_enter().as_millis(),
        reduced.launcher_enter().as_millis(),
        reduced.focus_ring().as_millis(),
        reduced.modal_enter().as_millis()
    )
}

fn canvas_content_route_evidence() -> String {
    let source = include_str!("../shell.rs");
    let old_append_call = ["self.render_", "workspace_panes(ui);"].join("");
    if source.contains("if self.render_focused_workspace_pane(ui)")
        && source.contains("self.render_main_workspace_pane(ui);")
        && !source.contains(&old_append_call)
    {
        "focused-workspace-pane-primary,main-pane-fallback".to_string()
    } else {
        "workspace-pane-appended-to-main".to_string()
    }
}

fn inspector_context_route_evidence() -> String {
    let source = include_str!("../shell.rs");
    if source.contains("focused_workspace_spec(&self.app)")
        && source.contains("render_workspace_context(ui, &spec)")
        && source.contains("workspace_context_summary")
    {
        "focused-workspace-pane-context,global-fallback".to_string()
    } else {
        "global-context-only".to_string()
    }
}

fn status_bar_right_evidence() -> String {
    let source = include_str!("../shell.rs");
    if source.contains("StudioStatusBarSummary::from_state(&self.app, &self.analysis)")
        && source.contains("summary.right_labels()")
    {
        "analysis-progress,ai-provider,version".to_string()
    } else {
        "static-status-text".to_string()
    }
}
