use crate::{CoreError, StdCore};
use std_types::{ActionExecution, ActionExecutionStatus, ActionPreview, ActionType, RegistryEntry};

pub(crate) fn action_preview(entry: &RegistryEntry) -> ActionPreview {
    ActionPreview {
        action_id: entry.action.id,
        title: entry.action.name.clone(),
        subtitle: entry.action.description.clone(),
        action_type: entry.action.action_type.clone(),
        primary_command: primary_command(entry),
        metadata: entry.metadata.clone(),
        examples: entry.action.examples.clone(),
    }
}

pub(crate) fn execute_registry_entry(
    core: &StdCore,
    entry: &RegistryEntry,
    allow_external_runner: bool,
) -> Result<ActionExecution, CoreError> {
    let now = chrono::Utc::now();
    match &entry.action.action_type {
        ActionType::Command if entry.action.name == "Echo" => execute_echo(core, entry, now),
        ActionType::Command if entry.metadata.contains_key("plugin") => {
            execute_plugin(core, entry, now)
        }
        ActionType::Clipboard => Ok(completed_json(
            entry,
            entry.action.description.clone(),
            serde_json::json!({
                "clipboard": entry.action.description,
                "source": entry.metadata.get("clipboard_id"),
            }),
            now,
        )),
        ActionType::Skill => Ok(completed_json(
            entry,
            entry.action.description.clone(),
            serde_json::json!({
                "memory_id": entry.metadata.get("memory_id"),
                "scope": entry.metadata.get("scope"),
            }),
            now,
        )),
        ActionType::Command if entry.action.name == "Rebuild Index" => {
            rebuild_current_index(core, entry, now)
        }
        ActionType::Command => execute_command_action(core, entry, allow_external_runner, now),
        ActionType::AppLaunch if user_desktop_open_allowed(allow_external_runner) => {
            execute_app_launch(core, entry, now)
        }
        ActionType::AppLaunch => Ok(needs_external_runner(entry, now)),
        ActionType::Custom(kind) if kind == "file" => match entry.metadata.get("path") {
            Some(path) if user_desktop_open_allowed(allow_external_runner) => {
                Ok(run_open_path(core, entry, path, now))
            }
            Some(_) => Ok(needs_external_runner(entry, now)),
            None => Ok(needs_external_runner(entry, now)),
        },
        _ => Ok(needs_external_runner(entry, now)),
    }
}

