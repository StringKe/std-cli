use std_egui::{motion::MotionContext, tokens::ThemeSmokeReport};

pub(crate) struct StudioSurfaceSmoke {
    dark_canvas_surface_layer: String,
    light_canvas_surface_layer: String,
    dark_sidebar_surface_layer: String,
    light_sidebar_surface_layer: String,
    dark_inspector_surface_layer: String,
    light_inspector_surface_layer: String,
    dark_bottom_panel_surface_layer: String,
    light_bottom_panel_surface_layer: String,
    dark_status_surface_layer: String,
    light_status_surface_layer: String,
    dark_host_chrome_surface_layer: String,
    light_host_chrome_surface_layer: String,
    dark_selected_surface_layer: String,
    light_selected_surface_layer: String,
    canvas_outer_frame: String,
    native_clear_color_contract: &'static str,
    host_viewport_contract: String,
    host_chrome_contract: &'static str,
    standard_modal_enter_ms: u128,
    reduced_modal_enter_ms: u128,
    reduced_focus_ring_ms: u128,
    reduce_motion_contract: String,
    surface_contract: String,
    settings_theme_contract: String,
    doc_reference: String,
}

impl StudioSurfaceSmoke {
    pub(crate) fn new() -> Self {
        let theme = ThemeSmokeReport::new();
        let standard_motion = MotionContext::standard();
        let reduced_motion = MotionContext::reduced();
        Self {
            dark_canvas_surface_layer: surface("dark_canvas", "bg/surface-0", theme.dark_surface_0),
            light_canvas_surface_layer: surface(
                "light_canvas",
                "bg/surface-0",
                theme.light_surface_0,
            ),
            dark_sidebar_surface_layer: surface(
                "dark_sidebar",
                "bg/surface-1",
                theme.dark_surface_1,
            ),
            light_sidebar_surface_layer: surface(
                "light_sidebar",
                "bg/surface-1",
                theme.light_surface_1,
            ),
            dark_inspector_surface_layer: surface(
                "dark_inspector",
                "bg/surface-1",
                theme.dark_surface_1,
            ),
            light_inspector_surface_layer: surface(
                "light_inspector",
                "bg/surface-1",
                theme.light_surface_1,
            ),
            dark_bottom_panel_surface_layer: surface(
                "dark_bottom_panel",
                "bg/surface-1",
                theme.dark_surface_1,
            ),
            light_bottom_panel_surface_layer: surface(
                "light_bottom_panel",
                "bg/surface-1",
                theme.light_surface_1,
            ),
            dark_status_surface_layer: surface("dark_status", "bg/surface-1", theme.dark_surface_1),
            light_status_surface_layer: surface(
                "light_status",
                "bg/surface-1",
                theme.light_surface_1,
            ),
            dark_host_chrome_surface_layer: surface(
                "dark_host_chrome",
                crate::host_chrome::host_chrome_surface_token(),
                theme.dark_surface_1,
            ),
            light_host_chrome_surface_layer: surface(
                "light_host_chrome",
                crate::host_chrome::host_chrome_surface_token(),
                theme.light_surface_1,
            ),
            dark_selected_surface_layer: rgba_surface(
                "dark_selected",
                "accent/weak",
                theme.dark_selection,
            ),
            light_selected_surface_layer: rgba_surface(
                "light_selected",
                "accent/weak",
                theme.light_selection,
            ),
            canvas_outer_frame: canvas_outer_frame_contract(),
            native_clear_color_contract: crate::studio_clear_color_contract(),
            host_viewport_contract: crate::viewport::studio_host_viewport_contract(),
            host_chrome_contract: crate::host_chrome::host_chrome_surface_contract(),
            standard_modal_enter_ms: standard_motion.modal_enter().as_millis(),
            reduced_modal_enter_ms: reduced_motion.modal_enter().as_millis(),
            reduced_focus_ring_ms: reduced_motion.focus_ring().as_millis(),
            reduce_motion_contract: "STD_REDUCE_MOTION=1 collapses modal enter and focus ring"
                .to_string(),
            surface_contract: "canvas:L1,sidebar:L2,inspector:L2,bottom:L2,status:L2,selected:L4"
                .to_string(),
            settings_theme_contract: settings_theme_contract(),
            doc_reference: "docs/22#03-main-window-layout".to_string(),
        }
    }

    pub(crate) fn pass(&self) -> bool {
        self.dark_canvas_surface_layer.contains("#1C1E22")
            && self.light_canvas_surface_layer.contains("#FAFBFD")
            && self.dark_sidebar_surface_layer.contains("#24272C")
            && self.light_sidebar_surface_layer.contains("#F2F5F8")
            && self.dark_inspector_surface_layer
                == self
                    .dark_sidebar_surface_layer
                    .replace("dark_sidebar", "dark_inspector")
            && self.light_inspector_surface_layer
                == self
                    .light_sidebar_surface_layer
                    .replace("light_sidebar", "light_inspector")
            && self
                .dark_bottom_panel_surface_layer
                .contains("bg/surface-1")
            && self
                .light_bottom_panel_surface_layer
                .contains("bg/surface-1")
            && self.dark_status_surface_layer.contains("bg/surface-1")
            && self.light_status_surface_layer.contains("bg/surface-1")
            && self
                .dark_host_chrome_surface_layer
                .contains("bg/surface-1:#24272C")
            && self
                .light_host_chrome_surface_layer
                .contains("bg/surface-1:#F2F5F8")
            && self.dark_selected_surface_layer.contains("#4E9CFF@46")
            && self.light_selected_surface_layer.contains("#0A6BFF@31")
            && self.canvas_outer_frame == "unframed,no_nested_card"
            && self.native_clear_color_contract
                == "native_clear_color=bg/surface-0,not-transparent,not-system-black-white"
            && self.host_viewport_contract
                == "host_viewport=single-borderless-egui-viewport,panes=internal-egui-workspace-panes,size=1280x800,min=1080x640,decorations=false,resizable=true,native_child_windows=false,detached_panels=false,extra_viewports=false"
            && self.host_chrome_contract
                == "host_chrome=egui-owned,borderless,native-controls=false,surface=bg/surface-1"
            && self.standard_modal_enter_ms == 220
            && self.reduced_modal_enter_ms == 0
            && self.reduced_focus_ring_ms == 0
            && self.reduce_motion_contract.contains("STD_REDUCE_MOTION=1")
            && self.surface_contract.contains("canvas:L1")
            && self.surface_contract.contains("selected:L4")
            && self
                .settings_theme_contract
                .contains("theme_modes=system|dark|light")
            && self
                .settings_theme_contract
                .contains("profile=theme-profile=requested|effective")
            && self
                .settings_theme_contract
                .contains("controls=theme:segmented-control")
            && self.doc_reference == "docs/22#03-main-window-layout"
    }

