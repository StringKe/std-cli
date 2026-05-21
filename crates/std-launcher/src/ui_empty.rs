use crate::{ui_metrics, ui_parts::keycap};
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
    let enter_pressed = !input::ime_composing(ui.ctx()) && input::enter().pressed(ui.ctx());
    if fallback.clicked() || enter_pressed {
        std_launcher::ask_ai_fallback_query(trimmed).map(EmptyAction::AskAi)
    } else {
        None
    }
}

fn render_empty_query(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::lg() as f32);
    ui.vertical_centered(|ui| {
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.ready.title"))
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.add_space(Space::two_xs() as f32);
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.ready.detail"))
                .font(Text::footnote())
                .color(Color::fg_tertiary(&ctx)),
        );
    });
}

fn render_no_matches(ui: &mut egui::Ui, query: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::md() as f32);
    ui.vertical_centered(|ui| {
        render_no_matches_icon(ui, &ctx);
        ui.add_space(Space::xs() as f32);
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.no_matches.title"))
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.add_space(Space::two_xs() as f32);
        ui.label(
            egui::RichText::new(i18n::t("launcher.empty.no_matches.detail"))
                .font(Text::footnote())
                .color(Color::fg_secondary(&ctx)),
        );
    });
    ui.add_space(Space::sm() as f32);
    ask_ai_row(
        ui,
        &format!("{} \"{}\"", i18n::t("launcher.empty.ask_ai"), query),
    )
}

fn render_no_matches_icon(ui: &mut egui::Ui, ctx: &egui::Context) {
    let (rect, _response) =
        ui.allocate_exact_size(ui_metrics::no_matches_icon_size(), egui::Sense::hover());
    let geometry = ui_metrics::no_matches_icon_geometry(rect);
    let stroke = egui::Stroke::new(1.6, Color::fg_tertiary(ctx));
    ui.painter()
        .circle_stroke(geometry.center, geometry.radius, stroke);
    ui.painter()
        .line_segment([geometry.handle_start, geometry.handle_end], stroke);
}

fn ask_ai_row(ui: &mut egui::Ui, label: &str) -> egui::Response {
    let ctx = ui.ctx().clone();
    let response = ui.allocate_response(
        egui::vec2(ui.available_width(), ui_metrics::ask_ai_row_height()),
        egui::Sense::click(),
    );
    let fill = if response.hovered() {
        Color::bg_surface_2(&ctx)
    } else {
        Color::bg_surface_1(&ctx)
    };
    ui.painter()
        .rect_filled(response.rect, egui::CornerRadius::same(Radius::md()), fill);
    let rect = response.rect.shrink2(egui::vec2(Space::sm() as f32, 0.0));
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ask_ai_query_uses_launcher_question_prefix() {
        let mut state = std_launcher::LauncherState::new();
        state.update_query("  missing workflow  ");

        assert_eq!(
            state.no_match_fallback_query().as_deref(),
            Some("? missing workflow")
        );
    }

    #[test]
    fn no_matches_icon_uses_docs_lg_size() {
        let (size, radius) = crate::ui_metrics_empty::no_matches_icon_metrics_for_scale(
            std_egui::tokens::UiScale::default(),
        );

        assert_eq!(size, egui::vec2(32.0, 32.0));
        assert_eq!(radius, 9.0);
    }

    #[test]
    fn ask_ai_fallback_uses_inline_result_row_height() {
        assert_eq!(ui_metrics::ask_ai_row_height(), 34.0);
    }
}
