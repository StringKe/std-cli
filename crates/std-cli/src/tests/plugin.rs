use super::support::spawn_test_http_server;
use super::*;

#[test]
fn plugin_commands_list_search_and_run_manifest_action() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("smoke");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "smoke",
            "description": "Smoke plugin",
            "permissions": ["shell"],
            "actions": [{
                "name": "Plugin Smoke",
                "description": "Run plugin smoke",
                "when_to_use": "When validating plugin action discovery",
                "kind": "shell",
                "command": "printf plugin-smoke",
                "tags": ["plugin-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let listed = run_cli(["std", "plugin", "list"]).unwrap();
    let searched = run_cli(["std", "search", "plugin-smoke"]).unwrap();
    let output = run_cli(["std", "plugin", "run", "plugin-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(listed.contains("plugin.json"));
    assert!(searched.contains("Plugin Smoke"));
    assert!(output.contains("\"action_name\": \"Plugin Smoke\""));
    assert!(output.contains("plugin-smoke"));
}

#[test]
fn plugin_check_validates_manifest_without_running_action() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugin");
    std::fs::create_dir_all(plugin_dir.join("data")).unwrap();
    std::fs::write(plugin_dir.join("main.js"), "std.print('not executed');").unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "checked",
            "description": "Checked plugin",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Checked Action",
                "description": "Validate checked plugin",
                "when_to_use": "When validating plugin check",
                "kind": "javascript",
                "script": "main.js",
                "tags": ["checked-plugin"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let output = run_cli([
        "std",
        "plugin",
        "check",
        plugin_dir.join("plugin.json").to_str().unwrap(),
    ])
    .unwrap();

    let report: serde_json::Value = serde_json::from_str(&output).unwrap();
    assert_eq!(report["status"].as_str(), Some("PASS"));
    assert_eq!(report["plugin_name"].as_str(), Some("checked"));
    assert_eq!(report["actions"].as_u64(), Some(1));
    assert!(report["permissions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|permission| permission.as_str() == Some("code")));
}

#[test]
fn plugin_check_rejects_code_action_without_code_permission() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugin");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(plugin_dir.join("main.js"), "std.print('blocked');").unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "blocked",
            "description": "Blocked plugin",
            "permissions": [],
            "actions": [{
                "name": "Blocked Action",
                "description": "Validate blocked plugin",
                "when_to_use": "When validating plugin check rejection",
                "kind": "javascript",
                "script": "main.js"
            }]
        })
        .to_string(),
    )
    .unwrap();

    let error = run_cli([
        "std",
        "plugin",
        "check",
        plugin_dir.join("plugin.json").to_str().unwrap(),
    ])
    .unwrap_err();

    assert!(error.to_string().contains("requires Code permission"));
}

#[test]
fn plugin_command_runs_javascript_manifest_action() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("code");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("main.js"),
        r#"std.emit({ plugin: "code-smoke" });"#,
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "code",
            "description": "Code plugin",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Code Smoke",
                "description": "Run code plugin smoke",
                "when_to_use": "When validating code plugin action discovery",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-code-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let searched = run_cli(["std", "search", "plugin-code-smoke"]).unwrap();
    let output = run_cli(["std", "plugin", "run", "plugin-code-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(searched.contains("Plugin Code Smoke"));
    assert!(output.contains("\"action_name\": \"Plugin Code Smoke\""));
    assert!(output.contains("code-smoke"));
    assert!(output.contains("\"runtime\": \"deno_core\""));
}

#[test]
fn plugin_command_runs_typescript_manifest_action() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("typed");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("main.ts"),
        r#"type Payload = { label: string };
const payload: Payload = { label: "typescript-smoke" };
std.emit({ plugin: payload.label });
"#,
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "typed",
            "description": "Typed plugin",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin TypeScript Smoke",
                "description": "Run TypeScript plugin smoke",
                "when_to_use": "When validating TypeScript plugin action discovery",
                "kind": "typescript",
                "script": "main.ts",
                "timeout_ms": 1000,
                "tags": ["plugin-typescript-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let searched = run_cli(["std", "search", "plugin-typescript-smoke"]).unwrap();
    let output = run_cli(["std", "plugin", "run", "plugin-typescript-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(searched.contains("Plugin TypeScript Smoke"));
    assert!(output.contains("\"action_name\": \"Plugin TypeScript Smoke\""));
    assert!(output.contains("typescript-smoke"));
    assert!(output.contains("\"runtime\": \"deno_core\""));
}

#[test]
fn plugin_command_runs_scoped_file_reader_action() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("reader");
    std::fs::create_dir_all(plugin_dir.join("data")).unwrap();
    std::fs::write(plugin_dir.join("data").join("note.txt"), "cli scoped note").unwrap();
    std::fs::write(
        plugin_dir.join("main.js"),
        format!(
            r#"const body = std.readTextFile("{}");
std.emit({{ body }});
"#,
            plugin_dir.join("data").join("note.txt").display()
        ),
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "reader",
            "description": "Reader plugin",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Plugin Reader Smoke",
                "description": "Run scoped reader plugin smoke",
                "when_to_use": "When validating scoped reader plugin action discovery",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-reader-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "plugin", "run", "plugin-reader-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("\"action_name\": \"Plugin Reader Smoke\""));
    assert!(output.contains("cli scoped note"));
    assert!(output.contains("\"status\": \"Completed\""));
}

#[test]
fn plugin_command_runs_scoped_file_writer_action() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("writer");
    let target = plugin_dir.join("data").join("out.txt");
    std::fs::create_dir_all(plugin_dir.join("data")).unwrap();
    std::fs::write(
        plugin_dir.join("main.js"),
        format!(
            r#"std.writeTextFile("{}", "cli written note");
std.emit({{ written: std.readTextFile("{}") }});
"#,
            target.display(),
            target.display()
        ),
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "writer",
            "description": "Writer plugin",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Plugin Writer Smoke",
                "description": "Run scoped writer plugin smoke",
                "when_to_use": "When validating scoped writer plugin action discovery",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-writer-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "plugin", "run", "plugin-writer-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("\"action_name\": \"Plugin Writer Smoke\""));
    assert!(output.contains("cli written note"));
    assert_eq!(std::fs::read_to_string(target).unwrap(), "cli written note");
}

#[test]
fn plugin_command_runs_scoped_network_action() {
    let (url, host_scope) = spawn_test_http_server("cli network body");
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("network");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("main.js"),
        format!(
            r#"const body = std.httpGet("{}");
std.emit({{ body }});
"#,
            url
        ),
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "network",
            "description": "Network plugin",
            "permissions": ["code", "network"],
            "network_hosts": [host_scope],
            "actions": [{
                "name": "Plugin Network Smoke",
                "description": "Run scoped network plugin smoke",
                "when_to_use": "When validating scoped network plugin action discovery",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-network-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let output = run_cli(["std", "plugin", "run", "plugin-network-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(output.contains("\"action_name\": \"Plugin Network Smoke\""));
    assert!(output.contains("cli network body"));
    assert!(output.contains("\"status\": \"Completed\""));
}

#[test]
fn plugin_command_reads_captured_clipboard_history() {
    let temp = tempfile::tempdir().unwrap();
    let config_path = temp.path().join("std-cli.json");
    let plugin_dir = temp.path().join("data").join("plugins").join("clipboard");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("main.js"),
        r#"const records = std.clipboardHistory(1);
std.emit({ content: records[0].content, source: records[0].source });
"#,
    )
    .unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "clipboard",
            "description": "Clipboard plugin",
            "permissions": ["code", "clipboard"],
            "actions": [{
                "name": "Plugin Clipboard Smoke",
                "description": "Run clipboard plugin smoke",
                "when_to_use": "When validating clipboard plugin host data",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-clipboard-smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(
        &config_path,
        serde_json::json!({
            "data_dir": temp.path().join("data"),
        })
        .to_string(),
    )
    .unwrap();
    std::env::set_var("STDCLI_CONFIG", &config_path);

    let captured = run_cli([
        "std",
        "clipboard",
        "capture",
        "cargo test --workspace",
        "--source",
        "test",
    ])
    .unwrap();
    let output = run_cli(["std", "plugin", "run", "plugin-clipboard-smoke"]).unwrap();

    std::env::remove_var("STDCLI_CONFIG");

    assert!(captured.contains("cargo test --workspace"));
    assert!(output.contains("\"action_name\": \"Plugin Clipboard Smoke\""));
    assert!(output.contains("cargo test --workspace"));
    assert!(output.contains("\\\"source\\\":\\\"test\\\""));
    assert!(output.contains("\"status\": \"Completed\""));
}