    pub(crate) fn output(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_surface_smoke {status}\ndark_canvas_surface_layer={}\nlight_canvas_surface_layer={}\ndark_sidebar_surface_layer={}\nlight_sidebar_surface_layer={}\ndark_inspector_surface_layer={}\nlight_inspector_surface_layer={}\ndark_bottom_panel_surface_layer={}\nlight_bottom_panel_surface_layer={}\ndark_status_surface_layer={}\nlight_status_surface_layer={}\ndark_host_chrome_surface_layer={}\nlight_host_chrome_surface_layer={}\ndark_selected_surface_layer={}\nlight_selected_surface_layer={}\ncanvas_outer_frame={}\nnative_clear_color_contract={}\nhost_viewport_contract={}\nhost_chrome_contract={}\nstandard_modal_enter_ms={}\nreduced_modal_enter_ms={}\nreduced_focus_ring_ms={}\nreduce_motion_contract={}\nsurface_contract={}\nsettings_theme_contract={}\ndoc_reference={}",
            self.dark_canvas_surface_layer,
            self.light_canvas_surface_layer,
            self.dark_sidebar_surface_layer,
            self.light_sidebar_surface_layer,
            self.dark_inspector_surface_layer,
            self.light_inspector_surface_layer,
            self.dark_bottom_panel_surface_layer,
            self.light_bottom_panel_surface_layer,
            self.dark_status_surface_layer,
            self.light_status_surface_layer,
            self.dark_host_chrome_surface_layer,
            self.light_host_chrome_surface_layer,
            self.dark_selected_surface_layer,
            self.light_selected_surface_layer,
            self.canvas_outer_frame,
            self.native_clear_color_contract,
            self.host_viewport_contract,
            self.host_chrome_contract,
            self.standard_modal_enter_ms,
            self.reduced_modal_enter_ms,
            self.reduced_focus_ring_ms,
            self.reduce_motion_contract,
            self.surface_contract,
            self.settings_theme_contract,
            self.doc_reference
        )
    }
}

fn settings_theme_contract() -> String {
    let contract = crate::views::settings_model::settings_contract();
    format!(
        "theme_modes={};controls=theme:{},motion:{},contrast:{},transparency:{},zoom:{};profile={}",
        contract.theme_modes.join("|"),
        contract.theme_control,
        contract.motion_control,
        contract.contrast_control,
        contract.transparency_control,
        contract.zoom_control,
        contract.appearance_profile
    )
}

fn surface(name: &str, token: &str, color: egui::Color32) -> String {
    format!("{name}={token}:{}", color_hex(color))
}

fn rgba_surface(name: &str, token: &str, color: egui::Color32) -> String {
    format!("{name}={token}:{}@{}", color_hex(color), color.a())
}

fn color_hex(color: egui::Color32) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r(), color.g(), color.b())
}

fn canvas_outer_frame_contract() -> String {
    let source = include_str!("../shell.rs");
    let body = source
        .split("fn render_active_workspace")
        .nth(1)
        .and_then(|section| section.split("fn render_context").next())
        .unwrap_or("");
    if body.contains("egui::ScrollArea::vertical()") && !body.contains("ui::surface_frame") {
        return "unframed,no_nested_card".to_string();
    }
    "canvas_outer_frame=FAIL".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_surface_smoke_reports_light_and_dark_workspace_layers() {
        let report = StudioSurfaceSmoke::new();

        assert!(report.pass(), "{}", report.output());
        assert!(report.output().contains("studio_surface_smoke PASS"));
        assert!(report.output().contains("dark_canvas_surface_layer"));
        assert!(report.output().contains("light_canvas_surface_layer"));
        assert!(report
            .output()
            .contains("host_chrome_contract=host_chrome=egui-owned,borderless"));
        assert!(report.output().contains("native-controls=false"));
        assert!(report.output().contains("canvas_outer_frame=unframed"));
        assert!(report.output().contains("standard_modal_enter_ms=220"));
        assert!(report.output().contains("reduced_modal_enter_ms=0"));
        assert!(report.output().contains("reduced_focus_ring_ms=0"));
        assert!(report
            .output()
            .contains("reduce_motion_contract=STD_REDUCE_MOTION=1"));
        assert!(report.output().contains("surface_contract=canvas:L1"));
        assert!(report
            .output()
            .contains("settings_theme_contract=theme_modes=system|dark|light"));
        assert!(report.output().contains("controls=theme:segmented-control"));
        assert!(report
            .output()
            .contains("profile=theme-profile=requested|effective"));
    }
}
