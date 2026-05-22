use std_studio::StudioApp;

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
        visual_contract: analysis_visual_contract(
            search_hits,
            answer.sources.len(),
            inspection.component_count,
            inspection.relation_count,
        ),
        inspect_components: inspection.component_count,
        inspect_relations: inspection.relation_count,
        inspect_history: inspection.history_count,
        answer_has_evidence,
    })
}

fn analysis_visual_contract(
    search_hits: usize,
    answer_sources: usize,
    components: usize,
    relations: usize,
) -> String {
    format!(
        "toolbar=target-path|re-index|qa-input;tabs=Overview|Components|Symbols|Relations|Q&A;overview=target|index|activity;coverage=overview:PASS|components:PASS|relations:PASS|history:PASS;symbols=search-hits:{};qa=sources:{};components={};relations={}",
        search_hits, answer_sources, components, relations
    )
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
