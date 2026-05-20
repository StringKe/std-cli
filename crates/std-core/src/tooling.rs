use crate::{events::EventBus, CoreError, StdCore};
use std_types::{StdEvent, StdEventType};

impl StdCore {
    pub fn execute_tool(
        &self,
        name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value, CoreError> {
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
