use eframe::egui;
use std_egui::tokens::{Space, UiScale};
use std_egui::LauncherPhase;
use std_launcher::LauncherState;
use std_launcher::PANEL_WIDTH;

const SEARCH_HEIGHT: f32 = 64.0;
const ACTION_BAR_HEIGHT: f32 = 36.0;
const RESULT_ROW_HEIGHT: f32 = 36.0;
const GROUP_HEADER_ROW_HEIGHT: f32 = 24.0;
const MAX_RESULT_ROWS: f32 = 6.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 520.0;
#[cfg(test)]
const PANEL_VERTICAL_ANCHOR: f32 = 0.28;

pub(crate) fn scale() -> UiScale {
    UiScale::from_env()
}

pub(crate) fn window_margin() -> f32 {
    0.0
}

pub(crate) fn panel_inner_padding() -> f32 {
    scale().f32(Space::MD as f32)
}

pub(crate) fn panel_inner_padding_for_state(state: &LauncherState) -> f32 {
    if panel_is_expanded(state) {
        panel_inner_padding()
    } else {
        0.0
    }
}

pub(crate) fn panel_rect(available: egui::Rect, state: &LauncherState) -> egui::Rect {
    let margin = window_margin();
    let body_height = body_height(state, available.height());
    let panel_width = panel_surface_width().min(available.width() - margin * 2.0);
    let panel_height = panel_height(state, body_height).min(available.height() - margin * 2.0);
    panel_surface_rect(available, panel_width, panel_height, margin)
}

fn panel_surface_width() -> f32 {
    std_launcher::panel_surface_width(UiScale::from_env().value())
}

fn panel_surface_rect(
    available: egui::Rect,
    panel_width: f32,
    panel_height: f32,
    margin: f32,
) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(available.left() + margin, available.top() + margin),
        egui::vec2(panel_width, panel_height),
    )
}

#[cfg(test)]
fn screen_anchor_position(monitor_size: egui::Vec2, panel_size: egui::Vec2) -> egui::Pos2 {
    let x = ((monitor_size.x - panel_size.x) * 0.5).max(0.0);
    let y = (monitor_size.y * PANEL_VERTICAL_ANCHOR).min((monitor_size.y - panel_size.y).max(0.0));
    egui::pos2(x, y)
}

pub(crate) fn result_row_size(available_width: f32) -> egui::Vec2 {
    crate::ui_metrics_results::result_row_size(scale(), available_width)
}

pub(crate) fn result_row_shrink() -> egui::Vec2 {
    crate::ui_metrics_results::result_row_shrink(scale())
}

pub(crate) fn result_list_slot_height() -> f32 {
    scale().f32(RESULT_ROW_HEIGHT)
}

pub(crate) fn group_header_slot_height() -> f32 {
    scale().f32(GROUP_HEADER_ROW_HEIGHT)
}

pub(crate) fn loading_progress_rect(available_width: f32, top_left: egui::Pos2) -> egui::Rect {
    crate::ui_metrics_results::loading_progress_rect(scale(), available_width, top_left)
}

pub(crate) fn loading_progress_size(available_width: f32) -> egui::Vec2 {
    crate::ui_metrics_results::loading_progress_size(scale(), available_width)
}

pub(crate) fn group_divider_rect(available_width: f32, top_left: egui::Pos2) -> egui::Rect {
    crate::ui_metrics_results::group_divider_rect(scale(), available_width, top_left)
}

pub(crate) fn group_header_label_offset_y() -> f32 {
    crate::ui_metrics_results::group_header_label_offset_y(scale())
}

