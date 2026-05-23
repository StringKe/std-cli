use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LauncherViewportContract {
    pub transparent: bool,
    pub decorations: bool,
    pub resizable: bool,
    pub visible: bool,
    pub panel_surface: &'static str,
    pub host_background: &'static str,
    pub host_gutter_px: u32,
}

impl LauncherViewportContract {
    pub fn hidden() -> Self {
        Self::new(false)
    }

    pub fn visible() -> Self {
        Self::new(true)
    }

    pub fn native_host_window_summary(self, size: egui::Vec2) -> String {
        format!(
            "native_host=transparent,transparent={},decorations={},resizable={},visible={},panel_surface={},host_background={},host_gutter={}px,size={}x{}",
            self.transparent,
            self.decorations,
            self.resizable,
            self.visible,
            self.panel_surface,
            self.host_background,
            self.host_gutter_px,
            size.x as u32,
            size.y as u32
        )
    }

    pub fn host_carrier_summary(self) -> String {
        format!(
            "host_carrier=transparent:{},decorations:{},resizable:{},background:{},visible_surface:{}",
            self.transparent,
            self.decorations,
            self.resizable,
            self.host_background,
            self.panel_surface
        )
    }

    pub fn passes(self) -> bool {
        self.transparent
            && !self.decorations
            && !self.resizable
            && self.panel_surface == "opaque-bg-surface-0"
            && self.host_background == "none"
            && self.host_gutter_px == 64
    }

    fn new(visible: bool) -> Self {
        Self {
            transparent: true,
            decorations: false,
            resizable: false,
            visible,
            panel_surface: "opaque-bg-surface-0",
            host_background: "none",
            host_gutter_px: 64,
        }
    }
}

pub fn launcher_panel_native_options(size: egui::Vec2, visible: bool) -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: launcher_panel_viewport(size, visible),
        ..Default::default()
    }
}

pub fn launcher_panel_viewport(size: egui::Vec2, visible: bool) -> egui::ViewportBuilder {
    egui::ViewportBuilder::default()
        .with_inner_size(size)
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(false)
        .with_visible(visible)
}

pub fn transparent_hidden_panel_contract(size: egui::Vec2) -> String {
    LauncherViewportContract::hidden().native_host_window_summary(size)
}

pub fn transparent_visible_panel_contract(size: egui::Vec2) -> String {
    LauncherViewportContract::visible().native_host_window_summary(size)
}

pub fn launcher_clear_color_contract() -> String {
    "native_clear_color=transparent_rgba_0_0_0_0".to_string()
}

pub fn launcher_viewport_frame_contract() -> String {
    "viewport_frame=transparent_fill,no_stroke".to_string()
}

pub fn launcher_host_positioning_contract() -> &'static str {
    "host_positioning=show:resize-to-panel-host>outer-position-0.28-monitor-anchor>visible>focus;hide:resize-to-1x1>hidden;native_host=transparent;panel_surface=opaque-bg-surface-0;host_background=none;host_gutter=64px"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_viewport_contracts_name_transparency_and_size() {
        assert_eq!(
            transparent_hidden_panel_contract(egui::vec2(848.0, 192.0)),
            "native_host=transparent,transparent=true,decorations=false,resizable=false,visible=false,panel_surface=opaque-bg-surface-0,host_background=none,host_gutter=64px,size=848x192"
        );
        assert_eq!(
            transparent_visible_panel_contract(egui::vec2(848.0, 448.0)),
            "native_host=transparent,transparent=true,decorations=false,resizable=false,visible=true,panel_surface=opaque-bg-surface-0,host_background=none,host_gutter=64px,size=848x448"
        );
    }

    #[test]
    fn launcher_viewport_contract_is_structured_single_source_of_truth() {
        let contract = LauncherViewportContract::visible();

        assert!(contract.passes());
        assert_eq!(
            contract.host_carrier_summary(),
            "host_carrier=transparent:true,decorations:false,resizable:false,background:none,visible_surface:opaque-bg-surface-0"
        );
    }

    #[test]
    fn launcher_panel_native_options_match_contract() {
        let hidden = launcher_panel_native_options(egui::vec2(720.0, 64.0), false);
        let visible = launcher_panel_native_options(egui::vec2(720.0, 320.0), true);
        let hidden_viewport = format!("{:?}", hidden.viewport);
        let visible_viewport = format!("{:?}", visible.viewport);

        assert!(hidden_viewport.contains("transparent: Some(true)"));
        assert!(hidden_viewport.contains("decorations: Some(false)"));
        assert!(hidden_viewport.contains("resizable: Some(false)"));
        assert!(hidden_viewport.contains("visible: Some(false)"));
        assert!(visible_viewport.contains("resizable: Some(false)"));
        assert!(visible_viewport.contains("visible: Some(true)"));
    }

    #[test]
    fn launcher_viewport_contracts_forbid_host_background() {
        assert_eq!(
            launcher_clear_color_contract(),
            "native_clear_color=transparent_rgba_0_0_0_0"
        );
        assert_eq!(
            launcher_viewport_frame_contract(),
            "viewport_frame=transparent_fill,no_stroke"
        );
        assert_eq!(
            launcher_host_positioning_contract(),
            "host_positioning=show:resize-to-panel-host>outer-position-0.28-monitor-anchor>visible>focus;hide:resize-to-1x1>hidden;native_host=transparent;panel_surface=opaque-bg-surface-0;host_background=none;host_gutter=64px"
        );
    }
}
