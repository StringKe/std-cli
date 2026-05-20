use super::*;

#[test]
fn config_command_prints_storage_paths() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "launcher_hotkey": "Alt+Space",
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "config"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("launcher_hotkey=Alt+Space"));
    assert!(output.contains("workflows_dir="));
}

#[test]
fn config_commands_get_set_list_and_path() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "launcher_hotkey": "Alt+Space",
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let path = run_cli(["std", "config", "path"]).unwrap();
    let before = run_cli(["std", "config", "get", "launcher_hotkey"]).unwrap();
    let set_hotkey = run_cli(["std", "config", "set", "launcher_hotkey", "Cmd+Space"]).unwrap();
    let set_ai = run_cli(["std", "config", "set", "enable_ai", "true"]).unwrap();
    let after = run_cli(["std", "config", "get", "launcher_hotkey"]).unwrap();
    let listed = run_cli(["std", "config", "list"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    let saved: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&config_path).unwrap()).unwrap();
    assert_eq!(path, config_path.display().to_string());
    assert_eq!(before, "Alt+Space");
    assert!(set_hotkey.contains("launcher_hotkey=Cmd+Space"));
    assert!(set_ai.contains("enable_ai=true"));
    assert_eq!(after, "Cmd+Space");
    assert!(listed.contains("launcher_hotkey=Cmd+Space"));
    assert_eq!(saved["launcher_hotkey"].as_str(), Some("Cmd+Space"));
    assert_eq!(saved["enable_ai"].as_bool(), Some(true));
}

#[test]
fn config_command_rejects_invalid_environment_field() {
    std::env::set_var("STDCLI_ENABLE_AI", "yes");

    let error = run_cli(["std", "config", "get", "enable_ai"]).unwrap_err();

    std::env::remove_var("STDCLI_ENABLE_AI");

    assert!(error.to_string().contains("Config error"));
    assert!(error.to_string().contains("STDCLI_ENABLE_AI invalid"));
}

#[test]
fn search_command_finds_builtin_action() {
    let output = run_cli(["std", "search", "terminal"]).unwrap();

    assert!(output.contains("Open Terminal"));
}

#[test]
fn doctor_command_checks_local_runtime_surface() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "doctor"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert_doctor_runtime_output(&output);
    assert_doctor_ui_output(&output);
    assert!(temp.path().join("data").join("workflows").is_dir());
    assert!(temp.path().join("data").join("index").is_dir());
    assert!(temp.path().join("data").join("memory").is_dir());
}

fn assert_doctor_runtime_output(output: &str) {
    assert!(output.contains("doctor PASS"));
    assert!(output.contains("storage=PASS"));
    assert!(output.contains("planner=PASS"));
    assert!(output.contains("workflow_dry_run=PASS"));
    assert!(output.contains("index_components="));
    assert!(output.contains("plugins=0"));
    assert!(output.contains("quality=PASS"));
    assert!(output.contains("quality_ci=PASS"));
    assert!(output.contains("dylint_lint=PASS"));
    assert!(output.contains("quality_tools=rustfmt,clippy,dylint,cargo-deny,cargo-machete"));
    assert!(output.contains("source_file_limit=500"));
    assert!(output.contains("config_file_limit=300"));
    assert!(output.contains("config_files="));
    assert!(output.contains("max_config_file="));
    assert!(output.contains("workspace_crates="));
    assert!(output.contains("launcher=PASS"));
    assert!(output.contains("studio=PASS"));
    assert!(output.contains("release_plan=PASS"));
    assert!(output.contains("install_plan=PASS"));
    assert!(output.contains("config_path="));
}

fn assert_doctor_ui_output(output: &str) {
    assert!(output.contains("ui_docs=PASS"));
    assert!(output.contains("ui_docs_count=7"));
    assert!(output.contains("launcher_ui_gates=theme-smoke,surface-smoke"));
    assert!(
        output.contains("studio_ui_gates=smoke,workspace-policy-smoke,theme-smoke,preview-smoke")
    );
    assert!(output.contains("desktop_automation_default=blocked"));
    assert!(output.contains("manual_desktop_acceptance=explicit_opt_in_only"));
    assert!(output.contains("ui_completion=INCOMPLETE_REAL_GUI_REQUIRED"));
}

