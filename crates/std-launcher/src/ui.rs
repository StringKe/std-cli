use crate::{
    ui_action_panel, ui_keyboard,
    ui_parts::{keycap, quiet_button, quiet_label, weak_status_fill},
    ui_results,
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n, input,
    tokens::{Color, Elevation, Radius, Space, Text},
    LauncherFeedback, LauncherPhase,
};
use std_launcher::{LauncherPerformanceReport, LauncherState};
use std_types::ActionExecutionStatus;

const PANEL_WIDTH: f32 = 720.0;
const WINDOW_MARGIN: f32 = Space::SM as f32;
const SEARCH_HEIGHT: f32 = 64.0;
const ACTION_BAR_HEIGHT: f32 = 36.0;
const RESULT_ROW_HEIGHT: f32 = 36.0;
const GROUP_ROW_HEIGHT: f32 = 24.0;
const MAX_RESULT_ROWS: f32 = 6.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 520.0;

pub(crate) fn launcher_initial_window_inner_size() -> egui::Vec2 {
    egui::vec2(
        PANEL_WIDTH + WINDOW_MARGIN * 2.0,
        SEARCH_HEIGHT + WINDOW_MARGIN * 2.0,
    )
}

pub(crate) fn launcher_window_inner_size(state: &LauncherState) -> egui::Vec2 {
    let body_height = launcher_body_height(state, DEFAULT_VIEWPORT_HEIGHT);
    egui::vec2(
        PANEL_WIDTH + WINDOW_MARGIN * 2.0,
        launcher_panel_height(state, body_height) + WINDOW_MARGIN * 2.0,
    )
}

