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
    let passes = valid && preview_surface_passes(&surface, scenario.theme);
    format!(
        "{}={}:phase={:?},results={},feedback={},{}",
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
        surface.summary()
    )
}

pub(crate) fn preview_size_summary(scenario: &crate::preview::LauncherPreviewScenario) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let viewport = ui::launcher_window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), viewport);
    let rect = ui_metrics::panel_rect(available, &state);
    let body = ui_metrics::body_height(&state, viewport.y);
    let padding = ui_metrics::panel_inner_padding_for_state(&state);
    let content_clearance =
        rect.height() - padding * 2.0 - ui_metrics::panel_content_height(&state, body);
    let fits = rect.min.y == 0.0
        && rect.max.y <= viewport.y
        && (rect.height() - viewport.y).abs() < 0.5
        && viewport.x >= rect.width()
        && body >= 0.0
        && content_clearance >= 0.0;
    let panel_frame = launcher_panel_frame_contract(&state);
    let search_surface = launcher_search_surface_contract(&state);
    format!(
        "{}={}:viewport={}x{},panel={}x{},body={},bottom_clearance={},content_clearance={},panel_frame={},search_surface={}",
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
        search_surface
    )
}

fn launcher_panel_frame_contract(state: &LauncherState) -> &'static str {
    if ui_metrics::panel_frame_fills_viewport(state) {
        "fills_viewport"
    } else {
        "carrier_background_visible"
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
        "action-panel" => state.action_panel.open,
        _ => false,
    }
}

fn preview_surface_summary(theme: ThemeMode) -> PreviewSurfaceSummary {
    let ctx = egui::Context::default();
    apply_theme(&ctx, theme);
    PreviewSurfaceSummary {
        panel: color_hex(Color::bg_surface_0(&ctx)),
        search: color_hex(Color::bg_surface_1(&ctx)),
        result: color_hex(Color::bg_surface_1(&ctx)),
        selected: color_hex_alpha(Color::accent_weak(&ctx)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewSurfaceSummary {
    panel: String,
    search: String,
    result: String,
    selected: String,
}

impl PreviewSurfaceSummary {
    fn summary(&self) -> String {
        format!(
            "panel_token=bg/surface-0:{},search_token=bg/surface-1:{},result_token=bg/surface-1:{},selected_token=accent/weak:{}",
            self.panel, self.search, self.result, self.selected
        )
    }
}

fn preview_surface_passes(surface: &PreviewSurfaceSummary, theme: &str) -> bool {
    match theme {
        "dark" => {
            surface.panel == "#1C1E22"
                && surface.search == "#24272C"
                && surface.result == "#24272C"
                && surface.selected == "#4E9CFF@46"
        }
        "light" => {
            surface.panel == "#FAFBFD"
                && surface.search == "#F2F5F8"
                && surface.result == "#F2F5F8"
                && surface.selected == "#0A6BFF@31"
        }
        _ => false,
    }
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
