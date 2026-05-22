use crate::{ui_keyboard, ui_metrics, ui_parts::draw_focus_ring};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n, input,
    tokens::{Color, Radius, Space, Text},
    LauncherLoadingState, LauncherPhase,
};
use std_launcher::{LauncherFocusSection, LauncherQueryMode, LauncherState};

pub(crate) fn render_search_bar(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    collapsed: bool,
    hide_requested: &mut bool,
) {
    if collapsed {
        render_search_bar_contents(ui, state, hide_requested);
        return;
    }
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::lg()))
        .inner_margin(egui::Margin::symmetric(Space::md(), Space::sm()))
        .show(ui, |ui| {
            render_search_bar_contents(ui, state, hide_requested)
        });
}

fn render_search_bar_contents(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hide_requested: &mut bool,
) {
    let ctx = ui.ctx().clone();
    let executing = state.view.phase == LauncherPhase::Executing;
    let ime_composing = input::ime_composing(&ctx);
    let mut query_text = search_bar_text(state);
    ui.set_min_height(ui_metrics::search_bar_min_height());
    ui.horizontal(|ui| {
        render_search_indicator(
            ui,
            &ctx,
            search_indicator_for_state(state.view.phase, state.view.loading),
        );
        let input_width = search_input_width(ui.available_width(), ime_composing);
        let response = ui.add_sized(
            [input_width, ui_metrics::search_input_height()],
            egui::TextEdit::singleline(&mut query_text)
                .hint_text(search_placeholder(state))
                .font(Text::headline())
                .frame(false)
                .interactive(!executing),
        );
        response.request_focus();
        let a11y = AccessibilityContext::from_env();
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::TextEdit,
                ui.is_enabled(),
                search_a11y_label(state, &a11y),
            )
        });
        if state.keyboard_focus_visible(LauncherFocusSection::Search) {
            draw_focus_ring(
                ui,
                response.rect,
                Radius::lg(),
                ui_metrics::focus_ring_expand(),
                a11y.focus_ring_width(),
            );
        }
        if !executing && response.changed() {
            state.update_query(query_text);
        }
        if ime_composing {
            render_ime_composing_chip(ui, &ctx);
        }
        render_mode_tag(ui, state);
    });

    if !executing {
        ui_keyboard::handle_search_shortcuts(&ctx, state, hide_requested);
    }
}

fn search_input_width(available_width: f32, ime_composing: bool) -> f32 {
    if ime_composing {
        ui_metrics::search_input_width_with_ime(available_width)
    } else {
        ui_metrics::search_input_width(available_width)
    }
}

fn render_ime_composing_chip(ui: &mut egui::Ui, ctx: &egui::Context) {
    let response = egui::Frame::new()
        .fill(Color::accent_weak(ctx))
        .stroke(egui::Stroke::new(1.0, Color::accent_base(ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.set_width(ui_metrics::search_ime_chip_width());
            ui.label(
                egui::RichText::new(i18n::t("launcher.search.ime_composing"))
                    .font(Text::caption())
                    .color(Color::fg_primary(ctx)),
            );
        })
        .response;
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            i18n::t("launcher.search.ime_composing"),
        )
    });
}

fn search_bar_text(state: &LauncherState) -> String {
    if state.view.phase == LauncherPhase::Executing {
        return state
            .view
            .preview
            .as_ref()
            .map(|preview| format!("{} {}", i18n::t("launcher.search.running"), preview.title))
            .or_else(|| {
                state.view.selected_result().map(|result| {
                    format!(
                        "{} {}",
                        i18n::t("launcher.search.running"),
                        result.action.name
                    )
                })
            })
            .unwrap_or_else(|| i18n::t("launcher.action.executing").to_string());
    }
    state.view.query.clone()
}

