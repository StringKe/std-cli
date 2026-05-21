use eframe::egui;

pub fn transparent_hidden_panel_contract(size: egui::Vec2) -> String {
    viewport_contract(size, false)
}

pub fn transparent_visible_panel_contract(size: egui::Vec2) -> String {
    viewport_contract(size, true)
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
}
