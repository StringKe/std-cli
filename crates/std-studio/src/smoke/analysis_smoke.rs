use std_studio::StudioApp;

pub(crate) struct AnalysisWorkbenchSmoke {
    pub(crate) coverage_layers: String,
    pub(crate) search_hits: usize,
    pub(crate) answer_sources: usize,
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
                "overview={},components={},relations={},history={}",
                item.coverage.entity_overview,
                item.coverage.component_digest,
                item.coverage.symbol_relation_index,
                item.coverage.historical_context
            )
        })
        .unwrap_or_else(|| "missing".to_string());
    let search_hits = studio.search_analyses(query, 8)?.len();
    let answer = studio.ask_analyses(query, 5)?;
    let inspection = studio.inspect_analysis(inspect_target, 8)?.ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "analysis inspection missing")
    })?;
    let answer_has_evidence = answer
        .sources
        .iter()
        .any(|source| !source.evidence.is_empty());

    Ok(AnalysisWorkbenchSmoke {
        coverage_layers,
        search_hits,
        answer_sources: answer.sources.len(),
        inspect_components: inspection.component_count,
        inspect_relations: inspection.relation_count,
        inspect_history: inspection.history_count,
        answer_has_evidence,
    })
}
