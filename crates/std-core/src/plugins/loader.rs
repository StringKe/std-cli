use crate::{
    plugins::{
        runtime_paths::{resolve_plugin_fs_scopes, resolve_plugin_script},
        PluginActionKind, PluginHostData, PluginManifest, PluginPermission, PluginTool,
    },
    CoreError,
};
use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn read_plugin_manifest(path: &Path) -> Result<PluginManifest, CoreError> {
    let body = fs::read_to_string(path)?;
    Ok(serde_json::from_str(&body)?)
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PluginCheckReport {
    pub manifest_path: PathBuf,
    pub plugin_name: String,
    pub status: &'static str,
    pub actions: usize,
    pub permissions: Vec<PluginPermission>,
    pub fs_scopes: Vec<PathBuf>,
    pub network_hosts: Vec<String>,
}

pub fn check_plugin_manifest(path: &Path) -> Result<PluginCheckReport, CoreError> {
    let manifest_path = normalize_manifest_path(path)?;
    let manifest = read_plugin_manifest(&manifest_path)?;
    let plugin_dir = manifest_path.parent().ok_or_else(|| {
        CoreError::PluginInvalid(format!(
            "plugin manifest has no parent: {}",
            manifest_path.display()
        ))
    })?;
    validate_manifest_fields(&manifest)?;
    validate_plugin_permissions(&manifest, plugin_dir)?;
    Ok(PluginCheckReport {
        manifest_path,
        plugin_name: manifest.name,
        status: "PASS",
        actions: manifest.actions.len(),
        permissions: manifest.permissions,
        fs_scopes: manifest.fs_scopes,
        network_hosts: manifest.network_hosts,
    })
}

pub fn discover_plugin_manifests(plugins_dir: &Path) -> Result<Vec<PathBuf>, CoreError> {
    if !plugins_dir.is_dir() {
        return Ok(Vec::new());
    }

    let mut manifests = Vec::new();
    for entry in fs::read_dir(plugins_dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let manifest = path.join("plugin.json");
            if manifest.is_file() {
                manifests.push(manifest);
            }
        } else if path.file_name().and_then(|name| name.to_str()) == Some("plugin.json") {
            manifests.push(path);
        }
    }
    manifests.sort();
    Ok(manifests)
}

pub fn load_plugin_tools(plugins_dir: &Path) -> Result<Vec<PluginTool>, CoreError> {
    load_plugin_tools_with_host(plugins_dir, PluginHostData::default())
}

pub fn load_plugin_tools_with_host(
    plugins_dir: &Path,
    host_data: PluginHostData,
) -> Result<Vec<PluginTool>, CoreError> {
    let mut tools = Vec::new();
    for manifest_path in discover_plugin_manifests(plugins_dir)? {
        let manifest = Arc::new(read_plugin_manifest(&manifest_path)?);
        for action in &manifest.actions {
            tools.push(
                PluginTool::new(manifest.clone(), action.clone(), manifest_path.clone())
                    .with_host_data(host_data.clone()),
            );
        }
    }
    Ok(tools)
}

fn normalize_manifest_path(path: &Path) -> Result<PathBuf, CoreError> {
    let manifest_path = if path.is_dir() {
        path.join("plugin.json")
    } else {
        path.to_path_buf()
    };
    if !manifest_path.is_file() {
        return Err(CoreError::PluginInvalid(format!(
            "plugin manifest not found: {}",
            manifest_path.display()
        )));
    }
    Ok(manifest_path)
}

fn validate_manifest_fields(manifest: &PluginManifest) -> Result<(), CoreError> {
    require_non_empty("plugin name", &manifest.name)?;
    require_non_empty("plugin description", &manifest.description)?;
    if manifest.actions.is_empty() {
        return Err(CoreError::PluginInvalid(format!(
            "{} has no plugin actions",
            manifest.name
        )));
    }
    for action in &manifest.actions {
        require_non_empty("plugin action name", &action.name)?;
        require_non_empty("plugin action description", &action.description)?;
        require_non_empty("plugin action when_to_use", &action.when_to_use)?;
    }
    Ok(())
}

fn validate_plugin_permissions(
    manifest: &PluginManifest,
    plugin_dir: &Path,
) -> Result<(), CoreError> {
    for action in &manifest.actions {
        match action.kind {
            PluginActionKind::Shell => {
                require_permission(manifest, PluginPermission::Shell, &action.name)?;
                require_non_empty(
                    "shell plugin action command",
                    action.command.as_deref().unwrap_or_default(),
                )?;
            }
            PluginActionKind::Javascript | PluginActionKind::Typescript => {
                require_permission(manifest, PluginPermission::Code, &action.name)?;
                let script = action.script.as_deref().ok_or_else(|| {
                    CoreError::PluginInvalid(format!(
                        "code plugin action missing script: {}",
                        action.name
                    ))
                })?;
                resolve_plugin_script(plugin_dir, script)?;
            }
        }
    }
    if manifest.permissions.contains(&PluginPermission::FsScoped) {
        resolve_plugin_fs_scopes(manifest, plugin_dir)?;
    }
    Ok(())
}

fn require_permission(
    manifest: &PluginManifest,
    permission: PluginPermission,
    action_name: &str,
) -> Result<(), CoreError> {
    if manifest.permissions.contains(&permission) {
        Ok(())
    } else {
        Err(CoreError::PluginPermissionDenied(format!(
            "{action_name} requires {permission:?} permission"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), CoreError> {
    if value.trim().is_empty() {
        Err(CoreError::PluginInvalid(format!("{label} is empty")))
    } else {
        Ok(())
    }
}
