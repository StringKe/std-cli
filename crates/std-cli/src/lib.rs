//! Script-friendly terminal surface for std-cli.

mod batch;
mod config;
mod content;
mod doctor;
mod events;
mod index;
mod install;
mod release;
mod workflow;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std_core::{StdConfig, StdCore};
use std_index::Indexer;
use std_types::{ActionExecution, ActionExecutionStatus, ActionType};

use batch::run_batch_file;
use config::{config_get, config_set, format_config};
use content::{
    handle_app, handle_clipboard, handle_command_template, handle_memory, handle_plugin,
    handle_skill, handle_tool, AppCommand, ClipboardCommand, CommandTemplateCommand, MemoryCommand,
    PluginCommand, SkillCommand, ToolCommand,
};
use doctor::doctor;
use events::format_events;
use index::{format_index_document, handle_files, handle_index, FilesCommand, IndexCommand};
use install::{install_plan, install_run, install_verify};
use release::{release_package, release_plan, release_verify};
use workflow::{
    handle_plan, handle_workflow, run_workflow, trigger_workflow_action, WorkflowCommand,
};

#[derive(Debug, Parser)]
#[command(name = "std")]
#[command(about = "Local-first developer automation and understanding layer")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print or update resolved configuration.
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommand>,
    },
    /// Install built binaries and initialize local directories.
    Install {
        #[command(subcommand)]
        command: InstallCommand,
    },
    /// Build release artifacts and print release checklist.
    Release {
        #[command(subcommand)]
        command: ReleaseCommand,
    },
    /// Run local health checks across storage, registry, workflow, planner, index, and plugins.
    Doctor {
        #[arg(long)]
        json: bool,
    },
    /// Search registered actions.
    Search {
        query: String,
        #[arg(short, long, default_value_t = 10)]
        limit: usize,
    },
    /// Preview the first matching registered action.
    Preview { query: String },
    /// Execute the first matching registered action.
    Trigger {
        query: String,
        #[arg(long)]
        allow_external: bool,
    },
    /// Run a workflow by name or path.
    Run {
        workflow: String,
        #[arg(long)]
        allow_external: bool,
    },
    /// Run a JSON batch plan with actions and workflows.
    Batch {
        path: PathBuf,
        #[arg(long)]
        allow_external: bool,
        #[arg(long)]
        stop_on_error: bool,
    },
    /// Inspect and debug workflow definitions.
    Workflow {
        #[command(subcommand)]
        command: WorkflowCommand,
    },
    /// Print index storage paths and emit the planned rebuild target.
    Index {
        #[command(subcommand)]
        command: IndexCommand,
    },
    /// Analyze a project, workflow, app bundle, file, or directory.
    Analyze { path: PathBuf },
    /// Produce an AI-native execution plan from registered actions.
    Plan {
        goal: String,
        #[arg(long)]
        workflow: bool,
        #[arg(long)]
        save: bool,
    },
    /// Execute registered tools.
    Tool {
        #[command(subcommand)]
        command: ToolCommand,
    },
    /// Manage local plugins.
    Plugin {
        #[command(subcommand)]
        command: PluginCommand,
    },
    /// Manage local app bundles indexed by Launcher search.
    App {
        #[command(subcommand)]
        command: AppCommand,
    },
    /// Manage local memory.
    Memory {
        #[command(subcommand)]
        command: MemoryCommand,
    },
    /// Manage local skills.
    Skill {
        #[command(subcommand)]
        command: SkillCommand,
    },
    /// Manage saved command templates.
    Command {
        #[command(subcommand)]
        command: CommandTemplateCommand,
    },
    /// Manage clipboard history.
    Clipboard {
        #[command(subcommand)]
        command: ClipboardCommand,
    },
    /// Index and search local files.
    Files {
        #[command(subcommand)]
        command: FilesCommand,
    },
    /// Print recent events produced by the command or persisted audit history.
    Events {
        #[arg(long)]
        audit: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    List,
    Get { key: String },
    Set { key: String, value: String },
    Path,
}

