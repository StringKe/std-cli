use eframe::egui;
use std_egui::tokens::{Space, UiScale};
use std_egui::LauncherPhase;
use std_launcher::LauncherState;

const PANEL_WIDTH: f32 = 720.0;
const SEARCH_HEIGHT: f32 = 64.0;
const ACTION_BAR_HEIGHT: f32 = 36.0;
const RESULT_ROW_HEIGHT: f32 = 36.0;
const GROUP_ROW_HEIGHT: f32 = 24.0;
const MAX_RESULT_ROWS: f32 = 6.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 520.0;

pub(crate) fn panel_width() -> f32 {
    UiScale::from_env().f32(PANEL_WIDTH)
}

pub(crate) fn scale() -> UiScale {
    UiScale::from_env()
}

pub(crate) fn window_margin() -> f32 {
    Space::sm() as f32
}

pub(crate) fn result_row_height() -> f32 {
    scale().f32(RESULT_ROW_HEIGHT)
}

pub(crate) fn ask_ai_row_height() -> f32 {
    scale().f32(34.0)
}

pub(crate) fn action_bar_content_height() -> f32 {
    scale().f32(24.0)
}

pub(crate) fn action_summary_label_height() -> f32 {
    scale().f32(18.0)
}

pub(crate) fn initial_window_inner_size() -> egui::Vec2 {
    initial_window_inner_size_for_scale(UiScale::from_env())
}

pub(crate) fn window_inner_size(state: &LauncherState) -> egui::Vec2 {
    let scale = UiScale::from_env();
    let body_height = body_height_for_scale(state, DEFAULT_VIEWPORT_HEIGHT, scale);
    egui::vec2(
        scale.f32(PANEL_WIDTH) + scale.f32(Space::SM as f32) * 2.0,
        panel_height_for_scale(state, body_height, scale) + scale.f32(Space::SM as f32) * 2.0,
    )
}

pub(crate) fn panel_height(state: &LauncherState, body_height: f32) -> f32 {
    panel_height_for_scale(state, body_height, UiScale::from_env())
}

pub(crate) fn body_height(state: &LauncherState, viewport_height: f32) -> f32 {
    body_height_for_scale(state, viewport_height, UiScale::from_env())
}

pub(crate) fn panel_is_expanded(state: &LauncherState) -> bool {
    (state.view.phase != LauncherPhase::Empty || !state.view.results.is_empty())
        || state.controller.voice_active
        || state.view.feedback.is_some()
        || state.action_panel.open
}

fn initial_window_inner_size_for_scale(scale: UiScale) -> egui::Vec2 {
    egui::vec2(
        scale.f32(PANEL_WIDTH) + scale.f32(Space::SM as f32) * 2.0,
        scale.f32(SEARCH_HEIGHT) + scale.f32(Space::SM as f32) * 2.0,
    )
}

fn panel_height_for_scale(state: &LauncherState, body_height: f32, scale: UiScale) -> f32 {
    if !panel_is_expanded(state) {
        return scale.f32(SEARCH_HEIGHT);
    }
    scale.f32(SEARCH_HEIGHT)
        + body_height
        + scale.f32(ACTION_BAR_HEIGHT)
        + scale.f32(Space::MD as f32)
        + scale.f32(Space::SM as f32)
        + extra_status_height_for_scale(state, scale)
}

fn body_height_for_scale(state: &LauncherState, viewport_height: f32, scale: UiScale) -> f32 {
    let visible_rows = state.view.results.len().clamp(1, MAX_RESULT_ROWS as usize) as f32;
    let groups = crate::ui_results::group_count(&state.view.results).max(1) as f32;
    let desired = visible_rows * scale.f32(RESULT_ROW_HEIGHT)
        + groups * scale.f32(GROUP_ROW_HEIGHT)
        + scale.f32(Space::SM as f32);
    desired.clamp(scale.f32(128.0), viewport_height * 0.6)
}

fn extra_status_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    let mut height = 0.0;
    if state.controller.voice_active {
        height += scale.f32(44.0) + scale.f32(Space::XS as f32);
    }
    if state.view.feedback.is_some() {
        height += scale.f32(48.0) + scale.f32(Space::XS as f32);
    }
    height
}

#[cfg(test)]
fn row_metrics_for_scale(scale: UiScale) -> (f32, f32, f32, f32) {
    (
        scale.f32(RESULT_ROW_HEIGHT),
        scale.f32(34.0),
        scale.f32(24.0),
        scale.f32(18.0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_window_size_scales_with_ui_zoom() {
        let base = initial_window_inner_size_for_scale(UiScale::default());
        let zoomed = initial_window_inner_size_for_scale(UiScale::new(1.5));

        assert_eq!(base, egui::vec2(744.0, 88.0));
        assert_eq!(zoomed, egui::vec2(1116.0, 132.0));
    }

    #[test]
    fn expanded_window_size_scales_with_ui_zoom() {
        let mut state = LauncherState::new();
        state.update_query("index");
        let body_height = body_height_for_scale(&state, DEFAULT_VIEWPORT_HEIGHT, UiScale::new(1.5));
        let height = panel_height_for_scale(&state, body_height, UiScale::new(1.5));

        assert!(height > initial_window_inner_size_for_scale(UiScale::new(1.5)).y);
    }

    #[test]
    fn row_metrics_scale_with_ui_zoom() {
        assert_eq!(
            row_metrics_for_scale(UiScale::new(1.5)),
            (54.0, 51.0, 36.0, 27.0)
        );
    }
}
