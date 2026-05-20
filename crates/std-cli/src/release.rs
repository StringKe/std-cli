mod checksum;
mod files;
pub(crate) mod macos;
mod manifest;
mod quality;

use crate::CliError;
use checksum::sha256_file as sha256_release_file;
use chrono::Utc;
use files::{copy_tree_files, project_root};
use macos::{create_mac_app_bundle, default_app_bundles, verify_app_bundle};
use manifest::{
    manifest_array, release_checksum_paths, release_profile, release_rust_version,
    release_target_metadata, verify_manifest_paths, verify_release_checksums,
    verify_release_metadata,
};
use quality::{package_quality, quality_paths, verify_quality_manifest};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
};
use std_core::StdCore;

#[cfg(test)]
pub(crate) use checksum::sha256_file;

pub(crate) fn release_plan(core: &StdCore, version: &str) -> Result<String, CliError> {
    let dist_dir = PathBuf::from("dist").join(version);
    let install_prefix = core.config.data_dir.join("install-check");
    let lines = [
        format!("version={version}"),
        format!("dist_dir={}", dist_dir.display()),
        "build=cargo build --release --workspace".to_string(),
        "verify=mise run quality".to_string(),
        "doctor=std doctor".to_string(),
        "workflow_smoke=std plan terminal --save && std run terminal".to_string(),
        "workflow_trace=std workflow trace --limit 5".to_string(),
        "index_coverage=std index coverage".to_string(),
        "plugin_check=std plugin check examples/plugins/hello-js".to_string(),
        "launcher_smoke=std-launcher --smoke \"rebuild index\"".to_string(),
        "external_runner_note=std run <workflow> --allow-external only when external app/file execution is explicitly intended".to_string(),
        format!("package=std release package --version {version} --dist {}", dist_dir.display()),
        format!("release_verify=std release verify --dist {}", dist_dir.display()),
        format!("install_check=std install run --prefix {} --from {} && std install verify --prefix {}", install_prefix.display(), dist_dir.join("bin").display(), install_prefix.display()),
    ];
    Ok(lines.join("\n"))
}

pub(crate) fn release_package(
    core: &StdCore,
    version: &str,
    from: Option<&Path>,
    dist: Option<&Path>,
) -> Result<String, CliError> {
    let source_dir = from
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("target").join("release"));
    let dist_dir = dist
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("dist").join(version));
    let layout = ReleaseLayout::create(&dist_dir)?;

    let binaries = copy_release_binaries(&source_dir, &layout.bin_dir)?;
    let app_bundles = package_app_bundles(&source_dir, &layout.apps_dir, version)?;
    let docs = package_docs(&layout.docs_dir)?;
    let examples = package_examples(&layout.examples_dir)?;
    let quality = package_quality(&layout.quality_dir)?;
    let checksums = release_checksums(&binaries, &docs, &examples, &app_bundles, &quality)?;
    let manifest = json!({
        "name": "std-cli",
        "version": version,
        "package_version": env!("CARGO_PKG_VERSION"),
        "target": release_target_metadata(),
        "profile": release_profile(),
        "rust_version": release_rust_version(),
        "created_at": Utc::now(),
        "dist_dir": dist_dir,
        "source_dir": source_dir,
        "binaries": binaries,
        "app_bundles": app_bundles,
        "docs": docs,
        "examples": examples,
        "quality": quality,
        "checksums": checksums,
        "install_command": format!("std install run --prefix {} --from {}", core.config.data_dir.join("install-check").display(), layout.bin_dir.display()),
    });
    let manifest_path = dist_dir.join("release-manifest.json");
    fs::write(&manifest_path, serde_json::to_string_pretty(&manifest)?)?;

    Ok(format!(
        "release packaged\nversion={version}\ndist_dir={}\nmanifest={}\nbinaries=3\napp_bundles=2\nquality=PASS",
        dist_dir.display(),
        manifest_path.display()
    ))
}

