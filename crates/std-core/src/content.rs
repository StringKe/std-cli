use crate::{
    discovery::{clipboard_action, command_template_action, skill_action},
    events::EventBus,
    CoreError, StdCore,
};
use std_types::{ClipboardRecord, CommandTemplate, MemoryRecord, Skill, StdEvent, StdEventType};

impl StdCore {
    pub fn remember(
        &self,
        scope: impl Into<String>,
        title: impl Into<String>,
        body: impl Into<String>,
        tags: Vec<String>,
    ) -> Result<MemoryRecord, CoreError> {
        let now = chrono::Utc::now();
        let memory = MemoryRecord {
            id: uuid::Uuid::new_v4(),
            scope: scope.into(),
            title: title.into(),
            body: body.into(),
            tags,
            created_at: now,
            updated_at: now,
        };
        self.store.append_memory(&memory)?;
        self.publish(StdEvent::new(
            StdEventType::MemoryWritten,
            "std-core",
            serde_json::json!({
                "memory_id": memory.id,
                "title": memory.title,
                "scope": memory.scope,
            }),
        ))?;
        Ok(memory)
    }

    pub fn recall(&self, query: &str, limit: usize) -> Result<Vec<MemoryRecord>, CoreError> {
        let query = query.trim().to_lowercase();
        let mut memories = self.store.read_memories()?;
        if !query.is_empty() {
            memories.retain(|memory| {
                memory.title.to_lowercase().contains(&query)
                    || memory.body.to_lowercase().contains(&query)
                    || memory.scope.to_lowercase().contains(&query)
                    || memory
                        .tags
                        .iter()
                        .any(|tag| tag.to_lowercase().contains(&query))
            });
        }
        memories.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        memories.truncate(limit);
        Ok(memories)
    }

    pub fn capture_clipboard(
        &self,
        content: impl Into<String>,
        source: impl Into<String>,
    ) -> Result<ClipboardRecord, CoreError> {
        let record = ClipboardRecord {
            id: uuid::Uuid::new_v4(),
            content: content.into(),
            source: source.into(),
            created_at: chrono::Utc::now(),
        };
        self.store.append_clipboard(&record)?;
        self.register_action(clipboard_action(&record)?)?;
        Ok(record)
    }

    pub fn recall_clipboard(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<ClipboardRecord>, CoreError> {
        let query = query.trim().to_lowercase();
        let mut records = self.store.read_clipboard()?;
        if !query.is_empty() {
            records.retain(|record| {
                record.content.to_lowercase().contains(&query)
                    || record.source.to_lowercase().contains(&query)
            });
        }
        records.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        records.truncate(limit);
        Ok(records)
    }

    pub fn define_skill(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        when_to_use: impl Into<String>,
        examples: Vec<String>,
    ) -> Result<Skill, CoreError> {
        let skill = Skill {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            when_to_use: when_to_use.into(),
            input_schema: None,
            output_schema: None,
            examples,
        };
        self.store.append_skill(&skill)?;
        self.register_action(skill_action(&skill))?;
        Ok(skill)
    }

    pub fn list_skills(&self) -> Result<Vec<Skill>, CoreError> {
        let mut skills = self.store.read_skills()?;
        skills.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(skills)
    }

    pub fn define_command(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        template: impl Into<String>,
        examples: Vec<String>,
    ) -> Result<CommandTemplate, CoreError> {
        let command = CommandTemplate {
            id: uuid::Uuid::new_v4(),
            name: name.into(),
            description: description.into(),
            template: template.into(),
            examples,
        };
        self.store.append_command(&command)?;
        self.register_action(command_template_action(&command))?;
        Ok(command)
    }

    pub fn list_commands(&self) -> Result<Vec<CommandTemplate>, CoreError> {
        let mut commands = self.store.read_commands()?;
        commands.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(commands)
    }
}
