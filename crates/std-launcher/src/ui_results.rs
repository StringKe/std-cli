use crate::{
    ui_empty::{self, EmptyAction},
    ui_keyboard, ui_metrics,
    ui_result_model::{
        group_count as model_group_count, list_items, LauncherResultListItem,
        LauncherResultRowModel,
    },
    ui_result_nl, ui_result_rows, ui_results_surface, ui_results_virtual, ui_shortcut_help,
};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, Space, Text},
    LauncherResultMode,
};
use std_launcher::{LauncherFocusSection, LauncherState};
use std_types::SearchResult;

pub(crate) fn group_count(results: &[SearchResult]) -> usize {
    model_group_count(results)
}

pub(crate) fn render(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) -> bool {
    let mut hide_requested = false;
    ui_results_surface::show(ui, state, |ui, state| {
        hide_requested = render_results(ui, state, max_height);
    });
    hide_requested
}

fn render_results(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) -> bool {
    if state.view.result_mode == LauncherResultMode::NaturalLanguage {
        if std_launcher::launcher_shortcut_help_visible(&state.view.query) {
            ui_shortcut_help::render(ui);
            return false;
        }
        if let Some(suggestion) = state.view.nl_suggestion.as_ref() {
            ui_result_nl::render(ui, suggestion);
        }
        return false;
    }
    let mut clicked = None;
    let mut double_clicked = None;
    let items = list_items(
        &state.view.results,
        state.view.preview.as_ref(),
        state.view.selected,
    );
    egui::ScrollArea::vertical()
        .id_salt("launcher_results")
        .max_height(max_height)
        .auto_shrink([false, false])
        .show_viewport(ui, |ui, viewport| {
            if state.view.phase == std_egui::LauncherPhase::Searching {
                render_loading_progress_bar(ui, &ui.ctx().clone());
                ui.add_space(Space::xs() as f32);
            }
            let total_height = ui_results_virtual::total_height(&items);
            ui.set_min_height(total_height);
            let (start, end, mut y) =
                ui_results_virtual::visible_range(&items, viewport.min.y, viewport.max.y);
            ui.add_space(y);
            if state.view.results.is_empty() {
                if state.view.phase == std_egui::LauncherPhase::Searching {
                    render_progress(ui, i18n::t("launcher.results.searching"));
                    return;
                }
                if let Some(EmptyAction::AskAi(query)) = ui_empty::render_no_results(
                    ui,
                    &state.view.query,
                    state.empty_suggestion_selected,
                ) {
                    state.update_query(query);
                }
                if let Some(EmptyAction::SetQuery(query)) = ui_empty::take_empty_query_action(ui) {
                    state.update_query(query);
                }
                return;
            }
            if start == 0 {
                render_overflow_hint(ui, &state.view);
            }
            for item in &items[start..end] {
                let item_height = ui_results_virtual::item_height(item);
                let item_rect = egui::Rect::from_min_size(
                    egui::pos2(ui.min_rect().left(), y),
                    egui::vec2(ui.available_width(), item_height),
                );
                ui.scope_builder(
                    egui::UiBuilder::new().max_rect(item_rect),
                    |ui| match item {
                        LauncherResultListItem::Group { label } => {
                            ui_result_rows::group_header(ui, label)
                        }
                        LauncherResultListItem::Row(model) => {
                            let response = result_row(ui, model, &state.view);
                            if response.double_clicked() {
                                double_clicked = Some(model.result_index);
                            } else if response.clicked() {
                                clicked = Some(model.result_index);
                            }
                        }
                    },
                );
                y += item_height;
            }
        });

    if let Some(index) = double_clicked {
        return trigger_result_from_row(state, index);
    } else if let Some(index) = clicked {
        state.mark_pointer_focus(LauncherFocusSection::Results);
        state.view.selected = index;
        state.view.refresh_preview(&state.core);
    }
    false
}

