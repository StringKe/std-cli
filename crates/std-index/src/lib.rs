//! Local personal index foundation for std-cli.

mod app_bundle;
mod coverage;
mod files;
mod history;
mod relations;
mod search;
mod types;
mod util;
mod workflow;

use app_bundle::AppBundleMetadata;
use chrono::Utc;
use coverage::coverage_report;
use files::{
    digest_directory, digest_file, index_files, read_file_indexes, search_file_entries,
    write_file_index,
};
use history::historical_context;
use relations::{infer_component_purpose, infer_relations};
use search::{build_index_answer, inspect_document, search_documents};
use std::{
    fs,
    path::{Path, PathBuf},
};
pub use types::{
    ComponentDigest, EntityKind, EntityOverview, FileIndex, FileIndexEntry, FileIndexOptions,
    FileSearchResult, HistoricalContext, IndexAnswer, IndexAnswerSource, IndexCoverage,
    IndexCoverageItem, IndexCoverageReport, IndexDocument, IndexError, IndexInspection,
    IndexSearchResult, SymbolRelation,
};
use util::{entity_name, slug};
use uuid::Uuid;

pub struct Indexer;

impl Indexer {
    pub fn analyze(path: &Path) -> Result<IndexDocument, IndexError> {
        if !path.exists() {
            return Err(IndexError::EntityMissing(path.to_path_buf()));
        }

        let metadata = fs::metadata(path)?;
        let kind = classify_entity(path, &metadata);
        let app_bundle = (kind == EntityKind::AppBundle)
            .then(|| AppBundleMetadata::read(path))
            .flatten();
        let name = app_bundle
            .as_ref()
            .map(|metadata| metadata.display_name.clone())
            .unwrap_or_else(|| entity_name(path));
        let components = if metadata.is_dir() {
            digest_directory(path, &FileIndexOptions::default())?
        } else {
            vec![digest_file(path)?]
        };
        let mut relations = infer_relations(&components);
        if let Some(metadata) = &app_bundle {
            relations.extend(metadata.alias_relations(path));
        }
        let history = historical_context(path)?;

        Ok(IndexDocument {
            overview: EntityOverview {
                id: Uuid::new_v4(),
                kind,
                path: path.to_path_buf(),
                name: name.clone(),
                summary: entity_summary(&name, components.len(), app_bundle.as_ref()),
                created_at: Utc::now(),
            },
            relations,
            history,
            components,
        })
    }

    pub fn write_document(index_dir: &Path, doc: &IndexDocument) -> Result<PathBuf, IndexError> {
        fs::create_dir_all(index_dir)?;
        let file_name = format!("{}.json", slug(&doc.overview.name));
        let path = index_dir.join(file_name);
        fs::write(&path, serde_json::to_string_pretty(doc)?)?;
        Ok(path)
    }

    pub fn read_document(path: &Path) -> Result<IndexDocument, IndexError> {
        let body = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&body)?)
    }

    pub fn read_documents(index_dir: &Path) -> Result<Vec<IndexDocument>, IndexError> {
        if !index_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut documents = Vec::new();
        for entry in fs::read_dir(index_dir)? {
            let path = entry?.path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if name.starts_with("files-")
                || path.extension().and_then(|ext| ext.to_str()) != Some("json")
            {
                continue;
            }
            documents.push(Self::read_document(&path)?);
        }
        documents.sort_by(|a, b| a.overview.name.cmp(&b.overview.name));
        Ok(documents)
    }

    pub fn find_document(
        index_dir: &Path,
        name_or_path: &str,
    ) -> Result<Option<IndexDocument>, IndexError> {
        let query = name_or_path.trim();
        for document in Self::read_documents(index_dir)? {
            let document_path = document.overview.path.display().to_string();
            if document.overview.name == query
                || slug(&document.overview.name) == slug(query)
                || document_path == query
                || document_path.ends_with(query)
            {
                return Ok(Some(document));
            }
        }
        Ok(None)
    }

    pub fn search_documents(
        index_dir: &Path,
        query: &str,
        limit: usize,
    ) -> Result<Vec<IndexSearchResult>, IndexError> {
        let documents = Self::read_documents(index_dir)?;
        Ok(search_documents(documents, query, limit))
    }

    pub fn answer_from_documents(
        index_dir: &Path,
        query: &str,
        limit: usize,
    ) -> Result<IndexAnswer, IndexError> {
        let results = Self::search_documents(index_dir, query, limit)?;
        Ok(build_index_answer(query, results))
    }

    pub fn inspect_document(
        index_dir: &Path,
        name_or_path: &str,
        limit: usize,
    ) -> Result<Option<IndexInspection>, IndexError> {
        Ok(Self::find_document(index_dir, name_or_path)?
            .map(|document| inspect_document(document, limit)))
    }

    pub fn coverage_report(index_dir: &Path) -> Result<IndexCoverageReport, IndexError> {
        let documents = Self::read_documents(index_dir)?;
        Ok(coverage_report(documents))
    }

    pub fn index_files(root: &Path, options: FileIndexOptions) -> Result<FileIndex, IndexError> {
        if !root.exists() {
            return Err(IndexError::EntityMissing(root.to_path_buf()));
        }

        index_files(root, options)
    }

    pub fn write_file_index(index_dir: &Path, index: &FileIndex) -> Result<PathBuf, IndexError> {
        write_file_index(index_dir, index)
    }

    pub fn read_file_indexes(index_dir: &Path) -> Result<Vec<FileIndex>, IndexError> {
        read_file_indexes(index_dir)
    }

    pub fn search_files(
        root: &Path,
        query: &str,
        options: FileIndexOptions,
    ) -> Result<Vec<FileSearchResult>, IndexError> {
        let index = index_files(root, options)?;
        Ok(search_file_entries(index.entries, query, usize::MAX))
    }

    pub fn search_indexed_files(
        index_dir: &Path,
        query: &str,
        limit: usize,
    ) -> Result<Vec<FileSearchResult>, IndexError> {
        let entries = Self::read_file_indexes(index_dir)?
            .into_iter()
            .flat_map(|index| index.entries)
            .collect::<Vec<_>>();
        Ok(search_file_entries(entries, query, limit))
    }
}

fn entity_summary(
    name: &str,
    component_count: usize,
    app_bundle: Option<&AppBundleMetadata>,
) -> String {
    let base = format!("Local entity `{name}` with {component_count} indexed components");
    if let Some(metadata) = app_bundle {
        format!("{base}. {}", metadata.summary())
    } else {
        base
    }
}

fn classify_entity(path: &Path, metadata: &fs::Metadata) -> EntityKind {
    if path.extension().and_then(|ext| ext.to_str()) == Some("app") {
        EntityKind::AppBundle
    } else if path.file_name().and_then(|name| name.to_str()) == Some("workflow.json")
        || path.file_name().and_then(|name| name.to_str()) == Some("workflow.md")
    {
        EntityKind::Workflow
    } else if metadata.is_dir() {
        EntityKind::Project
    } else {
        EntityKind::File
    }
}

#[cfg(test)]
mod tests;
