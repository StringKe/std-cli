use crate::{preview::apply_preview_scenario, ui, ui_metrics};
use eframe::egui;
use std_egui::tokens::{apply_theme, Color, ThemeMode};
use std_launcher::LauncherState;
use std_types::ActionExecutionStatus;

pub(crate) fn preview_state_summary(scenario: &crate::preview::LauncherPreviewScenario) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let valid =
        matches!(scenario.theme, "dark" | "light") && preview_state_passes(&state, scenario.state);
    let theme = ThemeMode::resolve(scenario.theme);
    let surface = preview_surface_summary(theme);
    format!(
        "{}={}:phase={:?},results={},feedback={},{}",
        scenario.label(),
        if valid { "PASS" } else { "FAIL" },
        state.view.phase,
        state.view.results.len(),
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.title.as_str())
            .unwrap_or("none"),
        surface
    )
}

pub(crate) fn preview_size_summary(scenario: &crate::preview::LauncherPreviewScenario) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let viewport = ui::launcher_window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), viewport);
    let rect = ui_metrics::panel_rect(available, &state);
    let body = ui_metrics::body_height(&state, viewport.y);
    let content_clearance = rect.height()
        - ui_metrics::panel_inner_padding() * 2.0
        - ui_metrics::panel_content_height(&state, body);
    let fits = rect.min.y == 0.0
        && rect.max.y <= viewport.y
        && (rect.height() - viewport.y).abs() < 0.5
        && viewport.x >= rect.width()
        && body >= 0.0
        && content_clearance >= 0.0;
    let panel_frame = launcher_panel_frame_contract();
    format!(
        "{}={}:viewport={}x{},panel={}x{},body={},bottom_clearance={},content_clearance={},panel_frame={}",
        scenario.label(),
        if fits { "PASS" } else { "FAIL" },
        viewport.x.round() as u32,
        viewport.y.round() as u32,
        rect.width().round() as u32,
        rect.height().round() as u32,
        body.round() as u32,
        (viewport.y - rect.max.y).round() as i32,
        content_clearance.round() as i32,
        panel_frame
    )
}

fn launcher_panel_frame_contract() -> &'static str {
    let source = include_str!("ui.rs");
    if source.contains("ui.set_min_width(panel_rect.width())")
        && source.contains("ui.set_min_height(panel_rect.height())")
        && source.contains("ui.set_min_height(panel_rect.height() - padding * 2.0)")
    {
        "fills_viewport"
    } else {
        "natural_content_height"
    }
}

fn preview_state_passes(state: &LauncherState, state_name: &str) -> bool {
    match state_name {
        "empty" => {
            state.view.phase == std_egui::LauncherPhase::Empty
                && state.view.result_mode == std_egui::LauncherResultMode::SuggestedWorkflows
        }
        "results" => {
            state.view.phase == std_egui::LauncherPhase::WithResults
                && !state.view.results.is_empty()
        }
        "no-results" => {
            state.view.phase == std_egui::LauncherPhase::NoMatches && state.view.results.is_empty()
        }
        "defer" => state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.status == ActionExecutionStatus::NeedsExternalRunner)
            .unwrap_or(false),
        "error" => state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.status == ActionExecutionStatus::Failed)
            .unwrap_or(false),
        "searching" => state.view.phase == std_egui::LauncherPhase::Searching,
        "executing" => state.view.phase == std_egui::LauncherPhase::Executing,
        "action-panel" => state.action_panel.open,
        _ => false,
    }
}

fn preview_surface_summary(theme: ThemeMode) -> String {
    let ctx = egui::Context::default();
    apply_theme(&ctx, theme);
    format!(
        "panel={},search={},result={},selected={}",
        color_hex(Color::bg_surface_0(&ctx)),
        color_hex(Color::bg_surface_1(&ctx)),
        color_hex(Color::bg_surface_1(&ctx)),
        color_hex_alpha(Color::accent_weak(&ctx))
    )
}

fn color_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

fn color_hex_alpha(color: egui::Color32) -> String {
    format!(
        "#{:02X}{:02X}{:02X}@{}",
        color.r(),
        color.g(),
        color.b(),
        color.a()
    )
}
