use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StudioLayoutState {
    pub sidebar_open: bool,
    pub inspector_open: bool,
    pub bottom_panel_open: bool,
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
        });
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
}
