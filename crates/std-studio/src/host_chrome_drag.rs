use crate::layout::HOST_CHROME_HEIGHT;
use eframe::egui;
use std_egui::tokens::HostChromeSize;

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
    let inset = HostChromeSize::drag_inset();
    let reserved_right = (chrome_rect.right() - HostChromeSize::action_reserve_width())
        .max(chrome_rect.left() + HostChromeSize::drag_min_width());
    egui::Rect::from_min_max(
        egui::pos2(chrome_rect.left() + inset.x, chrome_rect.top() + inset.y),
        egui::pos2(
            reserved_right,
            (chrome_rect.top() + HOST_CHROME_HEIGHT - inset.y).min(chrome_rect.bottom() - inset.y),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_chrome_drag_region_reserves_right_controls() {
        let chrome =
            egui::Rect::from_min_size(egui::Pos2::ZERO, HostChromeSize::test_chrome_size());
        let drag = host_chrome_drag_rect(chrome);
        let inset = HostChromeSize::drag_inset();

        assert_eq!(drag.left(), inset.x);
        assert_eq!(drag.right(), 760.0);
        assert_eq!(drag.top(), inset.y);
        assert_eq!(drag.bottom(), 44.0);
        assert!(drag.right() < chrome.right() - HostChromeSize::drag_min_width());
    }

    #[test]
    fn host_chrome_drag_contract_excludes_controls() {
        assert_eq!(
            host_chrome_drag_contract(),
            "drag_region=background-only,left-identity-area;controls_reserved=true"
        );
    }
}
