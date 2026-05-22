use super::Space;
use crate::tokens::typography::UiScale;

pub struct LauncherSize;

impl LauncherSize {
    pub const PANEL_WIDTH: f32 = 720.0;
    pub const WINDOW_VERTICAL_ANCHOR: f32 = 0.28;
    pub const HIDDEN_HOST_SIZE: f32 = 1.0;
    pub const SEARCH_BAR_MIN_HEIGHT: f32 = 40.0;
    pub const SEARCH_INPUT_WIDTH_RESERVE: f32 = 72.0;
    pub const SEARCH_IME_CHIP_WIDTH: f32 = 112.0;
    pub const SEARCH_INPUT_HEIGHT: f32 = 36.0;
    pub const SEARCH_ICON_WIDTH: f32 = 24.0;
    pub const SEARCH_ICON_HEIGHT: f32 = 28.0;
    pub const SEARCH_ICON_CENTER_X: f32 = 10.0;
    pub const SEARCH_ICON_CENTER_Y_OFFSET: f32 = -2.0;
    pub const SEARCH_ICON_RADIUS: f32 = 5.0;
    pub const SEARCH_ICON_HANDLE_INSET: f32 = 4.0;
    pub const SEARCH_ICON_HANDLE_OUTSET: f32 = 9.0;
    pub const VOICE_INPUT_WIDTH_RESERVE: f32 = 112.0;
    pub const VOICE_INPUT_HEIGHT: f32 = 28.0;
    pub const ACTION_PANEL_WIDTH: f32 = 320.0;
    pub const ACTION_PANEL_HEADER_HEIGHT: f32 = 44.0;
    pub const ACTION_PANEL_ROW_HEIGHT: f32 = 32.0;
    pub const ACTION_PANEL_ROW_STEP: f32 = 34.0;
    pub const ACTION_PANEL_SEARCH_HEIGHT: f32 = 28.0;
    pub const NO_MATCHES_ICON_SIZE: f32 = 32.0;
    pub const NO_MATCHES_ICON_CENTER_OFFSET: f32 = 2.0;
    pub const NO_MATCHES_ICON_RADIUS: f32 = 9.0;
    pub const NO_MATCHES_ICON_HANDLE_INSET: f32 = 7.0;
    pub const NO_MATCHES_ICON_HANDLE_OUTSET: f32 = 13.0;
    pub const RESULT_ROW_HEIGHT: f32 = 36.0;
    pub const RESULT_ROW_SHRINK_X: f32 = 8.0;
    pub const GROUP_HEADER_ROW_HEIGHT: f32 = 24.0;
    pub const GROUP_DIVIDER_HEIGHT: f32 = 1.0;
    pub const GROUP_LABEL_OFFSET_Y: f32 = 4.0;
    pub const MAX_RESULT_ROWS: f32 = 6.0;
    pub const LOADING_PROGRESS_HEIGHT: f32 = 2.0;
    pub const LOADING_PROGRESS_WIDTH_RATIO: f32 = 0.38;
    pub const LOADING_PROGRESS_MIN_WIDTH: f32 = 120.0;
    pub const RESULT_ICON_SIZE: f32 = 20.0;
    pub const RESULT_ICON_TEXT_GAP: f32 = 12.0;
    pub const RESULT_ROW_TITLE_Y: f32 = 12.0;
    pub const RESULT_ROW_TITLE_HEIGHT: f32 = 18.0;
    pub const RESULT_ROW_SUBTITLE_Y: f32 = 28.0;
    pub const RESULT_RIGHT_AREA_WIDTH: f32 = 180.0;
    pub const RESULT_RIGHT_AREA_WIDTH_RATIO: f32 = 0.38;
    pub const RESULT_TEXT_RIGHT_GAP: f32 = 12.0;
    pub const RESULT_DIRECT_KEYCAP_WIDTH: f32 = 44.0;
    pub const RESULT_PRIMARY_KEYCAP_WIDTH: f32 = 52.0;
    pub const RESULT_ACTION_LABEL_WIDTH: f32 = 92.0;
    pub const RESULT_RIGHT_GAP: f32 = 8.0;

    pub fn panel_surface_width(scale: UiScale) -> f32 {
        scale.f32(Self::PANEL_WIDTH)
    }

