use crate::CliError;
use std::{
    fs,
    path::{Path, PathBuf},
};

const MAX_SOURCE_FILE_LINES: usize = 500;
const MAX_CONFIG_FILE_LINES: usize = 300;

pub(crate) struct WorkspaceDoctor {
    pub(crate) source_files: usize,
    pub(crate) max_source_file: PathBuf,
    pub(crate) max_source_lines: usize,
    pub(crate) workspace_crates: usize,
    pub(crate) source_file_limit: usize,
    pub(crate) config_file_limit: usize,
    pub(crate) quality_ci: &'static str,
    pub(crate) dylint_lint: &'static str,
}

pub(crate) fn check_workspace_quality() -> Result<WorkspaceDoctor, CliError> {
    let root = find_workspace_root()?;
    let cargo = read_required(&root.join("Cargo.toml"))?;
    check_workspace_manifest(&cargo)?;
    check_quality_configs(&root)?;
    check_surface_crates(&root)?;

    let mut scan = SourceScan::default();
    scan_source_files(&root.join("crates"), &mut scan)?;
    if scan.source_files == 0 {
        return Err(CliError::Doctor(
            "workspace has no Rust source files".to_string(),
        ));
    }
    Ok(WorkspaceDoctor {
        source_files: scan.source_files,
        max_source_file: scan.max_source_file,
        max_source_lines: scan.max_source_lines,
        workspace_crates: cargo.matches("crates/").count(),
        source_file_limit: MAX_SOURCE_FILE_LINES,
        config_file_limit: MAX_CONFIG_FILE_LINES,
        quality_ci: "PASS",
        dylint_lint: "PASS",
    })
}

pub(crate) fn check_text(text: &str, required: &str) -> Result<(), CliError> {
    if !text.contains(required) {
        return Err(CliError::Doctor(format!(
            "required text missing: {required}"
        )));
    }
    Ok(())
}

fn check_workspace_manifest(cargo: &str) -> Result<(), CliError> {
    for required in [
        "crates/std-core",
        "crates/std-orchestration",
        "crates/std-index",
        "crates/std-egui",
        "crates/std-launcher",
        "crates/std-studio",
        "crates/std-cli",
        "exclude = [\"crates/file_too_long\"]",
        "[workspace.metadata.dylint]",
        "path = \"crates/file_too_long\"",
        "[workspace.lints.clippy]",
    ] {
        check_text(cargo, required)?;
    }
    Ok(())
}

fn check_quality_configs(root: &Path) -> Result<(), CliError> {
    check_config_file(&root.join("Cargo.toml"))?;
    for config in ["Makefile", "clippy.toml", "rustfmt.toml", "deny.toml"] {
        check_config_file(&root.join(config))?;
    }
    check_ci_quality_workflow(root)?;
    check_rust_quality_configs(root)?;
    check_dylint_lint(root)?;

    let makefile = read_required(&root.join("Makefile"))?;
    for required in [
        "cargo fmt --all --check",
        "cargo clippy --workspace --all-targets -- -D warnings",
        "cargo dylint --workspace --all -- --all-targets",
        "cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml",
        "cargo deny check",
        "cargo machete",
    ] {
        check_text(&makefile, required)?;
    }
    Ok(())
}

fn check_ci_quality_workflow(root: &Path) -> Result<(), CliError> {
    let workflow = read_required(&root.join(".github/workflows/quality.yml"))?;
    check_config_file(&root.join(".github/workflows/quality.yml"))?;
    for required in [
        "components: rustfmt, clippy",
        "rustup toolchain install nightly-2025-09-18",
        "cargo install cargo-dylint --version 5.0.0 --locked",
        "cargo install dylint-link --version 5.0.0 --locked",
        "cargo install cargo-deny --locked",
        "cargo install cargo-machete --locked",
        "DYLINT_RUSTFLAGS=\"-D warnings\" cargo dylint --workspace --all -- --all-targets",
        "cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml",
    ] {
        check_text(&workflow, required)?;
    }
    Ok(())
}

