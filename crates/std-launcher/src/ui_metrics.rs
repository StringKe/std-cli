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
const PANEL_VERTICAL_ANCHOR: f32 = 0.28;

pub(crate) fn panel_width() -> f32 {
    UiScale::from_env().f32(PANEL_WIDTH)
}

pub(crate) fn scale() -> UiScale {
    UiScale::from_env()
}

pub(crate) fn window_margin() -> f32 {
    0.0
}

pub(crate) fn panel_rect(available: egui::Rect, state: &LauncherState) -> egui::Rect {
    let margin = window_margin();
    let panel_width = panel_width().min((available.width() - margin * 2.0).max(320.0));
    let body_height = body_height(state, available.height());
    let computed_height = panel_height(state, body_height).min(available.height() - margin * 2.0);
    let panel_height = if native_viewport_is_panel_sized(available, panel_width, margin) {
        available.height()
    } else {
        computed_height
    };
    panel_rect_for_available(available, panel_width, panel_height, margin, false)
}

fn native_viewport_is_panel_sized(available: egui::Rect, panel_width: f32, margin: f32) -> bool {
    margin == 0.0 && (available.width() - panel_width).abs() <= 1.0
}

fn panel_rect_for_available(
    available: egui::Rect,
    panel_width: f32,
    panel_height: f32,
    margin: f32,
    anchor_to_screen: bool,
) -> egui::Rect {
    let target_top = if anchor_to_screen {
        available.top() + available.height() * PANEL_VERTICAL_ANCHOR
    } else {
        available.top() + margin
    };
    let min_y = available.top() + margin;
    let max_y = available.bottom() - margin - panel_height;
    let top = target_top.clamp(min_y, max_y.max(min_y));
    egui::Rect::from_min_size(
        egui::pos2(available.center().x - panel_width * 0.5, top),
        egui::vec2(panel_width, panel_height),
    )
}

pub(crate) fn result_row_height() -> f32 {
    scale().f32(RESULT_ROW_HEIGHT)
}

pub(crate) fn loading_progress_rect(available_width: f32, top_left: egui::Pos2) -> egui::Rect {
    crate::ui_metrics_results::loading_progress_rect(scale(), available_width, top_left)
}

pub(crate) fn group_divider_rect(available_width: f32, top_left: egui::Pos2) -> egui::Rect {
    crate::ui_metrics_results::group_divider_rect(scale(), available_width, top_left)
}

pub(crate) fn ask_ai_row_height() -> f32 {
    scale().f32(34.0)
}

pub(crate) fn no_matches_icon_size() -> egui::Vec2 {
    crate::ui_metrics_empty::no_matches_icon_size(scale())
}

pub(crate) fn no_matches_icon_geometry(
    rect: egui::Rect,
) -> crate::ui_metrics_empty::EmptySearchIconGeometry {
    crate::ui_metrics_empty::no_matches_icon_geometry(scale(), rect)
}

pub(crate) fn action_bar_content_height() -> f32 {
    scale().f32(24.0)
}

pub(crate) fn action_summary_label_height() -> f32 {
    scale().f32(18.0)
}

pub(crate) fn feedback_text_height() -> f32 {
    scale().f32(40.0)
}

pub(crate) fn feedback_detail_height() -> f32 {
    scale().f32(18.0)
}

pub(crate) fn search_bar_min_height() -> f32 {
    crate::ui_metrics_search::search_bar_min_height(scale())
}

pub(crate) fn search_input_width(available_width: f32) -> f32 {
    crate::ui_metrics_search::search_input_width(scale(), available_width)
}

pub(crate) fn search_input_height() -> f32 {
    crate::ui_metrics_search::search_input_height(scale())
}

pub(crate) fn search_icon_size() -> egui::Vec2 {
    crate::ui_metrics_search::search_icon_size(scale())
}

pub(crate) fn search_icon_geometry(
    rect: egui::Rect,
) -> crate::ui_metrics_search::SearchIconGeometry {
    crate::ui_metrics_search::search_icon_geometry(scale(), rect)
}

pub(crate) fn focus_ring_expand() -> f32 {
    crate::ui_metrics_search::focus_ring_expand(scale())
}

