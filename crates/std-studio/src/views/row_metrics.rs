mod chips;
mod heights;
mod text;

pub(crate) use chips::*;
pub(crate) use heights::*;
pub(crate) use text::*;

pub(crate) fn settings_path_row_size(width: f32) -> eframe::egui::Vec2 {
    eframe::egui::Vec2::new(width, SETTINGS_PATH_ROW_HEIGHT)
}
