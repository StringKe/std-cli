//! std-types - Core data types for std-cli
//!
//! This crate contains all public, immutable data structures.
//! It is completely GUI-neutral and has no heavy dependencies.
//!
//! Shared product types for Action, Workflow, Memory, and cross-surface events.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type ActionId = Uuid;
pub type WorkflowId = Uuid;
pub type MemoryId = Uuid;
pub type EventId = Uuid;
pub type ClipboardId = Uuid;

/// The smallest triggerable unit visible to the user in Launcher or Studio.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Action {
    pub id: ActionId,
    pub name: String,
    pub description: String,
    pub when_to_use: String,
    pub action_type: ActionType,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
    pub examples: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Action {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        when_to_use: impl Into<String>,
        action_type: ActionType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            when_to_use: when_to_use.into(),
            action_type,
            input_schema: None,
            output_schema: None,
            examples: vec![],
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ActionType {
    AppLaunch,
    Workflow,
    Command,
    Skill,
    Memory,
    Clipboard,
    Custom(String),
}

impl ActionType {
    pub fn needs_external_runner(&self) -> bool {
        match self {
            ActionType::AppLaunch => true,
            ActionType::Custom(kind) => kind == "file",
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub when_to_use: String,
    pub input_schema: Option<serde_json::Value>,
    pub output_schema: Option<serde_json::Value>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandTemplate {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub template: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryRecord {
    pub id: MemoryId,
    pub scope: String,
    pub title: String,
    pub body: String,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClipboardRecord {
    pub id: ClipboardId,
    pub content: String,
    pub source: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StdEvent {
    pub id: EventId,
    pub event_type: StdEventType,
    pub source: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

impl StdEvent {
    pub fn new(
        event_type: StdEventType,
        source: impl Into<String>,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_type,
            source: source.into(),
            payload,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StdEventType {
    RegistryChanged,
    ActionPreviewed,
    ActionExecuted,
    WorkflowStarted,
    WorkflowStepCompleted,
    WorkflowCompleted,
    WorkflowFailed,
    IndexUpdated,
    AiPlannerProducedPlan,
    UiInteraction,
    MemoryWritten,
    ToolExecuted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanStep {
    pub action_id: Option<ActionId>,
    pub action_name: String,
    pub reason: String,
    pub parameters: serde_json::Value,
    #[serde(default)]
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiPlan {
    pub goal: String,
    pub steps: Vec<PlanStep>,
    pub created_at: DateTime<Utc>,
}

/// Lightweight search result used by Launcher.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    pub action: Action,
    pub score: f32,
    pub matched_fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionPreview {
    pub action_id: ActionId,
    pub title: String,
    pub subtitle: String,
    pub action_type: ActionType,
    pub primary_command: String,
    pub metadata: HashMap<String, String>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ActionExecutionStatus {
    Completed,
    Failed,
    NeedsExternalRunner,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActionExecution {
    pub action_id: ActionId,
    pub action_name: String,
    pub status: ActionExecutionStatus,
    pub message: String,
    pub output: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// Registry entry for quick lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub action: Action,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl RegistryEntry {
    pub fn from_action(action: Action, tags: Vec<String>) -> Self {
        Self {
            action,
            tags,
            metadata: HashMap::new(),
        }
    }

    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn action_can_be_serialized() {
        let mut action = Action::new(
            "StdFixtureTerminal",
            "Launch the default terminal",
            "When you need a shell",
            ActionType::AppLaunch,
        );
        action.examples.push("std-fixture-terminal".to_string());

        let json = serde_json::to_string(&action).unwrap();
        let deserialized: Action = serde_json::from_str(&json).unwrap();
        assert_eq!(action, deserialized);
    }

    proptest! {
        #[test]
        fn action_name_is_never_empty(name in "\\PC*") {
            if !name.is_empty() {
                let action = Action {
                    id: Uuid::new_v4(),
                    name,
                    description: "test".to_string(),
                    when_to_use: "test".to_string(),
                    action_type: ActionType::Custom("test".into()),
                    input_schema: None,
                    output_schema: None,
                    examples: vec![],
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                assert!(!action.name.is_empty());
            }
        }
    }
}