pub(crate) fn voice_input_width(available_width: f32) -> f32 {
    crate::ui_metrics_search::voice_input_width(scale(), available_width)
}

pub(crate) fn voice_input_height() -> f32 {
    crate::ui_metrics_search::voice_input_height(scale())
}

pub(crate) fn action_panel_width(anchor_width: f32) -> f32 {
    crate::ui_metrics_action_panel::width(scale(), anchor_width)
}

pub(crate) fn action_panel_height(item_count: usize) -> f32 {
    crate::ui_metrics_action_panel::height(scale(), item_count)
}

pub(crate) fn action_panel_search_height() -> f32 {
    crate::ui_metrics_action_panel::search_height(scale())
}

pub(crate) fn action_panel_focus_expand() -> f32 {
    crate::ui_metrics_action_panel::focus_expand(scale())
}

pub(crate) fn action_panel_row_height() -> f32 {
    crate::ui_metrics_action_panel::row_height(scale())
}

pub(crate) fn initial_window_inner_size() -> egui::Vec2 {
    initial_window_inner_size_for_scale(UiScale::from_env())
}

pub(crate) fn window_inner_size(state: &LauncherState) -> egui::Vec2 {
    let scale = UiScale::from_env();
    let body_height = body_height_for_scale(state, DEFAULT_VIEWPORT_HEIGHT, scale);
    egui::vec2(
        scale.f32(PANEL_WIDTH),
        panel_height_for_scale(state, body_height, scale),
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
    egui::vec2(scale.f32(PANEL_WIDTH), scale.f32(SEARCH_HEIGHT))
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

        assert_eq!(base, egui::vec2(720.0, 64.0));
        assert_eq!(zoomed, egui::vec2(1080.0, 96.0));
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

    #[test]
    fn search_metrics_scale_with_ui_zoom() {
        assert_eq!(
            crate::ui_metrics_search::search_metrics_for_scale(UiScale::new(1.5), 600.0),
            (66.0, 462.0, 54.0, 4.5, 42.0)
        );
    }

    #[test]
    fn action_panel_metrics_scale_with_ui_zoom() {
        assert_eq!(
            crate::ui_metrics_action_panel::metrics_for_scale(UiScale::new(1.5), 700.0),
            (480.0, 219.0, 42.0, 3.0, 48.0)
        );
    }

    #[test]
    fn empty_state_icon_metrics_scale_with_ui_zoom() {
        assert_eq!(
            crate::ui_metrics_empty::no_matches_icon_metrics_for_scale(UiScale::new(1.5)),
            (egui::vec2(48.0, 48.0), 13.5)
        );
    }

    #[test]
    fn loading_progress_metrics_scale_with_ui_zoom() {
        assert_eq!(
            crate::ui_metrics_results::loading_progress_metrics_for_scale(UiScale::new(1.5), 600.0),
            (228.0, 3.0)
        );
    }

    #[test]
    fn group_header_divider_scales_with_ui_zoom() {
        assert_eq!(
            crate::ui_metrics_results::group_header_metrics_for_scale(UiScale::new(1.5), 600.0),
            (600.0, 1.5)
        );
    }

    #[test]
    fn panel_rect_anchors_to_upper_screen_region() {
        let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1440.0, 900.0));
        let rect = panel_rect_for_available(available, 720.0, 320.0, window_margin(), true);

        assert_eq!(rect.width(), 720.0);
        assert_eq!(rect.min.x, 360.0);
        assert_eq!(rect.min.y, 252.0);
    }

    #[test]
    fn panel_rect_stays_inside_tightly_sized_native_window() {
        let mut state = LauncherState::new();
        state.update_query("index");
        let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
        let rect = panel_rect(available, &state);

        assert_eq!(rect.min.x, window_margin());
        assert_eq!(rect.min.y, window_margin());
        assert_eq!(rect.max.x, available.right());
        assert_eq!(rect.max.y, available.bottom());
    }

    #[test]
    fn panel_rect_clamps_when_viewport_is_short() {
        let mut state = LauncherState::new();
        state.update_query("index");
        state.view.preview_executing();
        let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 240.0));
        let rect = panel_rect(available, &state);

        assert!(rect.min.y >= window_margin());
        assert!(rect.max.y <= available.bottom());
    }
}
