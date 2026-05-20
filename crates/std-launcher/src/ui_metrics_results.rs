use eframe::egui;
use std_egui::tokens::UiScale;

const LOADING_PROGRESS_HEIGHT: f32 = 2.0;
const LOADING_PROGRESS_WIDTH_RATIO: f32 = 0.38;
const LOADING_PROGRESS_MIN_WIDTH: f32 = 120.0;
const GROUP_DIVIDER_HEIGHT: f32 = 1.0;

pub(crate) fn loading_progress_height(scale: UiScale) -> f32 {
    scale.f32(LOADING_PROGRESS_HEIGHT)
}

pub(crate) fn loading_progress_rect(
    scale: UiScale,
    available_width: f32,
    top_left: egui::Pos2,
) -> egui::Rect {
    let width = (available_width * LOADING_PROGRESS_WIDTH_RATIO)
        .max(scale.f32(LOADING_PROGRESS_MIN_WIDTH).min(available_width));
    egui::Rect::from_min_size(top_left, egui::vec2(width, loading_progress_height(scale)))
}

pub(crate) fn group_divider_height(scale: UiScale) -> f32 {
    scale.f32(GROUP_DIVIDER_HEIGHT)
}

pub(crate) fn group_divider_rect(
    scale: UiScale,
    available_width: f32,
    top_left: egui::Pos2,
) -> egui::Rect {
    egui::Rect::from_min_size(
        top_left,
        egui::vec2(available_width, group_divider_height(scale)),
    )
}

#[cfg(test)]
pub(crate) fn loading_progress_metrics_for_scale(
    scale: UiScale,
    available_width: f32,
) -> (f32, f32) {
    let rect = loading_progress_rect(scale, available_width, egui::Pos2::ZERO);
    (rect.width(), rect.height())
}

#[cfg(test)]
pub(crate) fn group_header_metrics_for_scale(scale: UiScale, available_width: f32) -> (f32, f32) {
    let rect = group_divider_rect(scale, available_width, egui::Pos2::ZERO);
    (rect.width(), rect.height())
}
