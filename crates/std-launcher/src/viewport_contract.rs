use eframe::egui;

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

fn viewport_contract(size: egui::Vec2, visible: bool) -> String {
    format!(
        "native=panel-surface,transparent=true,decorations=false,visible={visible},size={}x{}",
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
            "native=panel-surface,transparent=true,decorations=false,visible=false,size=720x64"
        );
        assert_eq!(
            transparent_visible_panel_contract(egui::vec2(720.0, 320.0)),
            "native=panel-surface,transparent=true,decorations=false,visible=true,size=720x320"
        );
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
    }
}
