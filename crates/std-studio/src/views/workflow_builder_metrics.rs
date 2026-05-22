use eframe::egui;
use std_egui::tokens::{Space, StudioSize};

use super::row_metrics;

pub(crate) const BUILDER_PANEL_GAP: f32 = Space::SM as f32;
pub(crate) const BUILDER_TOOLBAR_HEIGHT: f32 = StudioSize::TOOLBAR_HEIGHT;
pub(crate) const PROPERTY_SINGLE_LINE_HEIGHT: f32 = Space::LG as f32;
pub(crate) const PROPERTY_LABEL_HEIGHT: f32 = Space::LG as f32;

pub(crate) fn builder_columns(available_width: f32) -> Option<(f32, f32)> {
    StudioSize::workflow_builder_columns(available_width)
}

pub(crate) fn goal_input_size(available_width: f32) -> [f32; 2] {
    StudioSize::workflow_builder_goal_input_size(available_width)
}

pub(crate) fn parameter_editor_size(available_width: f32) -> [f32; 2] {
    StudioSize::workflow_builder_parameter_editor_size(available_width)
}

pub(crate) fn step_index_size() -> [f32; 2] {
    [
        row_metrics::BUILDER_INDEX_WIDTH,
        row_metrics::BUILDER_INDEX_HEIGHT,
    ]
}

pub(crate) fn ai_input_size(available_width: f32) -> [f32; 2] {
    StudioSize::workflow_builder_ai_input_size(available_width)
}

pub(crate) fn builder_pane_size(width: f32) -> egui::Vec2 {
    StudioSize::workflow_builder_pane_size(width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_metrics_match_documented_workflow_builder_layout() {
        assert_eq!(BUILDER_TOOLBAR_HEIGHT, 44.0);
        assert_eq!(row_metrics::BUILDER_STEP_ROW_HEIGHT, 48.0);
        assert_eq!(BUILDER_PANEL_GAP, Space::SM as f32);
        assert_eq!(StudioSize::WORKFLOW_BUILDER_WIDE_BREAKPOINT, 560.0);
        assert_eq!(
            StudioSize::workflow_builder_ai_input_size(640.0),
            [640.0, Space::XL as f32]
        );
    }

    #[test]
    fn narrow_builder_uses_stacked_layout() {
        assert_eq!(
            builder_columns(StudioSize::WORKFLOW_BUILDER_WIDE_BREAKPOINT - 1.0),
            None
        );
    }

    #[test]
    fn wide_builder_uses_stable_two_pane_layout() {
        let (left, right) = builder_columns(800.0).unwrap();

        assert_eq!(left, 378.24);
        assert_eq!(right, 409.76);
    }

    #[test]
    fn builder_inputs_use_tokenized_sizes() {
        assert_eq!(goal_input_size(800.0), [260.0, 28.0]);
        assert_eq!(parameter_editor_size(640.0), [640.0, 92.0]);
        assert_eq!(step_index_size(), [32.0, 22.0]);
        assert_eq!(ai_input_size(640.0), [640.0, 32.0]);
        assert_eq!(PROPERTY_SINGLE_LINE_HEIGHT, 24.0);
        assert_eq!(PROPERTY_LABEL_HEIGHT, 24.0);
    }
}