    pub fn host_gutter(scale: UiScale) -> f32 {
        scale.f32(Space::MD as f32)
    }

    pub fn hidden_host_size() -> egui::Vec2 {
        egui::vec2(Self::HIDDEN_HOST_SIZE, Self::HIDDEN_HOST_SIZE)
    }

    pub fn host_size(panel_size: egui::Vec2, scale: UiScale) -> egui::Vec2 {
        let gutter = Self::host_gutter(scale) * 2.0;
        egui::vec2(panel_size.x + gutter, panel_size.y + gutter)
    }

    pub fn panel_position_for_monitor(
        monitor_size: egui::Vec2,
        viewport_size: egui::Vec2,
    ) -> egui::Pos2 {
        let x = ((monitor_size.x - viewport_size.x) * 0.5).max(0.0);
        let y = (monitor_size.y * Self::WINDOW_VERTICAL_ANCHOR)
            .min((monitor_size.y - viewport_size.y).max(0.0));
        egui::pos2(x, y)
    }

    pub fn search_bar_min_height(scale: UiScale) -> f32 {
        scale.f32(Self::SEARCH_BAR_MIN_HEIGHT)
    }

    pub fn search_input_width(scale: UiScale, available_width: f32) -> f32 {
        (available_width - scale.f32(Self::SEARCH_INPUT_WIDTH_RESERVE)).max(scale.f32(160.0))
    }

    pub fn search_input_width_with_ime(scale: UiScale, available_width: f32) -> f32 {
        (available_width
            - scale.f32(Self::SEARCH_INPUT_WIDTH_RESERVE + Self::SEARCH_IME_CHIP_WIDTH))
        .max(scale.f32(160.0))
    }

    pub fn search_input_height(scale: UiScale) -> f32 {
        scale.f32(Self::SEARCH_INPUT_HEIGHT)
    }

    pub fn search_ime_chip_width(scale: UiScale) -> f32 {
        scale.f32(Self::SEARCH_IME_CHIP_WIDTH)
    }

    pub fn search_icon_size(scale: UiScale) -> egui::Vec2 {
        egui::vec2(
            scale.f32(Self::SEARCH_ICON_WIDTH),
            scale.f32(Self::SEARCH_ICON_HEIGHT),
        )
    }

    pub fn search_icon_center(scale: UiScale, rect: egui::Rect) -> egui::Pos2 {
        egui::pos2(
            rect.left() + scale.f32(Self::SEARCH_ICON_CENTER_X),
            rect.center().y + scale.f32(Self::SEARCH_ICON_CENTER_Y_OFFSET),
        )
    }

    pub fn voice_input_width(scale: UiScale, available_width: f32) -> f32 {
        (available_width - scale.f32(Self::VOICE_INPUT_WIDTH_RESERVE)).max(scale.f32(160.0))
    }

    pub fn voice_input_height(scale: UiScale) -> f32 {
        scale.f32(Self::VOICE_INPUT_HEIGHT)
    }

    pub fn action_panel_width(scale: UiScale, anchor_width: f32) -> f32 {
        scale.f32(Self::ACTION_PANEL_WIDTH).min(anchor_width)
    }

    pub fn action_panel_height(scale: UiScale, item_count: usize) -> f32 {
        scale.f32(Self::ACTION_PANEL_HEADER_HEIGHT)
            + scale.f32(Self::ACTION_PANEL_ROW_STEP) * item_count as f32
    }

    pub fn action_panel_search_height(scale: UiScale) -> f32 {
        scale.f32(Self::ACTION_PANEL_SEARCH_HEIGHT)
    }

    pub fn action_panel_row_height(scale: UiScale) -> f32 {
        scale.f32(Self::ACTION_PANEL_ROW_HEIGHT)
    }

    pub fn no_matches_icon_size(scale: UiScale) -> egui::Vec2 {
        egui::vec2(
            scale.f32(Self::NO_MATCHES_ICON_SIZE),
            scale.f32(Self::NO_MATCHES_ICON_SIZE),
        )
    }

    pub fn no_matches_icon_center(scale: UiScale, rect: egui::Rect) -> egui::Pos2 {
        let offset = scale.f32(Self::NO_MATCHES_ICON_CENTER_OFFSET);
        rect.center() - egui::vec2(offset, offset)
    }

