use eframe::egui;
use std_egui::tokens::{LauncherSize, UiScale};

pub(crate) fn search_bar_min_height(scale: UiScale) -> f32 {
    LauncherSize::search_bar_min_height(scale)
}

pub(crate) fn search_input_width(scale: UiScale, available_width: f32) -> f32 {
    LauncherSize::search_input_width(scale, available_width)
}

pub(crate) fn search_input_width_with_ime(scale: UiScale, available_width: f32) -> f32 {
    LauncherSize::search_input_width_with_ime(scale, available_width)
}

pub(crate) fn search_input_height(scale: UiScale) -> f32 {
    LauncherSize::search_input_height(scale)
}

pub(crate) fn search_ime_chip_width(scale: UiScale) -> f32 {
    LauncherSize::search_ime_chip_width(scale)
}

pub(crate) fn search_icon_size(scale: UiScale) -> egui::Vec2 {
    LauncherSize::search_icon_size(scale)
}

pub(crate) fn search_icon_geometry(scale: UiScale, rect: egui::Rect) -> SearchIconGeometry {
    let center = LauncherSize::search_icon_center(scale, rect);
    SearchIconGeometry {
        center,
        radius: scale.f32(LauncherSize::SEARCH_ICON_RADIUS),
        handle_start: egui::pos2(
            center.x + scale.f32(LauncherSize::SEARCH_ICON_HANDLE_INSET),
            center.y + scale.f32(LauncherSize::SEARCH_ICON_HANDLE_INSET),
        ),
        handle_end: egui::pos2(
            center.x + scale.f32(LauncherSize::SEARCH_ICON_HANDLE_OUTSET),
            center.y + scale.f32(LauncherSize::SEARCH_ICON_HANDLE_OUTSET),
        ),
    }
}

pub(crate) fn voice_input_width(scale: UiScale, available_width: f32) -> f32 {
    LauncherSize::voice_input_width(scale, available_width)
}

pub(crate) fn voice_input_height(scale: UiScale) -> f32 {
    LauncherSize::voice_input_height(scale)
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
        search_input_width_with_ime(scale, available_width),
        search_input_height(scale),
        voice_input_height(scale),
    )
}
