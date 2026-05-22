use eframe::egui;
use std_egui::tokens::UiScale;

const LOADING_PROGRESS_HEIGHT: f32 = 2.0;
const LOADING_PROGRESS_WIDTH_RATIO: f32 = 0.38;
const LOADING_PROGRESS_MIN_WIDTH: f32 = 120.0;
const GROUP_DIVIDER_HEIGHT: f32 = 1.0;
const GROUP_LABEL_OFFSET_Y: f32 = 4.0;
const RESULT_ICON_SIZE: f32 = 20.0;
const RESULT_ROW_TITLE_Y: f32 = 12.0;
const RESULT_ROW_TITLE_HEIGHT: f32 = 18.0;
const RESULT_ROW_SUBTITLE_Y: f32 = 28.0;
const RESULT_RIGHT_AREA_WIDTH: f32 = 180.0;
const RESULT_TEXT_RIGHT_GAP: f32 = 12.0;

pub(crate) fn loading_progress_height(scale: UiScale) -> f32 {
    scale.f32(LOADING_PROGRESS_HEIGHT)
}

pub(crate) fn loading_progress_size(scale: UiScale, available_width: f32) -> egui::Vec2 {
    egui::vec2(available_width, loading_progress_height(scale))
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

pub(crate) fn group_header_label_offset_y(scale: UiScale) -> f32 {
    scale.f32(GROUP_LABEL_OFFSET_Y)
}

pub(crate) fn result_row_size(scale: UiScale, available_width: f32) -> egui::Vec2 {
    egui::vec2(available_width, scale.f32(36.0))
}

pub(crate) fn result_row_shrink(scale: UiScale) -> egui::Vec2 {
    egui::vec2(scale.f32(8.0), 0.0)
}

pub(crate) fn result_row_layout(scale: UiScale, rect: egui::Rect) -> LauncherResultRowLayout {
    let icon_size = scale.f32(RESULT_ICON_SIZE);
    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(rect.left() + icon_size * 0.5, rect.center().y),
        egui::vec2(icon_size, icon_size),
    );
    let right_width = scale.f32(RESULT_RIGHT_AREA_WIDTH).min(rect.width() * 0.38);
    let right_rect = egui::Rect::from_min_max(
        egui::pos2(rect.right() - right_width, rect.top()),
        rect.right_bottom(),
    );
    let text_left = icon_rect.right() + scale.f32(12.0);
    let text_right = right_rect.left() - scale.f32(RESULT_TEXT_RIGHT_GAP);
    LauncherResultRowLayout {
        icon_rect,
        title_pos: egui::pos2(text_left, rect.top() + scale.f32(RESULT_ROW_TITLE_Y)),
        title_rect: egui::Rect::from_min_size(
            egui::pos2(text_left, rect.top()),
            egui::vec2(
                (text_right - text_left).max(0.0),
                scale.f32(RESULT_ROW_TITLE_HEIGHT),
            ),
        ),
        subtitle_pos: egui::pos2(text_left, rect.top() + scale.f32(RESULT_ROW_SUBTITLE_Y)),
        text_clip: egui::Rect::from_min_max(
            egui::pos2(text_left, rect.top()),
            egui::pos2(text_right.max(text_left), rect.bottom()),
        ),
        right_rect,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LauncherResultRowLayout {
    pub(crate) icon_rect: egui::Rect,
    pub(crate) title_pos: egui::Pos2,
    pub(crate) title_rect: egui::Rect,
    pub(crate) subtitle_pos: egui::Pos2,
    pub(crate) text_clip: egui::Rect,
    pub(crate) right_rect: egui::Rect,
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

#[cfg(test)]
pub(crate) fn group_header_slot_metrics_for_scale(
    scale: UiScale,
    available_width: f32,
) -> (f32, f32, f32) {
    let slot = egui::vec2(available_width, scale.f32(24.0));
    (slot.x, slot.y, group_header_label_offset_y(scale))
}

#[cfg(test)]
pub(crate) fn result_row_layout_metrics_for_scale(scale: UiScale, width: f32) -> (f32, f32, f32) {
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, result_row_size(scale, width));
    let layout = result_row_layout(scale, rect);
    (
        layout.icon_rect.width(),
        layout.text_clip.width(),
        layout.right_rect.width(),
    )
}