    pub fn no_matches_icon_radius(scale: UiScale) -> f32 {
        scale.f32(Self::NO_MATCHES_ICON_RADIUS)
    }

    pub fn no_matches_icon_handle_start(scale: UiScale, center: egui::Pos2) -> egui::Pos2 {
        egui::pos2(
            center.x + scale.f32(Self::NO_MATCHES_ICON_HANDLE_INSET),
            center.y + scale.f32(Self::NO_MATCHES_ICON_HANDLE_INSET),
        )
    }

    pub fn no_matches_icon_handle_end(scale: UiScale, center: egui::Pos2) -> egui::Pos2 {
        egui::pos2(
            center.x + scale.f32(Self::NO_MATCHES_ICON_HANDLE_OUTSET),
            center.y + scale.f32(Self::NO_MATCHES_ICON_HANDLE_OUTSET),
        )
    }

    pub fn result_row_height(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ROW_HEIGHT)
    }

    pub fn result_row_size(scale: UiScale, available_width: f32) -> egui::Vec2 {
        egui::vec2(available_width, Self::result_row_height(scale))
    }

    pub fn result_row_shrink(scale: UiScale) -> egui::Vec2 {
        egui::vec2(scale.f32(Self::RESULT_ROW_SHRINK_X), 0.0)
    }

    pub fn group_header_slot_height(scale: UiScale) -> f32 {
        scale.f32(Self::GROUP_HEADER_ROW_HEIGHT)
    }

    pub fn group_divider_height(scale: UiScale) -> f32 {
        scale.f32(Self::GROUP_DIVIDER_HEIGHT)
    }

    pub fn group_divider_rect(
        scale: UiScale,
        available_width: f32,
        top_left: egui::Pos2,
    ) -> egui::Rect {
        egui::Rect::from_min_size(
            top_left,
            egui::vec2(available_width, Self::group_divider_height(scale)),
        )
    }

    pub fn group_header_label_offset_y(scale: UiScale) -> f32 {
        scale.f32(Self::GROUP_LABEL_OFFSET_Y)
    }

    pub fn loading_progress_height(scale: UiScale) -> f32 {
        scale.f32(Self::LOADING_PROGRESS_HEIGHT)
    }

    pub fn loading_progress_size(scale: UiScale, available_width: f32) -> egui::Vec2 {
        egui::vec2(available_width, Self::loading_progress_height(scale))
    }

    pub fn loading_progress_rect(
        scale: UiScale,
        available_width: f32,
        top_left: egui::Pos2,
    ) -> egui::Rect {
        let min_width = scale.f32(Self::LOADING_PROGRESS_MIN_WIDTH);
        let width = (available_width * Self::LOADING_PROGRESS_WIDTH_RATIO)
            .max(min_width.min(available_width));
        egui::Rect::from_min_size(
            top_left,
            egui::vec2(width, Self::loading_progress_height(scale)),
        )
    }

    pub fn result_icon_size(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ICON_SIZE)
    }

    pub fn result_title_y(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ROW_TITLE_Y)
    }

    pub fn result_title_height(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ROW_TITLE_HEIGHT)
    }

    pub fn result_subtitle_y(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ROW_SUBTITLE_Y)
    }

    pub fn result_right_area_width(scale: UiScale, row_width: f32) -> f32 {
        scale
            .f32(Self::RESULT_RIGHT_AREA_WIDTH)
            .min(row_width * Self::RESULT_RIGHT_AREA_WIDTH_RATIO)
    }

    pub fn result_icon_text_gap(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ICON_TEXT_GAP)
    }

    pub fn result_text_right_gap(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_TEXT_RIGHT_GAP)
    }

    pub fn result_direct_keycap_width(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_DIRECT_KEYCAP_WIDTH)
    }

    pub fn result_primary_keycap_width(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_PRIMARY_KEYCAP_WIDTH)
    }

    pub fn result_action_label_width(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_ACTION_LABEL_WIDTH)
    }

    pub fn result_right_gap(scale: UiScale) -> f32 {
        scale.f32(Self::RESULT_RIGHT_GAP)
    }
}
