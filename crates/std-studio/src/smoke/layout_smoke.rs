use crate::{
    layout::StudioLayoutState,
    viewport::{STUDIO_MIN_WINDOW_SIZE, STUDIO_WINDOW_SIZE},
};
use std_egui::motion::MotionContext;

pub(crate) struct StudioLayoutSmoke {
    pub(crate) host_window_size: String,
    pub(crate) min_window_size: String,
    pub(crate) host_chrome_height: u32,
    pub(crate) status_bar_height: u32,
    pub(crate) sidebar_width: u32,
    pub(crate) collapsed_sidebar_width: u32,
    pub(crate) inspector_width: u32,
    pub(crate) inspector_default_open: bool,
    pub(crate) bottom_panel_height: u32,
    pub(crate) bottom_panel_default_open: bool,
    pub(crate) canvas_surface: String,
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
            host_chrome_height: 52,
            status_bar_height: 24,
            sidebar_width: layout.sidebar_width() as u32,
            collapsed_sidebar_width: collapsed.sidebar_width() as u32,
            inspector_width: layout.inspector_width() as u32,
            inspector_default_open: layout.inspector_open,
            bottom_panel_height: layout.bottom_panel_height() as u32,
            bottom_panel_default_open: layout.bottom_panel_open,
            canvas_surface: canvas_motion_evidence(),
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
