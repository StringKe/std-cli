pub(crate) const INPUT_HEIGHT: f32 = 28.0;
pub(crate) const TOOLBAR_HEIGHT: f32 = 44.0;
pub(crate) const TAB_BAR_HEIGHT: f32 = 36.0;
pub(crate) const WIDE_WORKSPACE_BREAKPOINT: f32 = 900.0;
pub(crate) const FORM_BUTTON_RESERVE_WIDTH: f32 = 110.0;
pub(crate) const PLUGIN_TOOLBAR_ACTIONS_WIDTH: f32 = 230.0;
pub(crate) const ANALYSIS_TOOLBAR_ACTIONS_WIDTH: f32 = 420.0;
pub(crate) const ANALYSIS_QUERY_ACTIONS_WIDTH: f32 = 176.0;
pub(crate) const ANALYSIS_FIELD_MIN_WIDTH: f32 = 260.0;
pub(crate) const SEARCH_RESULTS_MAX_HEIGHT: f32 = 430.0;
pub(crate) const DETAIL_BODY_MAX_HEIGHT: f32 = 480.0;
pub(crate) const PANEL_LIST_MAX_HEIGHT: f32 = 560.0;
pub(crate) const MEMORY_LIST_MAX_HEIGHT: f32 = 590.0;
pub(crate) const PLUGIN_CHECKS_MAX_HEIGHT: f32 = 190.0;
pub(crate) const MULTILINE_INPUT_HEIGHT: f32 = 220.0;

pub(crate) fn thirds_column_width(available_width: f32, gap: f32) -> f32 {
    (available_width - gap * 2.0) / 3.0
}

pub(crate) fn toolbar_field_width(available_width: f32, reserve_width: f32) -> f32 {
    (available_width - reserve_width).max(ANALYSIS_FIELD_MIN_WIDTH)
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
