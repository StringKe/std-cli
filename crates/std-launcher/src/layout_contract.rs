pub const PANEL_WIDTH: f32 = 720.0;
pub const PANEL_VIEWPORT_WIDTH_RATIO: f32 = 0.55;
pub const PANEL_MIN_WIDTH: f32 = 320.0;

pub fn panel_width_for_available(available_width: f32, margin: f32, scale: f32) -> f32 {
    let max_width = PANEL_WIDTH * scale;
    let min_width = PANEL_MIN_WIDTH * scale;
    let usable_width = (available_width - margin * 2.0).max(min_width);
    let ratio_width = usable_width * PANEL_VIEWPORT_WIDTH_RATIO;
    if usable_width <= max_width {
        usable_width
    } else {
        max_width.min(ratio_width.max(min_width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_uses_full_carrier_when_native_window_is_tight() {
        assert_eq!(panel_width_for_available(720.0, 0.0, 1.0), 720.0);
    }

    #[test]
    fn width_uses_docs_ratio_on_medium_carrier() {
        assert_eq!(panel_width_for_available(1000.0, 0.0, 1.0), 550.0);
    }

    #[test]
    fn width_caps_at_docs_max_on_wide_carrier() {
        assert_eq!(panel_width_for_available(1440.0, 0.0, 1.0), 720.0);
    }
}
