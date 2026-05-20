use std_core::{check_plugin_manifest, discover_plugin_manifests, PluginCheckReport, StdCore};
use std_types::{ActionExecution, ActionPreview, SearchResult};

#[derive(Debug, Clone, PartialEq)]
pub struct PluginManagerViewModel {
    pub manifest_paths: Vec<std::path::PathBuf>,
    pub plugin_actions: Vec<SearchResult>,
    pub selected: usize,
    pub preview: Option<ActionPreview>,
    pub last_execution: Option<ActionExecution>,
    pub check_reports: Vec<PluginCheckReport>,
}

impl PluginManagerViewModel {
    pub fn load(core: &StdCore) -> Self {
        let manifest_paths =
            discover_plugin_manifests(&core.config.plugins_dir()).unwrap_or_default();
        let check_reports = manifest_paths
            .iter()
            .filter_map(|path| check_plugin_manifest(path).ok())
            .collect::<Vec<_>>();
        let plugin_actions = core.search("plugin", 100).unwrap_or_default();
        let selected = 0;
        let preview = plugin_actions
            .first()
            .and_then(|result| core.preview_action(result.action.id).ok());
        Self {
            manifest_paths,
            plugin_actions,
            selected,
            preview,
            last_execution: None,
            check_reports,
        }
    }

    pub fn search(&mut self, core: &StdCore, query: &str) {
        self.plugin_actions = core.search(query, 100).unwrap_or_default();
        self.plugin_actions.retain(|result| {
            result.action.name.contains("Plugin")
                || result.action.description.to_lowercase().contains("plugin")
                || result.matched_fields.iter().any(|field| field == "tags")
        });
        self.selected = 0;
        self.refresh_preview(core);
    }

    pub fn refresh_preview(&mut self, core: &StdCore) -> Option<ActionPreview> {
        let action_id = self.plugin_actions.get(self.selected)?.action.id;
        let preview = core.preview_action(action_id).ok()?;
        self.preview = Some(preview.clone());
        Some(preview)
    }

    pub fn run_selected(&mut self, core: &StdCore) -> Option<ActionExecution> {
        let action = self.plugin_actions.get(self.selected)?.action.clone();
        let execution = core.execute_action(action.id).ok()?;
        self.last_execution = Some(execution.clone());
        Some(execution)
    }
}
