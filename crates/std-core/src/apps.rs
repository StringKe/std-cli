use crate::{CoreError, StdCore};
use std::{
    fs,
    path::{Path, PathBuf},
};

impl StdCore {
    pub fn register_app_bundle(&self, source: &Path) -> Result<PathBuf, CoreError> {
        validate_app_bundle(source)?;
        self.ensure_storage()?;
        let file_name = source.file_name().ok_or_else(|| {
            CoreError::AppInvalid(format!("app bundle has no file name: {}", source.display()))
        })?;
        let target = self.config.apps_dir().join(file_name);
        if target.exists() {
            fs::remove_dir_all(&target)?;
        }
        copy_dir(source, &target)?;
        self.register_local_content_actions()?;
        Ok(target)
    }

    pub fn list_registered_apps(&self) -> Result<Vec<PathBuf>, CoreError> {
        let apps_dir = self.config.apps_dir();
        if !apps_dir.is_dir() {
            return Ok(Vec::new());
        }
        let mut apps = fs::read_dir(apps_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("app"))
            .collect::<Vec<_>>();
        apps.sort();
        Ok(apps)
    }
}

fn validate_app_bundle(source: &Path) -> Result<(), CoreError> {
    if source.extension().and_then(|ext| ext.to_str()) == Some("app") && source.is_dir() {
        return Ok(());
    }
    Err(CoreError::AppInvalid(format!(
        "app bundle expected: {}",
        source.display()
    )))
}

fn copy_dir(source: &Path, target: &Path) -> Result<(), CoreError> {
    fs::create_dir_all(target)?;
    for entry in fs::read_dir(source)? {
        let source_path = entry?.path();
        let target_path = target.join(source_path.file_name().ok_or_else(|| {
            CoreError::AppInvalid(format!("path has no file name: {}", source_path.display()))
        })?);
        if source_path.is_dir() {
            copy_dir(&source_path, &target_path)?;
        } else if source_path.is_file() {
            fs::copy(&source_path, &target_path)?;
        }
    }
    Ok(())
}
