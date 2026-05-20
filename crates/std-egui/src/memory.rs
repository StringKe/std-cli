use std_core::StdCore;
use std_types::MemoryRecord;

#[derive(Debug, Clone, PartialEq)]
pub struct MemoryBrowserViewModel {
    pub query: String,
    pub memories: Vec<MemoryRecord>,
    pub selected: usize,
    pub draft_scope: String,
    pub draft_title: String,
    pub draft_body: String,
    pub draft_tags: Vec<String>,
    pub last_written: Option<MemoryRecord>,
}

impl MemoryBrowserViewModel {
    pub fn load(core: &StdCore) -> Self {
        Self {
            query: String::new(),
            memories: core.recall("", 100).unwrap_or_default(),
            selected: 0,
            draft_scope: "global".to_string(),
            draft_title: String::new(),
            draft_body: String::new(),
            draft_tags: Vec::new(),
            last_written: None,
        }
    }

    pub fn search(&mut self, core: &StdCore, query: impl Into<String>) {
        self.query = query.into();
        self.memories = core.recall(&self.query, 100).unwrap_or_default();
        self.selected = 0;
    }

    pub fn selected_memory(&self) -> Option<&MemoryRecord> {
        self.memories.get(self.selected)
    }

    pub fn select(&mut self, index: usize) {
        if index < self.memories.len() {
            self.selected = index;
        }
    }

    pub fn remember(
        &mut self,
        core: &StdCore,
        scope: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        tags: Vec<String>,
    ) -> Result<MemoryRecord, std_core::CoreError> {
        let memory = core.remember(scope, title, body, tags)?;
        core.register_local_content_actions()?;
        self.last_written = Some(memory.clone());
        self.search(core, "");
        Ok(memory)
    }
}
