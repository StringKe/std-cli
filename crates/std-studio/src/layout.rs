use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StudioLayoutState {
    pub sidebar_open: bool,
    pub inspector_open: bool,
    pub bottom_panel_open: bool,
    pub settings_open: bool,
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
            settings_open: false,
            command_palette_open: false,
            quick_open_open: false,
            command_query: String::new(),
            quick_open_query: String::new(),
            overlay_selected: 0,
            sidebar_width: 240.0,
            inspector_width: 320.0,
            bottom_panel_height: 240.0,
        }
    }
}

impl StudioLayoutState {
    pub(crate) fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|input| {
            if input.modifiers.command && input.key_pressed(egui::Key::B) {
                self.sidebar_open = !self.sidebar_open;
            }
            if input.modifiers.command && input.key_pressed(egui::Key::I) {
                self.inspector_open = !self.inspector_open;
            }
            if input.modifiers.command && input.key_pressed(egui::Key::J) {
                self.bottom_panel_open = !self.bottom_panel_open;
            }
            if input.modifiers.command && input.key_pressed(egui::Key::Comma) {
                self.open_settings();
            }
            if input.modifiers.command && input.key_pressed(egui::Key::Slash) {
                self.open_command_palette();
            }
            if input.modifiers.command && input.modifiers.shift && input.key_pressed(egui::Key::P) {
                self.open_command_palette();
            } else if input.modifiers.command
                && !input.modifiers.shift
                && input.key_pressed(egui::Key::P)
            {
                self.open_quick_open();
            }
        });
    }

    pub(crate) fn open_settings(&mut self) {
        self.settings_open = true;
        self.command_palette_open = false;
        self.quick_open_open = false;
        self.command_query.clear();
        self.quick_open_query.clear();
        self.overlay_selected = 0;
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
        self.settings_open = false;
        self.command_palette_open = false;
        self.quick_open_open = false;
        self.command_query.clear();
        self.quick_open_query.clear();
        self.overlay_selected = 0;
    }

    pub(crate) fn move_overlay_selection(&mut self, delta: isize, len: usize) {
        self.overlay_selected = crate::commands::move_selection(self.overlay_selected, delta, len);
    }

    pub(crate) fn clamp_overlay_selection(&mut self, len: usize) {
        self.overlay_selected = crate::commands::move_selection(self.overlay_selected, 0, len);
    }

    pub(crate) fn sidebar_width(&self) -> f32 {
        if self.sidebar_open {
            self.sidebar_width.clamp(200.0, 360.0)
        } else {
            48.0
        }
    }

    pub(crate) fn inspector_width(&self) -> f32 {
        self.inspector_width.clamp(280.0, 480.0)
    }

    pub(crate) fn bottom_panel_height(&self) -> f32 {
        self.bottom_panel_height.clamp(160.0, 480.0)
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
        assert!(!layout.settings_open);
        assert!(!layout.command_palette_open);
        assert!(!layout.quick_open_open);
        assert!(layout.command_query.is_empty());
        assert!(layout.quick_open_query.is_empty());
        assert_eq!(layout.overlay_selected, 0);
        assert_eq!(layout.sidebar_width(), 240.0);
        assert_eq!(layout.inspector_width(), 320.0);
        assert_eq!(layout.bottom_panel_height(), 240.0);
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

        layout.open_settings();
        assert!(layout.settings_open);
        assert!(!layout.command_palette_open);
        assert!(!layout.quick_open_open);
        assert!(layout.command_query.is_empty());

        layout.close_overlays();
        assert!(!layout.settings_open);
        assert!(!layout.command_palette_open);
        assert!(!layout.quick_open_open);
        assert_eq!(layout.overlay_selected, 0);
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