pub(crate) fn render_launcher_overlay(
    ui: &mut egui::Ui,
    state: &mut LauncherState,
    hotkey_status: &str,
    resident_status: &str,
    voice_transcript: &mut String,
) -> bool {
    let available = ui.max_rect();
    let panel_width = PANEL_WIDTH.min((available.width() - WINDOW_MARGIN * 2.0).max(320.0));
    let body_height = launcher_body_height(state, available.height());
    let panel_height = launcher_panel_height(state, body_height);
    let rect = egui::Rect::from_min_size(
        egui::pos2(
            available.center().x - panel_width * 0.5,
            available.top() + WINDOW_MARGIN,
        ),
        egui::vec2(
            panel_width,
            panel_height.min(available.height() - WINDOW_MARGIN * 2.0),
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
        .corner_radius(egui::CornerRadius::same(Radius::XL))
        .shadow(Elevation::level_3(&ctx))
        .inner_margin(egui::Margin::same(Space::MD))
        .show(ui, |ui| {
            ui.set_width(panel_rect.width() - Space::MD as f32 * 2.0);
            render_search_bar(ui, state, &mut hide_requested);
            if !launcher_panel_is_expanded(state) {
                return;
            }
            ui.add_space(Space::XS as f32);
            render_body(ui, state, body_height);
            ui.add_space(Space::XS as f32);
            let action_bar_rect = render_action_bar(ui, state, hotkey_status, resident_status);
            render_voice(ui, state, voice_transcript);
            render_feedback(ui, state);
            ui_action_panel::render(ui.ctx(), action_bar_rect, state);
        });
    hide_requested
}

fn launcher_panel_height(state: &LauncherState, body_height: f32) -> f32 {
    if !launcher_panel_is_expanded(state) {
        return SEARCH_HEIGHT;
    }
    SEARCH_HEIGHT
        + body_height
        + ACTION_BAR_HEIGHT
        + Space::MD as f32
        + Space::SM as f32
        + extra_status_height(state)
}

fn launcher_panel_is_expanded(state: &LauncherState) -> bool {
    (state.view.phase != LauncherPhase::Empty || !state.view.results.is_empty())
        || state.controller.voice_active
        || state.view.feedback.is_some()
        || state.action_panel.open
}

fn launcher_body_height(state: &LauncherState, viewport_height: f32) -> f32 {
    let visible_rows = state.view.results.len().clamp(1, MAX_RESULT_ROWS as usize) as f32;
    let groups = ui_results::group_count(&state.view.results).max(1) as f32;
    let desired = visible_rows * RESULT_ROW_HEIGHT + groups * GROUP_ROW_HEIGHT + Space::SM as f32;
    desired.clamp(launcher_body_min_height(), viewport_height * 0.6)
}

fn launcher_body_min_height() -> f32 {
    128.0
}

fn extra_status_height(state: &LauncherState) -> f32 {
    let mut height = 0.0;
    if state.controller.voice_active {
        height += 44.0 + Space::XS as f32;
    }
    if state.view.feedback.is_some() {
        height += 48.0 + Space::XS as f32;
    }
    height
}

fn render_search_bar(ui: &mut egui::Ui, state: &mut LauncherState, hide_requested: &mut bool) {
    let ctx = ui.ctx().clone();
    let executing = state.view.phase == LauncherPhase::Executing;
    let mut query_text = search_bar_text(state);
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_border(&ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::LG))
        .inner_margin(egui::Margin::symmetric(Space::MD, Space::SM))
        .show(ui, |ui| {
            ui.set_min_height(44.0);
            ui.horizontal(|ui| {
                render_search_icon(ui, &ctx);
                let response = ui.add_sized(
                    [ui.available_width() - 92.0, 36.0],
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
                draw_focus_ring(ui, response.rect, Radius::LG, a11y.focus_ring_width());
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
    let (rect, response) = ui.allocate_exact_size(egui::vec2(24.0, 28.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Other,
            ui.is_enabled(),
            i18n::t("launcher.search.icon"),
        )
    });
    let center = egui::pos2(rect.left() + 10.0, rect.center().y - 2.0);
    ui.painter().circle_stroke(center, 5.0, stroke);
    ui.painter().line_segment(
        [
            egui::pos2(center.x + 4.0, center.y + 4.0),
            egui::pos2(center.x + 9.0, center.y + 9.0),
        ],
        stroke,
    );
}

fn draw_focus_ring(ui: &egui::Ui, rect: egui::Rect, radius: u8, width: f32) {
    let outer = rect.expand(3.0);
    ui.painter().rect_stroke(
        outer,
        egui::CornerRadius::same(radius),
        egui::Stroke::new(width, Color::accent_base(ui.ctx())),
        egui::StrokeKind::Outside,
    );
}

fn render_action_bar(
    ui: &mut egui::Ui,
    state: &LauncherState,
    hotkey_status: &str,
    resident_status: &str,
) -> egui::Rect {
    let ctx = ui.ctx().clone();
    egui::Frame::new()
        .fill(Color::bg_surface_1(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let right_width = 272.0_f32.min(ui.available_width() * 0.48);
                let left_width = (ui.available_width() - right_width - Space::XS as f32).max(160.0);
                ui.allocate_ui_with_layout(
                    egui::vec2(left_width, 24.0),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| render_action_summary(ui, state, left_width),
                );
                ui.allocate_ui_with_layout(
                    egui::vec2(right_width, 24.0),
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        keycap(ui, &input::launcher_action_panel().label());
                        quiet_label(ui, i18n::t("launcher.action.actions"));
                        keycap(ui, "Enter");
                        quiet_label(ui, i18n::t("launcher.action.run"));
                        quiet_label(ui, hotkey_status);
                        ui.label(
                            egui::RichText::new(resident_status)
                                .font(Text::caption())
                                .color(Color::fg_secondary(&ctx)),
                        );
                    },
                );
            });
        })
        .response
        .rect
}

fn render_action_summary(ui: &mut egui::Ui, state: &LauncherState, max_width: f32) {
    let ctx = ui.ctx().clone();
    if state.view.phase == LauncherPhase::Executing {
        ui.horizontal(|ui| {
            ui.spinner();
            ui.label(
                egui::RichText::new(i18n::t("launcher.action.executing"))
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            );
        });
        return;
    }
    if let Some(preview) = state.view.preview.as_ref() {
        ui.add_sized(
            [max_width * 0.34, 18.0],
            egui::Label::new(
                egui::RichText::new(&preview.title)
                    .font(Text::footnote())
                    .color(Color::fg_primary(&ctx))
                    .strong(),
            )
            .truncate(),
        );
        ui.add_sized(
            [max_width * 0.62, 18.0],
            egui::Label::new(
                egui::RichText::new(&preview.subtitle)
                    .font(Text::caption())
                    .color(Color::fg_secondary(&ctx)),
            )
            .truncate(),
        );
        return;
    }
    ui.add_sized(
        [max_width, 18.0],
        egui::Label::new(
            egui::RichText::new(i18n::t("launcher.action.command_hint"))
                .font(Text::footnote())
                .color(Color::fg_secondary(&ctx)),
        )
        .truncate(),
    );
}

fn render_voice(ui: &mut egui::Ui, state: &mut LauncherState, voice_transcript: &mut String) {
    if !state.controller.voice_active {
        return;
    }
    let ctx = ui.ctx().clone();
    ui.add_space(Space::XS as f32);
    egui::Frame::new()
        .fill(Color::bg_surface_2(&ctx))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::same(Space::XS))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(i18n::t("launcher.voice.label"))
                        .color(Color::fg_secondary(&ctx)),
                );
                ui.add_sized(
                    [ui.available_width() - 112.0, 28.0],
                    egui::TextEdit::singleline(voice_transcript)
                        .hint_text(i18n::t("launcher.voice.placeholder")),
                );
                if quiet_button(ui, i18n::t("launcher.voice.apply")).clicked() {
                    state.apply_voice_transcript(voice_transcript.as_str());
                }
            });
        });
}

