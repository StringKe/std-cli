use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

pub(crate) fn surface_frame(ctx: &egui::Context) -> egui::Frame {
    egui::Frame::new()
        .fill(Color::bg_surface_1(ctx))
        .stroke(egui::Stroke::NONE)
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::same(Space::SM))
}

pub(crate) fn quiet_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(Color::fg_primary(&ctx)))
            .fill(Color::bg_surface_0(&ctx))
            .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
            .corner_radius(egui::CornerRadius::same(Radius::SM)),
    )
}

pub(crate) fn quiet_label(ui: &mut egui::Ui, text: &str) {
    let ctx = ui.ctx().clone();
    ui.label(
        egui::RichText::new(text)
            .font(Text::caption())
            .color(Color::fg_secondary(&ctx)),
    );
}

pub(crate) fn keycap(ui: &mut egui::Ui, text: &str) {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::SM))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(Text::caption())
                    .color(Color::fg_secondary(&ctx)),
            );
        });
}

pub(crate) fn weak_status_fill(ctx: &egui::Context, color: egui::Color32) -> egui::Color32 {
    let alpha = if ctx.style().visuals.dark_mode {
        42
    } else {
        28
    };
    egui::Color32::from_rgba_premultiplied(color.r(), color.g(), color.b(), alpha)
}
