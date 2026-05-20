use crate::StudioApp;
use std::path::{Path, PathBuf};
use std_types::{ActionExecution, SearchResult};

impl StudioApp {
    pub fn registered_apps(&self) -> Result<Vec<PathBuf>, std_core::CoreError> {
        self.core.list_registered_apps()
    }

    pub fn register_app_bundle(&mut self, source: &Path) -> Result<PathBuf, std_core::CoreError> {
        let path = self.core.register_app_bundle(source)?;
        self.refresh();
        Ok(path)
    }

    pub fn search_apps(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>, std_core::CoreError> {
        Ok(self
            .core
            .search(query, limit)?
            .into_iter()
            .filter(|result| result.action.action_type == std_types::ActionType::AppLaunch)
            .collect())
    }

    pub fn preview_app(
        &self,
        query: &str,
    ) -> Result<Option<std_types::ActionPreview>, std_core::CoreError> {
        let Some(result) = self.search_apps(query, 1)?.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(self.core.preview_action(result.action.id)?))
    }

    pub fn trigger_app(&self, query: &str) -> Result<Option<ActionExecution>, std_core::CoreError> {
        let Some(result) = self.search_apps(query, 1)?.into_iter().next() else {
            return Ok(None);
        };
        Ok(Some(self.core.execute_action(result.action.id)?))
    }
}
