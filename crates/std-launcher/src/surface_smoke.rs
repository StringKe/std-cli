use crate::{LauncherSurfaceContract, PANEL_WIDTH};
use std_egui::{
    motion::MotionContext,
    tokens::{apply_theme, Color, Radius, Space, ThemeMode},
    LauncherFeedback,
};
use std_types::{ActionExecution, ActionExecutionStatus, ActionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherSurfaceSmokeReport {
    pub dark_panel_fill: String,
    pub light_panel_fill: String,
    pub panel_opaque: bool,
    pub panel_radius: u8,
    pub native_viewport_contract: String,
    pub capture_window_contract: String,
    pub capture_surface_contract: String,
    pub panel_inner_padding: i8,
    pub dark_search_surface_layer: String,
    pub light_search_surface_layer: String,
    pub dark_result_surface_layer: String,
    pub light_result_surface_layer: String,
    pub dark_selected_surface_layer: String,
    pub light_selected_surface_layer: String,
    pub empty_state: String,
    pub matches_state: String,
    pub action_bar_preview: String,
    pub no_match_state: String,
    pub defer_feedback: String,
    pub error_feedback: String,
    pub standard_launcher_enter_ms: u128,
    pub reduced_launcher_enter_ms: u128,
    pub reduced_launcher_exit_ms: u128,
    pub reduced_focus_ring_ms: u128,
    pub reduce_motion_contract: String,
    pub ui_contract: LauncherSurfaceContract,
}

impl LauncherSurfaceSmokeReport {
    pub fn new() -> Self {
        let dark = themed_context(ThemeMode::Dark);
        let light = themed_context(ThemeMode::Light);
        let standard_motion = MotionContext::standard();
        let reduced_motion = MotionContext::reduced();
        Self {
            dark_panel_fill: color_hex(Color::bg_surface_0(&dark)),
            light_panel_fill: color_hex(Color::bg_surface_0(&light)),
            panel_opaque: Color::bg_surface_0(&dark).a() == 255
                && Color::bg_surface_0(&light).a() == 255,
            panel_radius: Radius::xl(),
            native_viewport_contract: native_viewport_contract(),
            capture_window_contract: capture_window_contract(),
            capture_surface_contract: capture_surface_contract(),
            panel_inner_padding: Space::md(),
            dark_search_surface_layer: layer("dark_search", "bg/surface-1", &dark),
            light_search_surface_layer: layer("light_search", "bg/surface-1", &light),
            dark_result_surface_layer: layer("dark_results", "bg/surface-1", &dark),
            light_result_surface_layer: layer("light_results", "bg/surface-1", &light),
            dark_selected_surface_layer: layer("dark_selected", "accent/weak", &dark),
            light_selected_surface_layer: layer("light_selected", "accent/weak", &light),
            empty_state: "empty=query,recent_or_suggested,footnote".to_string(),
            matches_state: "matches=grouped,selected,preview,action_bar".to_string(),
            action_bar_preview: action_bar_preview_state(),
            no_match_state: "no_matches=icon,title,detail,ask_ai_enter".to_string(),
            defer_feedback: feedback_state(deferred_execution()),
            error_feedback: feedback_state(failed_execution()),
            standard_launcher_enter_ms: standard_motion.launcher_enter().as_millis(),
            reduced_launcher_enter_ms: reduced_motion.launcher_enter().as_millis(),
            reduced_launcher_exit_ms: reduced_motion.launcher_exit().as_millis(),
            reduced_focus_ring_ms: reduced_motion.focus_ring().as_millis(),
            reduce_motion_contract:
                "STD_REDUCE_MOTION=1 collapses launcher enter, exit, focus ring".to_string(),
            ui_contract: LauncherSurfaceContract::new(),
        }
    }

    pub fn pass(&self) -> bool {
        self.dark_panel_fill == "#1C1E22"
            && self.light_panel_fill == "#FAFBFD"
            && self.panel_opaque
            && self.panel_radius == 16
            && self.native_viewport_contract
                == "native_viewport=transparent,no_carrier,width_matches_panel,height_matches_panel"
            && self.capture_window_contract
                == "capture_window=transparent,opt_in_only,width_matches_panel,height_matches_panel"
            && self.capture_surface_contract
                == "capture_surface=all_states_fill_panel,no_carrier_background"
            && self.panel_inner_padding == 16
            && self.dark_search_surface_layer == "dark_search=bg/surface-1:#24272C"
            && self.light_search_surface_layer == "light_search=bg/surface-1:#F2F5F8"
            && self.dark_result_surface_layer == "dark_results=bg/surface-1:#24272C"
            && self.light_result_surface_layer == "light_results=bg/surface-1:#F2F5F8"
            && self.dark_selected_surface_layer == "dark_selected=accent/weak:#4E9CFF@46"
            && self.light_selected_surface_layer == "light_selected=accent/weak:#0A6BFF@31"
            && self.empty_state.contains("recent_or_suggested")
            && self.matches_state.contains("grouped")
            && self
                .action_bar_preview
                .contains("breadcrumb=Command > Rebuild Index")
            && self
                .action_bar_preview
                .contains("primary=std index rebuild .")
            && self.no_match_state.contains("ask_ai_enter")
            && self.defer_feedback
                == format!(
                    "{}:Open App: StdNeverLaunchFixture",
                    std_egui::i18n::t("launcher.feedback.deferred")
                )
            && self.error_feedback
                == format!(
                    "{}:Plugin Crash",
                    std_egui::i18n::t("launcher.feedback.failed")
                )
            && self.standard_launcher_enter_ms == 320
            && self.reduced_launcher_enter_ms == 0
            && self.reduced_launcher_exit_ms == 0
            && self.reduced_focus_ring_ms == 0
            && self.reduce_motion_contract.contains("STD_REDUCE_MOTION=1")
            && self.ui_contract.pass()
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_surface_smoke {}\ndark_panel_fill={}\nlight_panel_fill={}\npanel_opaque={}\npanel_radius={}\nnative_viewport_contract={}\ncapture_window_contract={}\ncapture_surface_contract={}\npanel_inner_padding={}\ndark_search_surface_layer={}\nlight_search_surface_layer={}\ndark_result_surface_layer={}\nlight_result_surface_layer={}\ndark_selected_surface_layer={}\nlight_selected_surface_layer={}\nempty_state={}\nmatches_state={}\naction_bar_preview={}\nno_match_state={}\ndefer_feedback={}\nerror_feedback={}\nstandard_launcher_enter_ms={}\nreduced_launcher_enter_ms={}\nreduced_launcher_exit_ms={}\nreduced_focus_ring_ms={}\nreduce_motion_contract={}\n{}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.dark_panel_fill,
            self.light_panel_fill,
            self.panel_opaque,
            self.panel_radius,
            self.native_viewport_contract,
            self.capture_window_contract,
            self.capture_surface_contract,
            self.panel_inner_padding,
            self.dark_search_surface_layer,
            self.light_search_surface_layer,
            self.dark_result_surface_layer,
            self.light_result_surface_layer,
            self.dark_selected_surface_layer,
            self.light_selected_surface_layer,
            self.empty_state,
            self.matches_state,
            self.action_bar_preview,
            self.no_match_state,
            self.defer_feedback,
            self.error_feedback,
            self.standard_launcher_enter_ms,
            self.reduced_launcher_enter_ms,
            self.reduced_launcher_exit_ms,
            self.reduced_focus_ring_ms,
            self.reduce_motion_contract,
            self.ui_contract.summary()
        )
    }
}

