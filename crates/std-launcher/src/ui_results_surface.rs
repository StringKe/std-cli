use crate::ui_parts::{draw_focus_ring, surface_frame};
use eframe::egui;
use std_egui::tokens::FocusRing;
use std_launcher::{LauncherFocusSection, LauncherState};

pub(crate) fn show(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    contents: impl FnOnce(&mut egui::Ui, &mut LauncherState),
) {
    let response = surface_frame(ui.ctx()).show(ui, |ui| contents(ui, state));
    if state.keyboard_focus_visible(LauncherFocusSection::Results) {
        draw_focus_ring(ui, response.response.rect, FocusRing::launcher_results());
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn launcher_results_surface_owns_focus_ring_without_extra_header() {
        let source = include_str!("ui_results_surface.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        assert!(production_source.contains("surface_frame"));
        assert!(production_source.contains("LauncherFocusSection::Results"));
        assert!(production_source.contains("draw_focus_ring"));
        assert!(!production_source.contains("section_header"));
    }
}
