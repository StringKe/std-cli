use crate::{
    ui,
    views::{
        settings_binding,
        settings_model::SettingsCategory,
        settings_rows::{self, SettingsCategoryEvent},
        settings_toggle::{self, ToggleRowEvent},
    },
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

const SETTINGS_NAV_WIDTH: f32 = 240.0;
const SETTINGS_PANEL_GAP: f32 = Space::MD as f32;

impl StudioEguiApp {
    pub(crate) fn render_settings(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.settings.title"),
            i18n::t("studio.settings.detail"),
        );
        self.render_settings_workspace(ui);
    }

    fn render_settings_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 760.0 {
            self.render_settings_category_rail(ui);
            ui.add_space(Space::SM as f32);
            self.render_selected_settings_category(ui);
            return;
        }
        let content_width = (available_width - SETTINGS_NAV_WIDTH - SETTINGS_PANEL_GAP).max(360.0);
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(SETTINGS_NAV_WIDTH, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_settings_category_rail(ui),
            );
            ui.add_space(SETTINGS_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(content_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_selected_settings_category(ui),
            );
        });
    }

    fn render_settings_category_rail(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.settings.nav.title"),
                i18n::t("studio.settings.nav.detail"),
            );
            for category in SettingsCategory::ALL {
                if let SettingsCategoryEvent::Select(category) =
                    settings_rows::category_row(ui, category, category == self.settings_category)
                {
                    self.settings_category = category;
                }
            }
        });
    }

    fn render_selected_settings_category(&mut self, ui: &mut egui::Ui) {
        match self.settings_category {
            SettingsCategory::Appearance => self.render_appearance_settings(ui),
            SettingsCategory::Hotkeys => self.render_hotkey_settings(ui),
            SettingsCategory::AiProvider => self.render_ai_settings(ui),
            SettingsCategory::Index => self.render_index_settings(ui),
            SettingsCategory::Plugins => self.render_plugin_settings(ui),
            SettingsCategory::Privacy => self.render_privacy_settings(ui),
            SettingsCategory::About => self.render_about_settings(ui),
        }
    }

    fn render_appearance_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::Appearance);
            ui.label(i18n::t("studio.settings.theme.label"));
            if let Some(theme) = settings_rows::theme_mode_control(ui, &self.settings_theme) {
                self.settings_theme = theme.to_string();
                self.save_setting("theme", self.settings_theme.clone());
            }
            ui.add_space(Space::SM as f32);
            ui::chip(
                ui,
                i18n::t("studio.settings.theme.contract"),
                ui::selected_bg(ui.ctx()),
            );
            ui.add_space(Space::SM as f32);
            self.render_theme_profile(ui);
            ui.add_space(Space::SM as f32);
            if let ToggleRowEvent::Toggle(enabled) = settings_toggle::toggle_row(
                ui,
                i18n::t("studio.settings.motion.reduce"),
                i18n::t("studio.settings.motion.reduce.detail"),
                self.settings_reduce_motion,
            ) {
                self.settings_reduce_motion = enabled;
                self.save_setting(
                    "appearance.reduce_motion",
                    self.settings_reduce_motion.to_string(),
                );
            }
        });
    }

    fn render_theme_profile(&self, ui: &mut egui::Ui) {
        let Some(profile) = self.theme_profile else {
            return;
        };
        ui.label(format!(
            "{} {:?} / {:?}",
            i18n::t("studio.settings.theme.active"),
            profile.requested,
            profile.effective
        ));
    }

    fn render_hotkey_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::Hotkeys);
            let binding = settings_binding::binding_editor_row(
                ui,
                i18n::t("studio.settings.hotkey.label"),
                i18n::t("studio.settings.hotkey.contract"),
                i18n::t("studio.settings.hotkey.save"),
                &mut self.settings_hotkey,
            );
            if binding.save_clicked {
                self.save_setting("launcher_hotkey", self.settings_hotkey.clone());
            }
            ui.add_space(Space::SM as f32);
            ui::chip(
                ui,
                i18n::t("studio.settings.hotkey.contract"),
                ui::selected_bg(ui.ctx()),
            );
            ui.add_space(Space::SM as f32);
            ui::section_header(
                ui,
                i18n::t("studio.settings.hotkey.registry.title"),
                i18n::t("studio.settings.hotkey.registry.detail"),
            );
            for shortcut in std_core::shortcuts::shortcut_registry(&self.app.core.config) {
                if let settings_rows::ShortcutRowEvent::Reset("launcher.global.toggle") =
                    settings_rows::shortcut_row(ui, &shortcut)
                {
                    self.settings_hotkey = shortcut.default_binding.to_string();
                    self.save_setting("launcher_hotkey", self.settings_hotkey.clone());
                }
            }
        });
    }

    fn render_ai_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::AiProvider);
            if let ToggleRowEvent::Toggle(enabled) = settings_toggle::toggle_row(
                ui,
                i18n::t("studio.settings.ai.enable"),
                i18n::t("studio.settings.ai.detail"),
                self.settings_enable_ai,
            ) {
                self.settings_enable_ai = enabled;
                self.save_setting("enable_ai", self.settings_enable_ai.to_string());
            }
        });
    }

    fn render_index_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::Index);
            self.render_resolved_paths(ui);
        });
    }

    fn render_plugin_settings(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::Plugins);
            settings_rows::config_path_row(ui, &self.app.config_path().display().to_string());
            let storage = settings_binding::binding_editor_row(
                ui,
                i18n::t("studio.settings.data_dir.label"),
                i18n::t("studio.settings.storage.note"),
                i18n::t("studio.settings.data_dir.save"),
                &mut self.settings_data_dir,
            );
            if storage.save_clicked {
                self.save_setting("data_dir", self.settings_data_dir.clone());
            }
            ui.add_space(Space::SM as f32);
            ui::chip(
                ui,
                i18n::t("studio.settings.storage.note"),
                ui::selected_bg(ui.ctx()),
            );
        });
    }

    fn render_privacy_settings(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::Privacy);
            ui::chip(
                ui,
                i18n::t("studio.settings.privacy.contract"),
                ui::selected_bg(ui.ctx()),
            );
        });
    }

    fn render_about_settings(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            self.render_category_header(ui, SettingsCategory::About);
            ui.label(i18n::t("studio.settings.about.product"));
            ui.label(i18n::t("studio.settings.about.surface"));
        });
    }

    fn render_category_header(&self, ui: &mut egui::Ui, category: SettingsCategory) {
        ui::section_header(
            ui,
            i18n::t(category.title_key()),
            i18n::t(category.detail_key()),
        );
    }

    fn render_resolved_paths(&self, ui: &mut egui::Ui) {
        for (key, value) in self.resolved_paths() {
            settings_rows::resolved_path_row(ui, key, &value);
        }
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
