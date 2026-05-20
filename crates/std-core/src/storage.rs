use crate::{CoreError, StdConfig};
use chrono::Utc;
use std::{
    fs::{self, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};
use std_types::{ClipboardRecord, CommandTemplate, MemoryRecord, Skill, StdEvent};

#[derive(Debug, Clone)]
pub struct LocalStore {
    config: StdConfig,
}

impl LocalStore {
    pub fn new(config: StdConfig) -> Self {
        Self { config }
    }

    pub fn ensure_dirs(&self) -> Result<(), CoreError> {
        for path in [
            self.config.data_dir.clone(),
            self.config.workflows_dir(),
            self.config.index_dir(),
            self.config.memory_dir(),
            self.config.history_dir(),
            self.config.plugins_dir(),
            self.config.apps_dir(),
        ] {
            fs::create_dir_all(&path)?;
        }
        Ok(())
    }

    pub fn append_event(&self, event: &StdEvent) -> Result<(), CoreError> {
        self.ensure_dirs()?;
        let path = self.audit_log_path();
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        let mut line = serde_json::to_vec(event)?;
        line.push(b'\n');
        file.write_all(&line)?;
        Ok(())
    }

    pub fn read_events(&self) -> Result<Vec<StdEvent>, CoreError> {
        let path = self.audit_log_path();
        if !path.is_file() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut events = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            for event in serde_json::Deserializer::from_str(&line).into_iter::<StdEvent>() {
                match event {
                    Ok(event) => events.push(event),
                    Err(_) => break,
                }
            }
        }
        Ok(events)
    }

    pub fn append_memory(&self, memory: &MemoryRecord) -> Result<(), CoreError> {
        self.ensure_dirs()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.memory_log_path())?;
        let line = serde_json::to_string(memory)?;
        writeln!(file, "{line}")?;
        Ok(())
    }

    pub fn read_memories(&self) -> Result<Vec<MemoryRecord>, CoreError> {
        let path = self.memory_log_path();
        if !path.is_file() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut memories = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            memories.push(serde_json::from_str(&line)?);
        }
        Ok(memories)
    }

    pub fn append_clipboard(&self, record: &ClipboardRecord) -> Result<(), CoreError> {
        self.ensure_dirs()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.clipboard_log_path())?;
        let line = serde_json::to_string(record)?;
        writeln!(file, "{line}")?;
        Ok(())
    }

    pub fn append_skill(&self, skill: &Skill) -> Result<(), CoreError> {
        self.ensure_dirs()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.skills_log_path())?;
        let line = serde_json::to_string(skill)?;
        writeln!(file, "{line}")?;
        Ok(())
    }

    pub fn read_skills(&self) -> Result<Vec<Skill>, CoreError> {
        let path = self.skills_log_path();
        if !path.is_file() {
            return Ok(Vec::new());
        }
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut skills = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            skills.push(serde_json::from_str(&line)?);
        }
        Ok(skills)
    }

    pub fn append_command(&self, command: &CommandTemplate) -> Result<(), CoreError> {
        self.ensure_dirs()?;
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.commands_log_path())?;
        let line = serde_json::to_string(command)?;
        writeln!(file, "{line}")?;
        Ok(())
    }

    pub fn read_commands(&self) -> Result<Vec<CommandTemplate>, CoreError> {
        let path = self.commands_log_path();
        if !path.is_file() {
            return Ok(Vec::new());
        }
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut commands = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            commands.push(serde_json::from_str(&line)?);
        }
        Ok(commands)
    }

    pub fn read_clipboard(&self) -> Result<Vec<ClipboardRecord>, CoreError> {
        let path = self.clipboard_log_path();
        if !path.is_file() {
            return Ok(Vec::new());
        }

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let mut records = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                continue;
            }
            records.push(serde_json::from_str(&line)?);
        }
        Ok(records)
    }

    pub fn audit_log_path(&self) -> PathBuf {
        self.config
            .history_dir()
            .join(format!("audit-{}.jsonl", Utc::now().format("%Y-%m-%d")))
    }

    pub fn memory_log_path(&self) -> PathBuf {
        self.config.memory_dir().join("memory.jsonl")
    }

    pub fn skills_log_path(&self) -> PathBuf {
        self.config.memory_dir().join("skills.jsonl")
    }

    pub fn commands_log_path(&self) -> PathBuf {
        self.config.memory_dir().join("commands.jsonl")
    }

    pub fn clipboard_log_path(&self) -> PathBuf {
        self.config.history_dir().join("clipboard.jsonl")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_store_creates_dirs_and_persists_events() {
        let temp = tempfile::tempdir().unwrap();
        let config = StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        };
        let store = LocalStore::new(config);
        let event = StdEvent::new(
            std_types::StdEventType::RegistryChanged,
            "test",
            serde_json::json!({"name": "Open Terminal"}),
        );

        store.append_event(&event).unwrap();
        let events = store.read_events().unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].id, event.id);
        assert!(store.audit_log_path().is_file());
    }

    #[test]
    fn local_store_reads_concatenated_audit_events() {
        let temp = tempfile::tempdir().unwrap();
        let config = StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        };
        let store = LocalStore::new(config);
        store.ensure_dirs().unwrap();
        let event_a = StdEvent::new(
            std_types::StdEventType::RegistryChanged,
            "test",
            serde_json::json!({"name": "A"}),
        );
        let event_b = StdEvent::new(
            std_types::StdEventType::ToolExecuted,
            "test",
            serde_json::json!({"name": "B"}),
        );
        let body = format!(
            "{}{}\n",
            serde_json::to_string(&event_a).unwrap(),
            serde_json::to_string(&event_b).unwrap()
        );
        fs::write(store.audit_log_path(), body).unwrap();

        let events = store.read_events().unwrap();

        assert_eq!(events.len(), 2);
        assert_eq!(events[0].id, event_a.id);
        assert_eq!(events[1].id, event_b.id);
    }

    #[test]
    fn local_store_persists_memories() {
        let temp = tempfile::tempdir().unwrap();
        let config = StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        };
        let store = LocalStore::new(config);
        let now = Utc::now();
        let memory = MemoryRecord {
            id: uuid::Uuid::new_v4(),
            scope: "global".to_string(),
            title: "Tool rule".to_string(),
            body: "Use std run for Workflow execution".to_string(),
            tags: vec!["workflow".to_string()],
            created_at: now,
            updated_at: now,
        };

        store.append_memory(&memory).unwrap();
        let memories = store.read_memories().unwrap();

        assert_eq!(memories.len(), 1);
        assert_eq!(memories[0].title, "Tool rule");
    }

    #[test]
    fn local_store_persists_clipboard_history() {
        let temp = tempfile::tempdir().unwrap();
        let config = StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        };
        let store = LocalStore::new(config);
        let record = ClipboardRecord {
            id: uuid::Uuid::new_v4(),
            content: "cargo test".to_string(),
            source: "test".to_string(),
            created_at: Utc::now(),
        };

        store.append_clipboard(&record).unwrap();
        let records = store.read_clipboard().unwrap();

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].content, "cargo test");
    }

    #[test]
    fn local_store_persists_skills_and_commands() {
        let temp = tempfile::tempdir().unwrap();
        let config = StdConfig {
            data_dir: temp.path().join("data"),
            ..StdConfig::default()
        };
        let store = LocalStore::new(config);
        let skill = Skill {
            id: uuid::Uuid::new_v4(),
            name: "Summarize Diff".to_string(),
            description: "Summarize a git diff".to_string(),
            when_to_use: "When preparing review notes".to_string(),
            input_schema: None,
            output_schema: None,
            examples: vec!["std skill run Summarize Diff".to_string()],
        };
        let command = CommandTemplate {
            id: uuid::Uuid::new_v4(),
            name: "List Rust Tests".to_string(),
            description: "List Rust tests".to_string(),
            template: "cargo test -- --list".to_string(),
            examples: vec!["cargo test -- --list".to_string()],
        };

        store.append_skill(&skill).unwrap();
        store.append_command(&command).unwrap();
        let skills = store.read_skills().unwrap();
        let commands = store.read_commands().unwrap();

        assert_eq!(skills[0].name, "Summarize Diff");
        assert_eq!(commands[0].template, "cargo test -- --list");
    }
}
