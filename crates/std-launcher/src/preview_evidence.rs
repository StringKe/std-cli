use crate::{
    preview::apply_preview_scenario,
    preview_affordance::LauncherAffordanceSummary,
    preview_behavior::PreviewStateBehavior,
    preview_surface_evidence::{
        preview_surface_passes, preview_surface_summary, PreviewNativeHostSurface,
        PreviewNoMatchFallback, PreviewStateSurface,
    },
    ui, ui_metrics,
};
use eframe::egui;
use std_egui::tokens::ThemeMode;
use std_launcher::LauncherState;
use std_types::ActionExecutionStatus;

pub(crate) fn preview_state_summary(
    scenario: &crate::preview_contract::LauncherPreviewScenario,
) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let valid =
        matches!(scenario.theme, "dark" | "light") && preview_state_passes(&state, scenario.state);
    let theme = ThemeMode::resolve(scenario.theme);
    let surface = preview_surface_summary(theme);
    let affordance = LauncherAffordanceSummary::for_scenario(scenario.state);
    let state_surface = PreviewStateSurface::for_state(&state, scenario.state);
    let no_match_fallback = PreviewNoMatchFallback::for_state(&state, scenario.state);
    let host = PreviewNativeHostSurface::for_state(&state);
    let behavior = PreviewStateBehavior::for_state(&state, scenario.state);
    let passes = valid
        && preview_surface_passes(&surface, scenario.theme)
        && affordance.passes(scenario.state)
        && state_surface.passes(scenario.state)
        && no_match_fallback.passes(scenario.state)
        && host.passes()
        && behavior.passes(scenario.state)
        && feedback_status_icon_passes(scenario.state);
    format!(
        "{}={}:phase={:?},results={},feedback={},{},{},{},{},{},{},{}",
        scenario.label(),
        if passes { "PASS" } else { "FAIL" },
        state.view.phase,
        state.view.results.len(),
        state
            .view
            .feedback
            .as_ref()
            .map(|feedback| feedback.title.as_str())
            .unwrap_or("none"),
        affordance.summary(),
        state_surface.summary(),
        no_match_fallback.summary(),
        host.summary(),
        behavior.summary(),
        feedback_status_icon_summary(scenario.state),
        surface.summary()
    )
}

pub(crate) fn preview_size_summary(
    scenario: &crate::preview_contract::LauncherPreviewScenario,
) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let viewport = ui::launcher_window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), viewport);
    let rect = ui_metrics::panel_rect(available, &state);
    let body = ui_metrics::body_height(&state, rect.height());
    let padding = ui_metrics::panel_inner_padding_for_state(&state);
    let content_clearance = preview_content_clearance(&state, rect.height(), padding, body);
    let fits = rect.min.y >= 0.0
        && rect.max.y <= viewport.y + 0.5
        && viewport.x >= rect.width()
        && body >= 0.0
        && content_clearance >= 0.0;
    let panel_frame = launcher_panel_frame_contract(&state);
    let search_surface = launcher_search_surface_contract(&state);
    format!(
        "{}={}:viewport={}x{},panel={}x{},body={},bottom_clearance={},content_clearance={},budget_source=panel,panel_frame={},search_surface={},{}",
        scenario.label(),
        if fits { "PASS" } else { "FAIL" },
        viewport.x.round() as u32,
        viewport.y.round() as u32,
        rect.width().round() as u32,
        rect.height().round() as u32,
        body.round() as u32,
        (viewport.y - rect.max.y).round() as i32,
        content_clearance.round() as i32,
        panel_frame,
        search_surface,
        ui_metrics::panel_surface_geometry_summary(&state)
    )
}

fn preview_content_clearance(
    state: &LauncherState,
    panel_height: f32,
    padding: f32,
    body: f32,
) -> f32 {
    if !ui_metrics::panel_is_expanded(state) {
        return panel_height - ui_metrics::search_bar_min_height();
    }
    panel_height - padding * 2.0 - ui_metrics::panel_content_height(state, body)
}

fn launcher_panel_frame_contract(state: &LauncherState) -> &'static str {
    if ui_metrics::panel_is_only_visible_surface(state) {
        "transparent_host_with_opaque_panel_surface"
    } else {
        "native_background_visible_fail"
    }
}

fn launcher_search_surface_contract(state: &LauncherState) -> &'static str {
    if ui_metrics::panel_is_expanded(state) {
        "nested_search_surface"
    } else {
        "panel_as_search_surface"
    }
}

fn preview_state_passes(state: &LauncherState, state_name: &str) -> bool {
    match state_name {
        "empty" => {
            state.view.phase == std_egui::LauncherPhase::Empty
                && state.view.result_mode == std_egui::LauncherResultMode::SuggestedWorkflows
        }
        "collapsed" => {
            state.view.phase == std_egui::LauncherPhase::Empty
                && state.view.results.is_empty()
                && state.view.preview.is_none()
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
        "loading" | "searching" => state.view.phase == std_egui::LauncherPhase::Searching,
        "executing" => state.view.phase == std_egui::LauncherPhase::Executing,
        "ime" => {
            state.view.phase == std_egui::LauncherPhase::WithResults
                && state.ime_preedit.as_deref() == Some("zhong")
                && state.view.query == "index"
        }
        "action-panel" => state.action_panel.open,
        _ => false,
    }
}

fn feedback_status_icon_summary(state_name: &str) -> &'static str {
    match state_name {
        "defer" => "status_icon=deferred",
        "error" => "status_icon=failed",
        "executing" => "status_icon=not-rendered",
        _ => "status_icon=not-rendered",
    }
}

fn feedback_status_icon_passes(state_name: &str) -> bool {
    match state_name {
        "defer" => feedback_status_icon_summary(state_name) == "status_icon=deferred",
        "error" => feedback_status_icon_summary(state_name) == "status_icon=failed",
        _ => true,
    }
}