fn execute_command_action(
    core: &StdCore,
    entry: &RegistryEntry,
    allow_external_runner: bool,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<ActionExecution, CoreError> {
    match entry.metadata.get("command") {
        Some(_) if !external_runner_allowed(allow_external_runner) => {
            Ok(needs_external_runner(entry, created_at))
        }
        Some(command) => Ok(run_shell_command(core, entry, command, created_at)),
        None => Ok(needs_external_runner(entry, created_at)),
    }
}

fn external_runner_allowed(allow_external_runner: bool) -> bool {
    allow_external_runner && crate::desktop_automation_allowed()
}

fn user_desktop_open_allowed(allow_external_runner: bool) -> bool {
    allow_external_runner && !crate::std_test_mode_enabled()
}

fn execute_echo(
    core: &StdCore,
    entry: &RegistryEntry,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<ActionExecution, CoreError> {
    let output = core.execute_tool("Echo", serde_json::json!({}))?;
    Ok(completed_json(
        entry,
        "tool executed".to_string(),
        output,
        created_at,
    ))
}

fn execute_plugin(
    core: &StdCore,
    entry: &RegistryEntry,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<ActionExecution, CoreError> {
    let output = match core.execute_tool(&entry.action.name, serde_json::json!({})) {
        Ok(output) => output,
        Err(error) => {
            return Ok(ActionExecution {
                action_id: entry.action.id,
                action_name: entry.action.name.clone(),
                status: ActionExecutionStatus::Failed,
                message: error.to_string(),
                output: Some(serde_json::json!({ "error": error.to_string() })),
                created_at,
            });
        }
    };
    let failed = output
        .get("timed_out")
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
        || output
            .get("exit_code")
            .and_then(|value| value.as_i64())
            .map(|code| code != 0)
            .unwrap_or(false);
    Ok(ActionExecution {
        action_id: entry.action.id,
        action_name: entry.action.name.clone(),
        status: if failed {
            ActionExecutionStatus::Failed
        } else {
            ActionExecutionStatus::Completed
        },
        message: plugin_message(&output, failed),
        output: Some(output),
        created_at,
    })
}

fn plugin_message(output: &serde_json::Value, failed: bool) -> String {
    if failed {
        output
            .get("stderr")
            .and_then(|value| value.as_str())
            .unwrap_or("plugin failed")
            .to_string()
    } else {
        "plugin executed".to_string()
    }
}

fn execute_app_launch(
    core: &StdCore,
    entry: &RegistryEntry,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<ActionExecution, CoreError> {
    match entry.metadata.get("path") {
        Some(path) => Ok(run_open_path(core, entry, path, created_at)),
        None if entry.action.name == "Open Terminal" => Ok(run_shell_command(
            core,
            entry,
            "open -a Terminal",
            created_at,
        )),
        None => Ok(needs_external_runner(entry, created_at)),
    }
}

fn rebuild_current_index(
    core: &StdCore,
    entry: &RegistryEntry,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<ActionExecution, CoreError> {
    let root = std::env::current_dir()?;
    let (document, output_path) = core.analyze_and_store_entity(&root)?;
    Ok(ActionExecution {
        action_id: entry.action.id,
        action_name: entry.action.name.clone(),
        status: ActionExecutionStatus::Completed,
        message: format!("index rebuilt: {}", output_path.display()),
        output: Some(serde_json::json!({
            "entity": document.overview.path,
            "components": document.components.len(),
            "relations": document.relations.len(),
            "output": output_path,
        })),
        created_at,
    })
}

fn completed_json(
    entry: &RegistryEntry,
    message: String,
    output: serde_json::Value,
    created_at: chrono::DateTime<chrono::Utc>,
) -> ActionExecution {
    ActionExecution {
        action_id: entry.action.id,
        action_name: entry.action.name.clone(),
        status: ActionExecutionStatus::Completed,
        message,
        output: Some(output),
        created_at,
    }
}

fn needs_external_runner(
    entry: &RegistryEntry,
    created_at: chrono::DateTime<chrono::Utc>,
) -> ActionExecution {
    let command = primary_command(entry);
    ActionExecution {
        action_id: entry.action.id,
        action_name: entry.action.name.clone(),
        status: ActionExecutionStatus::NeedsExternalRunner,
        message: command,
        output: Some(serde_json::json!({
            "deferred": true,
            "reason": "external runner action requires explicit user trigger",
        })),
        created_at,
    }
}

fn primary_command(entry: &RegistryEntry) -> String {
    match &entry.action.action_type {
        ActionType::Workflow => entry
            .metadata
            .get("path")
            .map(|path| format!("std run {path}"))
            .unwrap_or_else(|| format!("std run {}", entry.action.name)),
        ActionType::Clipboard => entry.action.description.clone(),
        ActionType::AppLaunch => entry
            .metadata
            .get("path")
            .map(|path| format!("open {path}"))
            .unwrap_or_else(|| entry.action.name.clone()),
        ActionType::Command => entry
            .metadata
            .get("command")
            .cloned()
            .or_else(|| entry.action.examples.first().cloned())
            .unwrap_or_else(|| entry.action.name.clone()),
        ActionType::Skill => entry
            .action
            .examples
            .first()
            .cloned()
            .unwrap_or_else(|| entry.action.description.clone()),
        ActionType::Custom(kind) if kind == "file" => entry
            .metadata
            .get("path")
            .map(|path| format!("open {path}"))
            .unwrap_or_else(|| entry.action.name.clone()),
        ActionType::Custom(_) => entry
            .action
            .examples
            .first()
            .cloned()
            .unwrap_or_else(|| entry.action.name.clone()),
    }
}

fn run_shell_command(
    core: &StdCore,
    entry: &RegistryEntry,
    command: &str,
    created_at: chrono::DateTime<chrono::Utc>,
) -> ActionExecution {
    let args = vec!["-c".to_string(), command.to_string()];
    let output = core.run_external_command("sh", &args);
    command_execution(entry, command, output, created_at)
}

fn run_open_path(
    core: &StdCore,
    entry: &RegistryEntry,
    path: &str,
    created_at: chrono::DateTime<chrono::Utc>,
) -> ActionExecution {
    let args = vec![path.to_string()];
    let output = core.run_external_command("open", &args);
    command_execution(entry, &format!("open {path}"), output, created_at)
}

fn command_execution(
    entry: &RegistryEntry,
    command: &str,
    output: Result<std::process::Output, std::io::Error>,
    created_at: chrono::DateTime<chrono::Utc>,
) -> ActionExecution {
    match output {
        Ok(output) => command_output_execution(entry, command, output, created_at),
        Err(error) => ActionExecution {
            action_id: entry.action.id,
            action_name: entry.action.name.clone(),
            status: ActionExecutionStatus::Failed,
            message: error.to_string(),
            output: Some(serde_json::json!({
                "command": command,
                "error": error.to_string(),
            })),
            created_at,
        },
    }
}

fn command_output_execution(
    entry: &RegistryEntry,
    command: &str,
    output: std::process::Output,
    created_at: chrono::DateTime<chrono::Utc>,
) -> ActionExecution {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let status = if output.status.success() {
        ActionExecutionStatus::Completed
    } else {
        ActionExecutionStatus::Failed
    };
    ActionExecution {
        action_id: entry.action.id,
        action_name: entry.action.name.clone(),
        status,
        message: if stderr.is_empty() {
            command.to_string()
        } else {
            stderr.clone()
        },
        output: Some(serde_json::json!({
            "command": command,
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
        })),
        created_at,
    }
}
