use super::{
    elevation::elevation_blur, ControlSize, Elevation, FocusRing, HostChromeSize, NavigationSize,
    OverlaySize, Radius, Space, StudioSize,
};
use crate::{a11y::AccessibilityContext, tokens::typography::UiScale};

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
fn studio_sizes_export_workflow_builder_geometry() {
    assert_eq!(StudioSize::WORKFLOW_BUILDER_WIDE_BREAKPOINT, 560.0);
    assert_eq!(StudioSize::WORKFLOW_BUILDER_LEFT_RATIO, 0.48);
    assert_eq!(StudioSize::WORKFLOW_BUILDER_PANE_MIN_WIDTH, 260.0);
    assert_eq!(StudioSize::workflow_builder_panel_gap(), 12.0);
    assert_eq!(StudioSize::workflow_builder_columns(559.0), None);
    assert_eq!(
        StudioSize::workflow_builder_columns(800.0),
        Some((378.24, 409.76))
    );
    assert_eq!(
        StudioSize::workflow_builder_goal_input_size(800.0),
        [260.0, 28.0]
    );
    assert_eq!(
        StudioSize::workflow_builder_parameter_editor_size(640.0),
        [640.0, 92.0]
    );
    assert_eq!(
        StudioSize::workflow_builder_ai_input_size(640.0),
        [640.0, 32.0]
    );
    assert_eq!(
        StudioSize::workflow_builder_pane_size(320.0),
        egui::vec2(320.0, 0.0)
    );
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
