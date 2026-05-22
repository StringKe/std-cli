use crate::StudioEguiApp;
use eframe::egui;
use std_egui::tokens::Space;

const PLUGIN_PANEL_GAP: f32 = Space::SM as f32;
const PLUGIN_THREE_COLUMN_MIN_WIDTH: f32 = 900.0;

impl StudioEguiApp {
    pub(crate) fn render_plugin_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < PLUGIN_THREE_COLUMN_MIN_WIDTH {
            self.render_plugin_workspace_stacked(ui);
            return;
        }
        let column_width = (available_width - PLUGIN_PANEL_GAP * 2.0) / 3.0;
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_plugin_manifests(ui),
            );
            ui.add_space(PLUGIN_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_plugin_actions(ui),
            );
            ui.add_space(PLUGIN_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_plugin_inspector(ui),
            );
        });
    }

    fn render_plugin_workspace_stacked(&mut self, ui: &mut egui::Ui) {
        self.render_plugin_manifests(ui);
        ui.add_space(PLUGIN_PANEL_GAP);
        self.render_plugin_actions(ui);
        ui.add_space(PLUGIN_PANEL_GAP);
        self.render_plugin_inspector(ui);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn plugin_workspace_shell_matches_docs22_manager_regions() {
        let source = include_str!("plugin_workspace_shell.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        for required in [
            "PLUGIN_THREE_COLUMN_MIN_WIDTH",
            "render_plugin_manifests",
            "render_plugin_actions",
            "render_plugin_inspector",
            "render_plugin_workspace_stacked",
            "horizontal_top",
        ] {
            assert!(production_source.contains(required), "missing {required}");
        }
    }
}
