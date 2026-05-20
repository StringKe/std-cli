use crate::{
    shell_parts::{panel_frame, path_label},
    ui, StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::StudioPane;

impl StudioEguiApp {
    pub(crate) fn render_shell(&mut self, ctx: &egui::Context) {
        self.handle_overlay_keyboard(ctx);
        egui::TopBottomPanel::top("studio_app_chrome")
            .exact_height(52.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_app_chrome(ui));
        egui::SidePanel::left("studio_nav")
            .resizable(self.layout.sidebar_open)
            .default_width(self.layout.sidebar_width())
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_navigation(ui));
        if self.layout.inspector_open {
            egui::SidePanel::right("studio_context")
                .resizable(true)
                .default_width(self.layout.inspector_width())
                .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
                .show(ctx, |ui| self.render_context(ui));
        }
        if self.layout.bottom_panel_open {
            egui::TopBottomPanel::bottom("studio_bottom_panel")
                .resizable(true)
                .default_height(self.layout.bottom_panel_height())
                .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
                .show(ctx, |ui| self.render_bottom_panel(ui));
        }
        egui::TopBottomPanel::bottom("studio_status")
            .exact_height(24.0)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_status_bar(ui));
        egui::CentralPanel::default()
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_0(ctx)))
            .show(ctx, |ui| self.render_active_workspace(ui));
        self.render_overlays(ctx);
    }

    fn render_active_workspace(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| match self.app.active_pane {
                StudioPane::Dashboard => self.render_dashboard(ui),
                StudioPane::Workflows => self.render_workflows(ui),
                StudioPane::Apps => self.render_apps(ui),
                StudioPane::Memory => self.render_memory(ui),
                StudioPane::Plugins => self.render_plugins(ui),
                StudioPane::Analysis => self.render_analysis(ui),
                StudioPane::History => self.render_history(ui),
                StudioPane::Operations => self.render_operations(ui),
                StudioPane::Settings => self.render_settings(ui),
            });
        });
        self.render_workspace_panes(ui);
    }

    fn render_context(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.shell.context.title"),
                i18n::t("studio.shell.context.detail"),
            );
            ui::metric(ui, "Actions", self.app.dashboard.action_count, "registered");
            ui.add_space(Space::XS as f32);
            ui::metric(ui, "Memory", self.app.dashboard.memory_count, "records");
            ui.add_space(Space::XS as f32);
            ui::metric(
                ui,
                "Audit Events",
                self.app.dashboard.audit_event_count,
                "recent local trail",
            );
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.shell.runtime.title"),
                i18n::t("studio.shell.runtime.detail"),
            );
            path_label(ui, "Config", self.app.config_path().display().to_string());
            path_label(
                ui,
                "Data",
                self.app.core.config.data_dir.display().to_string(),
            );
            path_label(
                ui,
                "Workflows",
                self.app.core.config.workflows_dir().display().to_string(),
            );
        });
        ui.add_space(Space::SM as f32);
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.shell.status.title"),
                i18n::t("studio.shell.status.detail"),
            );
            if self.status.is_empty() {
                ui.label(
                    egui::RichText::new(i18n::t("studio.shell.idle"))
                        .color(ui::muted_text(ui.ctx())),
                );
            } else {
                ui.label(egui::RichText::new(&self.status).color(ui::strong_text(ui.ctx())));
            }
        });
    }

    fn render_status_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label(
                egui::RichText::new(self.app.active_pane.content_key())
                    .color(ui::muted_text(ui.ctx())),
            );
            ui.separator();
            ui.label(format!(
                "{} {}",
                self.app.open_workspace_panes().count(),
                i18n::t("studio.shell.panes")
            ));
            ui.separator();
            ui.label(if self.layout.inspector_open {
                i18n::t("studio.shell.inspector")
            } else {
                i18n::t("studio.shell.inspector_hidden")
            });
            ui.separator();
            ui.label(if self.layout.bottom_panel_open {
                i18n::t("studio.shell.bottom_panel")
            } else {
                i18n::t("studio.shell.bottom_hidden")
            });
            ui.separator();
            ui.label(format!(
                "{} {}",
                self.app.plugin_manager.manifest_paths.len(),
                i18n::t("studio.shell.plugins")
            ));
            ui.separator();
            ui.label(format!(
                "{} {}",
                self.app.memory_browser.memories.len(),
                i18n::t("studio.shell.memories")
            ));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(i18n::t("studio.shell.external_deferred"))
                        .color(ui::muted_text(ui.ctx())),
                );
            });
        });
    }

    fn render_bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.shell.batch_debug.title"),
                i18n::t("studio.shell.batch_debug.detail"),
            );
            if let Some(report) = self.app.last_batch_report.as_ref() {
                ui.label(egui::RichText::new(format!("batch {:?}", report.status)));
            } else if let Some(execution) = self.app.last_workflow_execution.as_ref() {
                ui.label(egui::RichText::new(format!(
                    "workflow {:?}",
                    execution.status
                )));
            } else if self.status.is_empty() {
                ui.label(
                    egui::RichText::new(i18n::t("studio.shell.idle"))
                        .color(ui::muted_text(ui.ctx())),
                );
            } else {
                ui.label(egui::RichText::new(&self.status).color(ui::strong_text(ui.ctx())));
            }
        });
    }
}
