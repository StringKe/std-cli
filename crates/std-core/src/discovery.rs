use crate::{app_bundle, CoreError};
use std::{fs, path::Path};
use std_index::{FileIndexEntry, Indexer};
use std_types::{
    Action, ActionType, ClipboardRecord, CommandTemplate, MemoryRecord, RegistryEntry, Skill,
};

pub(crate) fn discover_workflow_actions(
    workflows_dir: &Path,
) -> Result<Vec<RegistryEntry>, CoreError> {
    if !workflows_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(workflows_dir)? {
        let path = entry?.path();
        let workflow_file = if path.is_dir() {
            if path.join("workflow.json").is_file() {
                path.join("workflow.json")
            } else if path.join("workflow.md").is_file() {
                path.join("workflow.md")
            } else {
                continue;
            }
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            path
        } else {
            continue;
        };

        let name = workflow_display_name(&workflow_file);
        let mut action = Action::new(
            format!("Run Workflow: {name}"),
            format!("Execute workflow definition at {}", workflow_file.display()),
            "When this saved workflow should run",
            ActionType::Workflow,
        );
        action
            .examples
            .push(format!("std run {}", workflow_file.display()));
        let mut registry_entry =
            RegistryEntry::from_action(action, vec!["workflow".to_string(), "local".to_string()]);
        registry_entry
            .metadata
            .insert("path".to_string(), workflow_file.display().to_string());
        entries.push(registry_entry);
    }
    Ok(entries)
}

pub(crate) fn discover_memory_actions(memories: &[MemoryRecord]) -> Vec<RegistryEntry> {
    memories
        .iter()
        .map(|memory| {
            let mut action = Action::new(
                format!("Memory: {}", memory.title),
                memory.body.clone(),
                "When recalling saved personal context",
                ActionType::Skill,
            );
            action
                .examples
                .push(format!("std memory recall {}", memory.title));
            let mut entry = RegistryEntry::from_action(action, memory.tags.clone());
            entry
                .metadata
                .insert("memory_id".to_string(), memory.id.to_string());
            entry
                .metadata
                .insert("scope".to_string(), memory.scope.clone());
            entry
        })
        .collect()
}

pub(crate) fn discover_skill_actions(skills: &[Skill]) -> Vec<RegistryEntry> {
    skills.iter().map(skill_action).collect()
}

pub(crate) fn discover_command_template_actions(
    commands: &[CommandTemplate],
) -> Vec<RegistryEntry> {
    commands.iter().map(command_template_action).collect()
}

pub(crate) fn discover_clipboard_actions(records: &[ClipboardRecord]) -> Vec<RegistryEntry> {
    records
        .iter()
        .filter_map(|record| clipboard_action(record).ok())
        .collect()
}

pub(crate) fn discover_indexed_file_actions(
    index_dir: &Path,
) -> Result<Vec<RegistryEntry>, CoreError> {
    let indexes = Indexer::read_file_indexes(index_dir)?;
    Ok(indexes
        .into_iter()
        .flat_map(|index| index.entries)
        .map(file_action)
        .collect())
}

pub(crate) fn discover_app_actions(local_apps_dir: &Path) -> Vec<RegistryEntry> {
    app_bundle::discover_app_actions(local_apps_dir)
}

fn workflow_display_name(workflow_file: &Path) -> String {
    fs::read_to_string(workflow_file)
        .ok()
        .and_then(
            |body| match workflow_file.extension().and_then(|ext| ext.to_str()) {
                Some("json") => serde_json::from_str::<serde_json::Value>(&body)
                    .ok()
                    .and_then(|value| {
                        value
                            .get("name")
                            .and_then(|name| name.as_str())
                            .map(ToString::to_string)
                    }),
                Some("md") => body.lines().find_map(|line| {
                    line.trim()
                        .strip_prefix("name:")
                        .map(trim_frontmatter_scalar)
                        .map(ToString::to_string)
                }),
                _ => None,
            },
        )
        .or_else(|| {
            workflow_file
                .parent()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .map(ToString::to_string)
        })
        .or_else(|| {
            workflow_file
                .file_stem()
                .and_then(|name| name.to_str())
                .map(ToString::to_string)
        })
        .unwrap_or_else(|| "workflow".to_string())
}

fn trim_frontmatter_scalar(value: &str) -> &str {
    value.trim().trim_matches('"').trim_matches('\'').trim()
}

pub(crate) fn skill_action(skill: &Skill) -> RegistryEntry {
    let mut action = Action::new(
        format!("Skill: {}", skill.name),
        skill.description.clone(),
        skill.when_to_use.clone(),
        ActionType::Skill,
    );
    action.examples = skill.examples.clone();
    let mut entry =
        RegistryEntry::from_action(action, vec!["skill".to_string(), skill.name.clone()]);
    entry
        .metadata
        .insert("skill_id".to_string(), skill.id.to_string());
    entry
}

pub(crate) fn command_template_action(command: &CommandTemplate) -> RegistryEntry {
    let mut action = Action::new(
        format!("Command: {}", command.name),
        command.description.clone(),
        "When running this saved command template",
        ActionType::Command,
    );
    action.examples = if command.examples.is_empty() {
        vec![command.template.clone()]
    } else {
        command.examples.clone()
    };
    let mut entry = RegistryEntry::from_action(
        action,
        vec![
            "command".to_string(),
            "template".to_string(),
            command.name.clone(),
        ],
    );
    entry
        .metadata
        .insert("command_id".to_string(), command.id.to_string());
    entry
        .metadata
        .insert("command".to_string(), command.template.clone());
    entry
}

fn file_action(entry: FileIndexEntry) -> RegistryEntry {
    let mut action = Action::new(
        format!("Open File: {}", entry.name),
        if entry.snippet.is_empty() {
            format!("Open indexed file at {}", entry.path.display())
        } else {
            entry.snippet.clone()
        },
        "When opening or referencing an indexed local file",
        ActionType::Custom("file".to_string()),
    );
    action
        .examples
        .push(format!("open {}", entry.path.display()));
    let mut registry_entry = RegistryEntry::from_action(
        action,
        vec![
            "file".to_string(),
            "indexed".to_string(),
            entry.name.clone(),
        ],
    );
    registry_entry
        .metadata
        .insert("path".to_string(), entry.path.display().to_string());
    registry_entry
}

pub(crate) fn clipboard_action(record: &ClipboardRecord) -> Result<RegistryEntry, CoreError> {
    let preview = record
        .content
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(48)
        .collect::<String>();
    let mut action = Action::new(
        format!("Clipboard: {preview}"),
        record.content.clone(),
        "When reusing clipboard history",
        ActionType::Clipboard,
    );
    action.examples.push("std clipboard recall".to_string());
    let mut entry =
        RegistryEntry::from_action(action, vec!["clipboard".to_string(), record.source.clone()]);
    entry
        .metadata
        .insert("clipboard_id".to_string(), record.id.to_string());
    Ok(entry)
}
