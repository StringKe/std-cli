use eframe::egui;

pub(crate) const HOST_CHROME_HEIGHT: f32 = 52.0;
pub(crate) const STATUS_BAR_HEIGHT: f32 = 24.0;
pub(crate) const STATUS_DIVIDER_WIDTH: f32 = 1.0;
pub(crate) const STATUS_DIVIDER_HEIGHT: f32 = 16.0;
pub(crate) const SIDEBAR_DEFAULT_WIDTH: f32 = 240.0;
pub(crate) const SIDEBAR_MIN_WIDTH: f32 = 200.0;
pub(crate) const SIDEBAR_MAX_WIDTH: f32 = 360.0;
pub(crate) const SIDEBAR_COLLAPSED_WIDTH: f32 = 48.0;
pub(crate) const INSPECTOR_DEFAULT_WIDTH: f32 = 320.0;
pub(crate) const INSPECTOR_MIN_WIDTH: f32 = 280.0;
pub(crate) const INSPECTOR_MAX_WIDTH: f32 = 480.0;
pub(crate) const BOTTOM_PANEL_DEFAULT_HEIGHT: f32 = 240.0;
pub(crate) const BOTTOM_PANEL_MIN_HEIGHT: f32 = 160.0;
pub(crate) const BOTTOM_PANEL_MAX_HEIGHT: f32 = 480.0;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StudioLayoutState {
    pub sidebar_open: bool,
    pub inspector_open: bool,
    pub bottom_panel_open: bool,
    pub command_palette_open: bool,
    pub quick_open_open: bool,
    pub command_query: String,
    pub quick_open_query: String,
    pub overlay_selected: usize,
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub bottom_panel_height: f32,
}

impl Default for StudioLayoutState {
    fn default() -> Self {
        Self {
            sidebar_open: true,
            inspector_open: false,
            bottom_panel_open: false,
            command_palette_open: false,
            quick_open_open: false,
            command_query: String::new(),
            quick_open_query: String::new(),
            overlay_selected: 0,
            sidebar_width: SIDEBAR_DEFAULT_WIDTH,
            inspector_width: INSPECTOR_DEFAULT_WIDTH,
            bottom_panel_height: BOTTOM_PANEL_DEFAULT_HEIGHT,
        }
    }
}

impl StudioLayoutState {
    pub(crate) fn handle_keyboard(&mut self, ctx: &egui::Context) {
        if std_egui::input::studio_sidebar_toggle().pressed(ctx) {
            self.sidebar_open = !self.sidebar_open;
        }
        if std_egui::input::studio_inspector_toggle().pressed(ctx) {
            self.inspector_open = !self.inspector_open;
        }
        if std_egui::input::studio_bottom_panel_toggle().pressed(ctx) {
            self.bottom_panel_open = !self.bottom_panel_open;
        }
        if std_egui::input::studio_command_palette_slash().pressed(ctx)
            || std_egui::input::studio_command_palette().pressed(ctx)
        {
            self.open_command_palette();
        } else if std_egui::input::studio_quick_open().pressed(ctx) {
            self.open_quick_open();
        }
    }

    pub(crate) fn open_command_palette(&mut self) {
        self.command_palette_open = true;
        self.quick_open_open = false;
        self.command_query.clear();
        self.quick_open_query.clear();
        self.overlay_selected = 0;
    }

    pub(crate) fn open_quick_open(&mut self) {
        self.quick_open_open = true;
        self.command_palette_open = false;
        self.command_query.clear();
        self.quick_open_query.clear();
        self.overlay_selected = 0;
    }

    pub(crate) fn close_overlays(&mut self) {
        self.command_palette_open = false;
        self.quick_open_open = false;
        self.command_query.clear();
        self.quick_open_query.clear();
        self.overlay_selected = 0;
    }

    pub(crate) fn open_bottom_panel(&mut self) {
        self.bottom_panel_open = true;
    }

    pub(crate) fn move_overlay_selection(&mut self, delta: isize, len: usize) {
        self.overlay_selected = crate::commands::move_selection(self.overlay_selected, delta, len);
    }

    pub(crate) fn clamp_overlay_selection(&mut self, len: usize) {
        self.overlay_selected = crate::commands::move_selection(self.overlay_selected, 0, len);
    }

