use super::*;

#[test]
fn plugin_javascript_tool_reads_clipboard_history_with_permission() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("clipboard");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"const records = std.clipboardHistory(1);
std.emit({ content: records[0].content, source: records[0].source });
"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "clipboard",
            "description": "Clipboard plugin",
            "version": "0.1.0",
            "permissions": ["code", "clipboard"],
            "actions": [{
                "name": "Plugin Clipboard",
                "description": "Read clipboard history",
                "when_to_use": "When validating clipboard plugin host data reads",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-clipboard"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    let tools = load_plugin_tools_with_host(
        &temp.path().join("plugins"),
        PluginHostData {
            clipboard: vec![ClipboardRecord {
                id: uuid::Uuid::new_v4(),
                content: "cargo test --workspace".to_string(),
                source: "test".to_string(),
                created_at: chrono::Utc::now(),
            }],
        },
    )
    .unwrap();

    let output = tools[0].execute(serde_json::json!({})).unwrap();

    assert_eq!(
        output["stdout"].as_str(),
        Some("{\"content\":\"cargo test --workspace\",\"source\":\"test\"}")
    );
}

#[test]
fn plugin_javascript_tool_requires_clipboard_permission() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("clipboard");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        "std.print(JSON.stringify(std.clipboardHistory(1)));",
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "clipboard",
            "description": "Clipboard plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Clipboard",
                "description": "Read clipboard history",
                "when_to_use": "When validating clipboard plugin permission",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-clipboard"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    let tools = load_plugin_tools_with_host(
        &temp.path().join("plugins"),
        PluginHostData {
            clipboard: vec![ClipboardRecord {
                id: uuid::Uuid::new_v4(),
                content: "blocked clipboard".to_string(),
                source: "test".to_string(),
                created_at: chrono::Utc::now(),
            }],
        },
    )
    .unwrap();

    let error = tools[0].execute(serde_json::json!({})).unwrap_err();

    assert!(error.to_string().contains("requires clipboard permission"));
}

#[test]
fn plugin_javascript_tool_hard_times_out_infinite_loop() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("loop");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(plugin_dir.join("main.js"), "for (;;) {}").unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "loop",
            "description": "Loop plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "actions": [{
                "name": "Plugin Loop",
                "description": "Run loop plugin",
                "when_to_use": "When validating code plugin timeout",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 25,
                "tags": ["plugin-loop"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0].execute(serde_json::json!({})).unwrap();

    assert_eq!(output["runtime"].as_str(), Some("deno_core"));
    assert_eq!(output["timed_out"].as_bool(), Some(true));
    assert_eq!(output["exit_code"], serde_json::Value::Null);
    assert_eq!(output["stderr"].as_str(), Some("plugin command timed out"));
}

#[test]
fn plugin_shell_tool_is_blocked_in_test_mode() {
    let manifest = Arc::new(PluginManifest {
        name: "timeout".to_string(),
        description: "Timeout plugin".to_string(),
        version: "0.1.0".to_string(),
        permissions: vec![PluginPermission::Shell],
        fs_scopes: vec![],
        network_hosts: vec![],
        actions: vec![],
    });
    let tool = PluginTool::new(
        manifest,
        PluginActionManifest {
            name: "Timeout".to_string(),
            description: "Timeout".to_string(),
            when_to_use: "Testing timeout".to_string(),
            kind: PluginActionKind::Shell,
            command: Some("sleep 1".to_string()),
            script: None,
            timeout_ms: Some(10),
            tags: vec![],
        },
        PathBuf::from("plugin.json"),
    );

    let error = tool.execute(serde_json::json!({})).unwrap_err();

    assert_eq!(
        error.to_string(),
        "Plugin permission denied: STD_TEST_MODE blocked shell plugin command"
    );
}
