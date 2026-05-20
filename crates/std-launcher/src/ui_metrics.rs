use eframe::egui;
use std_egui::tokens::{Space, UiScale};
use std_egui::LauncherPhase;
use std_launcher::LauncherState;

const PANEL_WIDTH: f32 = 720.0;
const SEARCH_HEIGHT: f32 = 64.0;
const ACTION_BAR_HEIGHT: f32 = 36.0;
const RESULT_ROW_HEIGHT: f32 = 36.0;
const GROUP_ROW_HEIGHT: f32 = 24.0;
const MAX_RESULT_ROWS: f32 = 6.0;
const DEFAULT_VIEWPORT_HEIGHT: f32 = 520.0;

pub(crate) fn panel_width() -> f32 {
    UiScale::from_env().f32(PANEL_WIDTH)
}

pub(crate) fn window_margin() -> f32 {
    Space::sm() as f32
}

pub(crate) fn initial_window_inner_size() -> egui::Vec2 {
    initial_window_inner_size_for_scale(UiScale::from_env())
}

pub(crate) fn window_inner_size(state: &LauncherState) -> egui::Vec2 {
    let scale = UiScale::from_env();
    let body_height = body_height_for_scale(state, DEFAULT_VIEWPORT_HEIGHT, scale);
    egui::vec2(
        scale.f32(PANEL_WIDTH) + scale.f32(Space::SM as f32) * 2.0,
        panel_height_for_scale(state, body_height, scale) + scale.f32(Space::SM as f32) * 2.0,
    )
}

pub(crate) fn panel_height(state: &LauncherState, body_height: f32) -> f32 {
    panel_height_for_scale(state, body_height, UiScale::from_env())
}

pub(crate) fn body_height(state: &LauncherState, viewport_height: f32) -> f32 {
    body_height_for_scale(state, viewport_height, UiScale::from_env())
}

pub(crate) fn panel_is_expanded(state: &LauncherState) -> bool {
    (state.view.phase != LauncherPhase::Empty || !state.view.results.is_empty())
        || state.controller.voice_active
        || state.view.feedback.is_some()
        || state.action_panel.open
}

fn initial_window_inner_size_for_scale(scale: UiScale) -> egui::Vec2 {
    egui::vec2(
        scale.f32(PANEL_WIDTH) + scale.f32(Space::SM as f32) * 2.0,
        scale.f32(SEARCH_HEIGHT) + scale.f32(Space::SM as f32) * 2.0,
    )
}

fn panel_height_for_scale(state: &LauncherState, body_height: f32, scale: UiScale) -> f32 {
    if !panel_is_expanded(state) {
        return scale.f32(SEARCH_HEIGHT);
    }
    scale.f32(SEARCH_HEIGHT)
        + body_height
        + scale.f32(ACTION_BAR_HEIGHT)
        + scale.f32(Space::MD as f32)
        + scale.f32(Space::SM as f32)
        + extra_status_height_for_scale(state, scale)
}

fn body_height_for_scale(state: &LauncherState, viewport_height: f32, scale: UiScale) -> f32 {
    let visible_rows = state.view.results.len().clamp(1, MAX_RESULT_ROWS as usize) as f32;
    let groups = crate::ui_results::group_count(&state.view.results).max(1) as f32;
    let desired = visible_rows * scale.f32(RESULT_ROW_HEIGHT)
        + groups * scale.f32(GROUP_ROW_HEIGHT)
        + scale.f32(Space::SM as f32);
    desired.clamp(scale.f32(128.0), viewport_height * 0.6)
}

fn extra_status_height_for_scale(state: &LauncherState, scale: UiScale) -> f32 {
    let mut height = 0.0;
    if state.controller.voice_active {
        height += scale.f32(44.0) + scale.f32(Space::XS as f32);
    }
    if state.view.feedback.is_some() {
        height += scale.f32(48.0) + scale.f32(Space::XS as f32);
    }
    height
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn initial_window_size_scales_with_ui_zoom() {
        let base = initial_window_inner_size_for_scale(UiScale::default());
        let zoomed = initial_window_inner_size_for_scale(UiScale::new(1.5));

        assert_eq!(base, egui::vec2(744.0, 88.0));
        assert_eq!(zoomed, egui::vec2(1116.0, 132.0));
    }

    #[test]
    fn window_size_reads_runtime_zoom_without_gui() {
        let _guard = env_lock();
        std::env::set_var("STD_UI_ZOOM", "1.5");

        assert_eq!(initial_window_inner_size(), egui::vec2(1116.0, 132.0));

        std::env::remove_var("STD_UI_ZOOM");
    }
}
