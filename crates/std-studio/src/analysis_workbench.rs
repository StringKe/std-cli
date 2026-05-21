use std_index::{
    IndexAnswer, IndexCoverageReport, IndexDocument, IndexInspection, IndexSearchResult,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AnalysisWorkbenchTab {
    #[default]
    Overview,
    Components,
    Symbols,
    Relations,
    Qa,
}

impl AnalysisWorkbenchTab {
    pub fn key(self) -> &'static str {
        match self {
            Self::Overview => "overview",
            Self::Components => "components",
            Self::Symbols => "symbols",
            Self::Relations => "relations",
            Self::Qa => "qa",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Components => "Components",
            Self::Symbols => "Symbols",
            Self::Relations => "Relations",
            Self::Qa => "Q&A",
        }
    }

    pub fn all() -> [Self; 5] {
        [
            Self::Overview,
            Self::Components,
            Self::Symbols,
            Self::Relations,
            Self::Qa,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisWorkbenchViewModel {
    pub tabs: Vec<AnalysisTab>,
    pub overview_cards: Vec<AnalysisOverviewCard>,
    pub coverage_layers: Vec<AnalysisCoverageLayer>,
    pub search_hits: Vec<AnalysisSearchHit>,
    pub answer_sources: Vec<AnalysisAnswerSource>,
    pub inspection_summary: Option<AnalysisInspectionSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisTab {
    pub tab: AnalysisWorkbenchTab,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisOverviewCard {
    pub title: &'static str,
    pub value: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisCoverageLayer {
    pub key: &'static str,
    pub label: &'static str,
    pub complete: bool,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisSearchHit {
    pub title: String,
    pub detail: String,
    pub score: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisAnswerSource {
    pub entity: String,
    pub detail: String,
    pub evidence_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisInspectionSummary {
    pub entity: String,
    pub components: usize,
    pub relations: usize,
    pub history: usize,
}

impl AnalysisWorkbenchViewModel {
    pub fn build(
        document: Option<&IndexDocument>,
        coverage: Option<&IndexCoverageReport>,
        answer: Option<&IndexAnswer>,
        search: &[IndexSearchResult],
        inspection: Option<&IndexInspection>,
    ) -> Self {
        let coverage_layers = coverage_layers(document, coverage, inspection);
        let inspection_summary = inspection.map(|inspection| AnalysisInspectionSummary {
            entity: inspection.overview.name.clone(),
            components: inspection.component_count,
            relations: inspection.relation_count,
            history: inspection.history_count,
        });
        Self {
            tabs: tabs(document, answer, search, inspection),
            overview_cards: overview_cards(document, coverage, inspection),
            coverage_layers,
            search_hits: search_hits(search),
            answer_sources: answer_sources(answer),
            inspection_summary,
        }
    }
}

fn tabs(
    document: Option<&IndexDocument>,
    answer: Option<&IndexAnswer>,
    search: &[IndexSearchResult],
    inspection: Option<&IndexInspection>,
) -> Vec<AnalysisTab> {
    let components = document
        .map(|document| document.components.len())
        .or_else(|| inspection.map(|inspection| inspection.component_count))
        .unwrap_or(0);
    let relations = document
        .map(|document| document.relations.len())
        .or_else(|| inspection.map(|inspection| inspection.relation_count))
        .unwrap_or(0);
    AnalysisWorkbenchTab::all()
        .into_iter()
        .map(|tab| AnalysisTab {
            tab,
            count: match tab {
                AnalysisWorkbenchTab::Overview => {
                    usize::from(document.is_some() || inspection.is_some())
                }
                AnalysisWorkbenchTab::Components => components,
                AnalysisWorkbenchTab::Symbols => search.len(),
                AnalysisWorkbenchTab::Relations => relations,
                AnalysisWorkbenchTab::Qa => answer.map(|answer| answer.sources.len()).unwrap_or(0),
            },
        })
        .collect()
}

fn overview_cards(
    document: Option<&IndexDocument>,
    coverage: Option<&IndexCoverageReport>,
    inspection: Option<&IndexInspection>,
) -> Vec<AnalysisOverviewCard> {
    let name = document
        .map(|document| document.overview.name.clone())
        .or_else(|| inspection.map(|inspection| inspection.overview.name.clone()))
        .unwrap_or_else(|| "No target".to_string());
    let detail = document
        .map(|document| document.overview.path.display().to_string())
        .or_else(|| inspection.map(|inspection| inspection.overview.path.display().to_string()))
        .unwrap_or_else(|| "Run analysis to populate target metadata".to_string());
    let coverage_detail = coverage
        .map(|coverage| {
            format!(
                "{} complete, {} incomplete",
                coverage.complete, coverage.incomplete
            )
        })
        .unwrap_or_else(|| "coverage report not loaded".to_string());
    let recent = inspection
        .map(|inspection| format!("{} history entries indexed", inspection.history_count))
        .unwrap_or_else(|| "no inspection selected".to_string());
    vec![
        AnalysisOverviewCard {
            title: "Target",
            value: name,
            detail,
        },
        AnalysisOverviewCard {
            title: "Index",
            value: coverage
                .map(|coverage| format!("{} entities", coverage.total))
                .unwrap_or_else(|| "0 entities".to_string()),
            detail: coverage_detail,
        },
        AnalysisOverviewCard {
            title: "Activity",
            value: inspection
                .map(|inspection| format!("{} relations", inspection.relation_count))
                .unwrap_or_else(|| "No inspection".to_string()),
            detail: recent,
        },
    ]
}

fn coverage_layers(
    document: Option<&IndexDocument>,
    coverage: Option<&IndexCoverageReport>,
    inspection: Option<&IndexInspection>,
) -> Vec<AnalysisCoverageLayer> {
    let document_counts = document.map(|document| {
        (
            usize::from(!document.overview.summary.is_empty()),
            document.components.len(),
            document.relations.len(),
            document.history.len(),
        )
    });
    let coverage_counts = coverage.map(|coverage| {
        coverage
            .items
            .iter()
            .fold((0, 0, 0, 0), |mut counts, item| {
                counts.0 += usize::from(item.coverage.entity_overview);
                counts.1 += item.component_count;
                counts.2 += item.relation_count;
                counts.3 += item.history_count;
                counts
            })
    });
    let counts = document_counts.or(coverage_counts).unwrap_or((0, 0, 0, 0));
    let complete = inspection.map(|inspection| inspection.coverage.clone());
    vec![
        AnalysisCoverageLayer {
            key: "overview",
            label: "Overview",
            complete: complete
                .as_ref()
                .map(|coverage| coverage.entity_overview)
                .unwrap_or(counts.0 > 0),
            count: counts.0,
        },
        AnalysisCoverageLayer {
            key: "components",
            label: "Components",
            complete: complete
                .as_ref()
                .map(|coverage| coverage.component_digest)
                .unwrap_or(counts.1 > 0),
            count: counts.1,
        },
        AnalysisCoverageLayer {
            key: "relations",
            label: "Relations",
            complete: complete
                .as_ref()
                .map(|coverage| coverage.symbol_relation_index)
                .unwrap_or(counts.2 > 0),
            count: counts.2,
        },
        AnalysisCoverageLayer {
            key: "history",
            label: "History",
            complete: complete
                .as_ref()
                .map(|coverage| coverage.historical_context)
                .unwrap_or(counts.3 > 0),
            count: counts.3,
        },
    ]
}

fn search_hits(search: &[IndexSearchResult]) -> Vec<AnalysisSearchHit> {
    search
        .iter()
        .map(|hit| AnalysisSearchHit {
            title: hit.document.overview.name.clone(),
            detail: hit.matched_fields.join(","),
            score: format!("{:.2}", hit.score),
        })
        .collect()
}

fn answer_sources(answer: Option<&IndexAnswer>) -> Vec<AnalysisAnswerSource> {
    answer
        .map(|answer| {
            answer
                .sources
                .iter()
                .map(|source| AnalysisAnswerSource {
                    entity: source.entity.clone(),
                    detail: format!("{} {}", source.path.display(), source.reason),
                    evidence_count: source.evidence.len(),
                })
                .collect()
        })
        .unwrap_or_default()
}
