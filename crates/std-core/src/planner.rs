mod context;

use crate::{CoreError, EventBus, StdCore};
use chrono::Utc;
use context::PlannerContext;
use std_types::{AiPlan, PlanStep, StdEvent, StdEventType};

pub struct AiPlanner;

impl AiPlanner {
    pub fn plan(core: &StdCore, goal: &str) -> Result<AiPlan, CoreError> {
        let context = PlannerContext::load(core, goal)?;
        let mut results = core.search(goal, 5)?;
        if results.is_empty() {
            results = core.search("", 5)?;
        }

        let steps = results
            .into_iter()
            .map(|result| PlanStep {
                action_id: Some(result.action.id),
                action_name: result.action.name.clone(),
                reason: format!(
                    "Matched fields: {}; context: {}",
                    result.matched_fields.join(","),
                    context.summary()
                ),
                parameters: context.parameters(),
                evidence: context.evidence_for_action(&result.action.name, &result.matched_fields),
            })
            .collect::<Vec<_>>();

        let plan = AiPlan {
            goal: goal.to_string(),
            steps,
            created_at: Utc::now(),
        };

        core.publish(StdEvent::new(
            StdEventType::AiPlannerProducedPlan,
            "std-core",
            serde_json::json!({
                "goal": goal,
                "step_count": plan.steps.len(),
            }),
        ))?;

        Ok(plan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::StdConfig;
    use std_index::Indexer;

    #[test]
    fn planner_uses_registered_actions() {
        let temp = tempfile::tempdir().unwrap();
        let core = StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        });
        core.seed_builtin_actions().unwrap();

        let plan = AiPlanner::plan(&core, "terminal").unwrap();

        assert_eq!(plan.goal, "terminal");
        assert_eq!(plan.steps[0].action_name, "StdFixtureTerminal");
        assert!(plan.steps[0].action_id.is_some());
        assert!(plan.steps[0]
            .evidence
            .contains(&"action: StdFixtureTerminal".to_string()));
    }

    #[test]
    fn planner_includes_local_context_in_step_parameters() {
        let temp = tempfile::tempdir().unwrap();
        let project_dir = temp.path().join("project");
        std::fs::create_dir_all(project_dir.join("src")).unwrap();
        std::fs::write(
            project_dir.join("src").join("main.rs"),
            "fn main() {}\npub struct WorkflowContext {}\n",
        )
        .unwrap();
        let core = StdCore::with_config(StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        });
        core.seed_builtin_actions().unwrap();
        core.remember(
            "project",
            "Workflow rule",
            "Use std run for workflows",
            vec!["workflow".to_string()],
        )
        .unwrap();
        core.capture_clipboard("cargo test workflow", "test")
            .unwrap();
        let document = Indexer::analyze(&project_dir).unwrap();
        Indexer::write_document(&core.config.index_dir(), &document).unwrap();

        let plan = AiPlanner::plan(&core, "workflow").unwrap();
        let context = &plan.steps[0].parameters["context"];

        assert!(plan.steps[0].reason.contains("context:"));
        assert!(plan.steps[0]
            .evidence
            .contains(&"memory: Workflow rule".to_string()));
        assert!(plan.steps[0]
            .evidence
            .contains(&"clipboard: cargo test workflow".to_string()));
        assert!(plan.steps[0]
            .evidence
            .contains(&"index: project".to_string()));
        assert_eq!(context["memory_titles"][0].as_str(), Some("Workflow rule"));
        assert_eq!(
            context["clipboard_items"][0].as_str(),
            Some("cargo test workflow")
        );
        assert_eq!(context["indexed_entities"][0].as_str(), Some("project"));
        assert!(context["workflow_actions"]
            .as_array()
            .unwrap()
            .iter()
            .all(|value| value.as_str().unwrap().starts_with("Run Workflow:")));
    }
}
