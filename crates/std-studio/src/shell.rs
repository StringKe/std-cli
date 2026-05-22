use crate::{
    layout::{HOST_CHROME_HEIGHT, STATUS_BAR_HEIGHT, STATUS_DIVIDER_HEIGHT, STATUS_DIVIDER_WIDTH},
    shell_parts::{panel_frame, path_label},
    status_bar::StudioStatusBarSummary,
    ui,
    workspace_panes::{focused_workspace_spec, StudioWorkspaceSpec},
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::StudioPane;

impl StudioEguiApp {
    pub(crate) fn render_shell(&mut self, ctx: &egui::Context) {
        self.handle_overlay_keyboard(ctx);
        egui::TopBottomPanel::top("studio_app_chrome")
            .exact_height(HOST_CHROME_HEIGHT)
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
            .exact_height(STATUS_BAR_HEIGHT)
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_1(ctx)))
            .show(ctx, |ui| self.render_status_bar(ui));
        egui::CentralPanel::default()
            .frame(panel_frame(ctx, std_egui::tokens::Color::bg_surface_0(ctx)))
            .show(ctx, |ui| self.render_active_workspace(ui));
        self.render_overlays(ctx);
    }

    fn render_active_workspace(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(Space::SM as f32);
            if self.render_focused_workspace_pane(ui) {
                return;
            }
            self.render_main_workspace_pane(ui);
        });
    }

    fn render_main_workspace_pane(&mut self, ui: &mut egui::Ui) {
        match self.app.active_pane {
            StudioPane::Dashboard => self.render_dashboard(ui),
            StudioPane::Workflows => self.render_workflows(ui),
            StudioPane::Apps => self.render_apps(ui),
            StudioPane::Memory => self.render_memory(ui),
            StudioPane::Plugins => self.render_plugins(ui),
            StudioPane::Analysis => self.render_analysis(ui),
            StudioPane::History => self.render_history(ui),
            StudioPane::Operations => self.render_operations(ui),
            StudioPane::Settings => self.render_settings(ui),
        }
    }

    fn render_context(&mut self, ui: &mut egui::Ui) {
        if let Some(spec) = focused_workspace_spec(&self.app) {
            render_workspace_context(ui, &spec);
            ui.add_space(Space::SM as f32);
        }
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
        ui.set_height(STATUS_BAR_HEIGHT);
        ui.horizontal(|ui| {
            status_text(ui, self.app.active_pane.content_key());
            status_divider(ui);
            status_text(
                ui,
                &format!(
                    "{} {}",
                    self.app.open_workspace_panes().count(),
                    i18n::t("studio.shell.panes")
                ),
            );
            status_divider(ui);
            status_text(ui, self.app.workspace_policy.summary());
            status_divider(ui);
            status_text(
                ui,
                if self.layout.inspector_open {
                    i18n::t("studio.shell.inspector")
                } else {
                    i18n::t("studio.shell.inspector_hidden")
                },
            );
            status_divider(ui);
            status_text(
                ui,
                if self.layout.bottom_panel_open {
                    i18n::t("studio.shell.bottom_panel")
                } else {
                    i18n::t("studio.shell.bottom_hidden")
                },
            );
            status_divider(ui);
            status_text(
                ui,
                &format!(
                    "{} {}",
                    self.app.plugin_manager.manifest_paths.len(),
                    i18n::t("studio.shell.plugins")
                ),
            );
            status_divider(ui);
            status_text(
                ui,
                &format!(
                    "{} {}",
                    self.app.memory_browser.memories.len(),
                    i18n::t("studio.shell.memories")
                ),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let summary = StudioStatusBarSummary::from_state(&self.app, &self.analysis);
                for (index, label) in summary.right_labels().into_iter().enumerate() {
                    if index > 0 {
                        status_divider(ui);
                    }
                    status_text(ui, label);
                }
            });
        });
    }
}

#[cfg(test)]
pub(crate) fn workspace_context_summary(spec: &StudioWorkspaceSpec) -> String {
    format!(
        "inspector_context=pane:{};kind:{};lines={};actions={}",
        spec.title,
        spec.content_key,
        spec.lines.len(),
        workspace_context_actions(spec)
    )
}

fn render_workspace_context(ui: &mut egui::Ui, spec: &StudioWorkspaceSpec) {
    ui::surface_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.shell.context.title"),
            i18n::t("studio.shell.context.detail"),
        );
        path_label(ui, "Pane", spec.title.clone());
        path_label(ui, "Kind", spec.content_key.to_string());
        if let Some(path) = &spec.workflow_path {
            path_label(ui, "Workflow", path.display().to_string());
        }
        if let Some(path) = &spec.analysis_path {
            path_label(ui, "Analysis", path.display().to_string());
        }
        for line in spec.lines.iter().take(3) {
            path_label(ui, "Signal", line.clone());
        }
        path_label(ui, "Actions", workspace_context_actions(spec));
    });
}

fn workspace_context_actions(spec: &StudioWorkspaceSpec) -> String {
    let mut actions = vec!["show-main", "refresh", "close"];
    if spec.workflow_path.is_some() {
        actions.extend(["preview", "run"]);
    }
    if spec.analysis_path.is_some() {
        actions.push("analyze");
    }
    if spec.content_key == "plugins" {
        actions.push("reload");
    }
    actions.join(",")
}

fn status_text(ui: &mut egui::Ui, text: &str) {
    ui.label(
        egui::RichText::new(text)
            .font(std_egui::tokens::Text::caption())
            .color(ui::muted_text(ui.ctx())),
    );
}

fn status_divider(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let (rect, _response) = ui.allocate_exact_size(
        egui::Vec2::new(STATUS_DIVIDER_WIDTH, STATUS_DIVIDER_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().line_segment(
        [rect.center_top(), rect.center_bottom()],
        egui::Stroke::new(1.0, std_egui::tokens::Color::stroke_divider(&ctx)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_shell_uses_documented_host_and_status_heights() {
        assert_eq!(HOST_CHROME_HEIGHT, 52.0);
        assert_eq!(STATUS_BAR_HEIGHT, 24.0);
    }

    #[test]
    fn status_bar_renderer_uses_tokenized_caption_text_and_custom_dividers() {
        let source = include_str!("shell.rs");

        assert!(source.contains("status_text(ui, self.app.active_pane.content_key())"));
        assert!(source.contains("Text::caption()"));
        assert!(source.contains("status_divider(ui)"));
        assert!(!source.contains("\n            ui.separator()"));
    }

    #[test]
    fn canvas_promotes_focused_workspace_pane_as_primary_content() {
        let source = include_str!("shell.rs");

        assert!(source.contains("if self.render_focused_workspace_pane(ui)"));
        assert!(source.contains("return;"));
        assert!(source.contains("self.render_main_workspace_pane(ui);"));
        let old_append_call = ["self.render_", "workspace_panes(ui);"].join("");
        assert!(!source.contains(&old_append_call));
    }

    #[test]
    fn inspector_context_is_derived_from_focused_workspace_pane() {
        let mut app = StudioEguiApp::default();
        app.app.open_plugin_manager_pane();
        let spec = focused_workspace_spec(&app.app).unwrap();

        let summary = workspace_context_summary(&spec);

        assert!(summary.contains("inspector_context=pane:插件管理;kind:plugins"));
        assert!(summary.contains("actions=show-main,refresh,close,reload"));
    }
}
