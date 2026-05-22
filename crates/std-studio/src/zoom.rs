use crate::StudioEguiApp;

const ZOOM_MIN: f32 = 0.85;
const ZOOM_MAX: f32 = 1.5;
const ZOOM_STEP: f32 = 0.05;
const ZOOM_DEFAULT: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StudioZoomAction {
    In,
    Out,
    Reset,
}

impl StudioEguiApp {
    pub(crate) fn apply_zoom_shortcut(&mut self, action: StudioZoomAction) {
        let next = next_zoom_value(self.app.core.config.ui_scale(), action);
        self.settings_ui_scale = format_zoom(next);
        self.save_setting("appearance.ui_scale", self.settings_ui_scale.clone());
    }
}

pub(crate) fn next_zoom_value(current: f32, action: StudioZoomAction) -> f32 {
    let next = match action {
        StudioZoomAction::In => current + ZOOM_STEP,
        StudioZoomAction::Out => current - ZOOM_STEP,
        StudioZoomAction::Reset => ZOOM_DEFAULT,
    };
    snap_zoom(next.clamp(ZOOM_MIN, ZOOM_MAX))
}

fn snap_zoom(value: f32) -> f32 {
    ((value / ZOOM_STEP).round() * ZOOM_STEP * 100.0).round() / 100.0
}

fn format_zoom(value: f32) -> String {
    format!("{value:.2}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_zoom_shortcuts_step_clamp_and_reset_to_docs23_range() {
        assert_eq!(next_zoom_value(1.0, StudioZoomAction::In), 1.05);
        assert_eq!(next_zoom_value(1.0, StudioZoomAction::Out), 0.95);
        assert_eq!(next_zoom_value(1.37, StudioZoomAction::In), 1.4);
        assert_eq!(next_zoom_value(1.5, StudioZoomAction::In), 1.5);
        assert_eq!(next_zoom_value(0.85, StudioZoomAction::Out), 0.85);
        assert_eq!(next_zoom_value(1.25, StudioZoomAction::Reset), 1.0);
    }
}
