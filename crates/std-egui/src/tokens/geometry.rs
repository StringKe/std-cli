use super::{palette, typography::UiScale};
use crate::a11y::AccessibilityContext;

pub struct Space;

impl Space {
    pub const TWO_XS: i8 = 4;
    pub const XS: i8 = 8;
    pub const SM: i8 = 12;
    pub const MD: i8 = 16;
    pub const LG: i8 = 24;
    pub const XL: i8 = 32;
    pub const TWO_XL: i8 = 48;

    pub fn two_xs() -> i8 {
        UiScale::from_env().i8(Self::TWO_XS)
    }

    pub fn xs() -> i8 {
        UiScale::from_env().i8(Self::XS)
    }

    pub fn sm() -> i8 {
        UiScale::from_env().i8(Self::SM)
    }

    pub fn md() -> i8 {
        UiScale::from_env().i8(Self::MD)
    }

    pub fn lg() -> i8 {
        UiScale::from_env().i8(Self::LG)
    }

    pub fn xl() -> i8 {
        UiScale::from_env().i8(Self::XL)
    }

    pub fn two_xl() -> i8 {
        UiScale::from_env().i8(Self::TWO_XL)
    }

    pub(crate) fn md_for_scale(scale: UiScale) -> f32 {
        scale.f32(Self::MD as f32)
    }
}

pub struct Radius;

impl Radius {
    pub const SM: u8 = 4;
    pub const MD: u8 = 8;
    pub const LG: u8 = 12;
    pub const XL: u8 = 16;

    pub fn sm() -> u8 {
        UiScale::from_env().u8(Self::SM)
    }

    pub fn md() -> u8 {
        UiScale::from_env().u8(Self::MD)
    }

    pub fn lg() -> u8 {
        UiScale::from_env().u8(Self::LG)
    }

    pub fn xl() -> u8 {
        UiScale::from_env().u8(Self::XL)
    }
}

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

    pub fn input_size(width: f32) -> egui::Vec2 {
        egui::vec2(width, UiScale::from_env().f32(Self::INPUT_HEIGHT))
    }

    pub fn toolbar_field_width(available_width: f32, reserve_width: f32) -> f32 {
        (available_width - reserve_width).max(Self::ANALYSIS_FIELD_MIN_WIDTH)
    }

    pub fn thirds_column_width(available_width: f32, gap: f32) -> f32 {
        (available_width - gap * 2.0) / 3.0
    }
}

pub struct Elevation;

impl Elevation {
    pub fn level_2(ctx: &egui::Context) -> egui::Shadow {
        let a11y = AccessibilityContext::from_env();
        Self::level_2_for_scale(ctx, UiScale::from_env(), &a11y)
    }

    pub(crate) fn level_2_for_scale(
        ctx: &egui::Context,
        scale: UiScale,
        a11y: &AccessibilityContext,
    ) -> egui::Shadow {
        scaled_shadow(
            [0, scale.i8(8)],
            elevation_blur(scale.u8(24), a11y),
            palette::shadow_alpha(if ctx.style().visuals.dark_mode {
                128
            } else {
                26
            }),
        )
    }

    pub fn level_3(ctx: &egui::Context) -> egui::Shadow {
        let a11y = AccessibilityContext::from_env();
        Self::level_3_for_scale(ctx, UiScale::from_env(), &a11y)
    }

    pub(crate) fn level_3_for_scale(
        ctx: &egui::Context,
        scale: UiScale,
        a11y: &AccessibilityContext,
    ) -> egui::Shadow {
        scaled_shadow(
            [0, scale.i8(16)],
            elevation_blur(scale.u8(48), a11y),
            palette::shadow_alpha(if ctx.style().visuals.dark_mode {
                153
            } else {
                41
            }),
        )
    }
}

pub(crate) fn elevation_blur(default_blur: u8, a11y: &AccessibilityContext) -> u8 {
    if a11y.reduce_transparency {
        4
    } else {
        default_blur
    }
}

