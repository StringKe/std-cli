use std_egui::tokens::StudioSize;

pub(crate) const INPUT_HEIGHT: f32 = StudioSize::INPUT_HEIGHT;
pub(crate) const TOOLBAR_HEIGHT: f32 = StudioSize::TOOLBAR_HEIGHT;
pub(crate) const TAB_BAR_HEIGHT: f32 = StudioSize::TAB_BAR_HEIGHT;
pub(crate) const WIDE_WORKSPACE_BREAKPOINT: f32 = StudioSize::WIDE_WORKSPACE_BREAKPOINT;
pub(crate) const FORM_BUTTON_RESERVE_WIDTH: f32 = StudioSize::FORM_BUTTON_RESERVE_WIDTH;
pub(crate) const PLUGIN_TOOLBAR_ACTIONS_WIDTH: f32 = StudioSize::PLUGIN_TOOLBAR_ACTIONS_WIDTH;
pub(crate) const ANALYSIS_TOOLBAR_ACTIONS_WIDTH: f32 = StudioSize::ANALYSIS_TOOLBAR_ACTIONS_WIDTH;
pub(crate) const ANALYSIS_QUERY_ACTIONS_WIDTH: f32 = StudioSize::ANALYSIS_QUERY_ACTIONS_WIDTH;
pub(crate) const ANALYSIS_FIELD_MIN_WIDTH: f32 = StudioSize::ANALYSIS_FIELD_MIN_WIDTH;
pub(crate) const SEARCH_RESULTS_MAX_HEIGHT: f32 = StudioSize::SEARCH_RESULTS_MAX_HEIGHT;
pub(crate) const DETAIL_BODY_MAX_HEIGHT: f32 = StudioSize::DETAIL_BODY_MAX_HEIGHT;
pub(crate) const PANEL_LIST_MAX_HEIGHT: f32 = StudioSize::PANEL_LIST_MAX_HEIGHT;
pub(crate) const MEMORY_LIST_MAX_HEIGHT: f32 = StudioSize::MEMORY_LIST_MAX_HEIGHT;
pub(crate) const PLUGIN_CHECKS_MAX_HEIGHT: f32 = StudioSize::PLUGIN_CHECKS_MAX_HEIGHT;
pub(crate) const MULTILINE_INPUT_HEIGHT: f32 = StudioSize::MULTILINE_INPUT_HEIGHT;

pub(crate) fn thirds_column_width(available_width: f32, gap: f32) -> f32 {
    StudioSize::thirds_column_width(available_width, gap)
}

pub(crate) fn toolbar_field_width(available_width: f32, reserve_width: f32) -> f32 {
    StudioSize::toolbar_field_width(available_width, reserve_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn studio_metrics_match_docs_22_layout_contract() {
        assert_eq!(TOOLBAR_HEIGHT, 44.0);
        assert_eq!(TAB_BAR_HEIGHT, 36.0);
        assert_eq!(INPUT_HEIGHT, 28.0);
        assert_eq!(WIDE_WORKSPACE_BREAKPOINT, 900.0);
    }

    #[test]
    fn studio_column_metrics_keep_three_equal_panes() {
        assert_eq!(thirds_column_width(924.0, 12.0), 300.0);
    }

    #[test]
    fn toolbar_field_width_keeps_minimum_input_size() {
        assert_eq!(toolbar_field_width(300.0, FORM_BUTTON_RESERVE_WIDTH), 260.0);
        assert_eq!(toolbar_field_width(500.0, FORM_BUTTON_RESERVE_WIDTH), 390.0);
    }
}