pub(crate) fn result_row_layout(
    rect: egui::Rect,
) -> crate::ui_metrics_results::LauncherResultRowLayout {
    crate::ui_metrics_results::result_row_layout(scale(), rect)
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

pub(crate) fn action_bar_height() -> f32 {
    scale().f32(ACTION_BAR_HEIGHT)
}

pub(crate) fn action_summary_label_height() -> f32 {
    scale().f32(18.0)
}

pub(crate) fn feedback_text_height() -> f32 {
    feedback_text_height_for_scale(scale())
}

pub(crate) fn feedback_detail_height() -> f32 {
    scale().f32(36.0)
}

pub(crate) fn feedback_action_height() -> f32 {
    feedback_action_height_for_scale(scale())
}

pub(crate) fn feedback_panel_height_for_scale(scale: UiScale) -> f32 {
    feedback_text_height_for_scale(scale)
        + scale.f32(Space::XS as f32)
        + feedback_action_height_for_scale(scale)
        + scale.f32(Space::XS as f32)
}

fn feedback_text_height_for_scale(scale: UiScale) -> f32 {
    scale.f32(58.0)
}

pub(crate) fn feedback_action_height_for_scale(scale: UiScale) -> f32 {
    scale.f32(24.0)
}

pub(crate) struct FeedbackIconGeometry {
    pub(crate) center: egui::Pos2,
    pub(crate) radius: f32,
    pub(crate) stroke_width: f32,
    pub(crate) check_start: egui::Pos2,
    pub(crate) check_mid: egui::Pos2,
    pub(crate) check_end: egui::Pos2,
    pub(crate) cross_a_start: egui::Pos2,
    pub(crate) cross_a_end: egui::Pos2,
    pub(crate) cross_b_start: egui::Pos2,
    pub(crate) cross_b_end: egui::Pos2,
    pub(crate) alert_top: egui::Pos2,
    pub(crate) alert_mid: egui::Pos2,
    pub(crate) alert_dot: egui::Pos2,
    pub(crate) dot_radius: f32,
}

pub(crate) fn feedback_icon_size() -> egui::Vec2 {
    let scale = scale();
    egui::vec2(scale.f32(Space::MD as f32), scale.f32(Space::MD as f32))
}

pub(crate) fn feedback_icon_geometry(rect: egui::Rect) -> FeedbackIconGeometry {
    let scale = scale();
    let center = rect.center();
    let radius = scale.f32(Space::XS as f32);
    let half = scale.f32(Space::TWO_XS as f32);
    FeedbackIconGeometry {
        center,
        radius,
        stroke_width: scale.f32(1.5),
        check_start: egui::pos2(center.x - half, center.y),
        check_mid: egui::pos2(center.x, center.y + half),
        check_end: egui::pos2(center.x + radius, center.y - half),
        cross_a_start: egui::pos2(center.x - half, center.y - half),
        cross_a_end: egui::pos2(center.x + half, center.y + half),
        cross_b_start: egui::pos2(center.x + half, center.y - half),
        cross_b_end: egui::pos2(center.x - half, center.y + half),
        alert_top: egui::pos2(center.x, center.y - half),
        alert_mid: center,
        alert_dot: egui::pos2(center.x, center.y + half),
        dot_radius: scale.f32(1.5),
    }
}

pub(crate) fn search_bar_min_height() -> f32 {
    crate::ui_metrics_search::search_bar_min_height(scale())
}

pub(crate) fn search_input_width(available_width: f32) -> f32 {
    crate::ui_metrics_search::search_input_width(scale(), available_width)
}

pub(crate) fn search_input_width_with_ime(available_width: f32) -> f32 {
    crate::ui_metrics_search::search_input_width_with_ime(scale(), available_width)
}

pub(crate) fn search_input_height() -> f32 {
    crate::ui_metrics_search::search_input_height(scale())
}

pub(crate) fn search_ime_chip_width() -> f32 {
    crate::ui_metrics_search::search_ime_chip_width(scale())
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

pub(crate) fn action_panel_rect(anchor_rect: egui::Rect, item_count: usize) -> egui::Rect {
    let width = action_panel_width(anchor_rect.width());
    let height = action_panel_height(item_count);
    egui::Rect::from_min_size(
        egui::pos2(anchor_rect.right() - width, anchor_rect.top() - height),
        egui::vec2(width, height),
    )
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

pub(crate) fn panel_is_only_visible_surface(state: &LauncherState) -> bool {
    panel_surface_geometry(state).passes()
}

pub(crate) fn panel_surface_geometry_summary(state: &LauncherState) -> String {
    panel_surface_geometry(state).summary()
}

struct PanelSurfaceGeometry {
    window: egui::Vec2,
    panel: egui::Rect,
    frame_clear: bool,
}

impl PanelSurfaceGeometry {
    fn passes(&self) -> bool {
        self.frame_clear
            && self.panel.min == egui::Pos2::ZERO
            && (self.panel.width() - self.window.x).abs() <= 0.5
            && (self.panel.height() - self.window.y).abs() <= 0.5
    }

    fn summary(&self) -> String {
        format!(
            "native_window={}x{};panel_origin={}x{};panel_size={}x{};carrier_clearance={}x{};frame_clear={};panel_only={}",
            self.window.x.round() as u32,
            self.window.y.round() as u32,
            self.panel.min.x.round() as i32,
            self.panel.min.y.round() as i32,
            self.panel.width().round() as u32,
            self.panel.height().round() as u32,
            (self.window.x - self.panel.width()).round() as i32,
            (self.window.y - self.panel.height()).round() as i32,
            self.frame_clear,
            self.passes()
        )
    }
}

fn panel_surface_geometry(state: &LauncherState) -> PanelSurfaceGeometry {
    let viewport = window_inner_size(state);
    let available = egui::Rect::from_min_size(egui::Pos2::ZERO, viewport);
    let panel = panel_rect(available, state);
    let frame = crate::ui::launcher_viewport_frame();
    PanelSurfaceGeometry {
        window: viewport,
        panel,
        frame_clear: frame.fill == egui::Color32::TRANSPARENT && frame.stroke == egui::Stroke::NONE,
    }
}

pub(crate) fn panel_height(state: &LauncherState, body_height: f32) -> f32 {
    panel_height_for_scale(state, body_height, UiScale::from_env())
}

pub(crate) fn panel_content_height(state: &LauncherState, body_height: f32) -> f32 {
    panel_content_height_for_scale(state, body_height, UiScale::from_env())
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
        scale.f32(PANEL_WIDTH),
        collapsed_panel_height_for_scale(scale),
    )
}

fn panel_height_for_scale(state: &LauncherState, body_height: f32, scale: UiScale) -> f32 {
    if !panel_is_expanded(state) {
        return collapsed_panel_height_for_scale(scale);
    }
    let panel_padding = scale.f32(Space::MD as f32) * 2.0;
    panel_content_height_for_scale(state, body_height, scale) + panel_padding
}

fn panel_content_height_for_scale(state: &LauncherState, body_height: f32, scale: UiScale) -> f32 {
    scale.f32(SEARCH_HEIGHT)
        + body_height
        + scale.f32(ACTION_BAR_HEIGHT)
        + scale.f32(Space::MD as f32)
        + scale.f32(Space::SM as f32)
        + extra_status_height_for_scale(state, scale)
}

fn body_height_for_scale(state: &LauncherState, viewport_height: f32, scale: UiScale) -> f32 {
    if !panel_is_expanded(state) {
        return 0.0;
    }
    let visible_height = result_list_visible_height(state, scale);
    let desired = visible_height + scale.f32(Space::SM as f32);
    desired.clamp(
        scale.f32(128.0),
        body_height_available(state, viewport_height, scale),
    )
}

fn body_height_available(state: &LauncherState, viewport_height: f32, scale: UiScale) -> f32 {
    let chrome = panel_content_height_for_scale(state, 0.0, scale);
    (viewport_height - chrome).max(scale.f32(128.0))
}

#[cfg(test)]
fn result_list_slot_count(state: &LauncherState) -> usize {
    state.view.results.len() + crate::ui_results::group_count(&state.view.results)
}

fn result_list_visible_height(state: &LauncherState, scale: UiScale) -> f32 {
    let row_count = state.view.results.len().min(MAX_RESULT_ROWS as usize);
    let group_count = crate::ui_results::group_count(&state.view.results).min(row_count);
    let group_height = group_count as f32 * scale.f32(GROUP_HEADER_ROW_HEIGHT);
    let row_height = row_count as f32 * scale.f32(RESULT_ROW_HEIGHT);
    group_height + row_height
}

fn collapsed_panel_height_for_scale(scale: UiScale) -> f32 {
    scale.f32(SEARCH_HEIGHT)
}

fn extra_status_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    let mut height = 0.0;
    if state.controller.voice_active {
        height += scale.f32(44.0) + scale.f32(Space::XS as f32);
    }
    if state.view.feedback.is_some() {
        height += feedback_panel_height_for_scale(scale) + scale.f32(Space::XS as f32);
    }
    height
}

#[cfg(test)]
fn row_metrics_for_scale(scale: UiScale) -> (f32, f32, f32, f32, f32) {
    (
        scale.f32(RESULT_ROW_HEIGHT),
        scale.f32(GROUP_HEADER_ROW_HEIGHT),
        scale.f32(34.0),
        scale.f32(24.0),
        scale.f32(18.0),
    )
}

#[cfg(test)]
#[path = "ui_metrics_tests.rs"]
mod tests;
