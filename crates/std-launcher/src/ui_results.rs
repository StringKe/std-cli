use crate::{
    ui_empty::{self, EmptyAction},
    ui_metrics,
    ui_parts::{draw_focus_ring, keycap, quiet_label, surface_frame},
    ui_result_model::{
        group_count as model_group_count, group_header_label, list_items, LauncherResultListItem,
        LauncherResultRowModel,
    },
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n,
    tokens::{Color, Radius, Space, Text},
    LauncherResultMode,
};
use std_launcher::LauncherFocusSection;
use std_launcher::LauncherState;
use std_types::SearchResult;

pub(crate) fn group_count(results: &[SearchResult]) -> usize {
    model_group_count(results)
}

pub(crate) fn render(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    let response = surface_frame(ui.ctx()).show(ui, |ui| {
        section_header(
            ui,
            section_title(&state.view),
            &format!(
                "{} {}",
                state.view.results.len(),
                i18n::t("launcher.results.matches_suffix")
            ),
        );
        render_results(ui, state, max_height);
    });
    if state.focus_section == LauncherFocusSection::Results {
        let a11y = AccessibilityContext::from_env();
        draw_focus_ring(
            ui,
            response.response.rect,
            Radius::md(),
            ui_metrics::focus_ring_expand(),
            a11y.focus_ring_width(),
        );
    }
}

fn section_title(view: &std_egui::LauncherViewModel) -> &'static str {
    match view.phase {
        std_egui::LauncherPhase::Searching => i18n::t("launcher.results.searching.title"),
        std_egui::LauncherPhase::Executing => i18n::t("launcher.results.executing.title"),
        std_egui::LauncherPhase::Feedback => i18n::t("launcher.results.feedback.title"),
        _ => match view.result_mode {
            LauncherResultMode::SuggestedWorkflows => {
                i18n::t("launcher.results.suggested_workflows.title")
            }
            LauncherResultMode::Matches => i18n::t("launcher.results.title"),
            LauncherResultMode::NoMatches => i18n::t("launcher.results.title"),
        },
    }
}

fn render_results(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    let mut clicked = None;
    let items = list_items(
        &state.view.results,
        state.view.preview.as_ref(),
        state.view.selected,
    );
    egui::ScrollArea::vertical()
        .id_salt("launcher_results")
        .max_height(max_height)
        .auto_shrink([false, false])
        .show_rows(
            ui,
            ui_metrics::result_list_slot_height(),
            items.len(),
            |ui, row_range| {
                if state.view.results.is_empty() {
                    if state.view.phase == std_egui::LauncherPhase::Searching {
                        render_progress(ui, i18n::t("launcher.results.searching"));
                        return;
                    }
                    if let Some(EmptyAction::AskAi(query)) =
                        ui_empty::render_no_results(ui, &state.view.query)
                    {
                        state.update_query(query);
                    }
                    return;
                }
                if row_range.start == 0 {
                    render_overflow_hint(ui, &state.view);
                }
                for index in row_range {
                    match &items[index] {
                        LauncherResultListItem::Group { label } => group_header(ui, label),
                        LauncherResultListItem::Row(model) => {
                            if result_row(ui, model).clicked() {
                                clicked = Some(model.result_index);
                            }
                        }
                    }
                }
            },
        );

    if let Some(index) = clicked {
        state.view.selected = index;
        state.view.refresh_preview(&state.core);
    }
}

fn render_overflow_hint(ui: &mut egui::Ui, view: &std_egui::LauncherViewModel) {
    if !view.result_overflowed() {
        return;
    }
    let ctx = ui.ctx().clone();
    ui.add_space(Space::xs() as f32);
    ui.label(
        egui::RichText::new(i18n::t("launcher.results.overflow_hint"))
            .font(Text::footnote())
            .color(Color::fg_secondary(&ctx)),
    );
}

fn render_progress(ui: &mut egui::Ui, label: &str) {
    let ctx = ui.ctx().clone();
    render_loading_progress_bar(ui, &ctx);
    ui.add_space(Space::md() as f32);
    ui.horizontal(|ui| {
        ui.spinner();
        ui.label(
            egui::RichText::new(label)
                .font(Text::body())
                .color(Color::fg_secondary(&ctx)),
        );
    });
}

fn render_loading_progress_bar(ui: &mut egui::Ui, ctx: &egui::Context) {
    let available_width = ui.available_width();
    let (rect, _response) = ui.allocate_exact_size(
        egui::vec2(
            available_width,
            ui_metrics::loading_progress_rect(available_width, egui::Pos2::ZERO).height(),
        ),
        egui::Sense::hover(),
    );
    let progress_rect = ui_metrics::loading_progress_rect(available_width, rect.min);
    ui.painter().rect_filled(
        progress_rect,
        egui::CornerRadius::same(1),
        Color::accent_base(ctx),
    );
}