pub(crate) fn release_verify(dist_dir: &Path) -> Result<String, CliError> {
    let manifest_path = dist_dir.join("release-manifest.json");
    if !manifest_path.is_file() {
        return Err(CliError::Install(format!(
            "missing release manifest: {}",
            manifest_path.display()
        )));
    }
    let manifest: serde_json::Value = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;
    let version = manifest
        .get("version")
        .and_then(|value| value.as_str())
        .ok_or_else(|| CliError::Install("release manifest missing version".to_string()))?;
    let binaries = verify_manifest_paths(&manifest, "binaries", true)?;
    let docs = verify_manifest_paths(&manifest, "docs", true)?;
    let examples = verify_manifest_paths(&manifest, "examples", true)?;
    let app_bundles = verify_manifest_paths(&manifest, "app_bundles", false)?;
    for bundle in manifest_array(&manifest, "app_bundles")? {
        verify_app_bundle(Path::new(&bundle))?;
    }
    verify_release_metadata(&manifest)?;
    let quality = verify_quality_manifest(&manifest)?;
    let checksums = verify_release_checksums(&manifest)?;
    verify_install_command(&manifest)?;
    Ok(format!(
        "release verify PASS\nversion={version}\ndist_dir={}\nbinaries={binaries}\napp_bundles={app_bundles}\ndocs={docs}\nexamples={examples}\nquality={quality}\nchecksums={checksums}\nmetadata=PASS\ninstall_command=PASS",
        dist_dir.display()
    ))
}

struct ReleaseLayout {
    bin_dir: PathBuf,
    docs_dir: PathBuf,
    examples_dir: PathBuf,
    quality_dir: PathBuf,
    apps_dir: PathBuf,
}

impl ReleaseLayout {
    fn create(dist_dir: &Path) -> Result<Self, CliError> {
        let layout = Self {
            bin_dir: dist_dir.join("bin"),
            docs_dir: dist_dir.join("docs"),
            examples_dir: dist_dir.join("examples"),
            quality_dir: dist_dir.join("quality"),
            apps_dir: dist_dir.join("Applications"),
        };
        for path in [
            &layout.bin_dir,
            &layout.docs_dir,
            &layout.examples_dir,
            &layout.quality_dir,
            &layout.apps_dir,
        ] {
            fs::create_dir_all(path)?;
        }
        Ok(layout)
    }
}

fn copy_release_binaries(source_dir: &Path, bin_dir: &Path) -> Result<Vec<String>, CliError> {
    let mut binaries = Vec::new();
    for binary in ["std", "std-launcher", "std-studio"] {
        let source = source_dir.join(binary);
        if !source.is_file() {
            return Err(CliError::Install(format!(
                "missing binary: {}",
                source.display()
            )));
        }
        let target = bin_dir.join(binary);
        fs::copy(&source, &target)?;
        binaries.push(target.display().to_string());
    }
    Ok(binaries)
}

fn package_app_bundles(
    source_dir: &Path,
    apps_dir: &Path,
    version: &str,
) -> Result<Vec<String>, CliError> {
    let mut app_bundles = Vec::new();
    for app in default_app_bundles() {
        let source = source_dir.join(app.executable);
        let bundle = create_mac_app_bundle(apps_dir, &source, app, version)?;
        app_bundles.push(bundle.display().to_string());
    }
    Ok(app_bundles)
}

fn package_docs(docs_dir: &Path) -> Result<Vec<String>, CliError> {
    let root = project_root();
    let mut docs = Vec::new();
    let readme = root.join("README.md");
    if readme.is_file() {
        let target = docs_dir.join("README.md");
        fs::copy(&readme, &target)?;
        docs.push(target.display().to_string());
    }
    let project_docs_dir = root.join("docs");
    if project_docs_dir.is_dir() {
        docs.extend(copy_tree_files(
            &project_docs_dir,
            &docs_dir.join("reference"),
        )?);
    }
    Ok(docs)
}

fn package_examples(examples_dir: &Path) -> Result<Vec<String>, CliError> {
    let project_examples_dir = project_root().join("examples");
    if project_examples_dir.is_dir() {
        copy_tree_files(&project_examples_dir, examples_dir)
    } else {
        Ok(Vec::new())
    }
}

fn release_checksums(
    binaries: &[String],
    docs: &[String],
    examples: &[String],
    app_bundles: &[String],
    quality: &[String],
) -> Result<serde_json::Map<String, serde_json::Value>, CliError> {
    let mut checksums = serde_json::Map::new();
    for path in release_checksum_paths(binaries, docs, examples, app_bundles)? {
        checksums.insert(
            path.display().to_string(),
            serde_json::Value::String(sha256_release_file(&path)?),
        );
    }
    for path in quality_paths(quality) {
        checksums.insert(
            path.display().to_string(),
            serde_json::Value::String(sha256_release_file(&path)?),
        );
    }
    Ok(checksums)
}

fn verify_install_command(manifest: &serde_json::Value) -> Result<(), CliError> {
    let install_command = manifest
        .get("install_command")
        .and_then(|value| value.as_str())
        .ok_or_else(|| CliError::Install("release manifest missing install_command".to_string()))?;
    if !install_command.contains("std install run") {
        return Err(CliError::Install(
            "release manifest install_command must use std install run".to_string(),
        ));
    }
    Ok(())
}
