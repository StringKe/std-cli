use super::*;

#[test]
fn initial_window_size_scales_with_ui_zoom() {
    let base = initial_window_inner_size_for_scale(UiScale::default());
    let zoomed = initial_window_inner_size_for_scale(UiScale::new(1.5));

    assert_eq!(base, egui::vec2(720.0, 96.0));
    assert_eq!(zoomed, egui::vec2(1080.0, 144.0));
}

#[test]
fn expanded_window_size_scales_with_ui_zoom() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let body_height = body_height_for_scale(&state, DEFAULT_VIEWPORT_HEIGHT, UiScale::new(1.5));
    let height = panel_height_for_scale(&state, body_height, UiScale::new(1.5));

    assert!(height > initial_window_inner_size_for_scale(UiScale::new(1.5)).y);
}

#[test]
fn row_metrics_scale_with_ui_zoom() {
    assert_eq!(
        row_metrics_for_scale(UiScale::new(1.5)),
        (54.0, 51.0, 36.0, 27.0)
    );
}

#[test]
fn search_metrics_scale_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_search::search_metrics_for_scale(UiScale::new(1.5), 600.0),
        (66.0, 492.0, 54.0, 4.5, 42.0)
    );
}

#[test]
fn action_panel_metrics_scale_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_action_panel::metrics_for_scale(UiScale::new(1.5), 700.0),
        (480.0, 219.0, 42.0, 3.0, 48.0)
    );
}

#[test]
fn empty_state_icon_metrics_scale_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_empty::no_matches_icon_metrics_for_scale(UiScale::new(1.5)),
        (egui::vec2(48.0, 48.0), 13.5)
    );
}

#[test]
fn loading_progress_metrics_scale_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_results::loading_progress_metrics_for_scale(UiScale::new(1.5), 600.0),
        (228.0, 3.0)
    );
}

#[test]
fn group_header_divider_scales_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_results::group_header_metrics_for_scale(UiScale::new(1.5), 600.0),
        (600.0, 1.5)
    );
}

#[test]
fn group_header_uses_stable_virtual_list_slot_height() {
    assert_eq!(
        crate::ui_metrics_results::group_header_slot_metrics_for_scale(UiScale::new(1.5), 600.0),
        (600.0, 54.0, 6.0)
    );
}

#[test]
fn result_row_layout_reserves_icon_text_and_right_hint_regions() {
    assert_eq!(
        crate::ui_metrics_results::result_row_layout_metrics_for_scale(UiScale::new(1.5), 720.0),
        (30.0, 384.0, 270.0)
    );
}

#[test]
fn panel_rect_anchors_to_upper_screen_region() {
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1440.0, 900.0));
    let rect = panel_rect_for_available(available, 720.0, 320.0, window_margin(), true);

    assert_eq!(rect.width(), 720.0);
    assert_eq!(rect.min.x, 360.0);
    assert_eq!(rect.min.y, 252.0);
}

#[test]
fn panel_width_caps_at_docs_max_on_large_available_region() {
    let width = panel_width_for_available(1440.0, window_margin());

    assert_eq!(width, 720.0);
}

#[test]
fn panel_width_uses_docs_ratio_on_medium_available_region() {
    let width = panel_width_for_available(1000.0, window_margin());

    assert_eq!(width, 550.0);
}

#[test]
fn panel_rect_stays_inside_tightly_sized_native_window() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.min.x, 0.0);
    assert_eq!(rect.min.y, 0.0);
    assert_eq!(rect.max.x, available.right());
    assert!(rect.max.y <= available.bottom());
}

#[test]
fn panel_rect_matches_native_viewport_height() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.height(), available.height());
}

#[test]
fn panel_rect_centers_inside_wide_available_region() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(1000.0, 900.0));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.width(), 550.0);
    assert_eq!(rect.center().x, available.center().x);
    assert!(rect.width() < available.width());
}

#[test]
fn collapsed_panel_rect_matches_native_viewport() {
    let mut state = LauncherState::new();
    state.view.results.clear();
    state.view.preview = None;
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.height(), SEARCH_HEIGHT + Space::MD as f32 * 2.0);
    assert_eq!(rect.height(), available.height());
}

#[test]
fn panel_rect_clamps_when_viewport_is_short() {
    let mut state = LauncherState::new();
    state.update_query("index");
    state.view.preview_executing();
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 240.0));
    let rect = panel_rect(available, &state);

    assert!(rect.min.y >= 0.0);
    assert!(rect.max.y <= available.bottom());
}

#[test]
fn native_viewport_is_the_launcher_panel_not_a_carrier() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(available.width(), PANEL_WIDTH);
    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.min.x, 0.0);
    assert_eq!(rect.max.x, available.right());
    assert_eq!(rect.min.y, 0.0);
    assert_eq!(rect.max.y, available.bottom());
}

#[test]
fn native_viewport_height_includes_panel_inner_padding() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let body = body_height_for_scale(&state, DEFAULT_VIEWPORT_HEIGHT, UiScale::default());
    let chrome = Space::MD as f32 * 3.0 + Space::SM as f32;
    let expected = SEARCH_HEIGHT + body + ACTION_BAR_HEIGHT + chrome;

    assert_eq!(window_inner_size(&state).y, expected);
}

#[test]
fn body_height_counts_virtual_group_header_slots() {
    let mut state = LauncherState::new();
    state.update_query("terminal");
    let slots = result_list_slot_count(&state);
    let body = body_height_for_scale(&state, DEFAULT_VIEWPORT_HEIGHT, UiScale::default());

    assert!(slots > state.view.results.len());
    assert_eq!(
        body,
        slots.min(MAX_RESULT_ROWS as usize) as f32 * 36.0 + 12.0
    );
}