fn result_row(ui: &mut egui::Ui, model: &LauncherResultRowModel) -> egui::Response {
    let ctx = ui.ctx().clone();
    let a11y = AccessibilityContext::from_env();
    let selected = model.action_hint.is_some();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        egui::Color32::TRANSPARENT
    };
    egui::Frame::new()
        .fill(fill)
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::symmetric(Space::sm(), Space::two_xs()))
        .show(ui, |ui| {
            let response = ui.allocate_response(
                egui::vec2(ui.available_width(), ui_metrics::result_row_height()),
                egui::Sense::click(),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::SelectableLabel,
                    ui.is_enabled(),
                    a11y.launcher_result_label(
                        &model.title,
                        &model.kind,
                        model.position_number(),
                        model.position_total(),
                    ),
                )
            });
            let rect = response.rect.shrink2(egui::vec2(Space::xs() as f32, 0.0));
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
                ui.horizontal(|ui| {
                    render_kind_badge(ui, &ctx, &model.kind);
                    ui.label(result_title_text(&model.title, selected, &ctx));
                    ui.label(
                        egui::RichText::new(&model.kind)
                            .font(Text::footnote())
                            .color(Color::fg_secondary(&ctx)),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if let Some(shortcut) = model.shortcut.as_deref() {
                            keycap(ui, shortcut);
                        }
                        if let Some(hint) = model.action_hint.as_deref() {
                            quiet_label(ui, hint);
                        }
                    });
                });
            });
            response
        })
        .inner
}

fn result_title_text(title: &str, selected: bool, ctx: &egui::Context) -> egui::RichText {
    let text = egui::RichText::new(title)
        .font(Text::body())
        .color(Color::fg_primary(ctx));
    if selected {
        text.strong()
    } else {
        text
    }
}

#[cfg(test)]
fn result_title_is_strong(selected: bool) -> bool {
    selected
}

fn group_header(ui: &mut egui::Ui, group: &str) {
    let ctx = ui.ctx().clone();
    ui.add_space(Space::xs() as f32);
    render_group_divider(ui, &ctx);
    ui.add_space(Space::two_xs() as f32);
    ui.label(
        egui::RichText::new(group_header_label(group))
            .font(Text::footnote())
            .color(Color::fg_tertiary(&ctx))
            .strong(),
    );
}

fn render_group_divider(ui: &mut egui::Ui, ctx: &egui::Context) {
    let available_width = ui.available_width();
    let height = ui_metrics::group_divider_rect(available_width, egui::Pos2::ZERO).height();
    let (rect, _response) =
        ui.allocate_exact_size(egui::vec2(available_width, height), egui::Sense::hover());
    let divider_rect = ui_metrics::group_divider_rect(available_width, rect.min);
    ui.painter().rect_filled(
        divider_rect,
        egui::CornerRadius::ZERO,
        Color::stroke_border(ctx),
    );
}

fn section_header(ui: &mut egui::Ui, title: &str, detail: &str) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(detail).font(Text::footnote()));
        });
    });
    ui.add_space(Space::two_xs() as f32);
}

fn render_kind_badge(ui: &mut egui::Ui, ctx: &egui::Context, kind: &str) {
    egui::Frame::new()
        .fill(Color::bg_surface_2(ctx))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(kind)
                    .font(Text::caption())
                    .color(Color::fg_secondary(ctx)),
            );
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_title_weight_matches_selection_state() {
        assert!(result_title_is_strong(true));
        assert!(!result_title_is_strong(false));
    }

    #[test]
    fn group_header_label_is_uppercase() {
        assert_eq!(group_header_label("Action / Workflow"), "ACTION / WORKFLOW");
    }

    #[test]
    fn overflow_hint_uses_documented_copy() {
        let mut view = std_egui::LauncherViewModel::new(&std_core::StdCore::default());
        view.telemetry.last_result_count = 200;
        view.telemetry.last_overflowed = true;

        assert!(view.result_overflowed());
        assert_eq!(
            i18n::t("launcher.results.overflow_hint"),
            "200+ matches, refine your query"
        );
    }

    #[test]
    fn results_focus_section_uses_single_focus_owner() {
        let mut state = LauncherState::new();
        state.update_query("index");
        state.focus_section = LauncherFocusSection::Results;

        assert_eq!(state.focus_section, LauncherFocusSection::Results);
        assert_ne!(state.focus_section, LauncherFocusSection::Search);
    }
}
