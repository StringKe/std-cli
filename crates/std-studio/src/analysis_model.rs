use crate::index_error_from_core;
use std_core::StdCore;
use std_index::{
    IndexAnswer, IndexCoverageReport, IndexDocument, IndexError, IndexInspection,
    IndexSearchResult, Indexer,
};

pub(crate) fn analyze_entity(
    core: &StdCore,
    path: &std::path::Path,
) -> Result<IndexDocument, IndexError> {
    let (document, _) = core
        .analyze_and_store_entity(path)
        .map_err(index_error_from_core)?;
    Ok(document)
}

pub(crate) fn saved_analyses(core: &StdCore) -> Result<Vec<IndexDocument>, IndexError> {
    Indexer::read_documents(&core.config.index_dir())
}

pub(crate) fn search_analyses(
    core: &StdCore,
    query: &str,
    limit: usize,
) -> Result<Vec<IndexSearchResult>, IndexError> {
    Indexer::search_documents(&core.config.index_dir(), query, limit)
}

pub(crate) fn ask_analyses(
    core: &StdCore,
    query: &str,
    limit: usize,
) -> Result<IndexAnswer, IndexError> {
    Indexer::answer_from_documents(&core.config.index_dir(), query, limit)
}

pub(crate) fn inspect_analysis(
    core: &StdCore,
    name_or_path: &str,
    limit: usize,
) -> Result<Option<IndexInspection>, IndexError> {
    Indexer::inspect_document(&core.config.index_dir(), name_or_path, limit)
}

pub(crate) fn analysis_coverage_report(core: &StdCore) -> Result<IndexCoverageReport, IndexError> {
    Indexer::coverage_report(&core.config.index_dir())
}