fn scaled_shadow(offset: [i8; 2], blur: u8, color: egui::Color32) -> egui::Shadow {
    egui::Shadow {
        offset,
        blur,
        spread: 0,
        color,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exported_spacing_matches_eight_point_grid() {
        assert_eq!(Space::TWO_XS, 4);
        assert_eq!(Space::XS, 8);
        assert_eq!(Space::SM, 12);
        assert_eq!(Space::MD, 16);
        assert_eq!(Space::LG, 24);
        assert_eq!(Space::XL, 32);
        assert_eq!(Space::TWO_XL, 48);
    }

    #[test]
    fn scaled_spacing_and_radius_follow_ui_zoom() {
        let scale = UiScale::new(1.5);

        assert_eq!(scale.i8(Space::SM), 18);
        assert_eq!(scale.u8(Radius::XL), 24);
        assert_eq!(Space::md_for_scale(scale), 24.0);
    }

    #[test]
    fn launcher_focus_rings_use_token_geometry() {
        let search = FocusRing::launcher_search();
        let results = FocusRing::launcher_results();
        let action_panel = FocusRing::launcher_action_panel();

        assert_eq!(search.radius, Radius::lg());
        assert_eq!(results.radius, Radius::md());
        assert_eq!(action_panel.radius, Radius::md());
        assert!(search.expand > action_panel.expand);
        assert!(search.width >= 1.0);
    }

    #[test]
    fn control_sizes_export_switch_geometry() {
        assert_eq!(ControlSize::switch_size(), egui::vec2(48.0, 24.0));
        assert_eq!(ControlSize::switch_right_inset(), 40.0);
    }

    #[test]
    fn overlay_sizes_export_context_help_geometry() {
        assert_eq!(
            OverlaySize::context_help_anchor_offset(),
            egui::vec2(0.0, 116.0)
        );
        assert_eq!(OverlaySize::context_help_width(), 560.0);
        assert_eq!(
            OverlaySize::context_help_grid_spacing(),
            egui::vec2(16.0, 8.0)
        );
    }

    #[test]
    fn navigation_sizes_export_studio_shell_geometry() {
        assert_eq!(NavigationSize::nav_row_size(240.0), egui::vec2(240.0, 28.0));
        assert_eq!(NavigationSize::icon_rail_size(), egui::vec2(32.0, 28.0));
        assert_eq!(NavigationSize::icon_size(), egui::vec2(16.0, 16.0));
        assert_eq!(
            NavigationSize::pane_row_size(240.0),
            egui::vec2(240.0, 36.0)
        );
        assert_eq!(NavigationSize::pane_close_size(), egui::vec2(24.0, 24.0));
        assert_eq!(NavigationSize::pane_close_center_inset(), 14.0);
        assert_eq!(NavigationSize::pane_state_center_x(), 10.0);
        assert_eq!(NavigationSize::pane_state_radius(), 3.0);
        assert_eq!(NavigationSize::close_glyph_half(), 4.0);
    }

    #[test]
    fn host_chrome_sizes_export_studio_window_geometry() {
        assert_eq!(HostChromeSize::control_size(), egui::vec2(32.0, 24.0));
        assert_eq!(HostChromeSize::close_icon_half(), 4.0);
        assert_eq!(HostChromeSize::minimize_icon_half_width(), 4.0);
        assert_eq!(HostChromeSize::minimize_icon_y_offset(), 4.0);
        assert_eq!(HostChromeSize::maximize_icon_size(), egui::vec2(12.0, 8.0));
        assert_eq!(HostChromeSize::drag_inset(), egui::vec2(12.0, 8.0));
        assert_eq!(HostChromeSize::drag_min_width(), 320.0);
        assert_eq!(HostChromeSize::action_reserve_width(), 520.0);
        assert_eq!(HostChromeSize::test_chrome_size(), egui::vec2(1280.0, 52.0));
    }

    #[test]
    fn studio_sizes_export_workspace_layout_geometry() {
        assert_eq!(StudioSize::TOOLBAR_HEIGHT, 44.0);
        assert_eq!(StudioSize::TAB_BAR_HEIGHT, 36.0);
        assert_eq!(StudioSize::INPUT_HEIGHT, 28.0);
        assert_eq!(StudioSize::WIDE_WORKSPACE_BREAKPOINT, 900.0);
        assert_eq!(StudioSize::input_size(320.0), egui::vec2(320.0, 28.0));
        assert_eq!(StudioSize::thirds_column_width(924.0, 12.0), 300.0);
        assert_eq!(StudioSize::toolbar_field_width(300.0, 110.0), 260.0);
        assert_eq!(StudioSize::toolbar_field_width(500.0, 110.0), 390.0);
    }

    #[test]
    fn exported_elevation_matches_documented_shadow_levels() {
        let ctx = egui::Context::default();
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };
        let level_2 = Elevation::level_2_for_scale(&ctx, UiScale::default(), &a11y);
        let level_3 = Elevation::level_3_for_scale(&ctx, UiScale::default(), &a11y);

        assert_eq!(level_2.offset, [0, 8]);
        assert_eq!(level_2.blur, 24);
        assert_eq!(level_3.offset, [0, 16]);
        assert_eq!(level_3.blur, 48);
    }

    #[test]
    fn reduce_transparency_uses_harder_elevation_edges() {
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: true,
            high_contrast: false,
            bold_text: false,
        };

        assert_eq!(elevation_blur(24, &a11y), 4);
        assert_eq!(elevation_blur(48, &a11y), 4);
    }

    #[test]
    fn elevation_shadow_geometry_scales_with_ui_zoom() {
        let ctx = egui::Context::default();
        let a11y = AccessibilityContext {
            reduce_motion: false,
            reduce_transparency: false,
            high_contrast: false,
            bold_text: false,
        };

        let level_3 = Elevation::level_3_for_scale(&ctx, UiScale::new(1.5), &a11y);

        assert_eq!(level_3.offset, [0, 24]);
        assert_eq!(level_3.blur, 72);
    }
}
