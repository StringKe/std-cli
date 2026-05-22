use std_egui::tokens::UiScale;

const ACTION_PANEL_WIDTH: f32 = 320.0;
const ACTION_PANEL_HEADER_HEIGHT: f32 = 44.0;
const ACTION_PANEL_ROW_HEIGHT: f32 = 32.0;
const ACTION_PANEL_ROW_STEP: f32 = 34.0;
const ACTION_PANEL_SEARCH_HEIGHT: f32 = 28.0;

pub(crate) fn width(scale: UiScale, anchor_width: f32) -> f32 {
    scale.f32(ACTION_PANEL_WIDTH).min(anchor_width)
}

pub(crate) fn height(scale: UiScale, item_count: usize) -> f32 {
    scale.f32(ACTION_PANEL_HEADER_HEIGHT) + scale.f32(ACTION_PANEL_ROW_STEP) * item_count as f32
}

pub(crate) fn search_height(scale: UiScale) -> f32 {
    scale.f32(ACTION_PANEL_SEARCH_HEIGHT)
}

pub(crate) fn row_height(scale: UiScale) -> f32 {
    scale.f32(ACTION_PANEL_ROW_HEIGHT)
}

#[cfg(test)]
pub(crate) fn metrics_for_scale(scale: UiScale, anchor_width: f32) -> (f32, f32, f32, f32) {
    (
        width(scale, anchor_width),
        height(scale, 3),
        search_height(scale),
        row_height(scale),
    )
}
