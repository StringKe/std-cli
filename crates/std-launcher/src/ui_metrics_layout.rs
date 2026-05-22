use eframe::egui;
use std_egui::tokens::{LauncherSize, Space, UiScale};
use std_launcher::{LauncherState, PANEL_WIDTH};

pub(crate) fn host_gutter_for_scale(scale: UiScale) -> f32 {
    LauncherSize::host_gutter(scale)
}

pub(crate) struct LauncherLayoutBudget {
    pub(crate) content_height: f32,
    pub(crate) total_height: f32,
}

pub(crate) fn initial_window_inner_size_for_scale(scale: UiScale) -> egui::Vec2 {
    let panel_size = egui::vec2(
        scale.f32(PANEL_WIDTH),
        collapsed_panel_height_for_scale(scale),
    );
    LauncherSize::host_size(panel_size, scale)
}

pub(crate) fn window_inner_size_for_scale(state: &LauncherState, scale: UiScale) -> egui::Vec2 {
    let mut panel_height = LauncherSize::DEFAULT_VIEWPORT_HEIGHT;
    for _ in 0..4 {
        let body_height = body_height_for_scale(state, panel_height, scale);
        let next_height = panel_height_for_scale(state, body_height, scale);
        if (next_height - panel_height).abs() < 0.5 {
            panel_height = next_height;
            break;
        }
        panel_height = next_height;
    }
    LauncherSize::host_size(egui::vec2(scale.f32(PANEL_WIDTH), panel_height), scale)
}

pub(crate) fn panel_height_for_scale(
    state: &LauncherState,
    body_height: f32,
    scale: UiScale,
) -> f32 {
    if !crate::ui_metrics::panel_is_expanded(state) {
        return collapsed_panel_height_for_scale(scale);
    }
    layout_budget_for_scale(state, body_height, scale).total_height
}

pub(crate) fn panel_content_height_for_scale(
    state: &LauncherState,
    body_height: f32,
    scale: UiScale,
) -> f32 {
    layout_budget_for_scale(state, body_height, scale).content_height
}

pub(crate) fn body_height_for_scale(
    state: &LauncherState,
    viewport_height: f32,
    scale: UiScale,
) -> f32 {
    if !crate::ui_metrics::panel_is_expanded(state) {
        return 0.0;
    }
    let visible_height = crate::ui_metrics::result_list_visible_height_for_scale(state, scale);
    let desired = visible_height + scale.f32(Space::SM as f32);
    desired.clamp(
        LauncherSize::body_min_height(scale),
        body_height_available(state, viewport_height, scale),
    )
}

pub(crate) fn layout_budget_for_scale(
    state: &LauncherState,
    body_height: f32,
    scale: UiScale,
) -> LauncherLayoutBudget {
    let padding = scale.f32(Space::MD as f32);
    let content = launcher_content_height_for_scale(state, body_height, scale);
    LauncherLayoutBudget {
        content_height: content,
        total_height: content + padding * 2.0,
    }
}

pub(crate) fn search_section_height_for_scale(scale: UiScale) -> f32 {
    LauncherSize::search_section_height(scale)
}

pub(crate) fn launcher_status_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    voice_status_height_for_scale(state, scale) + feedback_status_height_for_scale(state, scale)
}

fn body_height_available(state: &LauncherState, viewport_height: f32, scale: UiScale) -> f32 {
    let chrome = panel_content_height_for_scale(state, 0.0, scale);
    (viewport_height - chrome).max(LauncherSize::body_min_height(scale))
}

fn collapsed_panel_height_for_scale(scale: UiScale) -> f32 {
    LauncherSize::search_panel_height(scale)
}

fn launcher_content_height_for_scale(
    state: &LauncherState,
    body_height: f32,
    scale: UiScale,
) -> f32 {
    search_section_height_for_scale(scale)
        + scale.f32(Space::XS as f32)
        + body_height
        + scale.f32(Space::XS as f32)
        + scale.f32(crate::ui_metrics::ACTION_BAR_HEIGHT)
        + launcher_status_height_for_scale(state, scale)
}

fn voice_status_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    if state.controller.voice_active {
        return scale.f32(Space::XS as f32) + LauncherSize::voice_row_height(scale);
    }
    0.0
}

fn feedback_status_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    if state.view.feedback.is_some() {
        return scale.f32(Space::XS as f32)
            + crate::ui_metrics::feedback_panel_height_for_scale(scale);
    }
    0.0
}