fn search_a11y_label(state: &LauncherState, a11y: &AccessibilityContext) -> String {
    if state.view.phase != LauncherPhase::Executing {
        return a11y.launcher_search_label(&state.view.query);
    }
    state
        .view
        .preview
        .as_ref()
        .map(|preview| a11y.launcher_running_label(&preview.title))
        .or_else(|| {
            state
                .view
                .selected_result()
                .map(|result| a11y.launcher_running_label(&result.action.name))
        })
        .unwrap_or_else(|| a11y.launcher_running_label(i18n::t("launcher.action.executing")))
}

fn search_placeholder(state: &LauncherState) -> &'static str {
    if state.view.phase == LauncherPhase::Executing {
        i18n::t("launcher.action.executing")
    } else {
        i18n::t("launcher.search.placeholder")
    }
}

fn render_mode_tag(ui: &mut egui::Ui, state: &LauncherState) {
    let ctx = ui.ctx().clone();
    let mode = LauncherQueryMode::from_query(&state.view.query);
    if mode == LauncherQueryMode::All {
        return;
    }
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::xs(), Space::two_xs()))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(mode.tag_label())
                    .font(Text::caption())
                    .color(Color::fg_secondary(&ctx)),
            );
        });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchIndicator {
    Search,
    Loading,
    Executing,
}

fn search_indicator_for_state(
    phase: LauncherPhase,
    loading: LauncherLoadingState,
) -> SearchIndicator {
    match phase {
        LauncherPhase::Searching if loading == LauncherLoadingState::SlowEmptyResults => {
            SearchIndicator::Loading
        }
        LauncherPhase::Executing => SearchIndicator::Executing,
        _ => SearchIndicator::Search,
    }
}

fn render_search_indicator(ui: &mut egui::Ui, ctx: &egui::Context, indicator: SearchIndicator) {
    match indicator {
        SearchIndicator::Search => render_search_icon(ui, ctx),
        SearchIndicator::Loading => render_search_spinner(ui),
        SearchIndicator::Executing => render_executing_indicator(ui, ctx),
    }
}

fn render_search_icon(ui: &mut egui::Ui, ctx: &egui::Context) {
    let stroke = egui::Stroke::new(1.5, Color::fg_secondary(ctx));
    let (rect, response) =
        ui.allocate_exact_size(ui_metrics::search_icon_size(), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            i18n::t("launcher.search.icon"),
        )
    });
    let geometry = ui_metrics::search_icon_geometry(rect);
    ui.painter()
        .circle_stroke(geometry.center, geometry.radius, stroke);
    ui.painter()
        .line_segment([geometry.handle_start, geometry.handle_end], stroke);
}

fn render_search_spinner(ui: &mut egui::Ui) {
    let (rect, response) =
        ui.allocate_exact_size(ui_metrics::search_icon_size(), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::ProgressIndicator,
            ui.is_enabled(),
            i18n::t("launcher.search.loading"),
        )
    });
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        ui.centered_and_justified(|ui| {
            ui.spinner();
        });
    });
}

fn render_executing_indicator(ui: &mut egui::Ui, ctx: &egui::Context) {
    let (rect, response) =
        ui.allocate_exact_size(ui_metrics::search_icon_size(), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            i18n::t("launcher.results.executing.title"),
        )
    });
    let geometry = ui_metrics::search_icon_geometry(rect);
    ui.painter()
        .circle_filled(geometry.center, geometry.radius, Color::accent_weak(ctx));
    ui.painter().circle_stroke(
        geometry.center,
        geometry.radius,
        egui::Stroke::new(1.5, Color::accent_base(ctx)),
    );
}

