use crate::ui_parts::{keycap, quiet_label};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_launcher::launcher_shortcut_help_rows;

pub(crate) fn render(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::xs() as f32);
    ui.label(
        egui::RichText::new("Keyboard shortcuts")
            .font(Text::body())
            .color(Color::fg_primary(&ctx))
            .strong(),
    );
    ui.add_space(Space::xs() as f32);
    for row in launcher_shortcut_help_rows() {
        shortcut_row(ui, row.key, row.action);
        ui.add_space(Space::two_xs() as f32);
    }
}

fn shortcut_row(ui: &mut egui::Ui, shortcut: &str, action: &str) {
    let ctx = ui.ctx().clone();
    let response =
        ui.allocate_response(egui::vec2(ui.available_width(), 32.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("{shortcut} {action}"),
        )
    });
    ui.painter().rect_filled(
        response.rect,
        egui::CornerRadius::same(Radius::md()),
        Color::bg_surface_1(&ctx),
    );
    let rect = response.rect.shrink2(egui::vec2(Space::sm() as f32, 0.0));
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.horizontal(|ui| {
            keycap(ui, shortcut);
            quiet_label(ui, action);
        });
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn shortcut_help_uses_inline_keycaps_not_tooltips() {
        let source = include_str!("ui_shortcut_help.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("keycap(ui, shortcut)"));
        assert!(implementation.contains("WidgetInfo::labeled"));
        assert!(!implementation.contains("on_hover_text"));
    }
}
