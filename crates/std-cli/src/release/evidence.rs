use crate::{release::manifest::manifest_array, CliError};
use serde_json::{json, Value};
use std::path::Path;

const DEFAULT_SAFE_ENV: &str = "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0";

pub(crate) fn release_evidence(
    version: &str,
    dist_dir: &Path,
    bin_dir: &Path,
    install_prefix: &Path,
) -> Value {
    json!({
        "policy": "current-run-release-install-evidence",
        "safe_env": DEFAULT_SAFE_ENV,
        "commands": [
            format!("{DEFAULT_SAFE_ENV} cargo build --release --workspace"),
            format!("{DEFAULT_SAFE_ENV} mise run quality"),
            format!("{DEFAULT_SAFE_ENV} std release package --version {version} --dist {}", dist_dir.display()),
            format!("{DEFAULT_SAFE_ENV} std release verify --dist {}", dist_dir.display()),
            format!("{DEFAULT_SAFE_ENV} std install run --prefix {} --from {}", install_prefix.display(), bin_dir.display()),
            format!("{DEFAULT_SAFE_ENV} std install verify --prefix {}", install_prefix.display()),
        ],
        "rules": [
            "release_verify_must_pass_current_manifest",
            "install_run_must_use_packaged_bin_dir",
            "install_verify_must_pass_same_prefix",
            "desktop_automation_default_off",
            "ui_preview_default_off",
            "background_ui_automation_default_off",
        ],
    })
}

pub(crate) fn verify_release_evidence(manifest: &Value) -> Result<usize, CliError> {
    let evidence = manifest.get("release_install_evidence").ok_or_else(|| {
        CliError::Install("release manifest missing release_install_evidence".to_string())
    })?;
    verify_string(evidence, "policy", "current-run-release-install-evidence")?;
    verify_string(evidence, "safe_env", DEFAULT_SAFE_ENV)?;
    let commands = manifest_array(evidence, "commands")?;
    let rules = manifest_array(evidence, "rules")?;
    for expected in [
        "cargo build --release --workspace",
        "mise run quality",
        "std release package --version",
        "std release verify --dist",
        "std install run --prefix",
        "std install verify --prefix",
    ] {
        if !commands.iter().any(|command| command.contains(expected)) {
            return Err(CliError::Install(format!(
                "release install evidence missing command: {expected}"
            )));
        }
    }
    for command in &commands {
        if !command.starts_with(DEFAULT_SAFE_ENV) {
            return Err(CliError::Install(format!(
                "release install evidence command missing safe env: {command}"
            )));
        }
    }
    for expected in [
        "release_verify_must_pass_current_manifest",
        "install_run_must_use_packaged_bin_dir",
        "install_verify_must_pass_same_prefix",
        "desktop_automation_default_off",
        "ui_preview_default_off",
        "background_ui_automation_default_off",
    ] {
        if !rules.iter().any(|rule| rule == expected) {
            return Err(CliError::Install(format!(
                "release install evidence missing rule: {expected}"
            )));
        }
    }
    Ok(commands.len())
}

fn verify_string(evidence: &Value, key: &str, expected: &str) -> Result<(), CliError> {
    let value = evidence
        .get(key)
        .and_then(|value| value.as_str())
        .ok_or_else(|| CliError::Install(format!("release install evidence missing {key}")))?;
    if value != expected {
        return Err(CliError::Install(format!(
            "release install evidence {key} mismatch: {value}"
        )));
    }
    Ok(())
}