    pub(crate) fn sidebar_width(&self) -> f32 {
        if self.sidebar_open {
            self.sidebar_width
                .clamp(SIDEBAR_MIN_WIDTH, SIDEBAR_MAX_WIDTH)
        } else {
            SIDEBAR_COLLAPSED_WIDTH
        }
    }

    pub(crate) fn inspector_width(&self) -> f32 {
        self.inspector_width
            .clamp(INSPECTOR_MIN_WIDTH, INSPECTOR_MAX_WIDTH)
    }

    pub(crate) fn bottom_panel_height(&self) -> f32 {
        self.bottom_panel_height
            .clamp(BOTTOM_PANEL_MIN_HEIGHT, BOTTOM_PANEL_MAX_HEIGHT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_layout_defaults_match_docs() {
        let layout = StudioLayoutState::default();

        assert!(layout.sidebar_open);
        assert!(!layout.inspector_open);
        assert!(!layout.bottom_panel_open);
        assert!(!layout.command_palette_open);
        assert!(!layout.quick_open_open);
        assert!(layout.command_query.is_empty());
        assert!(layout.quick_open_query.is_empty());
        assert_eq!(layout.overlay_selected, 0);
        assert_eq!(layout.sidebar_width(), 240.0);
        assert_eq!(layout.inspector_width(), 320.0);
        assert_eq!(layout.bottom_panel_height(), 240.0);
        assert_eq!(HOST_CHROME_HEIGHT, 52.0);
        assert_eq!(STATUS_BAR_HEIGHT, 24.0);
        assert_eq!(STATUS_DIVIDER_HEIGHT, 16.0);
    }

    #[test]
    fn studio_layout_collapsed_sidebar_keeps_icon_rail() {
        let layout = StudioLayoutState {
            sidebar_open: false,
            sidebar_width: 320.0,
            ..StudioLayoutState::default()
        };

        assert_eq!(layout.sidebar_width(), 48.0);
    }

    #[test]
    fn studio_layout_clamps_resizable_regions() {
        let layout = StudioLayoutState {
            sidebar_width: 100.0,
            inspector_width: 900.0,
            bottom_panel_height: 80.0,
            ..StudioLayoutState::default()
        };

        assert_eq!(layout.sidebar_width(), 200.0);
        assert_eq!(layout.inspector_width(), 480.0);
        assert_eq!(layout.bottom_panel_height(), 160.0);
    }

    #[test]
    fn studio_layout_opens_one_command_overlay_at_a_time() {
        let mut layout = StudioLayoutState::default();

        layout.open_quick_open();
        layout.quick_open_query = "plugin".to_string();
        layout.overlay_selected = 1;
        assert!(layout.quick_open_open);
        assert!(!layout.command_palette_open);

        layout.open_command_palette();
        assert!(layout.command_palette_open);
        assert!(!layout.quick_open_open);
        assert!(layout.quick_open_query.is_empty());
        assert!(layout.command_query.is_empty());
        assert_eq!(layout.overlay_selected, 0);

        layout.close_overlays();
        assert!(!layout.command_palette_open);
        assert!(!layout.quick_open_open);
        assert_eq!(layout.overlay_selected, 0);
    }

    #[test]
    fn studio_layout_can_open_batch_debug_panel_from_workflow_actions() {
        let mut layout = StudioLayoutState::default();

        layout.open_bottom_panel();

        assert!(layout.bottom_panel_open);
        assert_eq!(layout.bottom_panel_height(), 240.0);
    }

    #[test]
    fn studio_layout_moves_overlay_selection_without_wrap() {
        let mut layout = StudioLayoutState::default();

        layout.move_overlay_selection(-1, 3);
        assert_eq!(layout.overlay_selected, 0);
        layout.move_overlay_selection(1, 3);
        assert_eq!(layout.overlay_selected, 1);
        layout.move_overlay_selection(9, 3);
        assert_eq!(layout.overlay_selected, 2);
    }

    #[test]
    fn studio_layout_clamps_selection_after_filter_changes() {
        let mut layout = StudioLayoutState {
            overlay_selected: 4,
            ..StudioLayoutState::default()
        };

        layout.clamp_overlay_selection(2);
        assert_eq!(layout.overlay_selected, 1);

        layout.clamp_overlay_selection(0);
        assert_eq!(layout.overlay_selected, 0);
    }
}
