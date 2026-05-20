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

use std::{
    io,
    process::{Command, Output},
    sync::{Arc, RwLock},
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
        Self::with_config_and_command_runner(config, |program, args| {
            Command::new(program).args(args).output()
        })
    }

    pub fn with_config_and_command_runner(
        config: StdConfig,
        runner: impl Fn(&str, &[String]) -> Result<Output, io::Error> + Send + Sync + 'static,
    ) -> Self {
        let store = LocalStore::new(config.clone());
        Self {
            registry: Arc::new(RwLock::new(ActionRegistry::new())),
            event_log: Arc::new(RwLock::new(EventLog::new())),
            tools: Arc::new(RwLock::new(ToolRegistry::new())),
            store,
            command_runner: Arc::new(runner),
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
