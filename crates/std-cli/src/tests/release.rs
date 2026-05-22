use super::*;
use crate::release::sha256_file;

#[test]
fn release_package_copies_binaries_docs_and_manifest() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let source_dir = temp.path().join("release");
    let dist_dir = temp.path().join("dist").join("1.0.0");
    std::fs::create_dir_all(&source_dir).unwrap();
    for binary in ["std", "std-launcher", "std-studio"] {
        std::fs::write(source_dir.join(binary), format!("{binary}\n")).unwrap();
    }
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli([
        "std",
        "release",
        "package",
        "--version",
        "1.0.0",
        "--from",
        source_dir.to_str().unwrap(),
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap();
    let verified = run_cli([
        "std",
        "release",
        "verify",
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    let manifest_path = dist_dir.join("release-manifest.json");
    let manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();

    assert!(output.contains("release packaged"));
    assert!(output.contains("app_bundles=2"));
    assert!(output.contains("quality=PASS"));
    assert_release_files_exist(&dist_dir);
    let launcher_plist = std::fs::read_to_string(
        dist_dir
            .join("Applications")
            .join("std Launcher.app")
            .join("Contents")
            .join("Info.plist"),
    )
    .unwrap();
    assert!(launcher_plist.contains("<string>com.stringke.std-cli.launcher</string>"));
    assert!(launcher_plist.contains("<string>std-launcher</string>"));
    assert!(launcher_plist.contains("<key>LSUIElement</key>"));
    assert!(launcher_plist.contains("<true/>"));
    assert_manifest_metadata(&manifest_path, &manifest);
    assert_release_verify_output(&verified);
}

fn assert_release_files_exist(dist_dir: &std::path::Path) {
    for path in [
        dist_dir.join("bin").join("std"),
        dist_dir.join("bin").join("std-launcher"),
        dist_dir.join("bin").join("std-studio"),
        dist_dir.join("docs").join("README.md"),
        dist_dir
            .join("docs")
            .join("reference")
            .join("10_Tool_and_Plugin_System.md"),
        dist_dir
            .join("examples")
            .join("plugins")
            .join("hello-js")
            .join("plugin.json"),
        dist_dir
            .join("examples")
            .join("plugins")
            .join("scoped-fs")
            .join("main.js"),
        dist_dir.join("quality").join("Cargo.toml"),
        dist_dir.join("quality").join("mise.toml"),
        dist_dir.join("quality").join("clippy.toml"),
        dist_dir.join("quality").join("rustfmt.toml"),
        dist_dir.join("quality").join("deny.toml"),
        dist_dir.join("quality").join("quality-report.txt"),
        dist_dir
            .join("Applications")
            .join("std Launcher.app")
            .join("Contents")
            .join("MacOS")
            .join("std-launcher"),
        dist_dir
            .join("Applications")
            .join("std Studio.app")
            .join("Contents")
            .join("MacOS")
            .join("std-studio"),
    ] {
        assert!(path.is_file(), "missing release file: {}", path.display());
    }
}

fn assert_manifest_metadata(manifest_path: &std::path::Path, manifest: &serde_json::Value) {
    assert!(manifest_path.is_file());
    assert_eq!(manifest["version"].as_str(), Some("1.0.0"));
    assert_eq!(manifest["name"].as_str(), Some("std-cli"));
    assert_eq!(
        manifest["package_version"].as_str(),
        Some(env!("CARGO_PKG_VERSION"))
    );
    assert!(manifest["profile"].as_str().is_some());
    assert!(manifest["rust_version"].as_str().is_some());
    assert!(manifest["target"]["os"].as_str().is_some());
    assert!(manifest["target"]["arch"].as_str().is_some());
    assert!(manifest["target"]["family"].as_str().is_some());
    assert_eq!(manifest["binaries"].as_array().unwrap().len(), 3);
    assert_eq!(manifest["app_bundles"].as_array().unwrap().len(), 2);
    assert!(manifest["docs"].as_array().unwrap().len() >= 2);
    assert!(manifest["examples"].as_array().unwrap().len() >= 3);
    assert_eq!(manifest["quality"].as_array().unwrap().len(), 6);
    assert!(manifest["checksums"].as_object().unwrap().len() >= 16);
    assert!(manifest["install_command"]
        .as_str()
        .unwrap()
        .contains("std install run"));
    assert_quality_report_smoke_commands(manifest);
}

fn assert_quality_report_smoke_commands(manifest: &serde_json::Value) {
    let quality_report = std::fs::read_to_string(
        manifest["quality"]
            .as_array()
            .unwrap()
            .iter()
            .find_map(|path| {
                let path = path.as_str()?;
                path.ends_with("quality-report.txt").then_some(path)
            })
            .unwrap(),
    )
    .unwrap();
    for expected in [
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std doctor",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --smoke \"rebuild index\"",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --window-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --theme-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --surface-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --ui-semantics-smoke index",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --keyboard-smoke index",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --action-panel-smoke index",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-launcher --preview-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --workspace-policy-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --theme-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --surface-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std-studio --preview-smoke",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std workflow trace --limit 5",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std index coverage",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std plugin check examples/plugins/hello-js",
        "smoke=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 std install runtime-evidence --prefix .std-cli/install-check",
        "manual_desktop_acceptance=STD_ALLOW_DESKTOP_AUTOMATION=1 std-launcher --gui-hotkey-smoke Alt+Space",
        "background_ui_acceptance=STD_ALLOW_BACKGROUND_UI_AUTOMATION=1 cargo run -p std-cli -- ui background-smoke --harness-pid <pid> --window-id <window-id> --bundle-id dev.std-cli.background-ui-harness --window-title \"std-cli Background UI Harness <token>\" --harness-token <token>",
        "quality_command=mise run quality",
        "command=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo test -p std-cli workspace_file_limits_cover_sources_and_configs --lib",
        "command=STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0 cargo run -p std-egui --example a11y-audit",
    ] {
        assert!(quality_report.contains(expected));
    }
    assert!(!quality_report.contains("smoke=STD_ALLOW_DESKTOP_AUTOMATION=1"));
    assert!(!quality_report.contains("smoke=STD_ALLOW_UI_PREVIEW=1"));
    assert!(!quality_report.contains("smoke=std-launcher --"));
    assert!(!quality_report.contains("smoke=std-studio --"));
}

fn assert_release_verify_output(verified: &str) {
    for expected in [
        "release verify PASS",
        "binaries=3",
        "app_bundles=2",
        "docs=",
        "examples=",
        "quality=6",
        "checksums=",
        "metadata=PASS",
        "install_command=PASS",
    ] {
        assert!(verified.contains(expected));
    }
}

#[test]
fn release_verify_rejects_checksum_mismatch() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let source_dir = temp.path().join("release");
    let dist_dir = temp.path().join("dist").join("1.0.0");
    std::fs::create_dir_all(&source_dir).unwrap();
    for binary in ["std", "std-launcher", "std-studio"] {
        std::fs::write(source_dir.join(binary), format!("{binary}\n")).unwrap();
    }
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    run_cli([
        "std",
        "release",
        "package",
        "--version",
        "1.0.0",
        "--from",
        source_dir.to_str().unwrap(),
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap();
    std::fs::write(dist_dir.join("bin").join("std"), "tampered").unwrap();
    let error = run_cli([
        "std",
        "release",
        "verify",
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap_err();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(error.to_string().contains("release checksum mismatch"));
}

#[test]
fn release_verify_rejects_missing_manifest_paths() {
    let temp = tempfile::tempdir().unwrap();
    let dist_dir = temp.path().join("dist").join("1.0.0");
    std::fs::create_dir_all(dist_dir.join("bin")).unwrap();
    std::fs::write(dist_dir.join("bin").join("std"), "std").unwrap();
    std::fs::write(
        dist_dir.join("release-manifest.json"),
        serde_json::json!({
            "version": "1.0.0",
            "binaries": [
                dist_dir.join("bin").join("std"),
                dist_dir.join("bin").join("missing")
            ],
            "app_bundles": [],
            "docs": [],
            "install_command": "std install run --prefix /tmp/std --from /tmp/bin"
        })
        .to_string(),
    )
    .unwrap();

    let error = run_cli([
        "std",
        "release",
        "verify",
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap_err();

    assert!(error
        .to_string()
        .contains("release manifest binaries path missing"));
}

#[test]
fn release_verify_requires_packaged_examples() {
    let temp = tempfile::tempdir().unwrap();
    let dist_dir = temp.path().join("dist").join("1.0.0");
    let bin_dir = dist_dir.join("bin");
    let docs_dir = dist_dir.join("docs");
    let apps_dir = dist_dir.join("Applications");
    std::fs::create_dir_all(&bin_dir).unwrap();
    std::fs::create_dir_all(&docs_dir).unwrap();
    std::fs::create_dir_all(&apps_dir).unwrap();
    let binary = bin_dir.join("std");
    let doc = docs_dir.join("README.md");
    std::fs::write(&binary, "std").unwrap();
    std::fs::write(&doc, "docs").unwrap();
    let mut checksums = serde_json::Map::new();
    checksums.insert(
        binary.display().to_string(),
        serde_json::Value::String(sha256_file(&binary).unwrap()),
    );
    checksums.insert(
        doc.display().to_string(),
        serde_json::Value::String(sha256_file(&doc).unwrap()),
    );
    std::fs::write(
        dist_dir.join("release-manifest.json"),
        serde_json::json!({
            "name": "std-cli",
            "version": "1.0.0",
            "package_version": env!("CARGO_PKG_VERSION"),
            "target": {
                "os": std::env::consts::OS,
                "arch": std::env::consts::ARCH,
                "family": std::env::consts::FAMILY
            },
            "profile": "debug",
            "rust_version": "1.80",
            "binaries": [binary.display().to_string()],
            "docs": [doc.display().to_string()],
            "quality": [],
            "app_bundles": [],
            "checksums": checksums,
            "install_command": "std install run --prefix /tmp/std --from /tmp/std/bin"
        })
        .to_string(),
    )
    .unwrap();

    let error = run_cli([
        "std",
        "release",
        "verify",
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap_err();

    assert!(error
        .to_string()
        .contains("release manifest missing array: examples"));
}

#[test]
fn release_verify_requires_quality_evidence() {
    let temp = tempfile::tempdir().unwrap();
    let dist_dir = temp.path().join("dist").join("1.0.0");
    let bin_dir = dist_dir.join("bin");
    let docs_dir = dist_dir.join("docs");
    let examples_dir = dist_dir.join("examples");
    std::fs::create_dir_all(&bin_dir).unwrap();
    std::fs::create_dir_all(&docs_dir).unwrap();
    std::fs::create_dir_all(&examples_dir).unwrap();
    let binary = bin_dir.join("std");
    let doc = docs_dir.join("README.md");
    let example = examples_dir.join("smoke.workflow.json");
    std::fs::write(&binary, "std").unwrap();
    std::fs::write(&doc, "docs").unwrap();
    std::fs::write(&example, "{}").unwrap();
    let mut checksums = serde_json::Map::new();
    checksums.insert(
        binary.display().to_string(),
        serde_json::Value::String(sha256_file(&binary).unwrap()),
    );
    checksums.insert(
        doc.display().to_string(),
        serde_json::Value::String(sha256_file(&doc).unwrap()),
    );
    checksums.insert(
        example.display().to_string(),
        serde_json::Value::String(sha256_file(&example).unwrap()),
    );
    std::fs::write(
        dist_dir.join("release-manifest.json"),
        serde_json::json!({
            "name": "std-cli",
            "version": "1.0.0",
            "package_version": env!("CARGO_PKG_VERSION"),
            "target": {
                "os": std::env::consts::OS,
                "arch": std::env::consts::ARCH,
                "family": std::env::consts::FAMILY
            },
            "profile": "debug",
            "rust_version": "1.80",
            "binaries": [binary.display().to_string()],
            "docs": [doc.display().to_string()],
            "examples": [example.display().to_string()],
            "quality": [],
            "app_bundles": [],
            "checksums": checksums,
            "install_command": "std install run --prefix /tmp/std --from /tmp/std/bin"
        })
        .to_string(),
    )
    .unwrap();

    let error = run_cli([
        "std",
        "release",
        "verify",
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap_err();

    assert!(error
        .to_string()
        .contains("release manifest quality evidence is incomplete"));
}

#[test]
fn release_verify_rejects_missing_metadata() {
    let temp = tempfile::tempdir().unwrap();
    let dist_dir = temp.path().join("dist").join("1.0.0");
    let bin_dir = dist_dir.join("bin");
    let examples_dir = dist_dir.join("examples");
    std::fs::create_dir_all(&bin_dir).unwrap();
    std::fs::create_dir_all(&examples_dir).unwrap();
    let binary = bin_dir.join("std");
    let example = examples_dir.join("smoke.workflow.json");
    std::fs::write(&binary, "std").unwrap();
    std::fs::write(&example, "{}").unwrap();
    let mut checksums = serde_json::Map::new();
    checksums.insert(
        binary.display().to_string(),
        serde_json::Value::String(sha256_file(&binary).unwrap()),
    );
    checksums.insert(
        example.display().to_string(),
        serde_json::Value::String(sha256_file(&example).unwrap()),
    );
    std::fs::write(
        dist_dir.join("release-manifest.json"),
        serde_json::json!({
            "version": "1.0.0",
            "binaries": [binary],
            "app_bundles": [],
            "docs": [],
            "examples": [example],
            "quality": [],
            "checksums": checksums,
            "install_command": "std install run --prefix /tmp/std --from /tmp/bin"
        })
        .to_string(),
    )
    .unwrap();

    let error = run_cli([
        "std",
        "release",
        "verify",
        "--dist",
        dist_dir.to_str().unwrap(),
    ])
    .unwrap_err();

    assert!(error.to_string().contains("release manifest missing name"));
}
