use std_studio::{AnalysisVisibilityState, AnalysisWorkbenchViewModel, StudioApp};

pub(crate) struct AnalysisWorkbenchSmoke {
    pub(crate) coverage_layers: String,
    pub(crate) search_hits: usize,
    pub(crate) answer_sources: usize,
    pub(crate) visual_contract: String,
    pub(crate) inspect_components: usize,
    pub(crate) inspect_relations: usize,
    pub(crate) inspect_history: usize,
    pub(crate) answer_has_evidence: bool,
}

pub(crate) fn run_analysis_workbench_smoke(
    studio: &StudioApp,
    query: &str,
    inspect_target: &str,
) -> Result<AnalysisWorkbenchSmoke, Box<dyn std::error::Error>> {
    let coverage = studio.analysis_coverage_report()?;
    let coverage_layers = coverage
        .items
        .first()
        .map(|item| {
            format!(
                "overview:{},components:{},relations:{},history:{}",
                coverage_status(item.coverage.entity_overview),
                coverage_status(item.coverage.component_digest),
                coverage_status(item.coverage.symbol_relation_index),
                coverage_status(item.coverage.historical_context)
            )
        })
        .unwrap_or_else(|| "missing".to_string());
    let search = studio.search_analyses(query, 8)?;
    let answer = studio.ask_analyses(query, 5)?;
    let inspection = studio.inspect_analysis(inspect_target, 8)?.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "analysis inspection missing")
    })?;
    let answer_has_evidence = answer
        .sources
        .iter()
        .any(|source| !source.evidence.is_empty());
    let model = AnalysisWorkbenchViewModel::build(
        studio.active_analysis.as_ref(),
        Some(&coverage),
        Some(&answer),
        &search,
        Some(&inspection),
    );
    let visibility = AnalysisVisibilityState::from_workbench(&model);

    Ok(AnalysisWorkbenchSmoke {
        coverage_layers,
        search_hits: search.len(),
        answer_sources: answer.sources.len(),
        visual_contract: visibility.visual_contract(),
        inspect_components: inspection.component_count,
        inspect_relations: inspection.relation_count,
        inspect_history: inspection.history_count,
        answer_has_evidence,
    })
}

fn coverage_status(pass: bool) -> &'static str {
    if pass {
        "PASS"
    } else {
        "FAIL"
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn coverage_status_uses_text_not_bool_only() {
        assert_eq!(super::coverage_status(true), "PASS");
        assert_eq!(super::coverage_status(false), "FAIL");
    }
}
