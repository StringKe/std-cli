use eframe::egui;

pub fn transparent_hidden_panel_contract(size: egui::Vec2) -> String {
    viewport_contract(size, false)
}

pub fn transparent_visible_panel_contract(size: egui::Vec2) -> String {
    viewport_contract(size, true)
}

fn viewport_contract(size: egui::Vec2, visible: bool) -> String {
    format!(
        "carrier=transparent,decorations=false,visible={visible},size={}x{}",
        size.x as u32, size.y as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launcher_viewport_contracts_name_transparency_and_size() {
        assert_eq!(
            transparent_hidden_panel_contract(egui::vec2(848.0, 192.0)),
            "carrier=transparent,decorations=false,visible=false,size=848x192"
        );
        assert_eq!(
            transparent_visible_panel_contract(egui::vec2(848.0, 448.0)),
            "carrier=transparent,decorations=false,visible=true,size=848x448"
        );
    }
}
