use crate::{ui, StudioEguiApp};
use eframe::egui;

impl StudioEguiApp {
    pub(crate) fn render_plugins(&mut self, ui: &mut egui::Ui) {
        ui::section_header(
            ui,
            "Plugin Manager",
            "manifest checks, scoped permissions, JS/TS execution",
        );
        self.render_plugin_toolbar(ui);
        ui.add_space(10.0);
        ui.columns(3, |columns| {
            columns[0].vertical(|ui| self.render_plugin_manifests(ui));
            columns[1].vertical(|ui| self.render_plugin_actions(ui));
            columns[2].vertical(|ui| self.render_plugin_inspector(ui));
        });
    }

    fn render_plugin_toolbar(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add_sized(
                    [ui.available_width() - 230.0, 28.0],
                    egui::TextEdit::singleline(&mut self.plugin_query)
                        .hint_text("plugin action, tag, manifest"),
                );
                if ui::quiet_button(ui, "Search").clicked() {
                    let query = self.plugin_query.clone();
                    let results = self.app.search_plugins(&query);
                    self.status = format!("{} plugin actions", results.len());
                }
                if ui::quiet_button(ui, "Reload").clicked() {
                    self.reload_plugins();
                }
                if ui::quiet_button(ui, "Run").clicked() {
                    self.run_selected_plugin();
                }
            });
        });
    }

    fn render_plugin_manifests(&self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Manifests", "discovered plugin.json");
            if self.app.plugin_manager.manifest_paths.is_empty() {
                ui::empty_state(ui, "No plugin manifests");
            } else {
                egui::ScrollArea::vertical()
                    .max_height(560.0)
                    .show(ui, |ui| {
                        for path in &self.app.plugin_manager.manifest_paths {
                            ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                                ui.label(
                                    path.file_stem()
                                        .and_then(|name| name.to_str())
                                        .unwrap_or("plugin"),
                                );
                                ui.small(path.display().to_string());
                            });
                        }
                    });
            }
        });
    }

    fn render_plugin_actions(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Actions", "preview before execution");
            if self.app.plugin_manager.plugin_actions.is_empty() {
                ui::empty_state(ui, "No plugin actions");
                return;
            }
            let mut clicked_plugin = None;
            egui::ScrollArea::vertical()
                .max_height(560.0)
                .show(ui, |ui| {
                    for (index, result) in self.app.plugin_manager.plugin_actions.iter().enumerate()
                    {
                        let selected = index == self.app.plugin_manager.selected;
                        ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                            if ui.selectable_label(selected, &result.action.name).clicked() {
                                clicked_plugin = Some(index);
                            }
                            ui.small(&result.action.description);
                            ui.horizontal_wrapped(|ui| {
                                for field in &result.matched_fields {
                                    ui::chip(ui, field, ui::panel_alt(ui.ctx()));
                                }
                            });
                        });
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
            ui::section_header(ui, "Checks", "permissions and scopes");
            self.render_plugin_check_reports(ui);
        });
        ui.add_space(10.0);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Preview", "selected action");
            self.render_plugin_preview(ui);
        });
        ui.add_space(10.0);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(ui, "Execution", "last controlled run");
            self.render_plugin_execution(ui);
        });
    }

    fn render_plugin_check_reports(&self, ui: &mut egui::Ui) {
        if self.app.plugin_manager.check_reports.is_empty() {
            ui::empty_state(ui, "No manifest check reports");
            return;
        }
        egui::ScrollArea::vertical()
            .max_height(190.0)
            .show(ui, |ui| {
                for report in &self.app.plugin_manager.check_reports {
                    ui::subtle_frame(ui.ctx()).show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui::chip(ui, report.status, ui::ok_bg(ui.ctx()));
                            ui.label(format!("{} actions={}", report.plugin_name, report.actions));
                        });
                        ui.small(format!(
                            "permissions={}",
                            report
                                .permissions
                                .iter()
                                .map(|permission| format!("{permission:?}"))
                                .collect::<Vec<_>>()
                                .join(",")
                        ));
                        ui.small(format!(
                            "fs_scopes={} network_hosts={}",
                            report.fs_scopes.len(),
                            report.network_hosts.len()
                        ));
                    });
                }
            });
    }

    fn render_plugin_preview(&self, ui: &mut egui::Ui) {
        let Some(preview) = &self.app.plugin_manager.preview else {
            ui::empty_state(ui, "No action selected");
            return;
        };
        ui.label(&preview.title);
        ui.small(&preview.primary_command);
        ui.horizontal(|ui| {
            ui::chip(
                ui,
                &format!("{:?}", preview.action_type),
                ui::selected_bg(ui.ctx()),
            );
            ui.label(format!("examples={}", preview.examples.len()));
        });
        if !preview.metadata.is_empty() {
            for (key, value) in &preview.metadata {
                ui.small(format!("{key}={value}"));
            }
        }
    }

    fn render_plugin_execution(&self, ui: &mut egui::Ui) {
        let Some(execution) = &self.app.plugin_manager.last_execution else {
            ui::empty_state(ui, "No execution yet");
            return;
        };
        ui.horizontal(|ui| {
            ui::chip(
                ui,
                &format!("{:?}", execution.status),
                plugin_status_fill(ui.ctx(), &execution.status),
            );
            ui.label(&execution.action_name);
        });
        ui.label(&execution.message);
        if let Some(output) = &execution.output {
            ui.add_sized(
                [ui.available_width(), 120.0],
                egui::TextEdit::multiline(&mut output.to_string()).interactive(false),
            );
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

fn plugin_status_fill(
    ctx: &egui::Context,
    status: &std_types::ActionExecutionStatus,
) -> egui::Color32 {
    match status {
        std_types::ActionExecutionStatus::Completed => ui::ok_bg(ctx),
        std_types::ActionExecutionStatus::Failed => ui::warn_bg(ctx),
        std_types::ActionExecutionStatus::NeedsExternalRunner => ui::warn_bg(ctx),
    }
}
