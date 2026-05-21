use crate::{ui_metrics, ui_parts::keycap};
use eframe::egui;
use std_egui::{
    i18n, input,
    tokens::{Color, Radius, Space, Text},
};

pub(crate) enum EmptyAction {
    AskAi(String),
    SetQuery(String),
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
    ui.add_space(Space::xs() as f32);
    ui.label(
        egui::RichText::new(i18n::t("launcher.results.suggested_workflows.title"))
            .font(Text::body())
            .color(Color::fg_primary(&ctx))
            .strong(),
    );
    ui.add_space(Space::xs() as f32);
    for item in suggested_workflow_rows() {
        if suggested_row(ui, item).clicked() {
            ui.data_mut(|data| data.insert_temp(empty_action_id(), item.query.to_string()));
        }
        ui.add_space(Space::two_xs() as f32);
    }
    ui.add_space(Space::xs() as f32);
    ui.horizontal_wrapped(|ui| {
        ui.label(
            egui::RichText::new(empty_query_hint())
                .font(Text::footnote())
                .color(Color::fg_tertiary(&ctx)),
        );
    });
}

#[derive(Clone, Copy)]
struct SuggestedWorkflowRow {
    title_key: &'static str,
    detail_key: &'static str,
    shortcut: &'static str,
    query: &'static str,
}

fn suggested_workflow_rows() -> [SuggestedWorkflowRow; 3] {
    [
        SuggestedWorkflowRow {
            title_key: "launcher.empty.suggestion.rebuild.title",
            detail_key: "launcher.empty.suggestion.rebuild.detail",
            shortcut: "/",
            query: "/rebuild index",
        },
        SuggestedWorkflowRow {
            title_key: "launcher.empty.suggestion.ask.title",
            detail_key: "launcher.empty.suggestion.ask.detail",
            shortcut: "?",
            query: "? ",
        },
        SuggestedWorkflowRow {
            title_key: "launcher.empty.suggestion.studio.title",
            detail_key: "launcher.empty.suggestion.studio.detail",
            shortcut: ">",
            query: "> studio",
        },
    ]
}

fn suggested_row(ui: &mut egui::Ui, item: SuggestedWorkflowRow) -> egui::Response {
    let ctx = ui.ctx().clone();
    let response = ui.allocate_response(
        egui::vec2(ui.available_width(), ui_metrics::ask_ai_row_height()),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            i18n::t(item.title_key),
        )
    });
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
            keycap(ui, item.shortcut);
            ui.label(
                egui::RichText::new(i18n::t(item.title_key))
                    .font(Text::body())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            );
            ui.label(
                egui::RichText::new(i18n::t(item.detail_key))
                    .font(Text::footnote())
                    .color(Color::fg_secondary(&ctx)),
            );
        });
    });
    response
}

fn empty_query_hint() -> &'static str {
    i18n::t("launcher.empty.ready.detail")
}

pub(crate) fn take_empty_query_action(ui: &mut egui::Ui) -> Option<EmptyAction> {
    ui.data_mut(|data| data.remove_temp::<String>(empty_action_id()))
        .map(EmptyAction::SetQuery)
}

fn empty_action_id() -> egui::Id {
    egui::Id::new("launcher_empty_query_action")
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

    #[test]
    fn empty_query_uses_suggested_workflow_rows_not_blank_window() {
        let rows = suggested_workflow_rows();

        assert_eq!(rows.len(), 3);
        assert_eq!(
            i18n::t(rows[0].title_key),
            i18n::t("launcher.empty.suggestion.rebuild.title")
        );
        assert_eq!(rows[0].query, "/rebuild index");
        assert_eq!(rows[1].query, "? ");
        assert_eq!(rows[2].query, "> studio");
        assert!(empty_query_hint().contains('/'));
        assert!(empty_query_hint().contains('?'));
    }

    #[test]
    fn empty_suggestions_are_localized_clickable_query_intents() {
        let rows = suggested_workflow_rows();
        let labels = rows
            .iter()
            .map(|row| format!("{}=>{}", i18n::t(row.title_key), row.query))
            .collect::<Vec<_>>()
            .join("|");

        assert!(labels.contains("=>/rebuild index"));
        assert!(labels.contains("=>? "));
        assert!(labels.contains("=>> studio"));
        assert!(!labels.contains("UNKNOWN_I18N_KEY"));
    }
}
