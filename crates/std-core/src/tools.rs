use crate::CoreError;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use std_types::{Action, ActionType, RegistryEntry};

pub trait StdTool: Send + Sync {
    fn action(&self) -> Action;
    fn tags(&self) -> Vec<String>;
    fn execute(&self, args: Value) -> Result<Value, CoreError>;
}

#[derive(Clone, Default)]
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn StdTool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<T>(&mut self, tool: T) -> RegistryEntry
    where
        T: StdTool + 'static,
    {
        let action = tool.action();
        let name = normalize_tool_name(&action.name);
        let tags = tool.tags();
        self.tools.insert(name, Arc::new(tool));
        RegistryEntry::from_action(action, tags)
    }

    pub fn register_boxed<T>(&mut self, name: String, tool: T)
    where
        T: StdTool + 'static,
    {
        self.tools
            .insert(normalize_tool_name(&name), Arc::new(tool));
    }

    pub fn execute(&self, name: &str, args: Value) -> Result<Value, CoreError> {
        let tool = self
            .tools
            .get(&normalize_tool_name(name))
            .ok_or_else(|| CoreError::ToolNotFound(name.to_string()))?;
        tool.execute(args)
    }

    pub fn len(&self) -> usize {
        self.tools.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }
}

pub struct EchoTool;

impl StdTool for EchoTool {
    fn action(&self) -> Action {
        Action::new(
            "Echo",
            "Return the provided JSON payload unchanged",
            "When validating tool execution and data flow",
            ActionType::Command,
        )
    }

    fn tags(&self) -> Vec<String> {
        vec!["tool".to_string(), "debug".to_string()]
    }

    fn execute(&self, args: Value) -> Result<Value, CoreError> {
        Ok(args)
    }
}

fn normalize_tool_name(name: &str) -> String {
    name.trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_registry_executes_registered_tool() {
        let mut registry = ToolRegistry::new();
        let entry = registry.register(EchoTool);

        let output = registry
            .execute(&entry.action.name, serde_json::json!({"ok": true}))
            .unwrap();

        assert_eq!(registry.len(), 1);
        assert_eq!(output, serde_json::json!({"ok": true}));
    }
}
