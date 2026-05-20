use eframe::egui;
use std_egui::tokens::UiScale;

const NO_MATCHES_ICON_SIZE: f32 = 32.0;
const NO_MATCHES_ICON_RADIUS: f32 = 9.0;
const NO_MATCHES_ICON_HANDLE_INSET: f32 = 7.0;
const NO_MATCHES_ICON_HANDLE_OUTSET: f32 = 13.0;

pub(crate) fn no_matches_icon_size(scale: UiScale) -> egui::Vec2 {
    egui::vec2(
        scale.f32(NO_MATCHES_ICON_SIZE),
        scale.f32(NO_MATCHES_ICON_SIZE),
    )
}

pub(crate) fn no_matches_icon_geometry(
    scale: UiScale,
    rect: egui::Rect,
) -> EmptySearchIconGeometry {
    let center = rect.center() - egui::vec2(scale.f32(2.0), scale.f32(2.0));
    EmptySearchIconGeometry {
        center,
        radius: scale.f32(NO_MATCHES_ICON_RADIUS),
        handle_start: egui::pos2(
            center.x + scale.f32(NO_MATCHES_ICON_HANDLE_INSET),
            center.y + scale.f32(NO_MATCHES_ICON_HANDLE_INSET),
        ),
        handle_end: egui::pos2(
            center.x + scale.f32(NO_MATCHES_ICON_HANDLE_OUTSET),
            center.y + scale.f32(NO_MATCHES_ICON_HANDLE_OUTSET),
        ),
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
