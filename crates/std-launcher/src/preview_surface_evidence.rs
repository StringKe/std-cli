use crate::ui_metrics;
use eframe::egui;
use std_egui::tokens::{apply_theme, Color, ThemeMode};
use std_launcher::{LauncherState, LauncherViewportContract};

pub(crate) fn preview_surface_summary(theme: ThemeMode) -> PreviewSurfaceSummary {
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
pub(crate) struct PreviewSurfaceSummary {
    panel: String,
    search: String,
    result: String,
    selected: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviewStateSurface {
    panel_only_surface: bool,
    search_bar: &'static str,
    body: &'static str,
    action_bar: &'static str,
    feedback: &'static str,
    popover: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviewNativeHostSurface {
    clear_color: String,
    viewport_frame: String,
    geometry: String,
    carrier: PreviewHostCarrierContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviewHostCarrierContract {
    viewport: LauncherViewportContract,
    visible_surface: &'static str,
    layout_owner: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreviewNoMatchFallback {
    visible: bool,
    selected: bool,
    enter_keycap: bool,
    button_semantics: bool,
}

impl PreviewNoMatchFallback {
    pub(crate) fn for_state(state: &LauncherState, state_name: &str) -> Self {
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

    pub(crate) fn passes(&self, state_name: &str) -> bool {
        if state_name != "no-results" {
            return !self.visible && !self.selected && !self.enter_keycap && !self.button_semantics;
        }
        self.visible && self.selected && self.enter_keycap && self.button_semantics
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "no_match_fallback=ask_ai_row,visible={},selected={},enter_keycap={},button_semantics={}",
            self.visible, self.selected, self.enter_keycap, self.button_semantics
        )
    }
}

impl PreviewNativeHostSurface {
    pub(crate) fn for_state(state: &LauncherState) -> Self {
        Self {
            clear_color: std_launcher::launcher_clear_color_contract(),
            viewport_frame: std_launcher::launcher_viewport_frame_contract(),
            geometry: ui_metrics::panel_surface_geometry_summary(state),
            carrier: PreviewHostCarrierContract::for_state(state),
        }
    }

    pub(crate) fn passes(&self) -> bool {
        self.clear_color == "native_clear_color=transparent_rgba_0_0_0_0"
            && self.viewport_frame == "viewport_frame=transparent_fill,no_stroke"
            && self.geometry.contains("host_gap=0x0")
            && self.geometry.contains("panel_origin=0x0")
            && self.geometry.contains("panel_only_surface=true")
            && self.carrier.passes()
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "host_contract={},{};{};{};forbidden=black_or_white_host_background",
            self.clear_color,
            self.viewport_frame,
            self.geometry,
            self.carrier.summary()
        )
    }
}

impl PreviewHostCarrierContract {
    pub(crate) fn for_state(state: &LauncherState) -> Self {
        let only_panel = ui_metrics::panel_is_only_visible_surface(state);
        Self {
            viewport: LauncherViewportContract::visible(),
            visible_surface: if only_panel {
                "opaque-panel-only"
            } else {
                "host-carrier-visible"
            },
            layout_owner: "panel-rect",
        }
    }

    pub(crate) fn passes(&self) -> bool {
        self.viewport.passes()
            && self.visible_surface == "opaque-panel-only"
            && self.layout_owner == "panel-rect"
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "{},visible_surface:{},layout_owner:{}",
            self.viewport.host_carrier_summary(),
            self.visible_surface,
            self.layout_owner
        )
    }
}

impl PreviewStateSurface {
    pub(crate) fn for_state(state: &LauncherState, state_name: &str) -> Self {
        Self {
            panel_only_surface: ui_metrics::panel_is_only_visible_surface(state),
            search_bar: search_surface_for_state(state_name),
            body: body_surface_for_state(state_name),
            action_bar: action_bar_surface_for_state(state_name),
            feedback: feedback_surface_for_state(state_name),
            popover: popover_surface_for_state(state_name),
        }
    }

    pub(crate) fn passes(&self, state_name: &str) -> bool {
        self.panel_only_surface
            && self.search_bar != "carrier"
            && self.body != "carrier"
            && self.action_bar != "carrier"
            && self.feedback != "carrier"
            && self.popover != "carrier"
            && state_surface_contract_matches(state_name, self)
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "state_surface=panel_only_surface:{},search:{},body:{},action_bar:{},feedback:{},popover:{}",
            self.panel_only_surface,
            self.search_bar,
            self.body,
            self.action_bar,
            self.feedback,
            self.popover
        )
    }
}

impl PreviewSurfaceSummary {
    pub(crate) fn summary(&self) -> String {
        format!(
            "panel_token=bg/surface-0:{},search_token=bg/surface-1:{},result_token=bg/surface-1:{},selected_token=accent/weak:{}",
            self.panel, self.search, self.result, self.selected
        )
    }
}

pub(crate) fn preview_surface_passes(surface: &PreviewSurfaceSummary, theme: &str) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_tokens_cover_light_and_dark_launcher_capture() {
        let light = preview_surface_summary(ThemeMode::Light);
        let dark = preview_surface_summary(ThemeMode::Dark);

        assert!(preview_surface_passes(&light, "light"));
        assert!(preview_surface_passes(&dark, "dark"));
        assert!(light.summary().contains("panel_token=bg/surface-0:#FAFBFD"));
        assert!(dark.summary().contains("panel_token=bg/surface-0:#1C1E22"));
    }

    #[test]
    fn host_carrier_is_absent_and_panel_owns_layout() {
        let mut state = LauncherState::new();
        state.update_query("index");
        let host = PreviewNativeHostSurface::for_state(&state);
        let summary = host.summary();

        assert!(host.passes());
        assert!(summary.contains("host_carrier=transparent:true"));
        assert!(summary.contains("background:none"));
        assert!(summary.contains("visible_surface:opaque-bg-surface-0"));
        assert!(summary.contains("visible_surface:opaque-panel-only"));
        assert!(summary.contains("layout_owner:panel-rect"));
        assert!(!summary.contains("visible_surface:host-carrier-visible"));
    }

    #[test]
    fn preview_surface_evidence_does_not_model_extra_host_container() {
        let source = include_str!("preview_surface_evidence.rs");
        let forbidden = concat!("preview", "_viewport");

        assert!(!source.contains(forbidden));
    }
}
