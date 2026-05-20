use std::{
    env,
    path::{Path, PathBuf},
};

pub(super) fn home_config_path(relative: &str) -> PathBuf {
    home_dir().join(relative)
}

pub(super) fn default_data_dir() -> PathBuf {
    default_data_root()
}

#[cfg(test)]
fn default_data_root() -> PathBuf {
    env::temp_dir()
        .join("std-cli-test-data")
        .join(std::process::id().to_string())
}

#[cfg(not(test))]
fn default_data_root() -> PathBuf {
    home_dir().join(".std-cli")
}

pub(super) fn project_config_candidates(start_dir: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut ancestors = start_dir.ancestors().collect::<Vec<_>>();
    ancestors.reverse();
    for dir in ancestors {
        paths.push(dir.join("std-cli.toml"));
        paths.push(dir.join("std-cli.json"));
        paths.push(dir.join("std-cli.yaml"));
        paths.push(dir.join("std-cli.yml"));
    }
    paths
}

fn home_dir() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}
