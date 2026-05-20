use crate::{trigger_action, CliError};
use clap::Subcommand;
use std::path::{Path, PathBuf};
use std_core::{check_plugin_manifest, discover_plugin_manifests, StdCore};

#[derive(Debug, Subcommand)]
pub enum ToolCommand {
    Run {
        name: String,
        #[arg(default_value = "{}")]
        json: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum PluginCommand {
    List,
    Check { path: PathBuf },
    Run { query: String },
}

#[derive(Debug, Subcommand)]
pub enum AppCommand {
    Register { path: PathBuf },
    List,
}

#[derive(Debug, Subcommand)]
pub enum MemoryCommand {
    Remember {
        title: String,
        body: String,
        #[arg(long, default_value = "global")]
        scope: String,
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    Recall {
        query: String,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum SkillCommand {
    Define {
        name: String,
        description: String,
        when_to_use: String,
        #[arg(long, value_delimiter = ',')]
        examples: Vec<String>,
    },
    List,
    Run {
        query: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum CommandTemplateCommand {
    Define {
        name: String,
        description: String,
        template: String,
        #[arg(long, value_delimiter = ',')]
        examples: Vec<String>,
    },
    List,
    Run {
        query: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ClipboardCommand {
    Capture {
        content: String,
        #[arg(long, default_value = "manual")]
        source: String,
    },
    Recall {
        query: String,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
}

pub(crate) fn handle_tool(core: &StdCore, command: ToolCommand) -> Result<String, CliError> {
    match command {
        ToolCommand::Run { name, json } => {
            let args = serde_json::from_str(&json)?;
            let output = core.execute_tool(&name, args)?;
            Ok(serde_json::to_string_pretty(&output)?)
        }
    }
}

pub(crate) fn handle_plugin(core: &StdCore, command: PluginCommand) -> Result<String, CliError> {
    match command {
        PluginCommand::List => list_plugins(core),
        PluginCommand::Check { path } => check_plugin(&path),
        PluginCommand::Run { query } => trigger_action(core, &query, false),
    }
}

pub(crate) fn handle_app(core: &StdCore, command: AppCommand) -> Result<String, CliError> {
    match command {
        AppCommand::Register { path } => register_app(core, &path),
        AppCommand::List => list_apps(core),
    }
}

pub(crate) fn handle_memory(core: &StdCore, command: MemoryCommand) -> Result<String, CliError> {
    match command {
        MemoryCommand::Remember {
            title,
            body,
            scope,
            tags,
        } => {
            let memory = core.remember(scope, title, body, tags)?;
            Ok(serde_json::to_string_pretty(&memory)?)
        }
        MemoryCommand::Recall { query, limit } => {
            let memories = core.recall(&query, limit)?;
            Ok(serde_json::to_string_pretty(&memories)?)
        }
    }
}

pub(crate) fn handle_skill(core: &StdCore, command: SkillCommand) -> Result<String, CliError> {
    match command {
        SkillCommand::Define {
            name,
            description,
            when_to_use,
            examples,
        } => {
            let skill = core.define_skill(name, description, when_to_use, examples)?;
            Ok(serde_json::to_string_pretty(&skill)?)
        }
        SkillCommand::List => {
            let skills = core.list_skills()?;
            Ok(serde_json::to_string_pretty(&skills)?)
        }
        SkillCommand::Run { query } => trigger_action(core, &query, false),
    }
}

pub(crate) fn handle_command_template(
    core: &StdCore,
    command: CommandTemplateCommand,
) -> Result<String, CliError> {
    match command {
        CommandTemplateCommand::Define {
            name,
            description,
            template,
            examples,
        } => {
            let command = core.define_command(name, description, template, examples)?;
            Ok(serde_json::to_string_pretty(&command)?)
        }
        CommandTemplateCommand::List => {
            let commands = core.list_commands()?;
            Ok(serde_json::to_string_pretty(&commands)?)
        }
        CommandTemplateCommand::Run { query } => trigger_action(core, &query, false),
    }
}

pub(crate) fn handle_clipboard(
    core: &StdCore,
    command: ClipboardCommand,
) -> Result<String, CliError> {
    match command {
        ClipboardCommand::Capture { content, source } => {
            let record = core.capture_clipboard(content, source)?;
            Ok(serde_json::to_string_pretty(&record)?)
        }
        ClipboardCommand::Recall { query, limit } => {
            let records = core.recall_clipboard(&query, limit)?;
            Ok(serde_json::to_string_pretty(&records)?)
        }
    }
}

pub(crate) fn list_plugins(core: &StdCore) -> Result<String, CliError> {
    let manifests = discover_plugin_manifests(&core.config.plugins_dir())?;
    let lines = manifests
        .into_iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}

fn check_plugin(path: &Path) -> Result<String, CliError> {
    let report = check_plugin_manifest(path)?;
    Ok(serde_json::to_string_pretty(&report)?)
}

fn register_app(core: &StdCore, source: &Path) -> Result<String, CliError> {
    let target = core.register_app_bundle(source)?;
    Ok(format!("app registered\npath={}", target.display()))
}

fn list_apps(core: &StdCore) -> Result<String, CliError> {
    let apps = core
        .list_registered_apps()?
        .into_iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    Ok(apps.join("\n"))
}
