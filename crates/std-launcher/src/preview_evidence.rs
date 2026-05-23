use crate::{
    preview::apply_preview_scenario,
    preview_affordance::LauncherAffordanceSummary,
    preview_behavior::PreviewStateBehavior,
    preview_surface_evidence::{
        preview_surface_passes, preview_surface_summary, PreviewNativeHostSurface,
        PreviewNoMatchFallback, PreviewStateSurface,
    },
    ui, ui_metrics, ui_results,
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
    let structure = LauncherStructureContract::for_state(&state, scenario.state);
    let passes = valid
        && preview_surface_passes(&surface, scenario.theme)
        && affordance.passes(scenario.state)
        && state_surface.passes(scenario.state)
        && no_match_fallback.passes(scenario.state)
        && host.passes()
        && behavior.passes(scenario.state)
        && structure.passes(scenario.state)
        && feedback_status_icon_passes(scenario.state);
    format!(
        "{}={}:phase={:?},results={},feedback={},{},{},{},{},{},{},{},{}",
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
        structure.summary(),
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct LauncherStructureContract {
    overlay: &'static str,
    search: &'static str,
    results: String,
    preview: &'static str,
    action_bar: &'static str,
    feedback: &'static str,
    action_panel: &'static str,
    ownership: &'static str,
}

impl LauncherStructureContract {
    fn for_state(state: &LauncherState, state_name: &str) -> Self {
        let expanded = ui_metrics::panel_is_expanded(state);
        let groups = ui_results::group_count(&state.view.results);
        Self {
            overlay: "single-overlay",
            search: "visible",
            results: results_structure(state, state_name, groups),
            preview: preview_structure(state),
            action_bar: if expanded { "visible" } else { "hidden" },
            feedback: if state.view.feedback.is_some() {
                "visible"
            } else {
                "hidden"
            },
            action_panel: if state.action_panel.open {
                "foreground-popover"
            } else {
                "hidden"
            },
            ownership: "egui-panel-no-native-child",
        }
    }

    fn passes(&self, state_name: &str) -> bool {
        self.overlay == "single-overlay"
            && self.search == "visible"
            && self.ownership == "egui-panel-no-native-child"
            && self.results_pass(state_name)
            && self.preview_pass(state_name)
            && self.action_bar_pass(state_name)
            && self.feedback_pass(state_name)
            && self.action_panel_pass(state_name)
    }

    fn summary(&self) -> String {
        format!(
            "structure=overlay:{},search:{},results:{},preview:{},action_bar:{},feedback:{},action_panel:{},ownership:{}",
            self.overlay,
            self.search,
            self.results,
            self.preview,
            self.action_bar,
            self.feedback,
            self.action_panel,
            self.ownership
        )
    }

    fn results_pass(&self, state_name: &str) -> bool {
        match state_name {
            "collapsed" => self.results == "hidden",
            "empty" => self.results == "suggestions:3",
            "no-results" => self.results == "empty-state",
            "results" | "ime" | "action-panel" => self.results.starts_with("grouped:"),
            "loading" | "searching" => self.results == "progress",
            "executing" => self.results.starts_with("grouped:"),
            "defer" | "error" => self.results.starts_with("grouped:"),
            _ => true,
        }
    }

    fn preview_pass(&self, state_name: &str) -> bool {
        match state_name {
            "collapsed" | "loading" | "searching" => self.preview == "hidden",
            _ => self.preview == "action-bar-summary:selected-row",
        }
    }

    fn action_bar_pass(&self, state_name: &str) -> bool {
        if state_name == "collapsed" {
            self.action_bar == "hidden"
        } else {
            self.action_bar == "visible"
        }
    }

    fn feedback_pass(&self, state_name: &str) -> bool {
        match state_name {
            "defer" | "error" => self.feedback == "visible",
            _ => self.feedback == "hidden",
        }
    }

    fn action_panel_pass(&self, state_name: &str) -> bool {
        if state_name == "action-panel" {
            self.action_panel == "foreground-popover"
        } else {
            self.action_panel == "hidden"
        }
    }
}

fn results_structure(state: &LauncherState, state_name: &str, groups: usize) -> String {
    match state_name {
        "collapsed" => "hidden".to_string(),
        "empty" => "suggestions:3".to_string(),
        "no-results" => "empty-state".to_string(),
        "loading" | "searching" => "progress".to_string(),
        _ if state.view.results.is_empty() => "empty-state".to_string(),
        _ => format!("grouped:{groups}"),
    }
}

fn preview_structure(state: &LauncherState) -> &'static str {
    if state.view.preview.is_some() {
        "action-bar-summary:selected-row"
    } else {
        "hidden"
    }
}

#[cfg(test)]
mod structure_tests {
    use super::*;

    #[test]
    fn launcher_structure_contract_tracks_docs_21_overlay_parts() {
        for scenario in [
            "collapsed",
            "empty",
            "results",
            "no-results",
            "loading",
            "executing",
            "defer",
            "error",
            "ime",
            "action-panel",
        ] {
            let mut state = LauncherState::new();
            apply_preview_scenario(&mut state, scenario);
            let contract = LauncherStructureContract::for_state(&state, scenario);

            assert!(
                contract.passes(scenario),
                "{scenario}: {}",
                contract.summary()
            );
            assert!(contract
                .summary()
                .contains("structure=overlay:single-overlay"));
            assert!(contract
                .summary()
                .contains("ownership:egui-panel-no-native-child"));
        }
    }
}
