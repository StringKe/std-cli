use crate::{EntityKind, IndexCoverage, IndexDocument};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexCoverageItem {
    pub name: String,
    pub kind: EntityKind,
    pub path: PathBuf,
    pub coverage: IndexCoverage,
    pub component_count: usize,
    pub relation_count: usize,
    pub history_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexCoverageReport {
    pub total: usize,
    pub complete: usize,
    pub incomplete: usize,
    pub items: Vec<IndexCoverageItem>,
}

pub(crate) fn index_coverage(document: &IndexDocument) -> IndexCoverage {
    IndexCoverage {
        entity_overview: !document.overview.name.trim().is_empty()
            && !document.overview.summary.trim().is_empty(),
        component_digest: !document.components.is_empty(),
        symbol_relation_index: !document.relations.is_empty(),
        historical_context: !document.history.is_empty(),
    }
}

pub(crate) fn coverage_report(documents: Vec<IndexDocument>) -> IndexCoverageReport {
    let items = documents.into_iter().map(coverage_item).collect::<Vec<_>>();
    let complete = items.iter().filter(|item| item.coverage.complete()).count();
    IndexCoverageReport {
        total: items.len(),
        complete,
        incomplete: items.len().saturating_sub(complete),
        items,
    }
}

fn coverage_item(document: IndexDocument) -> IndexCoverageItem {
    let coverage = index_coverage(&document);
    IndexCoverageItem {
        name: document.overview.name,
        kind: document.overview.kind,
        path: document.overview.path,
        coverage,
        component_count: document.components.len(),
        relation_count: document.relations.len(),
        history_count: document.history.len(),
    }
}
