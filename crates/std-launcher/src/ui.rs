use crate::{
    ui_action_bar, ui_action_panel, ui_feedback, ui_keyboard, ui_metrics, ui_parts::quiet_button,
    ui_results,
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n,
    tokens::{Color, Elevation, Radius, Space, Text},
    LauncherPhase,
};
use std_launcher::LauncherState;

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
    let margin = ui_metrics::window_margin();
    let panel_width = ui_metrics::panel_width().min((available.width() - margin * 2.0).max(320.0));
    let body_height = ui_metrics::body_height(state, available.height());
    let panel_height = ui_metrics::panel_height(state, body_height);
    let rect = egui::Rect::from_min_size(
        egui::pos2(
            available.center().x - panel_width * 0.5,
            available.top() + margin,
        ),
        egui::vec2(
            panel_width,
            panel_height.min(available.height() - margin * 2.0),
        ),
    );

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
    egui::Frame::new()
        .fill(Color::bg_surface_0(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::xl()))
        .shadow(Elevation::level_3(&ctx))
        .inner_margin(egui::Margin::same(Space::md()))
        .show(ui, |ui| {
            ui.set_width(panel_rect.width() - Space::md() as f32 * 2.0);
            render_search_bar(ui, state, &mut hide_requested);
            if !ui_metrics::panel_is_expanded(state) {
                return;
            }
            ui.add_space(Space::xs() as f32);
            render_body(ui, state, body_height);
            ui.add_space(Space::xs() as f32);
            let action_bar_rect = ui_action_bar::render(ui, state, hotkey_status, resident_status);
            render_voice(ui, state, voice_transcript);
            ui_feedback::render(ui, state);
            ui_action_panel::render(ui.ctx(), action_bar_rect, state);
        });
    hide_requested
}

fn render_search_bar(ui: &mut egui::Ui, state: &mut LauncherState, hide_requested: &mut bool) {
    let ctx = ui.ctx().clone();
    let executing = state.view.phase == LauncherPhase::Executing;
    let mut query_text = search_bar_text(state);
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::lg()))
        .inner_margin(egui::Margin::symmetric(Space::md(), Space::sm()))
        .show(ui, |ui| {
            ui.set_min_height(ui_metrics::search_bar_min_height());
            ui.horizontal(|ui| {
                render_search_icon(ui, &ctx);
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
                draw_focus_ring(ui, response.rect, Radius::lg(), a11y.focus_ring_width());
                if !executing && response.changed() {
                    state.update_query(query_text);
                }
                if quiet_button(ui, "Esc").clicked() {
                    *hide_requested = true;
                }
            });
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

fn render_body(ui: &mut egui::Ui, state: &mut LauncherState, max_height: f32) {
    ui_results::render(ui, state, max_height);
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

fn draw_focus_ring(ui: &egui::Ui, rect: egui::Rect, radius: u8, width: f32) {
    let outer = rect.expand(ui_metrics::focus_ring_expand());
    ui.painter().rect_stroke(
        outer,
        egui::CornerRadius::same(radius),
        egui::Stroke::new(width, Color::accent_base(ui.ctx())),
        egui::StrokeKind::Outside,
    );
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
}
