use crate::{
    discovery::{
        discover_app_actions, discover_clipboard_actions, discover_command_template_actions,
        discover_indexed_file_actions, discover_memory_actions, discover_skill_actions,
        discover_workflow_actions,
    },
    plugins, CoreError, EchoTool, PluginHostData, StdCore,
};
use std_types::{Action, ActionType, RegistryEntry};

impl StdCore {
    pub fn seed_builtin_actions(&self) -> Result<(), CoreError> {
        let echo_entry = self
            .tools
            .write()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .register(EchoTool);
        self.register_action_if_missing(echo_entry)?;
        self.register_action_if_missing(terminal_action())?;
        self.register_action_if_missing(rebuild_index_action())?;
        self.register_plugin_tools()?;
        self.register_local_content_actions()
    }

    pub fn register_plugin_tools(&self) -> Result<(), CoreError> {
        let host_data = PluginHostData {
            clipboard: self.recall_clipboard("", usize::MAX)?,
        };
        for plugin_tool in
            plugins::load_plugin_tools_with_host(&self.config.plugins_dir(), host_data)?
        {
            let entry = plugin_tool.registry_entry();
            let action_name = entry.action.name.clone();
            self.tools
                .write()
                .map_err(|_| CoreError::RegistryLockPoisoned)?
                .register_boxed(action_name, plugin_tool);
            self.register_action_if_missing(entry)?;
        }
        Ok(())
    }

    pub fn register_local_content_actions(&self) -> Result<(), CoreError> {
        for entry in self.discover_local_content_actions()? {
            self.register_action_if_missing(entry)?;
        }
        Ok(())
    }

    pub fn discover_local_content_actions(&self) -> Result<Vec<RegistryEntry>, CoreError> {
        let mut entries = Vec::new();
        entries.extend(discover_workflow_actions(&self.config.workflows_dir())?);
        entries.extend(discover_skill_actions(&self.store.read_skills()?));
        entries.extend(discover_command_template_actions(
            &self.store.read_commands()?,
        ));
        entries.extend(discover_memory_actions(&self.store.read_memories()?));
        entries.extend(discover_clipboard_actions(&self.store.read_clipboard()?));
        entries.extend(discover_indexed_file_actions(&self.config.index_dir())?);
        entries.extend(discover_app_actions(&self.config.apps_dir()));
        Ok(entries)
    }
}

fn terminal_action() -> RegistryEntry {
    let mut action = Action::new(
        "Open Terminal",
        "Launch macOS Terminal",
        "When a shell is needed",
        ActionType::AppLaunch,
    );
    action.examples.push("open -a Terminal".to_string());
    RegistryEntry::from_action(action, vec!["terminal".to_string(), "app".to_string()])
}

fn rebuild_index_action() -> RegistryEntry {
    let mut action = Action::new(
        "Rebuild Index",
        "Refresh local project, workflow, memory, and history indexes",
        "When search or analysis data is stale",
        ActionType::Command,
    );
    action.examples.push("std index rebuild .".to_string());
    RegistryEntry::from_action(action, vec!["index".to_string(), "maintenance".to_string()])
        .with_metadata("command", "std index rebuild .")
}
