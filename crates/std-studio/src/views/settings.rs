use crate::{ui, StudioEguiApp};
use eframe::egui;

impl StudioEguiApp {
    pub(crate) fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui::section_header(ui, "Settings", "shared configuration and resolved paths");
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_runtime_settings(ui));
            columns[1].vertical(|ui| self.render_storage_settings(ui));
            columns[2].vertical(|ui| self.render_resolved_paths(ui));
        });
    }

    fn render_runtime_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Runtime", "launcher and AI");
            ui.label("Launcher hotkey");
            ui.text_edit_singleline(&mut self.settings_hotkey);
            if ui::quiet_button(ui, "Save Hotkey").clicked() {
                self.save_setting("launcher_hotkey", self.settings_hotkey.clone());
            }
            ui.add_space(8.0);
            ui.checkbox(&mut self.settings_enable_ai, "Enable AI planner");
            if ui::quiet_button(ui, "Save AI").clicked() {
                self.save_setting("enable_ai", self.settings_enable_ai.to_string());
            }
            ui.add_space(8.0);
            ui.label("Theme");
            ui.text_edit_singleline(&mut self.settings_theme);
            if ui::quiet_button(ui, "Save Theme").clicked() {
                self.save_setting("theme", self.settings_theme.clone());
            }
        });
    }

    fn render_storage_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Storage", "config path and data root");
            ui.small(format!("config={}", self.app.config_path().display()));
            ui.label("Data dir");
            ui.text_edit_singleline(&mut self.settings_data_dir);
            if ui::quiet_button(ui, "Save Data Dir").clicked() {
                self.save_setting("data_dir", self.settings_data_dir.clone());
            }
            ui.add_space(8.0);
            ui::chip(
                ui,
                "StdConfig writes and reloads shared core state",
                ui::selected_bg(ui.ctx()),
            );
        });
    }

    fn render_resolved_paths(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Resolved Paths", "current storage layout");
            for (key, value) in self.resolved_paths() {
                ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                    ui.label(egui::RichText::new(key).strong());
                    ui.small(value);
                });
            }
        });
    }

    fn resolved_paths(&self) -> Vec<(&'static str, String)> {
        vec![
            (
                "data_dir",
                self.app.core.config.data_dir.display().to_string(),
            ),
            (
                "workflows_dir",
                self.app.core.config.workflows_dir().display().to_string(),
            ),
            (
                "index_dir",
                self.app.core.config.index_dir().display().to_string(),
            ),
            (
                "memory_dir",
                self.app.core.config.memory_dir().display().to_string(),
            ),
            (
                "history_dir",
                self.app.core.config.history_dir().display().to_string(),
            ),
            (
                "plugins_dir",
                self.app.core.config.plugins_dir().display().to_string(),
            ),
            (
                "apps_dir",
                self.app.core.config.apps_dir().display().to_string(),
            ),
        ]
    }

    fn save_setting(&mut self, key: &str, value: String) {
        match self.app.save_config_field(key, &value) {
            Ok(path) => self.status = format!("saved {}", path.display()),
            Err(error) => self.status = error,
        }
        self.sync_settings_from_app();
    }
}
