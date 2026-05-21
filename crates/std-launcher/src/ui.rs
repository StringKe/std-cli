use crate::{
    ui_action_bar, ui_action_panel, ui_feedback, ui_keyboard, ui_metrics,
    ui_parts::{draw_focus_ring, quiet_button},
    ui_results,
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n,
    tokens::{Color, Elevation, Radius, Space, Text},
    LauncherPhase,
};
use std_launcher::LauncherFocusSection;
use std_launcher::LauncherQueryMode;
use std_launcher::LauncherState;

pub(crate) fn render_launcher_viewport(
    ctx: &egui::Context,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
) -> bool {
    let mut hide_requested = false;
    egui::CentralPanel::default()
        .frame(launcher_viewport_frame())
        .show(ctx, |ui| {
            hide_requested = render_launcher_overlay(
                ui,
                state,
                hotkey_status,
                resident_status,
                voice_transcript,
            );
        });
    hide_requested
}

pub(crate) fn launcher_viewport_frame() -> egui::Frame {
    egui::Frame::NONE.fill(egui::Color32::TRANSPARENT)
}

pub(crate) fn launcher_initial_window_inner_size() -> egui::Vec2 {
    ui_metrics::initial_window_inner_size()
}

pub(crate) fn launcher_window_inner_size(state: &LauncherState) -> egui::Vec2 {
    ui_metrics::window_inner_size(state)
}

pub(crate) fn render_launcher_overlay(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
) -> bool {
    let available = ui.max_rect();
    let body_height = ui_metrics::body_height(state, available.height());
    let rect = ui_metrics::panel_rect(available, state);

    let mut hide_requested = false;
    ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
        hide_requested = render_launcher_panel(
            ui,
            state,
            hotkey_status,
            resident_status,
            voice_transcript,
            body_height,
        );
    });
    hide_requested
}

pub(crate) fn render_launcher_panel(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
    body_height: f32,
) -> bool {
    let mut hide_requested = false;
    let ctx = ui.ctx().clone();
    let panel_rect = ui.max_rect();
    ui.set_min_width(panel_rect.width());
    ui.set_min_height(panel_rect.height());
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::xl()))
        .shadow(Elevation::level_3(&ctx))
        .inner_margin(egui::Margin::same(
            ui_metrics::panel_inner_padding_for_state(state) as i8,
        ))
        .show(ui, |ui| {
            let padding = ui_metrics::panel_inner_padding_for_state(state);
            ui.set_min_height(panel_rect.height() - padding * 2.0);
            ui.set_width(panel_rect.width() - padding * 2.0);
            let collapsed = !ui_metrics::panel_is_expanded(state);
            render_search_bar(ui, state, collapsed, &mut hide_requested);
            if !ui_metrics::panel_is_expanded(state) {
                return;
            }
            ui.add_space(Space::xs() as f32);
            hide_requested |= render_body(ui, state, body_height);
            ui.add_space(Space::xs() as f32);
            let action_bar_rect = ui_action_bar::render(ui, state, hotkey_status, resident_status);
            render_voice(ui, state, voice_transcript);
            ui_feedback::render(ui, state);
            ui_action_panel::render(ui.ctx(), action_bar_rect, state);
        });
    hide_requested
}

fn render_search_bar(
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
    let mut query_text = search_bar_text(state);
    ui.set_min_height(ui_metrics::search_bar_min_height());
    ui.horizontal(|ui| {
        render_search_indicator(ui, &ctx, search_indicator_for_phase(state.view.phase));
        let response = ui.add_sized(
            [
                ui_metrics::search_input_width(ui.available_width()),
                ui_metrics::search_input_height(),
            ],
            egui::TextEdit::singleline(&mut query_text)
                .hint_text(search_placeholder(state))
                .font(Text::headline())
                .interactive(!executing),
        );
        response.request_focus();
        let a11y = AccessibilityContext::from_env();
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::TextEdit,
                ui.is_enabled(),
                a11y.launcher_search_label(&state.view.query),
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
        render_mode_tag(ui, state);
    });

    if !executing {
        ui_keyboard::handle_search_shortcuts(&ctx, state, hide_requested);
    }
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

fn render_body(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) -> bool {
    ui_results::render(ui, state, max_height)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchIndicator {
    Search,
    Loading,
    Executing,
}

fn search_indicator_for_phase(phase: LauncherPhase) -> SearchIndicator {
    match phase {
        LauncherPhase::Searching => SearchIndicator::Loading,
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
    let (rect, _response) =
        ui.allocate_exact_size(ui_metrics::search_icon_size(), egui::Sense::hover());
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
        .circle_filled(geometry.center, geometry.radius, Color::accent_base(ctx));
}

fn render_voice(ui: &mut egui::Ui, state: &mut LauncherState, voice_transcript: &mut String) {
    if !state.controller.voice_active {
        return;
    }
    let ctx = ui.ctx().clone();
    ui.add_space(Space::xs() as f32);
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::md()))
        .inner_margin(egui::Margin::same(Space::xs()))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(i18n::t("launcher.voice.label"))
                        .color(Color::fg_secondary(&ctx)),
                );
                ui.add_sized(
                    [
                        ui_metrics::voice_input_width(ui.available_width()),
                        ui_metrics::voice_input_height(),
                    ],
                    egui::TextEdit::singleline(voice_transcript)
                        .hint_text(i18n::t("launcher.voice.placeholder")),
                );
                if quiet_button(ui, i18n::t("launcher.voice.apply")).clicked() {
                    state.apply_voice_transcript(voice_transcript.as_str());
                }
            });
        });
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

        assert!(text.starts_with(i18n::t("launcher.search.running")));
        assert!(text.contains("Rebuild Index"));
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
    fn launcher_viewport_frame_is_transparent_and_unstyled() {
        let frame = launcher_viewport_frame();

        assert_eq!(frame.fill, egui::Color32::TRANSPARENT);
        assert_eq!(frame.stroke, egui::Stroke::NONE);
    }

    #[test]
    fn launcher_panel_forces_frame_to_computed_viewport_size() {
        let source = include_str!("ui.rs");

        assert!(source.contains("ui.set_min_width(panel_rect.width())"));
        assert!(source.contains("ui.set_min_height(panel_rect.height())"));
        assert!(source.contains("ui.set_min_height(panel_rect.height() - padding * 2.0)"));
    }

    #[test]
    fn collapsed_launcher_does_not_nest_search_surface_inside_panel() {
        let source = include_str!("ui.rs");
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
            search_indicator_for_phase(LauncherPhase::Empty),
            SearchIndicator::Search
        );
        assert_eq!(
            search_indicator_for_phase(LauncherPhase::Searching),
            SearchIndicator::Loading
        );
        assert_eq!(
            search_indicator_for_phase(LauncherPhase::Executing),
            SearchIndicator::Executing
        );
    }

    fn search_mode_tag_label(state: &LauncherState) -> Option<&'static str> {
        let mode = LauncherQueryMode::from_query(&state.view.query);
        (mode != LauncherQueryMode::All).then_some(mode.tag_label())
    }
}
