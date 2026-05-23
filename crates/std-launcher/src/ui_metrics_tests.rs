use super::*;
use std_egui::tokens::LauncherSize;
use std_launcher::PANEL_WIDTH;

#[test]
fn initial_window_size_scales_with_ui_zoom() {
    let base = crate::ui_metrics_layout::initial_window_inner_size_for_scale(UiScale::default());
    let zoomed = crate::ui_metrics_layout::initial_window_inner_size_for_scale(UiScale::new(1.5));

    assert_eq!(base, egui::vec2(720.0, 64.0));
    assert_eq!(zoomed, egui::vec2(1080.0, 96.0));
}

#[test]
fn expanded_window_size_scales_with_ui_zoom() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let body_height = crate::ui_metrics_layout::body_height_for_scale(
        &state,
        LauncherSize::DEFAULT_VIEWPORT_HEIGHT,
        UiScale::new(1.5),
    );
    let height =
        crate::ui_metrics_layout::panel_height_for_scale(&state, body_height, UiScale::new(1.5));

    assert!(
        height > crate::ui_metrics_layout::initial_window_inner_size_for_scale(UiScale::new(1.5)).y
    );
}

#[test]
fn row_metrics_scale_with_ui_zoom() {
    assert_eq!(
        row_metrics_for_scale(UiScale::new(1.5)),
        (54.0, 36.0, 51.0, 36.0, 27.0)
    );
}

#[test]
fn search_metrics_scale_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_search::search_metrics_for_scale(UiScale::new(1.5), 600.0),
        (60.0, 492.0, 324.0, 54.0, 42.0)
    );
}

#[test]
fn action_panel_metrics_scale_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_action_panel::metrics_for_scale(UiScale::new(1.5), 700.0),
        (480.0, 219.0, 42.0, 48.0)
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
        (600.0, 36.0, 6.0)
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
fn result_right_affordance_layout_reserves_fixed_keycap_and_action_regions() {
    assert_eq!(
        crate::ui_metrics_results::result_right_affordance_metrics_for_scale(
            UiScale::new(1.5),
            720.0
        ),
        (66.0, 102.0, 78.0, 12.0)
    );
}

#[test]
fn result_keycap_inset_scales_with_ui_zoom() {
    assert_eq!(
        crate::ui_metrics_results::result_keycap_shrink_for_scale(UiScale::new(1.5)),
        egui::vec2(3.0, 9.0)
    );
}

#[test]
fn panel_rect_anchors_to_upper_screen_region() {
    let position = screen_anchor_position(egui::vec2(1440.0, 900.0), egui::vec2(720.0, 320.0));

    assert_eq!(position, egui::pos2(360.0, 252.0));
}

#[test]
fn panel_surface_width_matches_docs_width_without_host_gap() {
    let width = panel_surface_width();

    assert_eq!(width, 720.0);
}

#[test]
fn panel_rect_stays_inside_tightly_sized_native_window() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.min, egui::Pos2::ZERO);
    assert_eq!(rect.max.x, available.right());
    assert!(rect.max.y <= available.bottom());
}

#[test]
fn panel_rect_matches_panel_sized_transparent_native_host() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.height(), available.height());
    assert_eq!(rect.min, egui::Pos2::ZERO);
    assert_eq!(rect.max.x, available.right());
}

#[test]
fn collapsed_panel_rect_matches_native_host_window() {
    let mut state = LauncherState::new();
    state.view.results.clear();
    state.view.preview = None;
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.height(), LauncherSize::SEARCH_PANEL_HEIGHT);
    assert_eq!(rect.min, egui::Pos2::ZERO);
    assert_eq!(available.height(), LauncherSize::SEARCH_PANEL_HEIGHT);
}

#[test]
fn panel_rect_uses_available_panel_surface_when_window_is_short() {
    let mut state = LauncherState::new();
    state.update_query("index");
    state.view.preview_executing();
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 240.0));
    let rect = panel_rect(available, &state);

    assert_eq!(rect.min, egui::Pos2::ZERO);
    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.max.y, available.bottom());
}

#[test]
fn native_host_is_tightly_sized_to_panel_surface() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), window_inner_size(&state));
    let rect = panel_rect(available, &state);

    assert_eq!(available.width(), PANEL_WIDTH);
    assert_eq!(rect.width(), PANEL_WIDTH);
    assert_eq!(rect.min, egui::Pos2::ZERO);
    assert_eq!(rect.max.x, available.right());
    assert_eq!(rect.max.y, available.bottom());
    assert!(panel_is_only_visible_surface(&state));
    let summary = panel_surface_geometry_summary(&state);
    assert!(summary.contains("panel_origin=0x0"));
    assert!(summary.contains("host_gap=0x0"));
    assert!(summary.contains("frame_clear=true"));
    assert!(summary.contains("panel_only_surface=true"));
}

#[test]
fn collapsed_launcher_uses_docs_search_bar_height_without_outer_padding() {
    let mut state = LauncherState::new();
    state.view.results.clear();
    state.view.preview = None;
    let viewport = window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), viewport);
    let rect = panel_rect(available, &state);

    assert_eq!(viewport.y, LauncherSize::SEARCH_PANEL_HEIGHT);
    assert_eq!(rect.height(), LauncherSize::SEARCH_PANEL_HEIGHT);
    assert_eq!(panel_inner_padding_for_state(&state), 0.0);
}

