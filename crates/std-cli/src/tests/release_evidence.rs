use super::*;

#[test]
fn release_manifest_contains_current_install_evidence() {
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
        serde_json::from_str(&std::fs::read_to_string(manifest_path).unwrap()).unwrap();
    let evidence = &manifest["release_install_evidence"];
    assert_eq!(
        evidence["policy"].as_str(),
        Some("current-run-release-install-evidence")
    );
    assert_eq!(
        evidence["safe_env"].as_str(),
        Some("STD_TEST_MODE=1 STD_ALLOW_DESKTOP_AUTOMATION=0 STD_ALLOW_UI_PREVIEW=0 STD_ALLOW_BACKGROUND_UI_AUTOMATION=0")
    );
    let commands = evidence["commands"].as_array().unwrap();
    for expected in [
        "cargo build --release --workspace",
        "mise run quality",
        "std release package --version 1.0.0",
        "std release verify --dist",
        "std install run --prefix",
        "std install verify --prefix",
    ] {
        assert!(
            commands
                .iter()
                .any(|command| command.as_str().unwrap().contains(expected)),
            "{expected}"
        );
    }
    assert!(verified.contains("release_install_evidence=6"));
}

#[test]
fn release_verify_rejects_missing_install_evidence() {
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
    std::env::remove_var("STDCLI_CONFIG");

    let manifest_path = dist_dir.join("release-manifest.json");
    let mut manifest: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&manifest_path).unwrap()).unwrap();
    manifest
        .as_object_mut()
        .unwrap()
        .remove("release_install_evidence");
    std::fs::write(
        &manifest_path,
        serde_json::to_string_pretty(&manifest).unwrap(),
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
        .contains("release manifest missing release_install_evidence"));
}
