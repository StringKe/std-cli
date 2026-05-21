#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum AnalysisFocusArea {
    #[default]
    Target,
    Tabs,
    Content,
    Query,
    Coverage,
}

impl AnalysisFocusArea {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Target => "target",
            Self::Tabs => "tabs",
            Self::Content => "content",
            Self::Query => "query",
            Self::Coverage => "coverage",
        }
    }

    pub(crate) fn focus_id(self) -> egui::Id {
        egui::Id::new(("studio.analysis.focus", self.label()))
    }

    fn all() -> [Self; 5] {
        [
            Self::Target,
            Self::Tabs,
            Self::Content,
            Self::Query,
            Self::Coverage,
        ]
    }
}

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
    pub(crate) focus_area: AnalysisFocusArea,
}

impl AnalysisUiState {
    pub(crate) fn initial() -> Self {
        Self {
            path: ".".to_string(),
            query: "workflow".to_string(),
            active_tab: std_studio::AnalysisWorkbenchTab::Overview,
            focus_area: AnalysisFocusArea::Target,
            ..Self::default()
        }
    }

    pub(crate) fn focus_qa(&mut self) {
        self.active_tab = std_studio::AnalysisWorkbenchTab::Qa;
        self.focus_area = AnalysisFocusArea::Query;
    }

    pub(crate) fn toggle_relations_view(&mut self) {
        self.active_tab = std_studio::AnalysisWorkbenchTab::Relations;
        self.focus_area = AnalysisFocusArea::Content;
        self.relations_graph_mode = !self.relations_graph_mode;
    }

    pub(crate) fn focus_next(&mut self) {
        self.focus_area = adjacent_focus_area(self.focus_area, 1);
    }

    pub(crate) fn focus_previous(&mut self) {
        self.focus_area = adjacent_focus_area(self.focus_area, -1);
    }
}

fn adjacent_focus_area(current: AnalysisFocusArea, offset: isize) -> AnalysisFocusArea {
    let areas = AnalysisFocusArea::all();
    let current_index = areas
        .iter()
        .position(|area| *area == current)
        .unwrap_or_default();
    let next = (current_index as isize + offset).rem_euclid(areas.len() as isize) as usize;
    areas[next]
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
        assert_eq!(state.focus_area, AnalysisFocusArea::Query);

        state.toggle_relations_view();
        assert_eq!(state.active_tab, AnalysisWorkbenchTab::Relations);
        assert_eq!(state.focus_area, AnalysisFocusArea::Content);
        assert!(state.relations_graph_mode);

        state.toggle_relations_view();
        assert_eq!(state.active_tab, AnalysisWorkbenchTab::Relations);
        assert!(!state.relations_graph_mode);
    }

    #[test]
    fn analysis_focus_order_matches_docs_keyboard_flow() {
        let mut state = AnalysisUiState::initial();

        assert_eq!(state.focus_area.label(), "target");
        state.focus_next();
        assert_eq!(state.focus_area, AnalysisFocusArea::Tabs);
        state.focus_next();
        assert_eq!(state.focus_area, AnalysisFocusArea::Content);
        state.focus_next();
        assert_eq!(state.focus_area, AnalysisFocusArea::Query);
        state.focus_next();
        assert_eq!(state.focus_area, AnalysisFocusArea::Coverage);
        state.focus_next();
        assert_eq!(state.focus_area, AnalysisFocusArea::Target);
        state.focus_previous();
        assert_eq!(state.focus_area, AnalysisFocusArea::Coverage);
    }

    #[test]
    fn analysis_focus_areas_have_stable_egui_ids() {
        let mut state = AnalysisUiState::initial();
        let target = state.focus_area.focus_id();

        assert_eq!(target, AnalysisFocusArea::Target.focus_id());
        state.focus_next();
        assert_eq!(
            state.focus_area.focus_id(),
            AnalysisFocusArea::Tabs.focus_id()
        );
        assert_ne!(target, state.focus_area.focus_id());
    }

    #[test]
    fn analysis_focus_area_ids_cover_all_regions() {
        let ids = AnalysisFocusArea::all().map(AnalysisFocusArea::focus_id);

        assert_eq!(ids.len(), 5);
        for (index, id) in ids.iter().enumerate() {
            assert!(
                !ids[index + 1..].iter().any(|next| next == id),
                "focus id must be unique for {:?}",
                AnalysisFocusArea::all()[index]
            );
        }
        assert_ne!(
            AnalysisFocusArea::Content.focus_id(),
            AnalysisFocusArea::Coverage.focus_id()
        );
    }
}
