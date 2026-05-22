use super::base::{Radius, Space};
use crate::{a11y::AccessibilityContext, tokens::typography::UiScale};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusRing {
    pub radius: u8,
    pub expand: f32,
    pub width: f32,
}

impl FocusRing {
    pub fn launcher_search() -> Self {
        Self::new(Radius::lg(), 2.0)
    }

    pub fn launcher_results() -> Self {
        Self::new(Radius::md(), 2.0)
    }

    pub fn launcher_action_panel() -> Self {
        Self::new(Radius::md(), 1.0)
    }

    fn new(radius: u8, expand: f32) -> Self {
        let a11y = AccessibilityContext::from_env();
        Self {
            radius,
            expand: UiScale::from_env().f32(expand),
            width: a11y.focus_ring_width(),
        }
    }
}

pub struct ControlSize;

impl ControlSize {
    pub const SWITCH_WIDTH: f32 = 48.0;
    pub const SWITCH_HEIGHT: f32 = 24.0;
    pub const SWITCH_RIGHT_INSET: f32 = 40.0;

    pub fn switch_size() -> egui::Vec2 {
        let scale = UiScale::from_env();
        egui::vec2(
            scale.f32(Self::SWITCH_WIDTH),
            scale.f32(Self::SWITCH_HEIGHT),
        )
    }

    pub fn switch_right_inset() -> f32 {
        UiScale::from_env().f32(Self::SWITCH_RIGHT_INSET)
    }
}

pub struct OverlaySize;

impl OverlaySize {
    pub const CONTEXT_HELP_Y: f32 = 116.0;
    pub const CONTEXT_HELP_WIDTH: f32 = 560.0;

    pub fn context_help_anchor_offset() -> egui::Vec2 {
        egui::vec2(0.0, UiScale::from_env().f32(Self::CONTEXT_HELP_Y))
    }

    pub fn context_help_width() -> f32 {
        UiScale::from_env().f32(Self::CONTEXT_HELP_WIDTH)
    }

    pub fn context_help_grid_spacing() -> egui::Vec2 {
        egui::vec2(Space::md() as f32, Space::xs() as f32)
    }
}

pub struct NavigationSize;

impl NavigationSize {
    pub const ROW_HEIGHT: f32 = 28.0;
    pub const PANE_ROW_HEIGHT: f32 = 36.0;
    pub const PANE_STATE_RADIUS: f32 = 3.0;

    pub fn nav_row_size(width: f32) -> egui::Vec2 {
        egui::vec2(width, UiScale::from_env().f32(Self::ROW_HEIGHT))
    }

    pub fn icon_rail_size() -> egui::Vec2 {
        egui::vec2(
            Space::xl() as f32,
            UiScale::from_env().f32(Self::ROW_HEIGHT),
        )
    }

    pub fn icon_size() -> egui::Vec2 {
        egui::vec2(Space::md() as f32, Space::md() as f32)
    }

    pub fn pane_row_size(width: f32) -> egui::Vec2 {
        egui::vec2(width, UiScale::from_env().f32(Self::PANE_ROW_HEIGHT))
    }

    pub fn pane_close_size() -> egui::Vec2 {
        egui::vec2(Space::lg() as f32, Space::lg() as f32)
    }

    pub fn pane_close_center_inset() -> f32 {
        Space::sm() as f32 + Space::two_xs() as f32 / 2.0
    }

    pub fn pane_state_center_x() -> f32 {
        Space::xs() as f32 + Space::two_xs() as f32 / 2.0
    }

    pub fn pane_state_radius() -> f32 {
        UiScale::from_env().f32(Self::PANE_STATE_RADIUS)
    }

    pub fn close_glyph_half() -> f32 {
        Space::xs() as f32 / 2.0
    }
}

pub struct HostChromeSize;

impl HostChromeSize {
    pub const DRAG_MIN_WIDTH: f32 = 320.0;
    pub const ACTION_RESERVE_WIDTH: f32 = 520.0;
    pub const TEST_CHROME_WIDTH: f32 = 1280.0;
    pub const TEST_CHROME_HEIGHT: f32 = 52.0;

    pub fn control_size() -> egui::Vec2 {
        egui::vec2(Space::xl() as f32, Space::lg() as f32)
    }

    pub fn close_icon_half() -> f32 {
        Space::two_xs() as f32
    }

    pub fn minimize_icon_half_width() -> f32 {
        Space::xs() as f32 * 0.5
    }

    pub fn minimize_icon_y_offset() -> f32 {
        Space::xs() as f32 * 0.5
    }

