use std_egui::tokens::ThemeSmokeReport;

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
    dark_selected_surface_layer: String,
    light_selected_surface_layer: String,
    surface_contract: String,
    doc_reference: String,
}

impl StudioSurfaceSmoke {
    pub(crate) fn new() -> Self {
        let theme = ThemeSmokeReport::new();
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
            surface_contract: "canvas:L1,sidebar:L2,inspector:L2,bottom:L2,status:L2,selected:L4"
                .to_string(),
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
            && self.dark_selected_surface_layer.contains("#4E9CFF@46")
            && self.light_selected_surface_layer.contains("#0A6BFF@31")
            && self.surface_contract.contains("canvas:L1")
            && self.surface_contract.contains("selected:L4")
            && self.doc_reference == "docs/22#03-main-window-layout"
    }

    pub(crate) fn output(&self) -> String {
        let status = if self.pass() { "PASS" } else { "FAIL" };
        format!(
            "studio_surface_smoke {status}\ndark_canvas_surface_layer={}\nlight_canvas_surface_layer={}\ndark_sidebar_surface_layer={}\nlight_sidebar_surface_layer={}\ndark_inspector_surface_layer={}\nlight_inspector_surface_layer={}\ndark_bottom_panel_surface_layer={}\nlight_bottom_panel_surface_layer={}\ndark_status_surface_layer={}\nlight_status_surface_layer={}\ndark_selected_surface_layer={}\nlight_selected_surface_layer={}\nsurface_contract={}\ndoc_reference={}",
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
            self.dark_selected_surface_layer,
            self.light_selected_surface_layer,
            self.surface_contract,
            self.doc_reference
        )
    }
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
        assert!(report.output().contains("surface_contract=canvas:L1"));
    }
}
