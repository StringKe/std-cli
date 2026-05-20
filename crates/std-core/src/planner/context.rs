use crate::{CoreError, StdCore};
use std_index::Indexer;
use std_types::ActionType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PlannerContext {
    memory_titles: Vec<String>,
    clipboard_items: Vec<String>,
    indexed_entities: Vec<String>,
    workflow_actions: Vec<String>,
}

impl PlannerContext {
    pub(crate) fn load(core: &StdCore, goal: &str) -> Result<Self, CoreError> {
        let memory_titles = core
            .recall(goal, 3)?
            .into_iter()
            .map(|memory| memory.title)
            .collect::<Vec<_>>();
        let clipboard_items = core
            .recall_clipboard(goal, 3)?
            .into_iter()
            .map(|record| record.content.chars().take(80).collect::<String>())
            .collect::<Vec<_>>();
        let indexed_entities = Indexer::search_documents(&core.config.index_dir(), goal, 3)?
            .into_iter()
            .map(|result| result.document.overview.name)
            .collect::<Vec<_>>();
        let workflow_actions = core
            .search("Run Workflow", 5)?
            .into_iter()
            .filter(|result| result.action.action_type == ActionType::Workflow)
            .map(|result| result.action.name)
            .collect::<Vec<_>>();

        Ok(Self {
            memory_titles,
            clipboard_items,
            indexed_entities,
            workflow_actions,
        })
    }

    pub(crate) fn summary(&self) -> String {
        format!(
            "memories={}, clipboard={}, indexed_entities={}, workflows={}",
            self.memory_titles.len(),
            self.clipboard_items.len(),
            self.indexed_entities.len(),
            self.workflow_actions.len()
        )
    }

    pub(crate) fn parameters(&self) -> serde_json::Value {
        serde_json::json!({
            "context": {
                "memory_titles": self.memory_titles,
                "clipboard_items": self.clipboard_items,
                "indexed_entities": self.indexed_entities,
                "workflow_actions": self.workflow_actions,
            }
        })
    }

    pub(crate) fn evidence_for_action(
        &self,
        action_name: &str,
        matched_fields: &[String],
    ) -> Vec<String> {
        let mut evidence = Vec::new();
        evidence.push(format!("action: {action_name}"));
        if !matched_fields.is_empty() {
            evidence.push(format!("matched_fields: {}", matched_fields.join(",")));
        }
        evidence.extend(
            self.memory_titles
                .iter()
                .map(|title| format!("memory: {title}")),
        );
        evidence.extend(
            self.clipboard_items
                .iter()
                .map(|item| format!("clipboard: {item}")),
        );
        evidence.extend(
            self.indexed_entities
                .iter()
                .map(|entity| format!("index: {entity}")),
        );
        evidence.extend(
            self.workflow_actions
                .iter()
                .map(|workflow| format!("workflow: {workflow}")),
        );
        evidence.truncate(12);
        evidence
    }
}
