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
}
