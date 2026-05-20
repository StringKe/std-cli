use crate::{
    plugins::{PluginManifest, PluginPermission},
    CoreError,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn resolve_plugin_script(plugin_dir: &Path, script: &str) -> Result<PathBuf, CoreError> {
    let plugin_dir = fs::canonicalize(plugin_dir)?;
    let script_path = if Path::new(script).is_absolute() {
        PathBuf::from(script)
    } else {
        plugin_dir.join(script)
    };
    let script_path = fs::canonicalize(&script_path).map_err(|error| {
        CoreError::PluginInvalid(format!("plugin script not found: {}: {}", script, error))
    })?;
    if !script_path.starts_with(&plugin_dir) {
        return Err(CoreError::PluginPermissionDenied(format!(
            "plugin script outside plugin directory: {}",
            script_path.display()
        )));
    }
    Ok(script_path)
}

pub(crate) fn resolve_plugin_fs_scopes(
    manifest: &PluginManifest,
    plugin_dir: &Path,
) -> Result<Vec<PathBuf>, CoreError> {
    let base = fs::canonicalize(plugin_dir)?;
    let mut scopes = vec![base.clone()];
    for scope in &manifest.fs_scopes {
        let candidate = if scope.is_absolute() {
            scope.clone()
        } else {
            base.join(scope)
        };
        let canonical = fs::canonicalize(&candidate).map_err(|error| {
            CoreError::PluginInvalid(format!(
                "invalid fs scope {}: {}",
                candidate.display(),
                error
            ))
        })?;
        scopes.push(canonical);
    }
    scopes.sort();
    scopes.dedup();
    Ok(scopes)
}

pub(crate) fn manifest_allows_fs(manifest: &PluginManifest) -> bool {
    manifest.permissions.contains(&PluginPermission::FsScoped)
}

pub(crate) fn manifest_allows_network(manifest: &PluginManifest) -> bool {
    manifest.permissions.contains(&PluginPermission::Network)
}

pub(crate) fn manifest_allows_clipboard(manifest: &PluginManifest) -> bool {
    manifest.permissions.contains(&PluginPermission::Clipboard)
}