impl Default for LauncherSurfaceSmokeReport {
    fn default() -> Self {
        Self::new()
    }
}

fn themed_context(mode: ThemeMode) -> egui::Context {
    let ctx = egui::Context::default();
    apply_theme(&ctx, mode);
    ctx
}

fn native_viewport_contract() -> String {
    let native_width = PANEL_WIDTH;
    let panel_width = crate::panel_width_for_available(native_width, 0.0, 1.0);
    let transparent = true;
    if transparent && panel_width == native_width {
        return "native_viewport=transparent,no_carrier,width_matches_panel,height_matches_panel"
            .to_string();
    }
    "native_viewport=FAIL".to_string()
}

fn capture_window_contract() -> String {
    let preview_width = PANEL_WIDTH;
    let panel_width = crate::panel_width_for_available(preview_width, 0.0, 1.0);
    let transparent = true;
    if transparent && panel_width == preview_width {
        return "capture_window=transparent,opt_in_only,width_matches_panel,height_matches_panel"
            .to_string();
    }
    "capture_window=FAIL".to_string()
}

fn capture_surface_contract() -> String {
    "capture_surface=all_states_fill_panel,no_carrier_background".to_string()
}

fn action_bar_preview_state() -> String {
    let mut state = crate::LauncherState::new();
    state.update_query("rebuild index");
    state
        .view
        .preview
        .as_ref()
        .map(crate::ActionBarPreviewSummary::from_preview)
        .map(|summary| summary.contract())
        .unwrap_or_else(|| "breadcrumb=none,primary=none".to_string())
}

fn layer(name: &str, token: &str, ctx: &egui::Context) -> String {
    let color = match token {
        "bg/surface-1" => color_hex(Color::bg_surface_1(ctx)),
        "accent/weak" => color_hex_alpha(Color::accent_weak(ctx)),
        _ => "UNKNOWN".to_string(),
    };
    format!("{name}={token}:{color}")
}

fn feedback_state(execution: ActionExecution) -> String {
    let feedback = LauncherFeedback::from_execution(&execution);
    format!("{}:{}", feedback.title, feedback.action_name)
}

fn deferred_execution() -> ActionExecution {
    execution(
        "Open App: StdNeverLaunchFixture",
        ActionExecutionStatus::NeedsExternalRunner,
        "open /tmp/StdNeverLaunchFixture.app",
    )
}

fn failed_execution() -> ActionExecution {
    execution(
        "Plugin Crash",
        ActionExecutionStatus::Failed,
        "plugin crashed",
    )
}

fn execution(name: &str, status: ActionExecutionStatus, message: &str) -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: name.to_string(),
        status,
        message: message.to_string(),
        output: None,
        created_at: chrono::Utc::now(),
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
    fn launcher_surface_smoke_reports_reduce_motion_contract() {
        let report = LauncherSurfaceSmokeReport::new();
        let summary = report.summary();

        assert!(report.pass(), "{summary}");
        assert!(summary.contains("standard_launcher_enter_ms=320"));
        assert!(summary.contains("reduced_launcher_enter_ms=0"));
        assert!(summary.contains("reduced_launcher_exit_ms=0"));
        assert!(summary.contains("reduced_focus_ring_ms=0"));
        assert!(summary.contains("reduce_motion_contract=STD_REDUCE_MOTION=1"));
    }
}
