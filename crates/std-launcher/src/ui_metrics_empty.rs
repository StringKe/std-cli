use eframe::egui;
use std_egui::tokens::{LauncherSize, UiScale};

pub(crate) fn no_matches_icon_size(scale: UiScale) -> egui::Vec2 {
    LauncherSize::no_matches_icon_size(scale)
}

pub(crate) fn no_matches_icon_geometry(
    scale: UiScale,
    rect: egui::Rect,
) -> EmptySearchIconGeometry {
    let center = LauncherSize::no_matches_icon_center(scale, rect);
    EmptySearchIconGeometry {
        center,
        radius: LauncherSize::no_matches_icon_radius(scale),
        handle_start: LauncherSize::no_matches_icon_handle_start(scale, center),
        handle_end: LauncherSize::no_matches_icon_handle_end(scale, center),
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EmptySearchIconGeometry {
    pub center: egui::Pos2,
    pub radius: f32,
    pub handle_start: egui::Pos2,
    pub handle_end: egui::Pos2,
}

#[cfg(test)]
pub(crate) fn no_matches_icon_metrics_for_scale(scale: UiScale) -> (egui::Vec2, f32) {
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, no_matches_icon_size(scale));
    let geometry = no_matches_icon_geometry(scale, rect);
    (no_matches_icon_size(scale), geometry.radius)
}
