use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum IndexError {
    #[error("Index IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Index JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Entity path does not exist: {0}")]
    EntityMissing(PathBuf),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EntityKind {
    Project,
    Workflow,
    AppBundle,
    File,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityOverview {
    pub id: Uuid,
    pub kind: EntityKind,
    pub path: PathBuf,
    pub name: String,
    pub summary: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentDigest {
    pub path: PathBuf,
    pub purpose: String,
    pub size_bytes: u64,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub snippet: String,
    #[serde(default)]
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolRelation {
    pub symbol: String,
    pub relation: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HistoricalContext {
    pub source: String,
    pub summary: String,
    pub observed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexDocument {
    pub overview: EntityOverview,
    pub components: Vec<ComponentDigest>,
    pub relations: Vec<SymbolRelation>,
    pub history: Vec<HistoricalContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexInspection {
    pub overview: EntityOverview,
    pub coverage: IndexCoverage,
    pub component_count: usize,
    pub relation_count: usize,
    pub history_count: usize,
    pub key_components: Vec<ComponentDigest>,
    pub key_relations: Vec<SymbolRelation>,
    pub key_history: Vec<HistoricalContext>,
}

pub use crate::coverage::{IndexCoverageItem, IndexCoverageReport};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexCoverage {
    pub entity_overview: bool,
    pub component_digest: bool,
    pub symbol_relation_index: bool,
    pub historical_context: bool,
}

impl IndexCoverage {
    pub fn complete(&self) -> bool {
        self.entity_overview
            && self.component_digest
            && self.symbol_relation_index
            && self.historical_context
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexSearchResult {
    pub document: IndexDocument,
    pub score: f32,
    pub matched_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexAnswer {
    pub query: String,
    pub answer: String,
    pub sources: Vec<IndexAnswerSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndexAnswerSource {
    pub entity: String,
    pub path: PathBuf,
    pub reason: String,
    pub matched_fields: Vec<String>,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileIndexOptions {
    pub max_depth: usize,
    pub max_files: usize,
    pub max_file_bytes: u64,
}

impl Default for FileIndexOptions {
    fn default() -> Self {
        Self {
            max_depth: 6,
            max_files: 5_000,
            max_file_bytes: 128 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileIndexEntry {
    pub path: PathBuf,
    pub name: String,
    pub size_bytes: u64,
    pub modified_at: Option<DateTime<Utc>>,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileIndex {
    pub root: PathBuf,
    pub entries: Vec<FileIndexEntry>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileSearchResult {
    pub entry: FileIndexEntry,
    pub score: f32,
    pub matched_fields: Vec<String>,
}
