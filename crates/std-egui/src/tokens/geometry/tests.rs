use super::{
    elevation::elevation_blur, ControlSize, Elevation, FocusRing, HostChromeSize, LauncherSize,
    NavigationSize, OverlaySize, Radius, Space, StudioSize,
};
use crate::tokens::studio_rows;
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
fn studio_row_sizes_export_workspace_row_metrics() {
    assert_eq!(studio_rows::FILE_ROW_HEIGHT, 58.0);
    assert_eq!(studio_rows::BUILDER_STEP_ROW_HEIGHT, 48.0);
    assert_eq!(studio_rows::PLUGIN_LIST_ROW_HEIGHT, 88.0);
    assert_eq!(studio_rows::ANALYSIS_COVERAGE_ROW_HEIGHT, 78.0);
    assert_eq!(studio_rows::DASHBOARD_MEMORY_ROW_HEIGHT, 78.0);
    assert_eq!(studio_rows::SETTINGS_BINDING_ROW_HEIGHT, 64.0);
    assert_eq!(studio_rows::OPS_STEP_ROW_HEIGHT, 66.0);
    assert_eq!(studio_rows::TEXT_INSET_X, 12.0);
    assert_eq!(studio_rows::STATUS_CHIP_HEIGHT, 22.0);
    assert_eq!(studio_rows::status_chip_y_offset(), 11.0);
    assert_eq!(studio_rows::DASHBOARD_STEP_CHIP_HEIGHT, 24.0);
    assert_eq!(studio_rows::dashboard_step_chip_y_offset(), 12.0);
    assert_eq!(studio_rows::HISTORY_TIMELINE_PAYLOAD_LIMIT, 92);
}

#[test]
fn launcher_sizes_export_transparent_host_geometry() {
    let scale = UiScale::default();

    assert_eq!(LauncherSize::PANEL_WIDTH, 720.0);
    assert_eq!(LauncherSize::WINDOW_VERTICAL_ANCHOR, 0.28);
    assert_eq!(LauncherSize::panel_surface_width(scale), 720.0);
    assert_eq!(LauncherSize::host_gutter(scale), 16.0);
    assert_eq!(
        LauncherSize::host_size(egui::vec2(720.0, 64.0), scale),
        egui::vec2(752.0, 96.0)
    );
    assert_eq!(LauncherSize::hidden_host_size(), egui::vec2(1.0, 1.0));
    assert_eq!(
        LauncherSize::panel_position_for_monitor(
            egui::vec2(1440.0, 900.0),
            egui::vec2(720.0, 64.0)
        ),
        egui::pos2(360.0, 252.0)
    );
}

#[test]
fn launcher_sizes_export_search_geometry() {
    let scale = UiScale::new(1.5);
    let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(48.0, 42.0));
    let center = LauncherSize::search_icon_center(scale, rect);

    assert_eq!(LauncherSize::search_bar_min_height(scale), 60.0);
    assert_eq!(LauncherSize::search_input_width(scale, 600.0), 492.0);
    assert_eq!(
        LauncherSize::search_input_width_with_ime(scale, 600.0),
        324.0
    );
    assert_eq!(LauncherSize::search_input_height(scale), 54.0);
    assert_eq!(LauncherSize::search_ime_chip_width(scale), 168.0);
    assert_eq!(
        LauncherSize::search_icon_size(scale),
        egui::vec2(36.0, 42.0)
    );
    assert_eq!(center, egui::pos2(15.0, 18.0));
    assert_eq!(LauncherSize::voice_input_width(scale, 600.0), 432.0);
    assert_eq!(LauncherSize::voice_input_height(scale), 42.0);
}

