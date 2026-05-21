//! std-core - The strong, GUI-neutral core of std-cli
//!
//! All business logic lives here. Launcher and Studio are just thin renderers.

mod actions;
mod app_bundle;
mod apps;
mod bootstrap;
pub mod config;
mod content;
mod discovery;
mod events;
mod execution;
mod indexing;
pub mod planner;
pub mod plugins;
mod registry;
pub mod storage;
mod tooling;
pub mod tools;

pub use config::StdConfig;
pub use events::{EventBus, EventLog};
pub use planner::AiPlanner;
pub use plugins::{
    check_plugin_manifest, discover_plugin_manifests, load_plugin_tools, PluginCheckReport,
    PluginHostData, PluginManifest,
};
pub use registry::ActionRegistry;
pub use storage::LocalStore;
pub use tools::{EchoTool, StdTool, ToolRegistry};

use std::process::Command;
use std::{
    io,
    process::Output,
    sync::{Arc, OnceLock, RwLock},
};
use std_types::{ActionId, StdEvent};
use thiserror::Error;

type CommandRunner = dyn Fn(&str, &[String]) -> Result<Output, io::Error> + Send + Sync;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("Action not found: {0}")]
    ActionNotFound(ActionId),
    #[error("Duplicate action: {0}")]
    DuplicateAction(String),
    #[error("Registry lock poisoned")]
    RegistryLockPoisoned,
    #[error("Storage error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    #[error("Index error: {0}")]
    Index(#[from] std_index::IndexError),
    #[error("Plugin invalid: {0}")]
    PluginInvalid(String),
    #[error("Plugin permission denied: {0}")]
    PluginPermissionDenied(String),
    #[error("App invalid: {0}")]
    AppInvalid(String),
}

#[derive(Clone)]
pub struct StdCore {
    pub registry: Arc<RwLock<ActionRegistry>>,
    event_log: Arc<RwLock<EventLog>>,
    tools: Arc<RwLock<ToolRegistry>>,
    store: LocalStore,
    command_runner: Arc<CommandRunner>,
    pub config: StdConfig,
}

impl StdCore {
    pub fn new() -> Self {
        Self::with_config(StdConfig::default())
    }

    pub fn with_config(config: StdConfig) -> Self {
        default_core_with_config(config)
    }

    pub fn with_config_and_command_runner(
        config: StdConfig,
        runner: impl Fn(&str, &[String]) -> Result<Output, io::Error> + Send + Sync + 'static,
    ) -> Self {
        let store = LocalStore::new(config.clone());
        let command_runner: Arc<CommandRunner> = if std_test_mode_enabled() {
            Arc::new(blocked_test_mode_command_runner)
        } else {
            Arc::new(runner)
        };
        Self {
            registry: Arc::new(RwLock::new(ActionRegistry::new())),
            event_log: Arc::new(RwLock::new(EventLog::new())),
            tools: Arc::new(RwLock::new(ToolRegistry::new())),
            store,
            command_runner,
            config,
        }
    }

    pub fn ensure_storage(&self) -> Result<(), CoreError> {
        self.store.ensure_dirs()
    }

    pub fn read_audit_events(&self) -> Result<Vec<StdEvent>, CoreError> {
        self.store.read_events()
    }

    pub(crate) fn run_external_command(
        &self,
        program: &str,
        args: &[String],
    ) -> Result<Output, io::Error> {
        (self.command_runner)(program, args)
    }
}

#[cfg(not(test))]
fn default_core_with_config(config: StdConfig) -> StdCore {
    if runtime_test_mode_enabled() {
        return StdCore::with_config_and_command_runner(
            config,
            blocked_runtime_test_command_runner,
        );
    }
    StdCore::with_config_and_command_runner(config, |program, args| {
        Command::new(program).args(args).output()
    })
}

#[cfg(test)]
fn default_core_with_config(config: StdConfig) -> StdCore {
    StdCore::with_config_and_command_runner(config, blocked_test_mode_command_runner)
}

fn blocked_test_mode_command_runner(program: &str, args: &[String]) -> Result<Output, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!("STD_TEST_MODE blocked external command: {program} {args:?}"),
    ))
}

#[cfg(not(test))]
fn blocked_runtime_test_command_runner(
    program: &str,
    args: &[String],
) -> Result<Output, io::Error> {
    Err(io::Error::new(
        io::ErrorKind::PermissionDenied,
        format!("STD_TEST_MODE blocked external command: {program} {args:?}"),
    ))
}

#[cfg(not(test))]
fn runtime_test_mode_enabled() -> bool {
    std_test_mode_enabled()
}

pub fn std_test_mode_enabled() -> bool {
    if cfg!(test) {
        return true;
    }
    if running_under_cargo_test_context() {
        return true;
    }
    std::env::var("STD_TEST_MODE")
        .map(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
        .unwrap_or(false)
}

fn running_under_cargo_test_context() -> bool {
    static TEST_CONTEXT: OnceLock<bool> = OnceLock::new();
    *TEST_CONTEXT.get_or_init(|| {
        running_under_cargo_test_binary()
            || std::env::var("RUST_TEST_THREADS").is_ok()
            || parent_process_chain_contains_cargo_test()
    })
}

fn running_under_cargo_test_binary() -> bool {
    let Ok(exe) = std::env::current_exe() else {
        return false;
    };
    let Some(parent) = exe.parent().and_then(|path| path.file_name()) else {
        return false;
    };
    if parent != "deps" {
        return false;
    }
    let Some(file_name) = exe.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name.rsplit_once('-').is_some()
}

#[cfg(unix)]
fn parent_process_chain_contains_cargo_test() -> bool {
    let mut pid = std::process::id().to_string();
    for _ in 0..8 {
        let Ok(output) = Command::new("/bin/ps")
            .args(["-o", "ppid=", "-o", "comm=", "-p", &pid])
            .output()
        else {
            return false;
        };
        if !output.status.success() {
            return false;
        }
        let row = String::from_utf8_lossy(&output.stdout);
        let mut parts = row.split_whitespace();
        let Some(parent) = parts.next() else {
            return false;
        };
        let command = parts.collect::<Vec<_>>().join(" ");
        if command.contains("/deps/") && command.rsplit_once('-').is_some() {
            return true;
        }
        if command.ends_with("/cargo") || command.ends_with("cargo") {
            return true;
        }
        if parent == "0" || parent == pid {
            return false;
        }
        pid = parent.to_string();
    }
    false
}

#[cfg(not(unix))]
fn parent_process_chain_contains_cargo_test() -> bool {
    false
}

pub fn desktop_automation_allowed() -> bool {
    if cfg!(test) || std_test_mode_enabled() {
        return false;
    }
    std::env::var("STD_ALLOW_DESKTOP_AUTOMATION")
        .map(|value| value == "1")
        .unwrap_or(false)
}

pub fn sanitize_desktop_opt_ins_for_test_mode() {
    if std_test_mode_enabled() {
        std::env::remove_var("STD_ALLOW_DESKTOP_AUTOMATION");
        std::env::remove_var("STD_ALLOW_UI_PREVIEW");
    }
}

impl Default for StdCore {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus for StdCore {
    fn publish(&self, event: StdEvent) -> Result<(), CoreError> {
        self.event_log
            .write()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .push(event.clone());
        self.store.append_event(&event)?;
        Ok(())
    }

    fn events(&self) -> Result<Vec<StdEvent>, CoreError> {
        Ok(self
            .event_log
            .read()
            .map_err(|_| CoreError::RegistryLockPoisoned)?
            .list()
            .to_vec())
    }
}

#[cfg(test)]
mod tests;
