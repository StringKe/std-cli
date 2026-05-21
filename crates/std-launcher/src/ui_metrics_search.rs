use eframe::egui;
use std_egui::tokens::UiScale;

const SEARCH_BAR_MIN_HEIGHT: f32 = 44.0;
const SEARCH_INPUT_WIDTH_RESERVE: f32 = 72.0;
const SEARCH_INPUT_HEIGHT: f32 = 36.0;
const SEARCH_ICON_WIDTH: f32 = 24.0;
const SEARCH_ICON_HEIGHT: f32 = 28.0;
const SEARCH_ICON_CENTER_X: f32 = 10.0;
const SEARCH_ICON_CENTER_Y_OFFSET: f32 = -2.0;
const SEARCH_ICON_RADIUS: f32 = 5.0;
const SEARCH_ICON_HANDLE_INSET: f32 = 4.0;
const SEARCH_ICON_HANDLE_OUTSET: f32 = 9.0;
const FOCUS_RING_EXPAND: f32 = 3.0;
const VOICE_INPUT_WIDTH_RESERVE: f32 = 112.0;
const VOICE_INPUT_HEIGHT: f32 = 28.0;

pub(crate) fn search_bar_min_height(scale: UiScale) -> f32 {
    scale.f32(SEARCH_BAR_MIN_HEIGHT)
}

pub(crate) fn search_input_width(scale: UiScale, available_width: f32) -> f32 {
    (available_width - scale.f32(SEARCH_INPUT_WIDTH_RESERVE)).max(scale.f32(160.0))
}

pub(crate) fn search_input_height(scale: UiScale) -> f32 {
    scale.f32(SEARCH_INPUT_HEIGHT)
}

pub(crate) fn search_icon_size(scale: UiScale) -> egui::Vec2 {
    egui::vec2(scale.f32(SEARCH_ICON_WIDTH), scale.f32(SEARCH_ICON_HEIGHT))
}

pub(crate) fn search_icon_geometry(scale: UiScale, rect: egui::Rect) -> SearchIconGeometry {
    let center = egui::pos2(
        rect.left() + scale.f32(SEARCH_ICON_CENTER_X),
        rect.center().y + scale.f32(SEARCH_ICON_CENTER_Y_OFFSET),
    );
    SearchIconGeometry {
        center,
        radius: scale.f32(SEARCH_ICON_RADIUS),
        handle_start: egui::pos2(
            center.x + scale.f32(SEARCH_ICON_HANDLE_INSET),
            center.y + scale.f32(SEARCH_ICON_HANDLE_INSET),
        ),
        handle_end: egui::pos2(
            center.x + scale.f32(SEARCH_ICON_HANDLE_OUTSET),
            center.y + scale.f32(SEARCH_ICON_HANDLE_OUTSET),
        ),
    }
}

pub(crate) fn focus_ring_expand(scale: UiScale) -> f32 {
    scale.f32(FOCUS_RING_EXPAND)
}

pub(crate) fn voice_input_width(scale: UiScale, available_width: f32) -> f32 {
    (available_width - scale.f32(VOICE_INPUT_WIDTH_RESERVE)).max(scale.f32(160.0))
}

pub(crate) fn voice_input_height(scale: UiScale) -> f32 {
    scale.f32(VOICE_INPUT_HEIGHT)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SearchIconGeometry {
    pub center: egui::Pos2,
    pub radius: f32,
    pub handle_start: egui::Pos2,
    pub handle_end: egui::Pos2,
}

#[cfg(test)]
pub(crate) fn search_metrics_for_scale(
    scale: UiScale,
    available_width: f32,
) -> (f32, f32, f32, f32, f32) {
    (
        search_bar_min_height(scale),
        search_input_width(scale, available_width),
        search_input_height(scale),
        focus_ring_expand(scale),
        voice_input_height(scale),
    )
}
