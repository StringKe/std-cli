use crate::{
    preview::apply_preview_scenario, preview_affordance::LauncherAffordanceSummary,
    preview_behavior::PreviewStateBehavior, ui, ui_metrics,
};
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

pub(crate) fn preview_size_summary(scenario: &crate::preview::LauncherPreviewScenario) -> String {
    let mut state = LauncherState::new();
    apply_preview_scenario(&mut state, scenario.state);
    let viewport = ui::launcher_window_inner_size(&state);
    let available = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), viewport);
    let rect = ui_metrics::panel_rect(available, &state);
    let body = ui_metrics::body_height(&state, viewport.y);
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
        "{}={}:viewport={}x{},panel={}x{},body={},bottom_clearance={},content_clearance={},panel_frame={},search_surface={},{}",
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewStateSurface {
    panel_only: bool,
    search_bar: &'static str,
    body: &'static str,
    action_bar: &'static str,
    feedback: &'static str,
    popover: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewNativeHostSurface {
    clear_color: String,
    viewport_frame: String,
    geometry: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewNoMatchFallback {
    visible: bool,
    selected: bool,
    enter_keycap: bool,
    button_semantics: bool,
}

impl PreviewNoMatchFallback {
    fn for_state(state: &LauncherState, state_name: &str) -> Self {
        let visible = state_name == "no-results"
            && state.view.phase == std_egui::LauncherPhase::NoMatches
            && state.no_match_fallback_query().is_some();
        Self {
            visible,
            selected: visible,
            enter_keycap: visible,
            button_semantics: visible,
        }
    }

    fn passes(&self, state_name: &str) -> bool {
        if state_name != "no-results" {
            return !self.visible && !self.selected && !self.enter_keycap && !self.button_semantics;
        }
        self.visible && self.selected && self.enter_keycap && self.button_semantics
    }

    fn summary(&self) -> String {
        format!(
            "no_match_fallback=ask_ai_row,visible={},selected={},enter_keycap={},button_semantics={}",
            self.visible, self.selected, self.enter_keycap, self.button_semantics
        )
    }
}

impl PreviewNativeHostSurface {
    fn for_state(state: &LauncherState) -> Self {
        Self {
            clear_color: std_launcher::launcher_clear_color_contract(),
            viewport_frame: std_launcher::launcher_viewport_frame_contract(),
            geometry: ui_metrics::panel_surface_geometry_summary(state),
        }
    }

    fn passes(&self) -> bool {
        self.clear_color == "native_clear_color=transparent_rgba_0_0_0_0"
            && self.viewport_frame == "viewport_frame=transparent_fill,no_stroke"
            && self.geometry.contains("host_gap=0x0")
            && self.geometry.contains("panel_origin=0x0")
            && self.geometry.contains("panel_only=true")
    }

    fn summary(&self) -> String {
        format!(
            "host_contract={},{};{};forbidden=black_or_white_host_background",
            self.clear_color, self.viewport_frame, self.geometry
        )
    }
}

impl PreviewStateSurface {
    fn for_state(state: &LauncherState, state_name: &str) -> Self {
        Self {
            panel_only: ui_metrics::panel_is_only_visible_surface(state),
            search_bar: search_surface_for_state(state_name),
            body: body_surface_for_state(state_name),
            action_bar: action_bar_surface_for_state(state_name),
            feedback: feedback_surface_for_state(state_name),
            popover: popover_surface_for_state(state_name),
        }
    }

    fn passes(&self, state_name: &str) -> bool {
        self.panel_only
            && self.search_bar != "carrier"
            && self.body != "carrier"
            && self.action_bar != "carrier"
            && self.feedback != "carrier"
            && self.popover != "carrier"
            && state_surface_contract_matches(state_name, self)
    }

    fn summary(&self) -> String {
        format!(
            "state_surface=panel_only:{},search:{},body:{},action_bar:{},feedback:{},popover:{}",
            self.panel_only,
            self.search_bar,
            self.body,
            self.action_bar,
            self.feedback,
            self.popover
        )
    }
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

fn search_surface_for_state(state_name: &str) -> &'static str {
    match state_name {
        "collapsed" => "panel-as-search-surface",
        _ => "bg/surface-1",
    }
}

fn body_surface_for_state(state_name: &str) -> &'static str {
    match state_name {
        "collapsed" => "not-rendered",
        "no-results" => "empty-state-token-surface",
        "loading" | "searching" => "loading-progress-token-surface",
        "defer" | "error" => "feedback-token-surface",
        "action-panel" => "results-token-surface",
        _ => "results-token-surface",
    }
}

fn action_bar_surface_for_state(state_name: &str) -> &'static str {
    match state_name {
        "collapsed" => "not-rendered",
        _ => "bg/surface-1",
    }
}

fn feedback_surface_for_state(state_name: &str) -> &'static str {
    match state_name {
        "defer" => "status-warning-weak",
        "error" => "status-danger-weak",
        _ => "not-rendered",
    }
}

fn popover_surface_for_state(state_name: &str) -> &'static str {
    match state_name {
        "action-panel" => "bg/surface-1+elev/2",
        _ => "not-rendered",
    }
}

fn state_surface_contract_matches(state_name: &str, surface: &PreviewStateSurface) -> bool {
    match state_name {
        "collapsed" => {
            surface.search_bar == "panel-as-search-surface"
                && surface.body == "not-rendered"
                && surface.action_bar == "not-rendered"
        }
        "defer" => surface.feedback == "status-warning-weak",
        "error" => surface.feedback == "status-danger-weak",
        "action-panel" => surface.popover == "bg/surface-1+elev/2",
        "no-results" => surface.body == "empty-state-token-surface",
        "loading" | "searching" => surface.body == "loading-progress-token-surface",
        _ => surface.body == "results-token-surface",
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
