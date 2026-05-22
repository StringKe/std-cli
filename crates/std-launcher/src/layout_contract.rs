use std_egui::tokens::{LauncherSize, UiScale};

pub const PANEL_WIDTH: f32 = LauncherSize::PANEL_WIDTH;

pub fn panel_surface_width(scale: f32) -> f32 {
    LauncherSize::panel_surface_width(UiScale::new(scale))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_surface_width_matches_docs_panel_width() {
        assert_eq!(panel_surface_width(1.0), 720.0);
        assert_eq!(panel_surface_width(1.5), 1080.0);
    }
}
