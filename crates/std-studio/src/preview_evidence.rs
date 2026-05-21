use crate::{
    preview::seeded_preview_app,
    viewport::{STUDIO_MIN_WINDOW_SIZE, STUDIO_WINDOW_SIZE},
    StudioEguiApp, StudioPane,
};
use eframe::egui;
use std_egui::tokens::{apply_theme, Color, ThemeMode};

pub(crate) fn preview_matrix() -> Vec<String> {
    [
        "dark-dashboard",
        "dark-workflow",
        "dark-analysis",
        "dark-plugins",
        "dark-operations",
        "dark-settings",
        "dark-panes",
        "light-dashboard",
        "light-workflow",
        "light-analysis",
        "light-plugins",
        "light-operations",
        "light-settings",
        "light-panes",
    ]
    .into_iter()
    .map(ToString::to_string)
    .collect()
}

pub(crate) fn preview_state_summary(scenario: &str) -> String {
    let Some((theme, name)) = scenario.split_once('-') else {
        return format!("{scenario}=FAIL");
    };
    let app = seeded_preview_app(theme, name);
    let surface = preview_surface_summary(theme);
    let valid = matches!(theme, "dark" | "light")
        && preview_state_passes(&app, name)
        && preview_surface_passes(&surface, theme);
    format!(
        "{scenario}={}:pane={:?},workspace={},status={},{}",
        if valid { "PASS" } else { "FAIL" },
        app.app.active_pane,
        app.app.open_workspace_panes().count(),
        app.status,
        surface.summary()
    )
}

pub(crate) fn preview_size_summary(scenario: &str) -> String {
    let Some((theme, name)) = scenario.split_once('-') else {
        return format!("{scenario}=FAIL");
    };
    let app = seeded_preview_app(theme, name);
    let host = format_window_size(STUDIO_WINDOW_SIZE);
    let min = format_window_size(STUDIO_MIN_WINDOW_SIZE);
    let valid = matches!(theme, "dark" | "light")
        && !app.app.workspace_policy.allows_native_child_windows()
        && !app.app.workspace_policy.allows_detached_panels()
        && host == "1280x800"
        && min == "1080x640";
    format!(
        "{scenario}={}:host={},min={},workspace={},native_child_windows={},detached_panels={},settings_surface={}",
        if valid { "PASS" } else { "FAIL" },
        host,
        min,
        app.app.open_workspace_panes().count(),
        app.app.workspace_policy.allows_native_child_windows(),
        app.app.workspace_policy.allows_detached_panels(),
        settings_surface_policy(&app, name)
    )
}

fn settings_surface_policy(app: &StudioEguiApp, scenario: &str) -> &'static str {
    if scenario == "settings"
        && app.app.active_pane == StudioPane::Settings
        && !app.app.workspace_policy.allows_native_child_windows()
        && !app.app.workspace_policy.allows_detached_panels()
    {
        "internal-workspace-pane"
    } else {
        "not-settings"
    }
}

fn preview_state_passes(app: &StudioEguiApp, scenario: &str) -> bool {
    match scenario {
        "dashboard" => app.app.active_pane == StudioPane::Dashboard,
        "workflow" => {
            app.app.active_pane == StudioPane::Workflows
                && app.app.workflow_debug.is_some()
                && app.app.last_workflow_execution.is_some()
        }
        "analysis" => {
            app.app.active_pane == StudioPane::Analysis && !app.analysis.coverage_output.is_empty()
        }
        "plugins" => {
            app.app.active_pane == StudioPane::Plugins
                && app.app.open_workspace_panes().count() >= 1
        }
        "operations" => app.app.active_pane == StudioPane::Operations,
        "settings" => app.app.active_pane == StudioPane::Settings,
        "panes" => {
            app.app.open_workspace_panes().count() >= 3
                && !app.app.workspace_policy.allows_native_child_windows()
                && !app.app.workspace_policy.allows_detached_panels()
        }
        _ => false,
    }
}

fn preview_surface_summary(theme: &str) -> PreviewSurfaceSummary {
    let ctx = egui::Context::default();
    apply_theme(&ctx, ThemeMode::resolve(theme));
    PreviewSurfaceSummary {
        canvas: color_hex(Color::bg_surface_0(&ctx)),
        panel: color_hex(Color::bg_surface_1(&ctx)),
        inspector: color_hex(Color::bg_surface_1(&ctx)),
        selected: color_hex_alpha(Color::accent_weak(&ctx)),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreviewSurfaceSummary {
    canvas: String,
    panel: String,
    inspector: String,
    selected: String,
}

impl PreviewSurfaceSummary {
    fn summary(&self) -> String {
        format!(
            "canvas_token=bg/surface-0:{},panel_token=bg/surface-1:{},inspector_token=bg/surface-1:{},selected_token=accent/weak:{}",
            self.canvas, self.panel, self.inspector, self.selected
        )
    }
}

fn preview_surface_passes(surface: &PreviewSurfaceSummary, theme: &str) -> bool {
    match theme {
        "dark" => {
            surface.canvas == "#1C1E22"
                && surface.panel == "#24272C"
                && surface.inspector == "#24272C"
                && surface.selected == "#4E9CFF@46"
        }
        "light" => {
            surface.canvas == "#FAFBFD"
                && surface.panel == "#F2F5F8"
                && surface.inspector == "#F2F5F8"
                && surface.selected == "#0A6BFF@31"
        }
        _ => false,
    }
}

fn format_window_size(size: [f32; 2]) -> String {
    format!("{}x{}", size[0] as u32, size[1] as u32)
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
