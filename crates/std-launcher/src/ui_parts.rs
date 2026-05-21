use eframe::egui;
use std_egui::tokens::{Color, Space, Text};

pub(crate) fn surface_frame(ctx: &egui::Context) -> egui::Frame {
    egui::Frame::new()
        .fill(Color::bg_surface_1(ctx))
        .stroke(egui::Stroke::NONE)
        .corner_radius(egui::CornerRadius::same(std_egui::tokens::Radius::md()))
        .inner_margin(egui::Margin::same(Space::sm()))
}

pub(crate) fn quiet_button(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    ui.add(
        egui::Button::new(egui::RichText::new(label).color(Color::fg_primary(&ctx)))
            .fill(Color::bg_surface_0(&ctx))
            .stroke(egui::Stroke::new(1.0, Color::stroke_divider(&ctx)))
            .corner_radius(egui::CornerRadius::same(std_egui::tokens::Radius::sm())),
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
        .corner_radius(egui::CornerRadius::same(std_egui::tokens::Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(Text::caption())
                    .color(Color::fg_secondary(&ctx)),
            );
        });
}

pub(crate) fn draw_focus_ring(
    ui: &egui::Ui,
    rect: egui::Rect,
    radius: u8,
    expand: f32,
    width: f32,
) {
    ui.painter().rect_stroke(
        rect.expand(expand),
        egui::CornerRadius::same(radius),
        egui::Stroke::new(width, Color::accent_base(ui.ctx())),
        egui::StrokeKind::Outside,
    );
}
