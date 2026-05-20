use crate::{ui, StudioEguiApp};
use eframe::egui;
use std_egui::i18n;

impl StudioEguiApp {
    pub(crate) fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.settings.title"),
            i18n::t("studio.settings.detail"),
        );
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_runtime_settings(ui));
            columns[1].vertical(|ui| self.render_storage_settings(ui));
            columns[2].vertical(|ui| self.render_resolved_paths(ui));
        });
    }

    fn render_runtime_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.settings.runtime.title"),
                i18n::t("studio.settings.runtime.detail"),
            );
            ui.label(i18n::t("studio.settings.hotkey.label"));
            ui.text_edit_singleline(&mut self.settings_hotkey);
            if ui::quiet_button(ui, i18n::t("studio.settings.hotkey.save")).clicked() {
                self.save_setting("launcher_hotkey", self.settings_hotkey.clone());
            }
            ui.add_space(8.0);
            ui.checkbox(
                &mut self.settings_enable_ai,
                i18n::t("studio.settings.ai.enable"),
            );
            if ui::quiet_button(ui, i18n::t("studio.settings.ai.save")).clicked() {
                self.save_setting("enable_ai", self.settings_enable_ai.to_string());
            }
            ui.add_space(8.0);
            ui.label(i18n::t("studio.settings.theme.label"));
            ui.text_edit_singleline(&mut self.settings_theme);
            if ui::quiet_button(ui, i18n::t("studio.settings.theme.save")).clicked() {
                self.save_setting("theme", self.settings_theme.clone());
            }
        });
    }

    fn render_storage_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.settings.storage.title"),
                i18n::t("studio.settings.storage.detail"),
            );
            ui.small(format!("config={}", self.app.config_path().display()));
            ui.label(i18n::t("studio.settings.data_dir.label"));
            ui.text_edit_singleline(&mut self.settings_data_dir);
            if ui::quiet_button(ui, i18n::t("studio.settings.data_dir.save")).clicked() {
                self.save_setting("data_dir", self.settings_data_dir.clone());
            }
            ui.add_space(8.0);
            ui::chip(
                ui,
                i18n::t("studio.settings.storage.note"),
                ui::selected_bg(ui.ctx()),
            );
        });
    }

    fn render_resolved_paths(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.settings.paths.title"),
                i18n::t("studio.settings.paths.detail"),
            );
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
            Ok(path) => {
                self.status = format!("{} {}", i18n::t("studio.settings.saved"), path.display())
            }
            Err(error) => self.status = error,
        }
        self.sync_settings_from_app();
    }
}