#[test]
fn launcher_sizes_export_action_panel_geometry() {
    let scale = UiScale::new(1.5);

    assert_eq!(LauncherSize::ACTION_PANEL_WIDTH, 320.0);
    assert_eq!(LauncherSize::ACTION_PANEL_HEADER_HEIGHT, 44.0);
    assert_eq!(LauncherSize::ACTION_PANEL_ROW_HEIGHT, 32.0);
    assert_eq!(LauncherSize::ACTION_PANEL_ROW_STEP, 34.0);
    assert_eq!(LauncherSize::ACTION_PANEL_SEARCH_HEIGHT, 28.0);
    assert_eq!(LauncherSize::action_panel_width(scale, 700.0), 480.0);
    assert_eq!(LauncherSize::action_panel_width(scale, 360.0), 360.0);
    assert_eq!(LauncherSize::action_panel_height(scale, 3), 219.0);
    assert_eq!(LauncherSize::action_panel_search_height(scale), 42.0);
    assert_eq!(LauncherSize::action_panel_row_height(scale), 48.0);
}

#[test]
fn launcher_sizes_export_empty_state_geometry() {
    let scale = UiScale::new(1.5);
    let rect =
        egui::Rect::from_min_size(egui::Pos2::ZERO, LauncherSize::no_matches_icon_size(scale));
    let center = LauncherSize::no_matches_icon_center(scale, rect);

    assert_eq!(LauncherSize::NO_MATCHES_ICON_SIZE, 32.0);
    assert_eq!(LauncherSize::NO_MATCHES_ICON_CENTER_OFFSET, 2.0);
    assert_eq!(LauncherSize::NO_MATCHES_ICON_RADIUS, 9.0);
    assert_eq!(LauncherSize::NO_MATCHES_ICON_HANDLE_INSET, 7.0);
    assert_eq!(LauncherSize::NO_MATCHES_ICON_HANDLE_OUTSET, 13.0);
    assert_eq!(
        LauncherSize::no_matches_icon_size(scale),
        egui::vec2(48.0, 48.0)
    );
    assert_eq!(center, egui::pos2(21.0, 21.0));
    assert_eq!(LauncherSize::no_matches_icon_radius(scale), 13.5);
    assert_eq!(
        LauncherSize::no_matches_icon_handle_start(scale, center),
        egui::pos2(31.5, 31.5)
    );
    assert_eq!(
        LauncherSize::no_matches_icon_handle_end(scale, center),
        egui::pos2(40.5, 40.5)
    );
}

#[test]
fn launcher_sizes_export_result_list_geometry() {
    let scale = UiScale::new(1.5);
    let loading = LauncherSize::loading_progress_rect(scale, 600.0, egui::Pos2::ZERO);
    let group = LauncherSize::group_divider_rect(scale, 600.0, egui::Pos2::ZERO);

    assert_eq!(LauncherSize::RESULT_ROW_HEIGHT, 36.0);
    assert_eq!(LauncherSize::GROUP_HEADER_ROW_HEIGHT, 24.0);
    assert_eq!(LauncherSize::MAX_RESULT_ROWS, 6.0);
    assert_eq!(LauncherSize::result_row_height(scale), 54.0);
    assert_eq!(
        LauncherSize::result_row_size(scale, 720.0),
        egui::vec2(720.0, 54.0)
    );
    assert_eq!(
        LauncherSize::result_row_shrink(scale),
        egui::vec2(12.0, 0.0)
    );
    assert_eq!(LauncherSize::group_header_slot_height(scale), 36.0);
    assert_eq!(LauncherSize::group_header_label_offset_y(scale), 6.0);
    assert_eq!(group.width(), 600.0);
    assert_eq!(group.height(), 1.5);
    assert_eq!(loading.width(), 228.0);
    assert_eq!(loading.height(), 3.0);
}

#[test]
fn launcher_sizes_export_result_row_regions() {
    let scale = UiScale::new(1.5);

    assert_eq!(LauncherSize::result_icon_size(scale), 30.0);
    assert_eq!(LauncherSize::result_icon_text_gap(scale), 18.0);
    assert_eq!(LauncherSize::result_title_y(scale), 18.0);
    assert_eq!(LauncherSize::result_title_height(scale), 27.0);
    assert_eq!(LauncherSize::result_subtitle_y(scale), 42.0);
    assert_eq!(LauncherSize::result_right_area_width(scale, 720.0), 270.0);
    assert_eq!(LauncherSize::result_text_right_gap(scale), 18.0);
    assert_eq!(LauncherSize::result_direct_keycap_width(scale), 66.0);
    assert_eq!(LauncherSize::result_primary_keycap_width(scale), 78.0);
    assert_eq!(LauncherSize::result_action_label_width(scale), 138.0);
    assert_eq!(LauncherSize::result_right_gap(scale), 12.0);
    assert_eq!(
        LauncherSize::result_keycap_shrink(scale),
        egui::vec2(3.0, 9.0)
    );
}

