use std_egui::tokens::{LauncherSize, UiScale};

pub(crate) fn width(scale: UiScale, anchor_width: f32) -> f32 {
    LauncherSize::action_panel_width(scale, anchor_width)
}

pub(crate) fn height(scale: UiScale, item_count: usize) -> f32 {
    LauncherSize::action_panel_height(scale, item_count)
}

pub(crate) fn search_height(scale: UiScale) -> f32 {
    LauncherSize::action_panel_search_height(scale)
}

pub(crate) fn row_height(scale: UiScale) -> f32 {
    LauncherSize::action_panel_row_height(scale)
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
