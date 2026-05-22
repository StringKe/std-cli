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

const QUALITY_COMMANDS: [&str; 9] = [
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo fmt --all --check",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo clippy --workspace --all-targets -- -D warnings",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 DYLINT_RUSTFLAGS=\"-D warnings\" cargo dylint --workspace --all -- --all-targets",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo run -p std-egui --example a11y-audit",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test --workspace -- --test-threads=1",
    "cargo deny check",
    "cargo machete",
];

const SMOKE_COMMANDS: [&str; 21] = [
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std doctor",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --smoke \"rebuild index\"",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --window-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --theme-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --surface-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --ui-semantics-smoke index",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --keyboard-smoke index",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --action-panel-smoke index",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --app-localization-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --user-enter-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --close-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --preview-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --workspace-policy-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --theme-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --surface-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --preview-smoke",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std workflow trace --limit 5",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std index coverage",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std plugin check examples/plugins/hello-js",
    "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std install runtime-evidence --prefix .std-cli/install-check",
];

const MANUAL_DESKTOP_ACCEPTANCE: [&str; 1] =
    ["STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space"];
const BACKGROUND_UI_ACCEPTANCE: [&str; 2] = [
    "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-acceptance.sh",
    "STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title \"std-cli Background UI Harness <token>\" --harness-token <token>",
];
const MANUAL_UI_EVIDENCE: [&str; 6] = [
    "ui_capture_manifest=STD_UI_CAPTURE_MANIFEST=artifacts/ui/manual-acceptance/manifest.txt",
    "ui_capture_command=STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix",
    "ui_capture_rule=current-run-png-only",
    "background_ui_manifest=STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST=artifacts/ui/background-acceptance/manifest.txt",
    "background_ui_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance",
    "background_ui_rule=isolated-harness-only",
];

const UI_CAPTURE_EVIDENCE_RULES: [&str; 2] = [
    "ui_capture_pixels=samples+opaque_samples+unique_colors+black_pixels+white_pixels+transparent_pixels",
    "ui_capture_rejects=single-color+dominant-black+dominant-white-carrier",
];

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
    for command in BACKGROUND_UI_ACCEPTANCE {
        lines.push(format!("background_ui_acceptance={command}"));
    }
    for evidence in MANUAL_UI_EVIDENCE {
        lines.push(format!("manual_ui_evidence={evidence}"));
    }
    for rule in UI_CAPTURE_EVIDENCE_RULES {
        lines.push(format!("manual_ui_evidence_rule={rule}"));
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
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo fmt --all --check",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo clippy --workspace --all-targets -- -D warnings",
        "cargo dylint --workspace --all -- --all-targets",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo +nightly-2025-09-18 test --manifest-path crates/file_too_long/Cargo.toml",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo run -p std-egui --example a11y-audit",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test --workspace -- --test-threads=1",
        "cargo deny check",
        "cargo machete",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std doctor",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --smoke \"rebuild index\"",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --window-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --theme-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --surface-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --ui-semantics-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --keyboard-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --action-panel-smoke index",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --app-localization-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --user-enter-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --close-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --preview-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --workspace-policy-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --theme-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --surface-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --preview-smoke",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std workflow trace --limit 5",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std index coverage",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std plugin check examples/plugins/hello-js",
        "STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std install runtime-evidence --prefix .std-cli/install-check",
        "manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space",
        "background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 scripts/background-ui-acceptance.sh",
        "background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title \"std-cli Background UI Harness <token>\" --harness-token <token>",
        "manual_ui_evidence=ui_capture_manifest=STD_UI_CAPTURE_MANIFEST=artifacts/ui/manual-acceptance/manifest.txt",
        "manual_ui_evidence=ui_capture_command=STD_ALLOW_UI_PREVIEW=1 mise run ui-capture-matrix",
        "manual_ui_evidence=ui_capture_rule=current-run-png-only",
        "manual_ui_evidence=background_ui_manifest=STD_BACKGROUND_UI_ACCEPTANCE_MANIFEST=artifacts/ui/background-acceptance/manifest.txt",
        "manual_ui_evidence=background_ui_command=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 mise run ui-background-acceptance",
        "manual_ui_evidence=background_ui_rule=isolated-harness-only",
        "manual_ui_evidence_rule=ui_capture_pixels=samples+opaque_samples+unique_colors+black_pixels+white_pixels+transparent_pixels",
        "manual_ui_evidence_rule=ui_capture_rejects=single-color+dominant-black+dominant-white-carrier",
    ] {
        if !body.contains(expected) {
            return Err(CliError::Install(format!(
                "quality report missing: {expected}"
            )));
        }
    }
    Ok(())
}
