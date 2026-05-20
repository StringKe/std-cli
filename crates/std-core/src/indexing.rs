use crate::{events::EventBus, CoreError, StdCore};
use std::path::{Path, PathBuf};
use std_index::{FileIndex, FileIndexOptions, IndexDocument, Indexer};
use std_types::{StdEvent, StdEventType};

impl StdCore {
    pub fn analyze_and_store_entity(
        &self,
        path: &Path,
    ) -> Result<(IndexDocument, PathBuf), CoreError> {
        self.ensure_storage()?;
        let document = Indexer::analyze(path)?;
        let written = Indexer::write_document(&self.config.index_dir(), &document)?;
        self.publish_index_updated(
            "entity",
            &document.overview.path,
            serde_json::json!({
                "output": written,
                "components": document.components.len(),
                "relations": document.relations.len(),
                "history": document.history.len(),
            }),
        )?;
        Ok((document, written))
    }

    pub fn index_and_store_files(
        &self,
        root: &Path,
        options: FileIndexOptions,
    ) -> Result<(FileIndex, PathBuf), CoreError> {
        self.ensure_storage()?;
        let index = Indexer::index_files(root, options)?;
        let written = Indexer::write_file_index(&self.config.index_dir(), &index)?;
        self.register_local_content_actions()?;
        self.publish_index_updated(
            "files",
            &index.root,
            serde_json::json!({
                "output": written,
                "entries": index.entries.len(),
            }),
        )?;
        Ok((index, written))
    }

    fn publish_index_updated(
        &self,
        kind: &str,
        path: &Path,
        details: serde_json::Value,
    ) -> Result<(), CoreError> {
        self.publish(StdEvent::new(
            StdEventType::IndexUpdated,
            "std-core",
            serde_json::json!({
                "kind": kind,
                "path": path,
                "details": details,
            }),
        ))
    }
}
