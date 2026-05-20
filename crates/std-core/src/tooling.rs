use crate::{events::EventBus, CoreError, StdCore};
use std_types::{StdEvent, StdEventType};

impl StdCore {
    pub fn execute_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value, CoreError> {
        let should_block_shell_plugin = self
            .registry
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .get_by_name(name)
            .map(|entry| {
                entry.metadata.contains_key("plugin")
                    && entry
                        .metadata
                        .get("plugin_kind")
                        .map(|kind| kind == "shell")
                        .unwrap_or(false)
                    && crate::std_test_mode_enabled()
            })
            .unwrap_or(false);
        if should_block_shell_plugin {
            return Err(CoreError::PluginPermissionDenied(
                "STD_TEST_MODE blocked shell plugin command".to_string(),
            ));
        }
        let output = self
            .tools
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .execute(name, args)?;
        self.publish(StdEvent::new(
            StdEventType::ToolExecuted,
            "std-core",
            serde_json::json!({
                "tool": name,
            }),
        ))?;
        Ok(output)
    }
}