#[test]
fn doctor_command_can_print_machine_readable_json() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "doctor", "--json"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    let report: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(report["status"].as_str(), Some("PASS"));
    assert_eq!(report["storage"].as_str(), Some("PASS"));
    assert_eq!(report["quality"].as_str(), Some("PASS"));
    assert_eq!(report["quality_ci"].as_str(), Some("PASS"));
    assert_eq!(report["dylint_lint"].as_str(), Some("PASS"));
    assert_eq!(report["source_file_limit"].as_u64(), Some(500));
    assert_eq!(report["config_file_limit"].as_u64(), Some(300));
    assert!(report["config_files"].as_u64().unwrap() > 0);
    assert!(report["max_config_lines"].as_u64().unwrap() <= 300);
    assert_eq!(report["launcher"].as_str(), Some("PASS"));
    assert_eq!(report["studio"].as_str(), Some("PASS"));
    assert_eq!(report["ui_docs"].as_str(), Some("PASS"));
    assert_eq!(report["ui_docs_count"].as_u64(), Some(7));
    assert_eq!(
        report["manual_desktop_acceptance"].as_str(),
        Some("explicit_opt_in_only")
    );
    assert_eq!(
        report["desktop_automation_default"].as_str(),
        Some("blocked")
    );
    assert_eq!(
        report["ui_completion"].as_str(),
        Some("INCOMPLETE_REAL_GUI_REQUIRED")
    );
    assert!(report["launcher_ui_gates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|gate| gate.as_str() == Some("preview-smoke")));
    assert!(report["studio_ui_gates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|gate| gate.as_str() == Some("workspace-policy-smoke")));
    assert!(report["studio_ui_gates"]
        .as_array()
        .unwrap()
        .iter()
        .any(|gate| gate.as_str() == Some("preview-smoke")));
    assert_eq!(report["release_plan"].as_str(), Some("PASS"));
    assert_eq!(report["install_plan"].as_str(), Some("PASS"));
    assert!(report["quality_tools"]
        .as_array()
        .unwrap()
        .iter()
        .any(|tool| tool.as_str() == Some("dylint")));
}

#[test]
fn install_plan_and_release_plan_print_actionable_steps() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let install = run_cli([
        "std",
        "install",
        "plan",
        "--prefix",
        temp.path().join("prefix").to_str().unwrap(),
    ])
    .unwrap();
    let release = run_cli(["std", "release", "plan", "--version", "1.0.0"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(install.contains("binaries=std,std-launcher,std-studio"));
    assert!(install.contains("app_bundles=std Launcher.app,std Studio.app"));
    assert!(install.contains("apps_dir="));
    assert!(install.contains("std install run"));
    assert!(install.contains("std install verify --prefix"));
    assert!(release.contains("version=1.0.0"));
    assert!(release.contains("cargo build --release --workspace"));
    assert!(release.contains("verify=mise run quality"));
    assert!(release.contains("doctor=std doctor"));
    assert!(release.contains("workflow_smoke=std plan terminal --save && std run terminal"));
    assert!(release.contains("workflow_trace=std workflow trace --limit 5"));
    assert!(release.contains("index_coverage=std index coverage"));
    assert!(release.contains("plugin_check=std plugin check examples/plugins/hello-js"));
    assert!(release.contains("launcher_smoke=std-launcher --smoke \"rebuild index\""));
    assert!(release.contains("std run <workflow> --allow-external"));
    assert!(release.contains("std release package --version 1.0.0"));
    assert!(release.contains("install_check="));
    assert!(release.contains("std install verify --prefix"));
    assert!(release.contains("dist/1.0.0/bin"));
}

#[test]
fn install_run_copies_binaries_and_initializes_storage() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let source_dir = temp.path().join("release");
    let prefix = temp.path().join("prefix");
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
        "install",
        "run",
        "--prefix",
        prefix.to_str().unwrap(),
        "--from",
        source_dir.to_str().unwrap(),
    ])
    .unwrap();
    let verified = run_cli([
        "std",
        "install",
        "verify",
        "--prefix",
        prefix.to_str().unwrap(),
    ])
    .unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("installed"));
    assert!(verified.contains("install verify PASS"));
    assert!(verified.contains("binaries=3"));
    assert!(verified.contains("app_bundles=2"));
    assert!(verified.contains("storage=PASS"));
    assert!(prefix.join("bin").join("std").is_file());
    assert!(prefix.join("bin").join("std-launcher").is_file());
    assert!(prefix.join("bin").join("std-studio").is_file());
    assert!(prefix
        .join("Applications")
        .join("std Launcher.app")
        .join("Contents")
        .join("MacOS")
        .join("std-launcher")
        .is_file());
    assert!(prefix
        .join("Applications")
        .join("std Studio.app")
        .join("Contents")
        .join("MacOS")
        .join("std-studio")
        .is_file());
    assert!(temp.path().join("data").join("plugins").is_dir());
}

#[test]
fn install_verify_rejects_missing_binary() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let prefix = temp.path().join("prefix");
    std::fs::create_dir_all(prefix.join("bin")).unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let error = run_cli([
        "std",
        "install",
        "verify",
        "--prefix",
        prefix.to_str().unwrap(),
    ])
    .unwrap_err();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(error.to_string().contains("installed binary missing"));
}
