use crate::{
    ui,
    views::{
        plugin_rows::{self, PluginActionRowEvent},
        plugin_status_bar,
    },
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

const PLUGIN_PANEL_GAP: f32 = Space::SM as f32;

impl StudioEguiApp {
    pub(crate) fn render_plugins(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            i18n::t("studio.plugins.title"),
            i18n::t("studio.plugins.detail"),
        );
        self.render_plugin_toolbar(ui);
        ui.add_space(Space::SM as f32);
        self.render_plugin_status(ui);
        ui.add_space(Space::SM as f32);
        self.render_plugin_workspace(ui);
    }

    fn render_plugin_status(&self, ui: &mut egui::Ui) {
        let manager = &self.app.plugin_manager;
        let summary = std_studio::plugin_status::summarize_plugin_status(
            &manager.check_reports,
            &manager.plugin_actions,
            manager.preview.as_ref(),
            manager.last_execution.as_ref(),
        );
        plugin_status_bar::render(ui, &summary);
    }

    fn render_plugin_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        if available_width < 900.0 {
            self.render_plugin_manifests(ui);
            ui.add_space(PLUGIN_PANEL_GAP);
            self.render_plugin_actions(ui);
            ui.add_space(PLUGIN_PANEL_GAP);
            self.render_plugin_inspector(ui);
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

    fn render_plugin_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [ui.available_width() - 230.0, 28.0],
                    egui::TextEdit::singleline(&mut self.plugin_query)
                        .hint_text(i18n::t("studio.plugins.search.hint")),
                );
                if ui::quiet_button(ui, i18n::t("studio.plugins.search")).clicked() {
                    let query = self.plugin_query.clone();
                    let results = self.app.search_plugins(&query);
                    self.status = format!("{} plugin actions", results.len());
                }
                if ui::quiet_button(ui, i18n::t("studio.plugins.reload")).clicked() {
                    self.reload_plugins();
                }
                if ui::quiet_button(ui, i18n::t("studio.plugins.run")).clicked() {
                    self.run_selected_plugin();
                }
            });
        });
    }

    fn render_plugin_manifests(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.plugins.manifests.title"),
                i18n::t("studio.plugins.manifests.detail"),
            );
            if self.app.plugin_manager.manifest_paths.is_empty() {
                ui::empty_state(ui, i18n::t("studio.plugins.manifests.empty"));
            } else {
                egui::ScrollArea::vertical()
                    .max_height(560.0)
                    .show(ui, |ui| {
                        for path in &self.app.plugin_manager.manifest_paths {
                            plugin_rows::manifest_row(ui, path);
                        }
                    });
            }
        });
    }

    fn render_plugin_actions(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.plugins.actions.title"),
                i18n::t("studio.plugins.actions.detail"),
            );
            if self.app.plugin_manager.plugin_actions.is_empty() {
                ui::empty_state(ui, i18n::t("studio.plugins.actions.empty"));
                return;
            }
            let mut clicked_plugin = None;
            egui::ScrollArea::vertical()
                .max_height(560.0)
                .show(ui, |ui| {
                    for (index, result) in self.app.plugin_manager.plugin_actions.iter().enumerate()
                    {
                        let selected = index == self.app.plugin_manager.selected;
                        if let PluginActionRowEvent::Select(index) =
                            plugin_rows::action_row(ui, index, result, selected)
                        {
                            clicked_plugin = Some(index);
                        }
                    }
                });
            if let Some(index) = clicked_plugin {
                self.app.plugin_manager.selected = index;
                self.app.plugin_manager.refresh_preview(&self.app.core);
            }
        });
    }

    fn render_plugin_inspector(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.plugins.security.title"),
                i18n::t("studio.plugins.security.detail"),
            );
            plugin_rows::security_summary_panel(ui, &self.app.plugin_manager.check_reports);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.plugins.checks.title"),
                i18n::t("studio.plugins.checks.detail"),
            );
            self.render_plugin_check_reports(ui);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.plugins.preview.title"),
                i18n::t("studio.plugins.preview.detail"),
            );
            self.render_plugin_preview(ui);
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.plugins.execution.title"),
                i18n::t("studio.plugins.execution.detail"),
            );
            self.render_plugin_execution(ui);
        });
    }

    fn render_plugin_check_reports(&self, ui: &mut egui::Ui) {
        if self.app.plugin_manager.check_reports.is_empty() {
            ui::empty_state(ui, i18n::t("studio.plugins.checks.empty"));
            return;
        }
        egui::ScrollArea::vertical()
            .max_height(190.0)
            .show(ui, |ui| {
                for report in &self.app.plugin_manager.check_reports {
                    plugin_rows::check_report_row(ui, report);
                }
            });
    }

    fn render_plugin_preview(&self, ui: &mut egui::Ui) {
        let Some(preview) = &self.app.plugin_manager.preview else {
            ui::empty_state(ui, i18n::t("studio.plugins.preview.empty"));
            return;
        };
        plugin_rows::preview_panel(ui, preview);
    }

    fn render_plugin_execution(&self, ui: &mut egui::Ui) {
        let Some(execution) = &self.app.plugin_manager.last_execution else {
            ui::empty_state(ui, i18n::t("studio.plugins.execution.empty"));
            return;
        };
        plugin_rows::execution_panel(
            ui,
            &execution.action_name,
            &execution.status,
            &execution.message,
            execution.output.as_ref(),
        );
        if let Some(output) = &execution.output {
            plugin_rows::output_view(ui, output);
        }
    }

    fn reload_plugins(&mut self) {
        match self.app.reload_plugins() {
            Ok(manager) => {
                self.status = format!("{} plugin manifests", manager.manifest_paths.len())
            }
            Err(error) => self.status = error.to_string(),
        }
    }

    fn run_selected_plugin(&mut self) {
        match self.app.run_selected_plugin() {
            Some(execution) => {
                self.status = format!("{} {:?}", execution.action_name, execution.status)
            }
            None => self.status = "no plugin selected".to_string(),
        }
    }
}
