use crate::{tools::StdTool, CoreError};
mod command;
mod loader;
mod runtime;
mod runtime_http;
mod runtime_paths;
mod typescript;

pub use loader::{
    check_plugin_manifest, discover_plugin_manifests, load_plugin_tools,
    load_plugin_tools_with_host, read_plugin_manifest, PluginCheckReport,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};
use std_types::{Action, ActionType, ClipboardRecord, RegistryEntry};

use command::run_shell_with_timeout;
use runtime::{run_script_with_timeout, PluginScriptRun};
use runtime_paths::resolve_plugin_script;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginManifest {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub permissions: Vec<PluginPermission>,
    #[serde(default)]
    pub fs_scopes: Vec<PathBuf>,
    #[serde(default)]
    pub network_hosts: Vec<String>,
    #[serde(default)]
    pub actions: Vec<PluginActionManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginPermission {
    Shell,
    Code,
    FsScoped,
    Network,
    Clipboard,
    ReadOnly,
}

#[derive(Debug, Clone, Default)]
pub struct PluginHostData {
    pub clipboard: Vec<ClipboardRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginActionManifest {
    pub name: String,
    pub description: String,
    pub when_to_use: String,
    pub kind: PluginActionKind,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub timeout_ms: Option<u64>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginActionKind {
    Shell,
    Javascript,
    Typescript,
}

#[derive(Clone)]
pub struct PluginTool {
    manifest: Arc<PluginManifest>,
    action: PluginActionManifest,
    manifest_path: PathBuf,
    host_data: PluginHostData,
}

impl PluginTool {
    pub fn new(
        manifest: Arc<PluginManifest>,
        action: PluginActionManifest,
        manifest_path: PathBuf,
    ) -> Self {
        Self {
            manifest,
            action,
            manifest_path,
            host_data: PluginHostData::default(),
        }
    }

    pub fn with_host_data(mut self, host_data: PluginHostData) -> Self {
        self.host_data = host_data;
        self
    }

    pub fn registry_entry(&self) -> RegistryEntry {
        let mut action = Action::new(
            &self.action.name,
            &self.action.description,
            &self.action.when_to_use,
            ActionType::Command,
        );
        if let Some(command) = &self.action.command {
            action.examples.push(command.clone());
        }
        if let Some(script) = &self.action.script {
            action.examples.push(script.clone());
        }
        action
            .examples
            .push(format!("plugin:{}", self.manifest.name));

        let mut tags = self.action.tags.clone();
        tags.push("plugin".to_string());
        tags.push(self.manifest.name.clone());
        tags.push(format!("{:?}", self.action.kind).to_lowercase());
        let mut entry = RegistryEntry::from_action(action, tags);
        entry
            .metadata
            .insert("plugin".to_string(), self.manifest.name.clone());
        entry.metadata.insert(
            "plugin_manifest".to_string(),
            self.manifest_path.display().to_string(),
        );
        if let Some(command) = &self.action.command {
            entry
                .metadata
                .insert("command".to_string(), command.clone());
        }
        if let Some(script) = &self.action.script {
            entry.metadata.insert("script".to_string(), script.clone());
        }
        entry.metadata.insert(
            "plugin_kind".to_string(),
            self.action.kind.kind_key().to_string(),
        );
        entry
    }

    pub fn execute(&self, args: Value) -> Result<Value, CoreError> {
        match self.action.kind {
            PluginActionKind::Shell => {
                if !self.manifest.permissions.contains(&PluginPermission::Shell) {
                    return Err(CoreError::PluginPermissionDenied(format!(
                        "{} requires shell permission",
                        self.action.name
                    )));
                }
                let command = self.action.command.as_deref().ok_or_else(|| {
                    CoreError::PluginInvalid(format!(
                        "shell plugin action missing command: {}",
                        self.action.name
                    ))
                })?;
                let output = run_shell_with_timeout(
                    command,
                    Duration::from_millis(self.action.timeout_ms.unwrap_or(10_000)),
                )?;
                Ok(serde_json::json!({
                    "runtime": "shell",
                    "command": command,
                    "exit_code": output.exit_code,
                    "stdout": output.stdout,
                    "stderr": output.stderr,
                    "timed_out": output.timed_out,
                    "duration_ms": output.duration_ms,
                }))
            }
            PluginActionKind::Javascript | PluginActionKind::Typescript => {
                if !self.manifest.permissions.contains(&PluginPermission::Code) {
                    return Err(CoreError::PluginPermissionDenied(format!(
                        "{} requires code permission",
                        self.action.name
                    )));
                }
                let script = self.action.script.as_deref().ok_or_else(|| {
                    CoreError::PluginInvalid(format!(
                        "code plugin action missing script: {}",
                        self.action.name
                    ))
                })?;
                let plugin_dir = self
                    .manifest_path
                    .parent()
                    .map(Path::to_path_buf)
                    .ok_or_else(|| {
                        CoreError::PluginInvalid(format!(
                            "plugin manifest has no parent: {}",
                            self.manifest_path.display()
                        ))
                    })?;
                let script_path = resolve_plugin_script(&plugin_dir, script)?;
                let output = run_script_with_timeout(PluginScriptRun {
                    script_path: &script_path,
                    args: &args,
                    manifest: &self.manifest,
                    kind: self.action.kind.clone(),
                    plugin_dir: &plugin_dir,
                    host_data: &self.host_data,
                    timeout: Duration::from_millis(self.action.timeout_ms.unwrap_or(10_000)),
                })?;
                Ok(serde_json::json!({
                    "runtime": output.runtime,
                    "script": script_path,
                    "exit_code": output.exit_code,
                    "stdout": output.stdout,
                    "stderr": output.stderr,
                    "timed_out": output.timed_out,
                    "duration_ms": output.duration_ms,
                }))
            }
        }
    }
}

impl PluginActionKind {
    fn kind_key(&self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Javascript => "javascript",
            Self::Typescript => "typescript",
        }
    }
}

impl StdTool for PluginTool {
    fn action(&self) -> Action {
        self.registry_entry().action
    }

    fn tags(&self) -> Vec<String> {
        self.registry_entry().tags
    }

    fn execute(&self, args: Value) -> Result<Value, CoreError> {
        PluginTool::execute(self, args)
    }
}

#[cfg(test)]
mod tests;
