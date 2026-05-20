use crate::{
    release::{checksum::sha256_file, macos::app_bundle_executable},
    CliError,
};
use serde_json::json;
use std::path::{Path, PathBuf};

pub(crate) fn release_target_metadata() -> serde_json::Value {
    json!({
        "os": std::env::consts::OS,
        "arch": std::env::consts::ARCH,
        "family": std::env::consts::FAMILY,
    })
}

pub(crate) fn release_profile() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}

pub(crate) fn release_rust_version() -> &'static str {
    match option_env!("CARGO_PKG_RUST_VERSION") {
        Some(value) if !value.trim().is_empty() => value,
        _ => "UNKNOWN",
    }
}

pub(crate) fn manifest_array(
    manifest: &serde_json::Value,
    key: &str,
) -> Result<Vec<String>, CliError> {
    let values = manifest
        .get(key)
        .and_then(|value| value.as_array())
        .ok_or_else(|| CliError::Install(format!("release manifest missing array: {key}")))?;
    values
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(ToString::to_string)
                .ok_or_else(|| CliError::Install(format!("release manifest {key} must be strings")))
        })
        .collect()
}

pub(crate) fn verify_manifest_paths(
    manifest: &serde_json::Value,
    key: &str,
    must_be_file: bool,
) -> Result<usize, CliError> {
    let paths = manifest_array(manifest, key)?;
    for path in &paths {
        let path = Path::new(path);
        let valid = if must_be_file {
            path.is_file()
        } else {
            path.is_dir()
        };
        if !valid {
            return Err(CliError::Install(format!(
                "release manifest {key} path missing: {}",
                path.display()
            )));
        }
    }
    Ok(paths.len())
}

pub(crate) fn verify_release_metadata(manifest: &serde_json::Value) -> Result<(), CliError> {
    let name = manifest
        .get("name")
        .and_then(|value| value.as_str())
        .ok_or_else(|| CliError::Install("release manifest missing name".to_string()))?;
    if name != "std-cli" {
        return Err(CliError::Install(format!(
            "release manifest name mismatch: {name}"
        )));
    }

    verify_manifest_string(manifest, "package_version")?;
    verify_manifest_string(manifest, "profile")?;
    verify_manifest_string(manifest, "rust_version")?;
    verify_release_target(manifest)
}

pub(crate) fn release_checksum_paths(
    binaries: &[String],
    docs: &[String],
    examples: &[String],
    app_bundles: &[String],
) -> Result<Vec<PathBuf>, CliError> {
    let mut paths = Vec::new();
    paths.extend(binaries.iter().map(PathBuf::from));
    paths.extend(docs.iter().map(PathBuf::from));
    paths.extend(examples.iter().map(PathBuf::from));
    for bundle in app_bundles {
        paths.push(app_bundle_executable(Path::new(bundle))?);
    }
    Ok(paths)
}

pub(crate) fn verify_release_checksums(manifest: &serde_json::Value) -> Result<usize, CliError> {
    let values = manifest
        .get("checksums")
        .and_then(|value| value.as_object())
        .ok_or_else(|| CliError::Install("release manifest missing checksums".to_string()))?;
    if values.is_empty() {
        return Err(CliError::Install(
            "release manifest checksums must not be empty".to_string(),
        ));
    }
    for (path, expected) in values {
        let expected = expected.as_str().ok_or_else(|| {
            CliError::Install(format!("release manifest checksum must be string: {path}"))
        })?;
        let actual = sha256_file(Path::new(path))?;
        if actual != expected {
            return Err(CliError::Install(format!(
                "release checksum mismatch: {path}"
            )));
        }
    }
    Ok(values.len())
}

fn verify_manifest_string(manifest: &serde_json::Value, key: &str) -> Result<(), CliError> {
    let value = manifest
        .get(key)
        .and_then(|value| value.as_str())
        .ok_or_else(|| CliError::Install(format!("release manifest missing {key}")))?;
    if value.trim().is_empty() {
        return Err(CliError::Install(format!(
            "release manifest {key} is empty"
        )));
    }
    Ok(())
}

fn verify_release_target(manifest: &serde_json::Value) -> Result<(), CliError> {
    let target = manifest
        .get("target")
        .and_then(|value| value.as_object())
        .ok_or_else(|| CliError::Install("release manifest missing target".to_string()))?;
    for key in ["os", "arch", "family"] {
        let value = target
            .get(key)
            .and_then(|value| value.as_str())
            .ok_or_else(|| CliError::Install(format!("release manifest target missing {key}")))?;
        if value.trim().is_empty() {
            return Err(CliError::Install(format!(
                "release manifest target {key} is empty"
            )));
        }
    }
    Ok(())
}