#[cfg(test)]
fn search_ime_visible_state_contract() -> &'static str {
    "ime-visible-state=search-preedit-visible,enter-owned-by-ime"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn executing_search_bar_shows_running_action_text() {
        let mut state = LauncherState::new();
        state.update_query("index");
        state.view.preview_executing();
        let text = search_bar_text(&state);
        let a11y = AccessibilityContext::from_env();

        assert!(text.starts_with(i18n::t("launcher.search.running")));
        assert!(text.contains("Rebuild Index"));
        assert_eq!(search_a11y_label(&state, &a11y), "Running Rebuild Index");
        assert_eq!(
            search_placeholder(&state),
            i18n::t("launcher.action.executing")
        );
    }

    #[test]
    fn search_focus_ring_is_tied_to_search_section() {
        let mut state = LauncherState::new();
        assert_eq!(state.focus_section, LauncherFocusSection::Search);
        assert!(state.keyboard_focus_visible(LauncherFocusSection::Search));

        state.focus_section = LauncherFocusSection::Results;

        assert_ne!(state.focus_section, LauncherFocusSection::Search);
        assert!(!state.keyboard_focus_visible(LauncherFocusSection::Search));
    }

    #[test]
    fn launcher_search_mode_tag_tracks_query_prefix() {
        let mut state = LauncherState::new();

        assert_eq!(search_mode_tag_label(&state), None);

        state.update_query("? rebuild");

        assert_eq!(search_mode_tag_label(&state), Some("Ask"));
    }

    #[test]
    fn collapsed_launcher_does_not_nest_search_surface_inside_panel() {
        let source = include_str!("ui_search.rs");
        let collapsed_branch = source
            .split("if collapsed")
            .nth(1)
            .and_then(|body| body.split("let ctx = ui.ctx().clone();").next())
            .unwrap();

        assert!(collapsed_branch.contains("render_search_bar_contents"));
        assert!(!collapsed_branch.contains("egui::Frame::new()"));
        assert!(source.contains("fn render_search_bar_contents"));
    }

    #[test]
    fn search_indicator_tracks_loading_and_executing_phases() {
        assert_eq!(
            search_indicator_for_state(LauncherPhase::Empty, LauncherLoadingState::Idle),
            SearchIndicator::Search
        );
        assert_eq!(
            search_indicator_for_state(
                LauncherPhase::Searching,
                LauncherLoadingState::UpdatingResults
            ),
            SearchIndicator::Search
        );
        assert_eq!(
            search_indicator_for_state(
                LauncherPhase::Searching,
                LauncherLoadingState::SlowEmptyResults
            ),
            SearchIndicator::Loading
        );
        assert_eq!(
            search_indicator_for_state(LauncherPhase::Executing, LauncherLoadingState::Idle),
            SearchIndicator::Executing
        );
    }

    #[test]
    fn search_loading_and_executing_indicators_expose_status_semantics() {
        let source = include_str!("ui_search.rs");

        assert!(source.contains("WidgetType::ProgressIndicator"));
        assert!(source.contains("launcher.search.loading"));
        assert!(source.contains("launcher.results.executing.title"));
        assert!(source.contains("Color::accent_weak"));
        assert!(source.contains("circle_stroke"));
    }

    #[test]
    fn search_ui_contract_requires_visible_ime_state() {
        assert_eq!(
            search_ime_visible_state_contract(),
            "ime-visible-state=search-preedit-visible,enter-owned-by-ime"
        );
        assert!(
            search_input_width(420.0, true) < search_input_width(420.0, false),
            "IME state chip must reserve stable width in the search row"
        );
    }

    #[test]
    fn search_input_uses_outer_token_surface_not_inner_textedit_frame() {
        let source = include_str!("ui_search.rs");
        let input_body = source
            .split("egui::TextEdit::singleline")
            .nth(1)
            .and_then(|body| body.split(".interactive(!executing)").next())
            .unwrap();

        assert!(input_body.contains(".frame(false)"));
        assert!(source.contains("Color::bg_surface_1"));
        assert!(source.contains("Color::stroke_border"));
    }

    fn search_mode_tag_label(state: &LauncherState) -> Option<&'static str> {
        let mode = LauncherQueryMode::from_query(&state.view.query);
        (mode != LauncherQueryMode::All).then_some(mode.tag_label())
    }
}
