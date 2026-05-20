use std_core::{AiPlanner, StdCore};
use std_types::{AiPlan, MemoryRecord, StdEvent};

#[derive(Debug, Clone, PartialEq)]
pub struct StudioDashboardViewModel {
    pub action_count: usize,
    pub memory_count: usize,
    pub audit_event_count: usize,
    pub recent_memories: Vec<MemoryRecord>,
    pub recent_events: Vec<StdEvent>,
    pub suggested_plan: AiPlan,
}

impl StudioDashboardViewModel {
    pub fn load(core: &StdCore) -> Self {
        let action_count = core.search("", 1000).map(|items| items.len()).unwrap_or(0);
        let recent_memories = core.recall("", 10).unwrap_or_default();
        let recent_events = core.read_audit_events().unwrap_or_default();
        let suggested_plan = AiPlanner::plan(core, "rebuild index").unwrap_or_else(|_| AiPlan {
            goal: "rebuild index".to_string(),
            steps: Vec::new(),
            created_at: chrono::Utc::now(),
        });

        Self {
            action_count,
            memory_count: recent_memories.len(),
            audit_event_count: recent_events.len(),
            recent_memories,
            recent_events,
            suggested_plan,
        }
    }
}