    pub fn maximize_icon_size() -> egui::Vec2 {
        egui::vec2(Space::sm() as f32, Space::xs() as f32)
    }

    pub fn drag_inset() -> egui::Vec2 {
        egui::vec2(Space::sm() as f32, Space::xs() as f32)
    }

    pub fn drag_min_width() -> f32 {
        UiScale::from_env().f32(Self::DRAG_MIN_WIDTH)
    }

    pub fn action_reserve_width() -> f32 {
        UiScale::from_env().f32(Self::ACTION_RESERVE_WIDTH)
    }

    pub fn test_chrome_size() -> egui::Vec2 {
        egui::vec2(Self::TEST_CHROME_WIDTH, Self::TEST_CHROME_HEIGHT)
    }
}

pub struct StudioSize;

impl StudioSize {
    pub const INPUT_HEIGHT: f32 = 28.0;
    pub const TOOLBAR_HEIGHT: f32 = 44.0;
    pub const TAB_BAR_HEIGHT: f32 = 36.0;
    pub const WIDE_WORKSPACE_BREAKPOINT: f32 = 900.0;
    pub const FORM_BUTTON_RESERVE_WIDTH: f32 = 110.0;
    pub const PLUGIN_TOOLBAR_ACTIONS_WIDTH: f32 = 230.0;
    pub const ANALYSIS_TOOLBAR_ACTIONS_WIDTH: f32 = 420.0;
    pub const ANALYSIS_QUERY_ACTIONS_WIDTH: f32 = 176.0;
    pub const ANALYSIS_FIELD_MIN_WIDTH: f32 = 260.0;
    pub const SEARCH_RESULTS_MAX_HEIGHT: f32 = 430.0;
    pub const DETAIL_BODY_MAX_HEIGHT: f32 = 480.0;
    pub const PANEL_LIST_MAX_HEIGHT: f32 = 560.0;
    pub const MEMORY_LIST_MAX_HEIGHT: f32 = 590.0;
    pub const PLUGIN_CHECKS_MAX_HEIGHT: f32 = 190.0;
    pub const MULTILINE_INPUT_HEIGHT: f32 = 220.0;
    pub const WORKFLOW_BUILDER_WIDE_BREAKPOINT: f32 = 560.0;
    pub const WORKFLOW_BUILDER_LEFT_RATIO: f32 = 0.48;
    pub const WORKFLOW_BUILDER_PANE_MIN_WIDTH: f32 = 260.0;
    pub const WORKFLOW_BUILDER_PARAMETERS_HEIGHT: f32 = 92.0;

    pub fn input_size(width: f32) -> egui::Vec2 {
        egui::vec2(width, UiScale::from_env().f32(Self::INPUT_HEIGHT))
    }

    pub fn toolbar_field_width(available_width: f32, reserve_width: f32) -> f32 {
        (available_width - reserve_width).max(Self::ANALYSIS_FIELD_MIN_WIDTH)
    }

    pub fn thirds_column_width(available_width: f32, gap: f32) -> f32 {
        (available_width - gap * 2.0) / 3.0
    }

    pub fn workflow_builder_panel_gap() -> f32 {
        Space::sm() as f32
    }

    pub fn workflow_builder_columns(available_width: f32) -> Option<(f32, f32)> {
        if available_width < Self::WORKFLOW_BUILDER_WIDE_BREAKPOINT {
            return None;
        }
        let gap = Self::workflow_builder_panel_gap();
        let left = ((available_width - gap) * Self::WORKFLOW_BUILDER_LEFT_RATIO)
            .max(Self::WORKFLOW_BUILDER_PANE_MIN_WIDTH);
        let right = (available_width - left - gap).max(Self::WORKFLOW_BUILDER_PANE_MIN_WIDTH);
        Some((left, right))
    }

    pub fn workflow_builder_goal_input_size(available_width: f32) -> [f32; 2] {
        [
            available_width.min(Self::WORKFLOW_BUILDER_PANE_MIN_WIDTH),
            Self::INPUT_HEIGHT,
        ]
    }

    pub fn workflow_builder_parameter_editor_size(available_width: f32) -> [f32; 2] {
        [available_width, Self::WORKFLOW_BUILDER_PARAMETERS_HEIGHT]
    }

    pub fn workflow_builder_ai_input_size(available_width: f32) -> [f32; 2] {
        [available_width, Space::xl() as f32]
    }

    pub fn workflow_builder_pane_size(width: f32) -> egui::Vec2 {
        egui::vec2(width, 0.0)
    }
}

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
}
