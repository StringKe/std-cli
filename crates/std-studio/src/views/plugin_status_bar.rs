use crate::ui;
use eframe::egui;
use std_egui::{i18n, tokens::Space};
use std_studio::plugin_status::PluginStatusSummary;

pub(crate) fn render(ui: &mut egui::Ui, summary: &PluginStatusSummary) {
    ui::surface_frame(ui.ctx()).show(ui, |ui| {
        ui::section_header(
            ui,
            i18n::t("studio.plugins.status.title"),
            i18n::t("studio.plugins.status.detail"),
        );
        ui.horizontal_wrapped(|ui| {
            status_chip(ui, "manifest", &summary.manifest_status);
            status_chip(ui, "actions", &summary.action_status);
            status_chip(ui, "preview", &summary.preview_status);
            status_chip(ui, "runtime", &summary.runtime_status);
            status_chip(ui, "permissions", &summary.permission_status);
            status_chip(ui, "boundary", &summary.boundary_status);
        });
    });
}

fn status_chip(ui: &mut egui::Ui, label: &str, value: &str) {
    ui::chip(
        ui,
        &format!("{label} {value}"),
        std_egui::tokens::Color::bg_surface_2(ui.ctx()),
    );
    ui.add_space(Space::TWO_XS as f32);
}