fn render_feedback(ui: &mut egui::Ui, state: &LauncherState) {
    let ctx = ui.ctx().clone();
    let feedback = state.view.feedback.as_ref();
    if feedback.is_none() {
        return;
    }
    egui::Frame::new()
        .fill(feedback_fill(&ctx, feedback))
        .stroke(egui::Stroke::new(1.0, feedback_stroke(&ctx, feedback)))
        .corner_radius(egui::CornerRadius::same(Radius::MD))
        .inner_margin(egui::Margin::symmetric(Space::SM, Space::XS))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                if let Some(feedback) = feedback {
                    ui.label(
                        egui::RichText::new(feedback_marker(feedback))
                            .font(Text::body())
                            .color(feedback_stroke(&ctx, Some(feedback))),
                    );
                    ui.label(
                        egui::RichText::new(&feedback.title)
                            .font(Text::body())
                            .color(Color::fg_primary(&ctx))
                            .strong(),
                    );
                    ui.label(
                        egui::RichText::new(&feedback.action_name)
                            .font(Text::footnote())
                            .color(Color::fg_primary(&ctx)),
                    );
                    ui.label(
                        egui::RichText::new(&feedback.detail)
                            .font(Text::footnote())
                            .color(Color::fg_secondary(&ctx)),
                    );
                }
                render_performance(ui, &state.performance_report());
            });
        });
}

fn render_performance(ui: &mut egui::Ui, report: &LauncherPerformanceReport) {
    let ctx = ui.ctx().clone();
    let text = format!(
        "{}ms search  {}ms preview  {}ms action",
        report.last_search_ms, report.last_preview_ms, report.last_trigger_ms
    );
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(
            egui::RichText::new(text)
                .font(Text::code())
                .color(Color::fg_secondary(&ctx)),
        );
    });
}

fn feedback_fill(ctx: &egui::Context, feedback: Option<&LauncherFeedback>) -> egui::Color32 {
    match feedback.map(|feedback| &feedback.status) {
        Some(ActionExecutionStatus::Completed) => weak_status_fill(ctx, Color::status_success(ctx)),
        Some(ActionExecutionStatus::Failed) => weak_status_fill(ctx, Color::status_danger(ctx)),
        Some(ActionExecutionStatus::NeedsExternalRunner) => {
            weak_status_fill(ctx, Color::status_warning(ctx))
        }
        None => Color::bg_surface_1(ctx),
    }
}

fn feedback_stroke(ctx: &egui::Context, feedback: Option<&LauncherFeedback>) -> egui::Color32 {
    match feedback.map(|feedback| &feedback.status) {
        Some(ActionExecutionStatus::Completed) => Color::status_success(ctx),
        Some(ActionExecutionStatus::Failed) => Color::status_danger(ctx),
        Some(ActionExecutionStatus::NeedsExternalRunner) => Color::status_warning(ctx),
        None => Color::stroke_divider(ctx),
    }
}

fn feedback_marker(feedback: &LauncherFeedback) -> &'static str {
    match feedback.status {
        ActionExecutionStatus::Completed => "OK",
        ActionExecutionStatus::Failed => "ERR",
        ActionExecutionStatus::NeedsExternalRunner => "WAIT",
    }
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
