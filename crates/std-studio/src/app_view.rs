use crate::{
    app_rows::{self, AppRowEvent},
    studio_metrics, ui, StudioEguiApp,
};
use eframe::egui;
use std::path::PathBuf;
use std_egui::{i18n, tokens::Space};

const APP_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_apps(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.apps.title"),
            i18n::t("studio.apps.detail"),
        );
        self.render_apps_workspace(ui);
    }

    fn render_apps_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < studio_metrics::WIDE_WORKSPACE_BREAKPOINT {
            self.render_app_registration(ui);
            ui.add_space(APP_PANEL_GAP);
            self.render_app_search(ui);
            ui.add_space(APP_PANEL_GAP);
            self.render_registered_apps(ui);
            return;
        }
        let column_width = studio_metrics::thirds_column_width(available_width, APP_PANEL_GAP);
        ui.horizontal_top(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_app_registration(ui),
            );
            ui.add_space(APP_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_app_search(ui),
            );
            ui.add_space(APP_PANEL_GAP);
            ui.allocate_ui_with_layout(
                egui::vec2(column_width, 0.0),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_registered_apps(ui),
            );
        });
    }

    fn render_app_registration(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.apps.register.title"),
                i18n::t("studio.apps.register.detail"),
            );
            app_text_input(
                ui,
                i18n::t("studio.apps.bundle_path"),
                &mut self.app_bundle_path,
            );
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.apps.register")).clicked() {
                    self.register_app_bundle();
                }
                if ui::quiet_button(ui, i18n::t("studio.apps.use_fixture_app")).clicked() {
                    self.app_bundle_path = "/tmp/StdFixture.app".to_string();
                }
            });
            app_rows::storage_row(ui, &self.app.core.config.apps_dir());
        });
    }

    fn render_app_search(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.apps.search.title"),
                i18n::t("studio.apps.search.detail"),
            );
            app_text_input(ui, i18n::t("studio.apps.search.title"), &mut self.app_query);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.apps.search")).clicked() {
                    self.search_apps_status();
                }
                if ui::quiet_button(ui, i18n::t("studio.apps.preview")).clicked() {
                    self.preview_app_status();
                }
                if ui::quiet_button(ui, i18n::t("studio.apps.trigger")).clicked() {
                    self.trigger_app_status();
                }
            });
            ui::chip(
                ui,
                i18n::t("studio.apps.external_defer"),
                ui::warn_bg(ui.ctx()),
            );
            self.render_app_search_results(ui);
        });
    }

    fn render_registered_apps(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.apps.registered.title"),
                i18n::t("studio.apps.registered.detail"),
            );
            match self.app.registered_apps() {
                Ok(apps) if apps.is_empty() => {
                    ui::empty_state(ui, i18n::t("studio.apps.registered.empty"))
                }
                Ok(apps) => {
                    if let Some(name) = render_registered_app_rows(ui, apps) {
                        self.app_query = name;
                    }
                }
                Err(error) => {
                    ui.label(error.to_string());
                }
            }
        });
    }

    fn register_app_bundle(&mut self) {
        let source = PathBuf::from(&self.app_bundle_path);
        match self.app.register_app_bundle(&source) {
            Ok(path) => self.status = format!("registered {}", path.display()),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn search_apps_status(&mut self) {
        match self.app.search_apps(&self.app_query, 20) {
            Ok(results) => self.status = format!("{} apps", results.len()),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn preview_app_status(&mut self) {
        match self.app.preview_app(&self.app_query) {
            Ok(Some(preview)) => self.status = preview.primary_command,
            Ok(None) => self.status = "no app selected".to_string(),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn trigger_app_status(&mut self) {
        match self.app.trigger_app(&self.app_query) {
            Ok(Some(execution)) => {
                self.status = format!("{} {:?}", execution.action_name, execution.status);
            }
            Ok(None) => self.status = "no app selected".to_string(),
            Err(error) => self.status = error.to_string(),
        }
    }

    fn render_app_search_results(&mut self, ui: &mut egui::Ui) {
        let Ok(results) = self.app.search_apps(&self.app_query, 20) else {
            return;
        };
        if results.is_empty() {
            ui::empty_state(ui, i18n::t("studio.apps.matches.empty"));
            return;
        }
        egui::ScrollArea::vertical()
            .max_height(studio_metrics::SEARCH_RESULTS_MAX_HEIGHT)
            .show(ui, |ui| {
                for result in results {
                    if let AppRowEvent::Select(name) = app_rows::search_result_row(ui, &result) {
                        self.app_query = name;
                    }
                }
            });
    }
}

fn render_registered_app_rows(ui: &mut egui::Ui, apps: Vec<PathBuf>) -> Option<String> {
    let mut selected_name = None;
    egui::ScrollArea::vertical()
        .max_height(studio_metrics::PANEL_LIST_MAX_HEIGHT)
        .show(ui, |ui| {
            for path in apps {
                if let AppRowEvent::Select(name) = app_rows::registered_app_row(ui, &path) {
                    selected_name = Some(name);
                }
            }
        });
    selected_name
}

fn app_text_input(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.label(label);
    let response = ui.add_sized(
        [ui.available_width(), studio_metrics::INPUT_HEIGHT],
        egui::TextEdit::singleline(value),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            app_input_a11y_label(label, value),
        )
    });
}

fn app_input_a11y_label(label: &str, value: &str) -> String {
    let value = if value.trim().is_empty() {
        "empty"
    } else {
        value.trim()
    };
    format!("{label}, text box, value {value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_inputs_use_text_edit_widget_info() {
        let source = include_str!("app_view.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("app_text_input"));
        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("app_input_a11y_label"));
        assert!(implementation.contains("TextEdit::singleline"));
        assert!(!implementation.contains("ui.text_edit_singleline"));
    }

    #[test]
    fn app_input_a11y_label_exposes_value() {
        assert_eq!(
            app_input_a11y_label("Bundle path", "/tmp/StdFixture.app"),
            "Bundle path, text box, value /tmp/StdFixture.app"
        );
        assert_eq!(
            app_input_a11y_label("Search apps", " "),
            "Search apps, text box, value empty"
        );
    }
}
