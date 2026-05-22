use crate::app_bundle_profile::{unique_non_empty, AppProfile};
use std::{
    fs,
    path::{Path, PathBuf},
};
use std_types::{Action, ActionType, RegistryEntry};

pub(crate) fn discover_app_actions(local_apps_dir: &Path) -> Vec<RegistryEntry> {
    app_discovery_dirs(local_apps_dir)
        .into_iter()
        .flat_map(|dir| discover_apps_in_dir(&dir))
        .collect()
}

fn app_discovery_dirs(local_apps_dir: &Path) -> Vec<PathBuf> {
    if crate::std_test_mode_enabled() {
        return vec![local_apps_dir.to_path_buf()];
    }
    vec![
        local_apps_dir.to_path_buf(),
        default_apps_dir(),
        system_apps_dir(),
    ]
}

fn discover_apps_in_dir(dir: &Path) -> Vec<RegistryEntry> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("app"))
        .filter_map(app_registry_entry)
        .collect()
}

fn app_registry_entry(path: PathBuf) -> Option<RegistryEntry> {
    let profile = AppProfile::read(&path)?;
    let description = app_description(&path, &profile.names);
    let mut registry_entry = RegistryEntry::from_action(
        Action::new(
            format!("Open App: {}", profile.display_name),
            description,
            "When opening this local macOS application",
            ActionType::AppLaunch,
        ),
        app_tags(&profile.names),
    );
    registry_entry
        .metadata
        .insert("path".to_string(), path.display().to_string());
    registry_entry
        .metadata
        .insert("bundle_name".to_string(), profile.bundle_name);
    registry_entry
        .metadata
        .insert("aliases".to_string(), profile.names.join(","));
    Some(registry_entry)
}

fn app_description(path: &Path, names: &[String]) -> String {
    if names.is_empty() {
        return format!("Launch macOS app at {}", path.display());
    }
    format!("Aliases: {} / Path: {}", names.join(", "), path.display())
}

fn app_tags(names: &[String]) -> Vec<String> {
    let mut tags = vec!["app".to_string(), "macos".to_string()];
    tags.extend(names.iter().cloned());
    unique_non_empty(tags)
}

fn default_apps_dir() -> PathBuf {
    PathBuf::from("/Applications")
}

fn system_apps_dir() -> PathBuf {
    PathBuf::from("/System/Applications")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_app_discovery_uses_only_local_fixture_dir() {
        let local = PathBuf::from("/tmp/std-cli-fixture-apps");
        let dirs = app_discovery_dirs(&local);

        assert_eq!(dirs, vec![local]);
    }
}
