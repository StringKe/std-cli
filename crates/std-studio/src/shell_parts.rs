use crate::ui;
use eframe::egui;

pub(crate) fn path_label(ui: &mut egui::Ui, label: &str, value: String) {
    ui.label(egui::RichText::new(label).color(ui::muted_text(ui.ctx())));
    ui.label(
        egui::RichText::new(value)
            .monospace()
            .color(ui::strong_text(ui.ctx())),
    );
    ui.add_space(4.0);
}

pub(crate) fn panel_frame(ctx: &egui::Context, fill: egui::Color32) -> egui::Frame {
    egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(
            1.0,
            std_egui::tokens::Color::stroke_divider(ctx),
        ))
        .inner_margin(egui::Margin::symmetric(10, 6))
}
