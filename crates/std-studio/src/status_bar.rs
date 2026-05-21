use crate::analysis_state::AnalysisUiState;
use std_studio::StudioApp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StudioStatusBarSummary {
    pub(crate) analysis: String,
    pub(crate) ai_provider: String,
    pub(crate) version: String,
}

impl StudioStatusBarSummary {
    pub(crate) fn from_state(app: &StudioApp, analysis: &AnalysisUiState) -> Self {
        Self {
            analysis: analysis_status(analysis),
            ai_provider: ai_provider_status(app),
            version: format!("v{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub(crate) fn right_labels(&self) -> [&str; 3] {
        [&self.version, &self.ai_provider, &self.analysis]
    }

    #[cfg(test)]
    pub(crate) fn contract(&self) -> String {
        format!(
            "analysis={},ai_provider={},version={}",
            self.analysis, self.ai_provider, self.version
        )
    }
}

fn analysis_status(analysis: &AnalysisUiState) -> String {
    match &analysis.coverage_report {
        Some(report) => format!("Analysis {}/{} complete", report.complete, report.total),
        None if analysis.coverage_output.is_empty() => "Analysis not loaded".to_string(),
        None => "Analysis needs refresh".to_string(),
    }
}

fn ai_provider_status(app: &StudioApp) -> String {
    if app.core.config.enable_ai {
        "AI planner on".to_string()
    } else {
        "AI planner off".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_bar_summary_reports_docs22_right_side_fields() {
        let mut analysis = AnalysisUiState::initial();
        analysis.coverage_report = Some(std_index::IndexCoverageReport {
            total: 4,
            complete: 3,
            incomplete: 1,
            items: Vec::new(),
        });
        let mut app = StudioApp::default();
        app.core.config.enable_ai = true;

        let summary = StudioStatusBarSummary::from_state(&app, &analysis);

        assert_eq!(summary.analysis, "Analysis 3/4 complete");
        assert_eq!(summary.ai_provider, "AI planner on");
        assert_eq!(summary.version, "v0.1.0");
        assert_eq!(
            summary.contract(),
            "analysis=Analysis 3/4 complete,ai_provider=AI planner on,version=v0.1.0"
        );
        assert_eq!(
            summary.right_labels(),
            ["v0.1.0", "AI planner on", "Analysis 3/4 complete"]
        );
    }
}
