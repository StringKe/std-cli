use eframe::egui;

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
    viewport_contract(size, false)
}

pub fn transparent_visible_panel_contract(size: egui::Vec2) -> String {
    viewport_contract(size, true)
}

pub fn launcher_clear_color_contract() -> String {
    "native_clear_color=transparent_rgba_0_0_0_0".to_string()
}

pub fn launcher_viewport_frame_contract() -> String {
    "viewport_frame=transparent_fill,no_stroke".to_string()
}

pub fn launcher_host_positioning_contract() -> &'static str {
    "host_positioning=resize-fixed-transparent-carrier>outer-position-0.28-monitor-anchor>visible>focus;native_window=transparent-carrier;panel_surface=opaque-bg-surface-0;carrier_background=none"
}

fn viewport_contract(size: egui::Vec2, visible: bool) -> String {
    format!(
        "native=transparent-carrier,transparent=true,decorations=false,resizable=false,visible={visible},panel_surface=opaque,size={}x{}",
        size.x as u32, size.y as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_viewport_contracts_name_transparency_and_size() {
        assert_eq!(
            transparent_hidden_panel_contract(egui::vec2(720.0, 64.0)),
            "native=transparent-carrier,transparent=true,decorations=false,resizable=false,visible=false,panel_surface=opaque,size=720x64"
        );
        assert_eq!(
            transparent_visible_panel_contract(egui::vec2(720.0, 320.0)),
            "native=transparent-carrier,transparent=true,decorations=false,resizable=false,visible=true,panel_surface=opaque,size=720x320"
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
    fn launcher_viewport_contracts_forbid_carrier_background() {
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
            "host_positioning=resize-fixed-transparent-carrier>outer-position-0.28-monitor-anchor>visible>focus;native_window=transparent-carrier;panel_surface=opaque-bg-surface-0;carrier_background=none"
        );
    }
}
