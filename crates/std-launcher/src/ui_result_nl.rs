use eframe::egui;
use std_egui::{
    tokens::{Color, Radius, Space, Text},
    LauncherNlSuggestion,
};

pub(crate) fn render(ui: &mut egui::Ui, suggestion: &LauncherNlSuggestion) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::xs() as f32);
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::same(Space::sm()))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(format!("Ask: {}", suggestion.query))
                        .font(Text::body())
                        .color(Color::fg_primary(&ctx))
                        .strong(),
                );
                ui.add_space(Space::two_xs() as f32);
                ui.label(
                    egui::RichText::new(format!("confidence {} / 100", suggestion.confidence))
                        .font(Text::footnote())
                        .color(Color::fg_secondary(&ctx)),
                );
                ui.add_space(Space::xs() as f32);
                ui.horizontal_wrapped(|ui| {
                    for (index, action) in suggestion.actions.iter().enumerate() {
                        action_chip(ui, action, index == suggestion.selected_action, &ctx);
                    }
                });
            });
        });
}

fn action_chip(ui: &mut egui::Ui, action: &str, selected: bool, ctx: &egui::Context) {
    let fill = if selected {
        Color::accent_weak(ctx)
    } else {
        Color::bg_surface_2(ctx)
    };
    let stroke_color = if selected {
        Color::accent_base(ctx)
    } else {
        Color::stroke_border(ctx)
    };
    egui::Frame::new()
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke_color))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(action)
                    .font(Text::caption())
                    .color(if selected {
                        Color::fg_primary(ctx)
                    } else {
                        Color::fg_secondary(ctx)
                    }),
            );
        });
}
