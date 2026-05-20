use crate::{
    release::{files::project_root, manifest::manifest_array},
    CliError,
};
use std::{
    fs,
    path::{Path, PathBuf},
};

const QUALITY_FILES: [&str; 5] = [
    "Cargo.toml",
    "mise.toml",
    "clippy.toml",
    "rustfmt.toml",
    "deny.toml",
];

const QUALITY_COMMANDS: [&str; 8] = [
    "cargo fmt --all --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "DYLINT_RUSTFLAGS=\"-D warnings\" cargo dylint --workspace --all -- --all-targets",
    "cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml",
    "cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib",
    "cargo test --workspace -- --test-threads=1",
    "cargo deny check",
    "cargo machete",
];

const SMOKE_COMMANDS: [&str; 15] = [
    "std doctor",
    "std-launcher --smoke \"rebuild index\"",
    "std-launcher --window-smoke",
    "std-launcher --theme-smoke",
    "std-launcher --surface-smoke",
    "std-launcher --ui-semantics-smoke index",
    "std-launcher --keyboard-smoke index",
    "std-launcher --preview-smoke",
    "std-studio --smoke",
    "std-studio --workspace-policy-smoke",
    "std-studio --theme-smoke",
    "std-studio --preview-smoke",
    "std workflow trace --limit 5",
    "std index coverage",
    "std plugin check examples/plugins/hello-js",
];

const MANUAL_DESKTOP_ACCEPTANCE: [&str; 1] =
    ["STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space"];

pub(crate) fn package_quality(quality_dir: &Path) -> Result<Vec<String>, CliError> {
    fs::create_dir_all(quality_dir)?;
    let root = project_root();
    let mut packaged = Vec::new();
    for file in QUALITY_FILES {
        let source = root.join(file);
        if !source.is_file() {
            return Err(CliError::Install(format!(
                "quality file missing: {}",
                source.display()
            )));
        }
        let target = quality_dir.join(file);
        fs::copy(source, &target)?;
        packaged.push(target.display().to_string());
    }
    let report = quality_report();
    let report_path = quality_dir.join("quality-report.txt");
    fs::write(&report_path, report)?;
    packaged.push(report_path.display().to_string());
    packaged.sort();
    Ok(packaged)
}

pub(crate) fn verify_quality_manifest(manifest: &serde_json::Value) -> Result<usize, CliError> {
    let paths = manifest_array(manifest, "quality")?;
    if paths.len() < QUALITY_FILES.len() + 1 {
        return Err(CliError::Install(
            "release manifest quality evidence is incomplete".to_string(),
        ));
    }
    let mut report_found = false;
    for path in &paths {
        let path = PathBuf::from(path);
        if !path.is_file() {
            return Err(CliError::Install(format!(
                "release manifest quality path missing: {}",
                path.display()
            )));
        }
        if path.file_name().and_then(|name| name.to_str()) == Some("quality-report.txt") {
            report_found = true;
            verify_quality_report(&path)?;
        }
    }
    if !report_found {
        return Err(CliError::Install(
            "release manifest missing quality-report.txt".to_string(),
        ));
    }
    Ok(paths.len())
}

pub(crate) fn quality_paths(paths: &[String]) -> Vec<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

fn quality_report() -> String {
    let mut lines = vec![
        "std-cli quality gate".to_string(),
        "task_runner=mise".to_string(),
        "quality_command=mise run quality".to_string(),
        "source_file_limit=500".to_string(),
        "config_file_limit=300".to_string(),
    ];
    for command in QUALITY_COMMANDS {
        lines.push(format!("command={command}"));
    }
    for command in SMOKE_COMMANDS {
        lines.push(format!("smoke={command}"));
    }
    for command in MANUAL_DESKTOP_ACCEPTANCE {
        lines.push(format!("manual_desktop_acceptance={command}"));
    }
    lines.join("\n")
}

fn verify_quality_report(path: &Path) -> Result<(), CliError> {
    let body = fs::read_to_string(path)?;
    for expected in [
        "source_file_limit=500",
        "config_file_limit=300",
        "task_runner=mise",
        "quality_command=mise run quality",
        "cargo fmt --all --check",
        "cargo clippy --workspace --all-targets -- -D warnings",
        "cargo dylint --workspace --all -- --all-targets",
        "cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml",
        "cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib",
        "cargo test --workspace -- --test-threads=1",
        "cargo deny check",
        "cargo machete",
        "std doctor",
        "std-launcher --smoke \"rebuild index\"",
        "std-launcher --window-smoke",
        "std-launcher --theme-smoke",
        "std-launcher --surface-smoke",
        "std-launcher --ui-semantics-smoke index",
        "std-launcher --keyboard-smoke index",
        "std-launcher --preview-smoke",
        "std-studio --smoke",
        "std-studio --workspace-policy-smoke",
        "std-studio --theme-smoke",
        "std-studio --preview-smoke",
        "std workflow trace --limit 5",
        "std index coverage",
        "std plugin check examples/plugins/hello-js",
        "manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space",
    ] {
        if !body.contains(expected) {
            return Err(CliError::Install(format!(
                "quality report missing: {expected}"
            )));
        }
    }
    Ok(())
}
