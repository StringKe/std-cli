use super::*;

#[test]
fn plugin_manifest_loads_and_shell_tool_executes() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("smoke");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "smoke",
            "description": "Smoke plugin",
            "version": "0.1.0",
            "permissions": ["shell"],
            "actions": [{
                "name": "Plugin Smoke",
                "description": "Run smoke command",
                "when_to_use": "When validating plugin execution",
                "kind": "shell",
                "command": "printf plugin-smoke",
                "timeout_ms": 1000,
                "tags": ["smoke"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0].execute(serde_json::json!({})).unwrap();

    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].registry_entry().action.name, "Plugin Smoke");
    assert_eq!(output["stdout"].as_str(), Some("plugin-smoke"));
    assert_eq!(output["timed_out"].as_bool(), Some(false));
    assert_eq!(output["runtime"].as_str(), Some("shell"));
}

#[test]
fn plugin_manifest_loads_and_javascript_tool_executes() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("code");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"const input = std.args();
std.emit({ received: input.value, plugin: "code" });
"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "code",
            "description": "Code plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Code",
                "description": "Run code plugin",
                "when_to_use": "When validating code plugin execution",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-code"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0]
        .execute(serde_json::json!({
            "value": 42
        }))
        .unwrap();

    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].registry_entry().action.name, "Plugin Code");
    assert_eq!(output["exit_code"].as_i64(), Some(0));
    assert_eq!(
        output["stdout"].as_str(),
        Some("{\"received\":42,\"plugin\":\"code\"}")
    );
    assert_eq!(output["runtime"].as_str(), Some("deno_core"));
    assert!(tools[0]
        .registry_entry()
        .tags
        .contains(&"javascript".to_string()));
}

#[test]
fn javascript_plugin_can_read_plugin_directory() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("dir");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"std.emit({ plugin_dir: std.pluginDir() });"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "dir",
            "description": "Directory plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Directory",
                "description": "Read plugin directory",
                "when_to_use": "When validating plugin directory host API",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-dir"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0].execute(serde_json::json!({})).unwrap();
    let expected = format!(
        "{{\"plugin_dir\":\"{}\"}}",
        fs::canonicalize(plugin_dir).unwrap().display()
    );

    assert_eq!(output["stdout"].as_str(), Some(expected.as_str()));
}

#[test]
fn plugin_manifest_loads_and_typescript_tool_executes() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("typed");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.ts"),
        r#"type Input = { value: number };
interface Result {
  doubled: number;
}
const input: Input = std.args() as Input;
const doubled: number = input.value * 2;
const result: Result = { doubled };
std.emit({ plugin: "typed", result });
"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "typed",
            "description": "Typed plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Typed",
                "description": "Run TypeScript plugin",
                "when_to_use": "When validating TypeScript plugin execution",
                "kind": "typescript",
                "script": "main.ts",
                "timeout_ms": 1000,
                "tags": ["plugin-typed"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0]
        .execute(serde_json::json!({
            "value": 21
        }))
        .unwrap();

    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].registry_entry().action.name, "Plugin Typed");
    assert_eq!(output["runtime"].as_str(), Some("deno_core"));
    assert_eq!(
        output["stdout"].as_str(),
        Some("{\"plugin\":\"typed\",\"result\":{\"doubled\":42}}")
    );
    assert!(tools[0]
        .registry_entry()
        .tags
        .contains(&"typescript".to_string()));
}

#[test]
fn plugin_manifest_check_validates_code_script_and_permissions() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("checked");
    fs::create_dir_all(plugin_dir.join("data")).unwrap();
    fs::write(plugin_dir.join("main.js"), "std.print('checked');").unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "checked",
            "description": "Checked plugin",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Checked Action",
                "description": "Run checked action",
                "when_to_use": "When validating plugin checks",
                "kind": "javascript",
                "script": "main.js"
            }]
        })
        .to_string(),
    )
    .unwrap();

    let report = check_plugin_manifest(&plugin_dir).unwrap();

    assert_eq!(report.status, "PASS");
    assert_eq!(report.plugin_name, "checked");
    assert_eq!(report.actions, 1);
    assert!(report.permissions.contains(&PluginPermission::Code));
}

#[test]
fn plugin_manifest_check_rejects_missing_script() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("broken");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "broken",
            "description": "Broken plugin",
            "permissions": ["code"],
            "actions": [{
                "name": "Broken Action",
                "description": "Run broken action",
                "when_to_use": "When validating plugin check rejection",
                "kind": "javascript",
                "script": "missing.js"
            }]
        })
        .to_string(),
    )
    .unwrap();

    let error = check_plugin_manifest(&plugin_dir.join("plugin.json")).unwrap_err();

    assert!(error.to_string().contains("plugin script not found"));
}

#[test]
fn plugin_code_tool_requires_code_permission() {
    let manifest = Arc::new(PluginManifest {
        name: "denied".to_string(),
        description: "Denied plugin".to_string(),
        version: "0.1.0".to_string(),
        permissions: vec![PluginPermission::ReadOnly],
        fs_scopes: vec![],
        network_hosts: vec![],
        actions: vec![],
    });
    let tool = PluginTool::new(
        manifest,
        PluginActionManifest {
            name: "Denied".to_string(),
            description: "Denied".to_string(),
            when_to_use: "Testing code permission".to_string(),
            kind: PluginActionKind::Javascript,
            command: None,
            script: Some("main.js".to_string()),
            timeout_ms: Some(10),
            tags: vec![],
        },
        PathBuf::from("plugin.json"),
    );

    let error = tool.execute(serde_json::json!({})).unwrap_err();

    assert!(error.to_string().contains("requires code permission"));
}

#[test]
fn plugin_code_tool_rejects_script_outside_plugin_directory() {
    let temp = tempfile::tempdir().unwrap();
    let plugins_dir = temp.path().join("plugins");
    let plugin_dir = plugins_dir.join("escape");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(plugins_dir.join("escape.js"), "std.print('escape');").unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "escape",
            "description": "Escape plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Escape",
                "description": "Escape plugin dir",
                "when_to_use": "When validating plugin script containment",
                "kind": "javascript",
                "script": "../escape.js",
                "timeout_ms": 1000,
                "tags": ["plugin-escape"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&plugins_dir).unwrap();
    let error = tools[0].execute(serde_json::json!({})).unwrap_err();

    assert!(error
        .to_string()
        .contains("plugin script outside plugin directory"));
}