#[derive(Debug, Subcommand)]
pub enum InstallCommand {
    Plan {
        #[arg(long)]
        prefix: Option<PathBuf>,
    },
    Run {
        #[arg(long)]
        prefix: Option<PathBuf>,
        #[arg(long)]
        from: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        prefix: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReleaseCommand {
    Plan {
        #[arg(long, default_value = "0.1.0")]
        version: String,
    },
    Package {
        #[arg(long, default_value = "0.1.0")]
        version: String,
        #[arg(long)]
        from: Option<PathBuf>,
        #[arg(long)]
        dist: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        dist: PathBuf,
    },
}

pub fn run_cli<I, T>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    #[cfg(test)]
    std::env::set_var("STD_TEST_MODE", "1");
    std_core::sanitize_desktop_opt_ins_for_test_mode();

    let cli = Cli::parse_from(args);
    let config = StdConfig::try_load().map_err(|error| CliError::Config(error.to_string()))?;
    let core = StdCore::with_config(config);
    core.seed_builtin_actions()?;

    dispatch_command(&core, cli.command)
}

fn dispatch_command(core: &StdCore, command: Commands) -> Result<String, CliError> {
    match command {
        Commands::Config { command } => handle_config(core, command),
        Commands::Install { command } => handle_install(core, command),
        Commands::Release { command } => handle_release(core, command),
        Commands::Doctor { json } => doctor(core, json),
        Commands::Search { query, limit } => search_actions(core, &query, limit),
        Commands::Preview { query } => preview_action(core, &query),
        Commands::Trigger {
            query,
            allow_external,
        } => trigger_action(core, &query, allow_external),
        Commands::Run {
            workflow,
            allow_external,
        } => run_workflow(core, &workflow, allow_external),
        Commands::Batch {
            path,
            allow_external,
            stop_on_error,
        } => run_batch_file(core, &path, allow_external, stop_on_error),
        Commands::Workflow { command } => handle_workflow(core, command),
        Commands::Index { command } => handle_index(core, command),
        Commands::Analyze { path } => {
            let doc = Indexer::analyze(&path)?;
            Ok(format_index_document(&doc)?)
        }
        Commands::Plan {
            goal,
            workflow,
            save,
        } => handle_plan(core, &goal, workflow, save),
        Commands::Tool { command } => handle_tool(core, command),
        Commands::Plugin { command } => handle_plugin(core, command),
        Commands::App { command } => handle_app(core, command),
        Commands::Memory { command } => handle_memory(core, command),
        Commands::Skill { command } => handle_skill(core, command),
        Commands::Command { command } => handle_command_template(core, command),
        Commands::Clipboard { command } => handle_clipboard(core, command),
        Commands::Files { command } => handle_files(core, command),
        Commands::Events { audit } => format_events(core, audit),
    }
}

fn handle_config(core: &StdCore, command: Option<ConfigCommand>) -> Result<String, CliError> {
    match command {
        None | Some(ConfigCommand::List) => Ok(format_config(&core.config)),
        Some(ConfigCommand::Get { key }) => config_get(&core.config, &key),
        Some(ConfigCommand::Set { key, value }) => config_set(core.config.clone(), &key, &value),
        Some(ConfigCommand::Path) => Ok(StdConfig::writable_config_path().display().to_string()),
    }
}

fn handle_install(core: &StdCore, command: InstallCommand) -> Result<String, CliError> {
    match command {
        InstallCommand::Plan { prefix } => install_plan(core, prefix.as_deref()),
        InstallCommand::Run { prefix, from } => {
            install_run(core, prefix.as_deref(), from.as_deref())
        }
        InstallCommand::Verify { prefix } => install_verify(core, prefix.as_deref()),
    }
}

fn handle_release(core: &StdCore, command: ReleaseCommand) -> Result<String, CliError> {
    match command {
        ReleaseCommand::Plan { version } => release_plan(core, &version),
        ReleaseCommand::Package {
            version,
            from,
            dist,
        } => release_package(core, &version, from.as_deref(), dist.as_deref()),
        ReleaseCommand::Verify { dist } => release_verify(&dist),
    }
}

fn search_actions(core: &StdCore, query: &str, limit: usize) -> Result<String, CliError> {
    let lines = core
        .search(query, limit)?
        .into_iter()
        .map(|result| {
            format!(
                "{}\t{}\t{}",
                result.action.name, result.action.description, result.score
            )
        })
        .collect::<Vec<_>>();
    Ok(lines.join("\n"))
}

fn preview_action(core: &StdCore, query: &str) -> Result<String, CliError> {
    let result = core
        .search(query, 1)?
        .into_iter()
        .next()
        .ok_or_else(|| CliError::ActionSearchEmpty(query.to_string()))?;
    let preview = core.preview_action(result.action.id)?;
    Ok(serde_json::to_string_pretty(&preview)?)
}

fn trigger_action(core: &StdCore, query: &str, allow_external: bool) -> Result<String, CliError> {
    let result = core
        .search(query, 1)?
        .into_iter()
        .next()
        .ok_or_else(|| CliError::ActionSearchEmpty(query.to_string()))?;
    let execution = if result.action.action_type == ActionType::Workflow {
        let preview = core.preview_action(result.action.id)?;
        trigger_workflow_action(
            core,
            &result.action.name,
            preview.metadata.get("path"),
            allow_external,
        )?
    } else if !allow_external && result.action.action_type.needs_external_runner() {
        let preview = core.preview_action(result.action.id)?;
        ActionExecution {
            action_id: result.action.id,
            action_name: result.action.name,
            status: ActionExecutionStatus::NeedsExternalRunner,
            message: preview.primary_command,
            output: Some(serde_json::json!({
                "deferred": true,
                "reason": "external runner action requires explicit user trigger",
            })),
            created_at: chrono::Utc::now(),
        }
    } else {
        core.execute_action_with_external_runner(result.action.id, allow_external)?
    };
    Ok(serde_json::to_string_pretty(&execution)?)
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{0}")]
    Core(#[from] std_core::CoreError),
    #[error("{0}")]
    Orchestration(#[from] std_orchestration::OrchestrationError),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Index(#[from] std_index::IndexError),
    #[error("Index document not found: {0}")]
    IndexNotFound(String),
    #[error("No action matched query: {0}")]
    ActionSearchEmpty(String),
    #[error("Install error: {0}")]
    Install(String),
    #[error("Config error: {0}")]
    Config(String),
    #[error("Doctor error: {0}")]
    Doctor(String),
    #[error("App error: {0}")]
    App(String),
}

#[cfg(test)]
mod tests;