#[test]
fn launcher_sizes_export_action_bar_geometry() {
    let scale = UiScale::new(1.5);
    let right_width = LauncherSize::action_bar_right_width(720.0);

    assert_eq!(LauncherSize::ASK_AI_ROW_HEIGHT, 34.0);
    assert_eq!(LauncherSize::SEARCH_PANEL_HEIGHT, 64.0);
    assert_eq!(LauncherSize::DEFAULT_VIEWPORT_HEIGHT, 520.0);
    assert_eq!(LauncherSize::BODY_MIN_HEIGHT, 128.0);
    assert_eq!(LauncherSize::VOICE_ROW_HEIGHT, 44.0);
    assert_eq!(LauncherSize::ACTION_BAR_HEIGHT, 36.0);
    assert_eq!(LauncherSize::ACTION_BAR_CONTENT_HEIGHT, 24.0);
    assert_eq!(LauncherSize::ACTION_BAR_RIGHT_WIDTH, 272.0);
    assert_eq!(LauncherSize::ACTION_BAR_RIGHT_WIDTH_RATIO, 0.48);
    assert_eq!(LauncherSize::ACTION_BAR_MIN_LEFT_WIDTH, 160.0);
    assert_eq!(LauncherSize::ACTION_SUMMARY_LABEL_HEIGHT, 18.0);
    assert_eq!(LauncherSize::ask_ai_row_height(scale), 51.0);
    assert_eq!(LauncherSize::search_panel_height(scale), 96.0);
    assert_eq!(LauncherSize::search_section_height(scale), 96.0);
    assert_eq!(LauncherSize::body_min_height(scale), 192.0);
    assert_eq!(LauncherSize::voice_row_height(scale), 66.0);
    assert_eq!(LauncherSize::action_bar_height(scale), 54.0);
    assert_eq!(LauncherSize::action_bar_content_height(scale), 36.0);
    assert_eq!(right_width, 272.0);
    assert_eq!(
        LauncherSize::action_bar_left_width(scale, 720.0, right_width),
        436.0
    );
    assert_eq!(LauncherSize::action_summary_label_height(scale), 27.0);
}

#[test]
fn launcher_sizes_export_feedback_geometry() {
    let scale = UiScale::new(1.5);

    assert_eq!(LauncherSize::FEEDBACK_TEXT_HEIGHT, 58.0);
    assert_eq!(LauncherSize::FEEDBACK_DETAIL_HEIGHT, 36.0);
    assert_eq!(LauncherSize::FEEDBACK_MIN_TEXT_WIDTH, 240.0);
    assert_eq!(LauncherSize::FEEDBACK_ACTION_WIDTH, 76.0);
    assert_eq!(LauncherSize::FEEDBACK_ICON_STROKE_WIDTH, 1.5);
    assert_eq!(LauncherSize::feedback_text_height(scale), 87.0);
    assert_eq!(LauncherSize::feedback_detail_height(scale), 54.0);
    assert_eq!(
        LauncherSize::feedback_text_width(scale, 720.0, 228.0),
        480.0
    );
    assert_eq!(LauncherSize::feedback_actions_width(scale, 2), 228.0);
    assert_eq!(
        LauncherSize::feedback_icon_size(scale),
        egui::vec2(24.0, 24.0)
    );
    assert_eq!(LauncherSize::feedback_icon_radius(scale), 12.0);
    assert_eq!(LauncherSize::feedback_icon_half(scale), 6.0);
    assert_eq!(LauncherSize::feedback_icon_stroke_width(scale), 2.25);
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