fn check_rust_quality_configs(root: &Path) -> Result<(), CliError> {
    let clippy = read_required(&root.join("clippy.toml"))?;
    for required in [
        "too-many-lines-threshold = 120",
        "too-many-arguments-threshold = 6",
        "cognitive-complexity-threshold = 25",
        "allow-unwrap-in-tests = true",
    ] {
        check_text(&clippy, required)?;
    }
    let rustfmt = read_required(&root.join("rustfmt.toml"))?;
    for required in ["max_width = 100", "edition = \"2021\""] {
        check_text(&rustfmt, required)?;
    }
    let deny = read_required(&root.join("deny.toml"))?;
    for required in ["yanked = \"deny\"", "wildcards = \"deny\""] {
        check_text(&deny, required)?;
    }
    Ok(())
}

fn check_dylint_lint(root: &Path) -> Result<(), CliError> {
    check_text(
        &read_required(&root.join("crates/file_too_long/src/lib.rs"))?,
        "const MAX_SOURCE_FILE_LINES: usize = 500;",
    )?;
    check_text(
        &read_required(&root.join("crates/file_too_long/Cargo.toml"))?,
        "dylint_linting = \"5.0.0\"",
    )?;
    check_text(
        &read_required(&root.join("crates/file_too_long/rust-toolchain"))?,
        "channel = \"nightly-2025-09-18\"",
    )?;
    check_text(
        &read_required(&root.join("crates/file_too_long/.cargo/config.toml"))?,
        "dylint-link",
    )
}

fn check_surface_crates(root: &Path) -> Result<(), CliError> {
    check_text(
        &read_required(&root.join("crates/std-launcher/Cargo.toml"))?,
        "name = \"std-launcher\"",
    )?;
    check_text(
        &read_required(&root.join("crates/std-studio/Cargo.toml"))?,
        "name = \"std-studio\"",
    )
}

fn find_workspace_root() -> Result<PathBuf, CliError> {
    let mut current = std::env::current_dir()?;
    loop {
        if current.join("Cargo.toml").is_file() && current.join("crates").is_dir() {
            return Ok(current);
        }
        if !current.pop() {
            return Err(CliError::Doctor("workspace root not found".to_string()));
        }
    }
}

fn read_required(path: &Path) -> Result<String, CliError> {
    if !path.is_file() {
        return Err(CliError::Doctor(format!(
            "required file missing: {}",
            path.display()
        )));
    }
    Ok(fs::read_to_string(path)?)
}

fn check_config_file(path: &Path) -> Result<(), CliError> {
    let text = read_required(path)?;
    let lines = text.lines().count();
    if lines > MAX_CONFIG_FILE_LINES {
        return Err(CliError::Doctor(format!(
            "{} has {lines} lines, maximum is {MAX_CONFIG_FILE_LINES}",
            path.display()
        )));
    }
    Ok(())
}

#[derive(Default)]
struct SourceScan {
    source_files: usize,
    max_source_file: PathBuf,
    max_source_lines: usize,
}

fn scan_source_files(dir: &Path, scan: &mut SourceScan) -> Result<(), CliError> {
    if !dir.is_dir() {
        return Ok(());
    }
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.file_name().and_then(|name| name.to_str()) == Some("target") {
            continue;
        }
        if path.is_dir() {
            scan_source_files(&path, scan)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            check_source_file(&path, scan)?;
        }
    }
    Ok(())
}

fn check_source_file(path: &Path, scan: &mut SourceScan) -> Result<(), CliError> {
    let lines = fs::read_to_string(path)?.lines().count();
    if lines > MAX_SOURCE_FILE_LINES {
        return Err(CliError::Doctor(format!(
            "{} has {lines} lines, maximum is {MAX_SOURCE_FILE_LINES}",
            path.display()
        )));
    }
    scan.source_files += 1;
    if lines > scan.max_source_lines {
        scan.max_source_lines = lines;
        scan.max_source_file = path.to_path_buf();
    }
    Ok(())
}
