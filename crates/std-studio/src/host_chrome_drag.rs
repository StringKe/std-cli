use crate::layout::HOST_CHROME_HEIGHT;
use eframe::egui;
use std_egui::tokens::Space;

const HOST_CHROME_DRAG_INSET_X: f32 = Space::SM as f32;
const HOST_CHROME_DRAG_INSET_Y: f32 = Space::XS as f32;
const HOST_CHROME_DRAG_MIN_WIDTH: f32 = 320.0;
const HOST_CHROME_ACTION_RESERVE_WIDTH: f32 = 520.0;

pub(crate) fn install_host_chrome_drag_region(ui: &mut egui::Ui) -> egui::Response {
    let rect = host_chrome_drag_rect(ui.max_rect());
    let response = ui.interact(
        rect,
        ui.id().with("host_drag_background"),
        egui::Sense::click_and_drag(),
    );
    if response.drag_started() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    response
}

pub(crate) fn host_chrome_drag_contract() -> &'static str {
    "drag_region=background-only,left-identity-area;controls_reserved=true"
}

fn host_chrome_drag_rect(chrome_rect: egui::Rect) -> egui::Rect {
    let reserved_right = (chrome_rect.right() - HOST_CHROME_ACTION_RESERVE_WIDTH)
        .max(chrome_rect.left() + HOST_CHROME_DRAG_MIN_WIDTH);
    egui::Rect::from_min_max(
        egui::pos2(
            chrome_rect.left() + HOST_CHROME_DRAG_INSET_X,
            chrome_rect.top() + HOST_CHROME_DRAG_INSET_Y,
        ),
        egui::pos2(
            reserved_right,
            (chrome_rect.top() + HOST_CHROME_HEIGHT - HOST_CHROME_DRAG_INSET_Y)
                .min(chrome_rect.bottom() - HOST_CHROME_DRAG_INSET_Y),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_chrome_drag_region_reserves_right_controls() {
        let chrome = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1280.0, 52.0));
        let drag = host_chrome_drag_rect(chrome);

        assert_eq!(drag.left(), 12.0);
        assert_eq!(drag.right(), 760.0);
        assert_eq!(drag.top(), 8.0);
        assert_eq!(drag.bottom(), 44.0);
        assert!(drag.right() < chrome.right() - 320.0);
    }

    #[test]
    fn host_chrome_drag_contract_excludes_controls() {
        assert_eq!(
            host_chrome_drag_contract(),
            "drag_region=background-only,left-identity-area;controls_reserved=true"
        );
    }
}
