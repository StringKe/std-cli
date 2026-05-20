use crate::CliError;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub(crate) fn copy_tree_files(
    source_dir: &Path,
    target_dir: &Path,
) -> Result<Vec<String>, CliError> {
    let mut copied = Vec::new();
    copy_tree_files_inner(source_dir, source_dir, target_dir, &mut copied)?;
    copied.sort();
    Ok(copied)
}

pub(crate) fn project_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(|path| path.parent())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."))
}

fn copy_tree_files_inner(
    root: &Path,
    source_dir: &Path,
    target_dir: &Path,
    copied: &mut Vec<String>,
) -> Result<(), CliError> {
    fs::create_dir_all(target_dir)?;
    let mut entries = fs::read_dir(source_dir)?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|entry| entry.path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        let relative = path.strip_prefix(root).map_err(|error| {
            CliError::Install(format!(
                "release path is outside source tree: {}: {}",
                path.display(),
                error
            ))
        })?;
        let target = target_dir.join(relative);
        if path.is_dir() {
            copy_tree_files_inner(root, &path, target_dir, copied)?;
        } else if path.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&path, &target)?;
            copied.push(target.display().to_string());
        }
    }
    Ok(())
}