fn trigger_result_from_row(state: &mut LauncherState, index: usize) -> bool {
    state
        .trigger_result_by_user(index)
        .map(|execution| ui_keyboard::execution_hides_launcher(&execution))
        .unwrap_or(false)
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
        ui_metrics::loading_progress_size(available_width),
        egui::Sense::hover(),
    );
    let progress_rect = ui_metrics::loading_progress_rect(available_width, rect.min);
    ui.painter().rect_filled(
        progress_rect,
        egui::CornerRadius::ZERO,
        Color::accent_base(ctx),
    );
}

fn result_row(
    ui: &mut egui::Ui,
    model: &LauncherResultRowModel,
    view: &std_egui::LauncherViewModel,
) -> egui::Response {
    let response = ui_result_rows::result_row(ui, model);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            ui_result_rows::result_accessibility_label(model, view),
        )
    });
    response
}

#[cfg(test)]
fn result_row_keyboard_affordance(model: &LauncherResultRowModel) -> (String, String, &str) {
    ui_result_rows::result_row_keyboard_affordance(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selected_result_row_exposes_enter_and_action_hint() {
        let result = std_types::SearchResult {
            action: std_types::Action::new(
                "Rebuild Index",
                "Refresh local index",
                "test",
                std_types::ActionType::Command,
            ),
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };
        let row = LauncherResultRowModel::from_result(&result, None, 0, 1, true);

        assert_eq!(
            result_row_keyboard_affordance(&row).0,
            std_egui::input::launcher_result_keycap(0).unwrap()
        );
        assert_eq!(
            result_row_keyboard_affordance(&row).1,
            std_egui::input::enter().label()
        );
        assert_eq!(
            result_row_keyboard_affordance(&row).2,
            i18n::t("launcher.action.run")
        );
    }

    #[test]
    fn selected_result_row_keeps_number_shortcut_and_enter_primary_hint() {
        let result = std_types::SearchResult {
            action: std_types::Action::new(
                "Open Studio",
                "Open the workspace",
                "test",
                std_types::ActionType::AppLaunch,
            ),
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };
        let row = LauncherResultRowModel::from_result(&result, None, 2, 5, true);
        let (direct, primary, action) = result_row_keyboard_affordance(&row);

        assert_eq!(direct, std_egui::input::launcher_result_keycap(2).unwrap());
        assert_eq!(primary, std_egui::input::enter().label());
        assert_eq!(action, i18n::t("launcher.action.run"));
    }

    #[test]
    fn result_rows_double_click_primary_action_without_changing_single_click_select() {
        let source = include_str!("ui_results.rs");

        assert!(source.contains("response.double_clicked()"));
        assert!(source.contains("trigger_result_from_row(state, index)"));
        assert!(source.contains("ui_keyboard::execution_hides_launcher(&execution)"));
        assert!(source.contains("else if response.clicked()"));
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

    #[test]
    fn searching_progress_bar_is_pinned_above_results() {
        let source = include_str!("ui_results.rs");
        let viewport_body = source
            .split(".show_viewport(ui, |ui, viewport| {")
            .nth(1)
            .and_then(|body| body.split("let total_height").next())
            .unwrap();
        let progress_body = source
            .split("fn render_progress")
            .nth(1)
            .and_then(|body| body.split("fn render_loading_progress_bar").next())
            .unwrap();

        assert!(viewport_body.contains("LauncherPhase::Searching"));
        assert!(viewport_body.contains("render_loading_progress_bar"));
        assert!(!progress_body.contains("render_loading_progress_bar"));
    }

    #[test]
    fn lone_question_mark_renders_shortcut_help_not_nl_actions() {
        assert!(std_launcher::launcher_shortcut_help_visible("?"));
        assert!(!std_launcher::launcher_shortcut_help_visible("? rebuild"));

        let source = include_str!("ui_results.rs");
        let help_index = source.find("ui_shortcut_help::render(ui)").unwrap();
        let nl_index = source.find("ui_result_nl::render(ui, suggestion)").unwrap();

        assert!(help_index < nl_index);
    }
}
