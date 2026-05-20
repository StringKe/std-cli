use crate::{ui, StudioEguiApp};
use eframe::egui;
use std::path::{Path, PathBuf};
use std_egui::i18n;

impl StudioEguiApp {
    pub(crate) fn render_apps(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.apps.title"),
            i18n::t("studio.apps.detail"),
        );
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_app_registration(ui));
            columns[1].vertical(|ui| self.render_app_search(ui));
            columns[2].vertical(|ui| self.render_registered_apps(ui));
        });
    }

    fn render_app_registration(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.apps.register.title"),
                i18n::t("studio.apps.register.detail"),
            );
            ui.label(i18n::t("studio.apps.bundle_path"));
            ui.text_edit_singleline(&mut self.app_bundle_path);
            ui.horizontal(|ui| {
                if ui::quiet_button(ui, i18n::t("studio.apps.register")).clicked() {
                    self.register_app_bundle();
                }
                if ui::quiet_button(ui, i18n::t("studio.apps.use_wechat")).clicked() {
                    self.app_bundle_path = "/Applications/WeChat.app".to_string();
                }
            });
            ui.small(format!(
                "storage={}",
                self.app.core.config.apps_dir().display()
            ));
        });
    }

    fn render_app_search(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.apps.search.title"),
                i18n::t("studio.apps.search.detail"),
            );
            ui.text_edit_singleline(&mut self.app_query);
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
                Ok(apps) => render_registered_app_rows(ui, apps, &mut self.app_query),
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
            .max_height(430.0)
            .show(ui, |ui| {
                for result in results {
                    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                        if ui.button(&result.action.name).clicked() {
                            self.app_query = result.action.name.replace("Open App: ", "");
                        }
                        ui.horizontal_wrapped(|ui| {
                            ui::chip(
                                ui,
                                &format!("score={:.2}", result.score),
                                ui::panel_alt(ui.ctx()),
                            );
                            ui::chip(ui, "NeedsExternalRunner", ui::warn_bg(ui.ctx()));
                            for field in &result.matched_fields {
                                ui::chip(ui, field, ui::selected_bg(ui.ctx()));
                            }
                        });
                        ui.small(&result.action.description);
                    });
                }
            });
    }
}

fn render_registered_app_rows(ui: &mut egui::Ui, apps: Vec<PathBuf>, query: &mut String) {
    egui::ScrollArea::vertical()
        .max_height(560.0)
        .show(ui, |ui| {
            for path in apps {
                let name = app_name(&path);
                ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                    ui.label(&name);
                    ui.small(path.display().to_string());
                    if ui::quiet_button(ui, i18n::t("studio.apps.select")).clicked() {
                        *query = name;
                    }
                });
            }
        });
}

fn app_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("App")
        .to_string()
}
