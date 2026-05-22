use eframe::egui;
use std_egui::tokens::{LauncherSize, Space, UiScale};
use std_egui::LauncherPhase;
use std_launcher::LauncherState;

pub(crate) const SEARCH_HEIGHT: f32 = 64.0;
pub(crate) const ACTION_BAR_HEIGHT: f32 = LauncherSize::ACTION_BAR_HEIGHT;
pub(crate) const MAX_RESULT_ROWS: f32 = LauncherSize::MAX_RESULT_ROWS;
pub(crate) const DEFAULT_VIEWPORT_HEIGHT: f32 = 520.0;
#[cfg(test)]
const PANEL_VERTICAL_ANCHOR: f32 = 0.28;

pub(crate) fn scale() -> UiScale {
    UiScale::from_env()
}

pub(crate) fn window_margin() -> f32 {
    host_gutter()
}

pub(crate) fn host_gutter() -> f32 {
    crate::ui_metrics_layout::host_gutter_for_scale(scale())
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
    LauncherSize::result_row_height(scale())
}

pub(crate) fn group_header_slot_height() -> f32 {
    LauncherSize::group_header_slot_height(scale())
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

pub(crate) fn result_right_affordance_layout(
    rect: egui::Rect,
    has_action: bool,
) -> crate::ui_metrics_results::LauncherResultRightAffordanceLayout {
    crate::ui_metrics_results::result_right_affordance_layout(scale(), rect, has_action)
}

pub(crate) fn ask_ai_row_height() -> f32 {
    LauncherSize::ask_ai_row_height(scale())
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
    LauncherSize::action_bar_content_height(scale())
}

pub(crate) fn action_bar_height() -> f32 {
    LauncherSize::action_bar_height(scale())
}

pub(crate) fn action_summary_label_height() -> f32 {
    LauncherSize::action_summary_label_height(scale())
}

pub(crate) fn feedback_text_height() -> f32 {
    feedback_text_height_for_scale(scale())
}

pub(crate) fn feedback_detail_height() -> f32 {
    LauncherSize::feedback_detail_height(scale())
}

pub(crate) fn feedback_panel_height_for_scale(scale: UiScale) -> f32 {
    feedback_text_height_for_scale(scale)
}

fn feedback_text_height_for_scale(scale: UiScale) -> f32 {
    LauncherSize::feedback_text_height(scale)
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
    LauncherSize::feedback_icon_size(scale())
}

pub(crate) fn feedback_icon_geometry(rect: egui::Rect) -> FeedbackIconGeometry {
    let scale = scale();
    let center = rect.center();
    let radius = LauncherSize::feedback_icon_radius(scale);
    let half = LauncherSize::feedback_icon_half(scale);
    FeedbackIconGeometry {
        center,
        radius,
        stroke_width: LauncherSize::feedback_icon_stroke_width(scale),
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
        dot_radius: LauncherSize::feedback_icon_stroke_width(scale),
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

pub(crate) fn action_panel_row_height() -> f32 {
    crate::ui_metrics_action_panel::row_height(scale())
}

pub(crate) fn initial_window_inner_size() -> egui::Vec2 {
    crate::ui_metrics_layout::initial_window_inner_size_for_scale(UiScale::from_env())
}

pub(crate) fn window_inner_size(state: &LauncherState) -> egui::Vec2 {
    crate::ui_metrics_layout::window_inner_size_for_scale(state, UiScale::from_env())
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
            && self.panel.min.x > 0.0
            && self.panel.min.y > 0.0
            && self.panel.max.x < self.window.x
            && self.panel.max.y < self.window.y
    }

    fn summary(&self) -> String {
        format!(
            "native_host={}x{};host_background=none;panel_surface=opaque;panel_origin={}x{};panel_size={}x{};host_gap={}x{};frame_clear={};panel_floats={}",
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
    crate::ui_metrics_layout::panel_height_for_scale(state, body_height, UiScale::from_env())
}

pub(crate) fn panel_content_height(state: &LauncherState, body_height: f32) -> f32 {
    crate::ui_metrics_layout::panel_content_height_for_scale(
        state,
        body_height,
        UiScale::from_env(),
    )
}

pub(crate) fn body_height(state: &LauncherState, viewport_height: f32) -> f32 {
    crate::ui_metrics_layout::body_height_for_scale(state, viewport_height, UiScale::from_env())
}

pub(crate) fn panel_is_expanded(state: &LauncherState) -> bool {
    (state.view.phase != LauncherPhase::Empty || !state.view.results.is_empty())
        || state.controller.voice_active
        || state.view.feedback.is_some()
        || state.action_panel.open
}

#[cfg(test)]
fn result_list_slot_count(state: &LauncherState) -> usize {
    state.view.results.len() + crate::ui_results::group_count(&state.view.results)
}

pub(crate) fn result_list_visible_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    let row_count = state.view.results.len().min(MAX_RESULT_ROWS as usize);
    let group_count = crate::ui_results::group_count(&state.view.results).min(row_count);
    let group_height = group_count as f32 * LauncherSize::group_header_slot_height(scale);
    let row_height = row_count as f32 * LauncherSize::result_row_height(scale);
    group_height + row_height
}

#[cfg(test)]
fn row_metrics_for_scale(scale: UiScale) -> (f32, f32, f32, f32, f32) {
    (
        LauncherSize::result_row_height(scale),
        LauncherSize::group_header_slot_height(scale),
        LauncherSize::ask_ai_row_height(scale),
        LauncherSize::action_bar_content_height(scale),
        LauncherSize::action_summary_label_height(scale),
    )
}

#[cfg(test)]
#[path = "ui_metrics_tests.rs"]
mod tests;
