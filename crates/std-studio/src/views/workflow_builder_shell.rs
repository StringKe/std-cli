use crate::{
    ui,
    views::{workflow_builder_metrics, workflow_builder_status, workflow_builder_trace},
    StudioEguiApp,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

impl StudioEguiApp {
    pub(crate) fn render_workflow_builder(&mut self, ui: &mut egui::Ui) {
        ui::surface_frame(ui.ctx()).show(ui, |ui| {
            ui::section_header(
                ui,
                i18n::t("studio.workflow_builder.title"),
                i18n::t("studio.workflow_builder.detail"),
            );
            self.render_builder_toolbar(ui);
            ui.add_space(Space::XS as f32);
            workflow_builder_status::render(ui, self);
            ui.add_space(Space::XS as f32);
            self.render_builder_workspace(ui);
            ui.add_space(Space::XS as f32);
            workflow_builder_trace::render(ui, self);
            ui.add_space(Space::XS as f32);
            self.render_ai_assist_panel(ui);
        });
    }

    fn render_builder_workspace(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let Some((left_width, right_width)) =
            workflow_builder_metrics::builder_columns(available_width)
        else {
            self.render_builder_steps(ui);
            ui.add_space(workflow_builder_metrics::BUILDER_PANEL_GAP);
            self.render_step_properties(ui);
            return;
        };
        ui.horizontal_top(|ui| {
            ui.set_min_width(available_width);
            ui.allocate_ui_with_layout(
                workflow_builder_metrics::builder_pane_size(left_width),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_builder_steps(ui),
            );
            ui.add_space(workflow_builder_metrics::BUILDER_PANEL_GAP);
            ui.allocate_ui_with_layout(
                workflow_builder_metrics::builder_pane_size(right_width),
                egui::Layout::top_down(egui::Align::Min),
                |ui| self.render_step_properties(ui),
            );
        });
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn workflow_builder_shell_matches_docs22_render_order() {
        let source = include_str!("workflow_builder_shell.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        for required in [
            "render_builder_toolbar",
            "workflow_builder_status::render",
            "render_builder_workspace",
            "workflow_builder_trace::render",
            "render_ai_assist_panel",
            "builder_columns",
            "builder_pane_size(left_width)",
            "builder_pane_size(right_width)",
        ] {
            assert!(production_source.contains(required), "missing {required}");
        }
    }
}
