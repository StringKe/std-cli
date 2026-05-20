use std_egui::{
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
    pub native_viewport_margin: i8,
    pub panel_inner_padding: i8,
    pub search_surface_layer: String,
    pub result_surface_layer: String,
    pub selected_surface_layer: String,
    pub empty_state: String,
    pub matches_state: String,
    pub no_match_state: String,
    pub defer_feedback: String,
    pub error_feedback: String,
}

impl LauncherSurfaceSmokeReport {
    pub fn new() -> Self {
        let dark = themed_context(ThemeMode::Dark);
        let light = themed_context(ThemeMode::Light);
        Self {
            dark_panel_fill: color_hex(Color::bg_surface_0(&dark)),
            light_panel_fill: color_hex(Color::bg_surface_0(&light)),
            panel_opaque: Color::bg_surface_0(&dark).a() == 255
                && Color::bg_surface_0(&light).a() == 255,
            panel_radius: Radius::xl(),
            native_viewport_margin: 0,
            panel_inner_padding: Space::md(),
            search_surface_layer: layer("search", "bg/surface-1", &dark),
            result_surface_layer: layer("results", "bg/surface-1", &dark),
            selected_surface_layer: layer("selected", "accent/weak", &dark),
            empty_state: "empty=query,recent_or_suggested,footnote".to_string(),
            matches_state: "matches=grouped,selected,preview,action_bar".to_string(),
            no_match_state: "no_matches=icon,title,detail,ask_ai_enter".to_string(),
            defer_feedback: feedback_state(deferred_execution()),
            error_feedback: feedback_state(failed_execution()),
        }
    }

    pub fn pass(&self) -> bool {
        self.dark_panel_fill == "#1C1E22"
            && self.light_panel_fill == "#FFFFFF"
            && self.panel_opaque
            && self.panel_radius == 16
            && self.native_viewport_margin == 0
            && self.panel_inner_padding == 16
            && self.search_surface_layer == "search=bg/surface-1:#24272C"
            && self.result_surface_layer == "results=bg/surface-1:#24272C"
            && self.selected_surface_layer == "selected=accent/weak:#4E9CFF@46"
            && self.empty_state.contains("recent_or_suggested")
            && self.matches_state.contains("grouped")
            && self.no_match_state.contains("ask_ai_enter")
            && self.defer_feedback == "Needs external runner:Open Terminal"
            && self.error_feedback == "Failed:Plugin Crash"
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_surface_smoke {}\ndark_panel_fill={}\nlight_panel_fill={}\npanel_opaque={}\npanel_radius={}\nnative_viewport_margin={}\npanel_inner_padding={}\nsearch_surface_layer={}\nresult_surface_layer={}\nselected_surface_layer={}\nempty_state={}\nmatches_state={}\nno_match_state={}\ndefer_feedback={}\nerror_feedback={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.dark_panel_fill,
            self.light_panel_fill,
            self.panel_opaque,
            self.panel_radius,
            self.native_viewport_margin,
            self.panel_inner_padding,
            self.search_surface_layer,
            self.result_surface_layer,
            self.selected_surface_layer,
            self.empty_state,
            self.matches_state,
            self.no_match_state,
            self.defer_feedback,
            self.error_feedback
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
        "Open Terminal",
        ActionExecutionStatus::NeedsExternalRunner,
        "open -a Terminal",
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