#[test]
fn native_host_window_height_includes_panel_inner_padding() {
    let mut state = LauncherState::new();
    state.update_query("index");
    let body = crate::ui_metrics_layout::body_height_for_scale(
        &state,
        LauncherSize::DEFAULT_VIEWPORT_HEIGHT,
        UiScale::default(),
    );
    let expected = Space::MD as f32 * 2.0
        + crate::ui_metrics_layout::search_section_height_for_scale(UiScale::default())
        + Space::XS as f32
        + body
        + Space::XS as f32
        + ACTION_BAR_HEIGHT;

    assert_eq!(window_inner_size(&state).y, expected);
}

#[test]
fn expanded_panel_height_budget_matches_rendered_sections_without_clipping() {
    for scenario in ["results", "defer", "error", "executing", "action-panel"] {
        let mut state = LauncherState::new();
        crate::preview::apply_preview_scenario(&mut state, scenario);
        let viewport = window_inner_size(&state);
        let available = egui::Rect::from_min_size(egui::Pos2::ZERO, viewport);
        let panel = panel_rect(available, &state);
        let body = crate::ui_metrics_layout::body_height_for_scale(
            &state,
            panel.height(),
            UiScale::default(),
        );
        let budget =
            crate::ui_metrics_layout::layout_budget_for_scale(&state, body, UiScale::default());

        assert_eq!(viewport.y, budget.total_height, "{scenario}");
        assert_eq!(panel.height(), budget.total_height, "{scenario}");
        assert!(available.contains_rect(panel), "{scenario}");
    }
}

#[test]
fn feedback_status_height_budget_covers_rendered_panel() {
    let mut state = LauncherState::new();
    crate::preview::apply_preview_scenario(&mut state, "defer");
    let scale = UiScale::default();
    let budget = crate::ui_metrics_layout::launcher_status_height_for_scale(&state, scale);
    let rendered_panel = feedback_panel_height_for_scale(scale);
    let rendered_budget = rendered_panel + scale.f32(Space::XS as f32);

    assert_eq!(feedback_panel_height_for_scale(scale), 74.0);
    assert_eq!(budget, rendered_budget);
    assert_eq!(budget, 82.0);
}

#[test]
fn body_height_counts_virtual_group_header_slots() {
    let mut state = LauncherState::new();
    state.update_query("terminal");
    let slots = result_list_slot_count(&state);
    let body = crate::ui_metrics_layout::body_height_for_scale(
        &state,
        LauncherSize::DEFAULT_VIEWPORT_HEIGHT,
        UiScale::default(),
    );

    assert!(slots > state.view.results.len());
    let desired = slots.min(MAX_RESULT_ROWS as usize) as f32 * 36.0
        - crate::ui_results::group_count(&state.view.results).min(slots) as f32 * 12.0
        + 12.0;
    assert_eq!(desired, 72.0);
    assert_eq!(body, LauncherSize::BODY_MIN_HEIGHT);
}

#[test]
fn all_preview_visible_states_keep_transparent_host_and_opaque_panel() {
    for scenario in [
        "empty",
        "results",
        "no-results",
        "defer",
        "error",
        "searching",
        "executing",
        "action-panel",
    ] {
        let mut state = LauncherState::new();
        crate::preview::apply_preview_scenario(&mut state, scenario);

        assert!(panel_is_only_visible_surface(&state), "{scenario}");
    }
}

#[test]
fn action_panel_popover_stays_inside_panel_capture_window() {
    let mut state = LauncherState::new();
    crate::preview::apply_preview_scenario(&mut state, "action-panel");
    let viewport = window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::Pos2::ZERO, viewport);
    let panel = panel_rect(available, &state);
    let anchor = egui::Rect::from_min_size(
        egui::pos2(
            panel.left() + Space::MD as f32,
            panel.bottom() - Space::MD as f32 - 36.0,
        ),
        egui::vec2(panel.width() - Space::MD as f32 * 2.0, 36.0),
    );
    let popover = action_panel_rect(anchor, state.action_panel.items.len());

    assert!(available.contains_rect(popover));
    assert!(panel.contains_rect(popover));
    assert_eq!(popover.width(), 320.0);
    assert!(popover.bottom() <= anchor.top());
}

#[test]
fn empty_suggested_workflows_panel_uses_full_native_height() {
    let mut state = LauncherState::new();
    crate::preview::apply_preview_scenario(&mut state, "empty");
    let viewport = window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::Pos2::ZERO, viewport);
    let panel = panel_rect(available, &state);
    let body =
        crate::ui_metrics_layout::body_height_for_scale(&state, panel.height(), UiScale::default());

    assert!(
        body <= viewport.y
            - crate::ui_metrics_layout::panel_content_height_for_scale(
                &state,
                0.0,
                UiScale::default()
            )
    );
    assert_eq!(panel.height(), viewport.y);
    assert!(panel_is_only_visible_surface(&state));
}
