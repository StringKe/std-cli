#[derive(Default)]
pub(crate) struct AnalysisUiState {
    pub(crate) path: String,
    pub(crate) query: String,
    pub(crate) answer: String,
    pub(crate) search_output: String,
    pub(crate) coverage_output: String,
    pub(crate) last_answer: Option<std_index::IndexAnswer>,
    pub(crate) search_results: Vec<std_index::IndexSearchResult>,
    pub(crate) last_inspection: Option<std_index::IndexInspection>,
    pub(crate) coverage_report: Option<std_index::IndexCoverageReport>,
    pub(crate) active_tab: std_studio::AnalysisWorkbenchTab,
    pub(crate) relations_graph_mode: bool,
}

impl AnalysisUiState {
    pub(crate) fn initial() -> Self {
        Self {
            path: ".".to_string(),
            query: "workflow".to_string(),
            active_tab: std_studio::AnalysisWorkbenchTab::Overview,
            ..Self::default()
        }
    }

    pub(crate) fn focus_qa(&mut self) {
        self.active_tab = std_studio::AnalysisWorkbenchTab::Qa;
    }

    pub(crate) fn toggle_relations_view(&mut self) {
        self.active_tab = std_studio::AnalysisWorkbenchTab::Relations;
        self.relations_graph_mode = !self.relations_graph_mode;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_studio::AnalysisWorkbenchTab;

    #[test]
    fn analysis_keyboard_state_focuses_qa_and_toggles_relations() {
        let mut state = AnalysisUiState::initial();

        state.focus_qa();
        assert_eq!(state.active_tab, AnalysisWorkbenchTab::Qa);

        state.toggle_relations_view();
        assert_eq!(state.active_tab, AnalysisWorkbenchTab::Relations);
        assert!(state.relations_graph_mode);

        state.toggle_relations_view();
        assert_eq!(state.active_tab, AnalysisWorkbenchTab::Relations);
        assert!(!state.relations_graph_mode);
    }
}
