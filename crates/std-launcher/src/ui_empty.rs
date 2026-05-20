use crate::ui_parts::keycap;
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Radius, Space, Text},
};

pub(crate) enum EmptyAction {
    AskAi(String),
}

pub(crate) fn render_no_results(ui: &mut egui::Ui, query: &str) -> Option<EmptyAction> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        render_empty_query(ui);
        return None;
    }

    let fallback = render_no_matches(ui, trimmed);
    let enter_pressed =
        !input::ime_composing(ui.ctx()) && ui.input(|input| input.key_pressed(egui::Key::Enter));
    if fallback.clicked() || enter_pressed {
        Some(EmptyAction::AskAi(ask_ai_query(trimmed)))
    } else {
        None
    }
}

fn ask_ai_query(query: &str) -> String {
    format!("? {}", query.trim())
}

fn render_empty_query(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::LG as f32);
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.ready.title"))
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.add_space(Space::TWO_XS as f32);
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.ready.detail"))
                .font(Text::footnote())
                .color(Color::fg_tertiary(&ctx)),
        );
    });
}

fn render_no_matches(ui: &mut egui::Ui, query: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::MD as f32);
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.no_matches.title"))
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.add_space(Space::TWO_XS as f32);
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.no_matches.detail"))
                .font(Text::footnote())
                .color(Color::fg_secondary(&ctx)),
        );
    });
    ui.add_space(Space::SM as f32);
    ask_ai_row(
        ui,
        &format!("{} \"{}\"", i18n::t("launcher.empty.ask_ai"), query),
    )
}

fn ask_ai_row(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::SM, Space::XS))
        .show(ui, |ui| {
            let response =
                ui.allocate_response(egui::vec2(ui.available_width(), 34.0), egui::Sense::click());
            let rect = response.rect.shrink2(egui::vec2(Space::XS as f32, 0.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(label)
                            .font(Text::body())
                            .color(Color::fg_primary(&ctx)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        keycap(ui, "Enter");
                    });
                });
            });
            response
        })
        .inner
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_ai_query_uses_launcher_question_prefix() {
        assert_eq!(ask_ai_query("  missing workflow  "), "? missing workflow");
    }
}
