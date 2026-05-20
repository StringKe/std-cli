use super::*;

#[test]
fn plugin_javascript_tool_gets_scoped_http_url() {
    let (url, host_scope) = spawn_test_http_server("network-body");
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("network");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"const body = std.httpGet(std.args().url);
std.emit({ body });
"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "network",
            "description": "Network plugin",
            "version": "0.1.0",
            "permissions": ["code", "network"],
            "network_hosts": [host_scope],
            "actions": [{
                "name": "Plugin Network",
                "description": "Read scoped HTTP URL",
                "when_to_use": "When validating scoped plugin HTTP reads",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-network"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0]
        .execute(serde_json::json!({
            "url": url,
        }))
        .unwrap();

    assert_eq!(
        output["stdout"].as_str(),
        Some("{\"body\":\"network-body\"}")
    );
}

#[test]
fn plugin_javascript_tool_requires_network_permission() {
    let (url, host_scope) = spawn_test_http_server("network-body");
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("network");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        "std.print(std.httpGet(std.args().url));",
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "network",
            "description": "Network plugin",
            "version": "0.1.0",
            "permissions": ["code"],
            "network_hosts": [host_scope],
            "actions": [{
                "name": "Plugin Network",
                "description": "Read scoped HTTP URL",
                "when_to_use": "When validating scoped plugin HTTP reads",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-network"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let error = tools[0]
        .execute(serde_json::json!({
            "url": url,
        }))
        .unwrap_err();

    assert!(error.to_string().contains("requires network permission"));
}

#[test]
fn plugin_javascript_tool_rejects_unscoped_http_host() {
    let (url, _host_scope) = spawn_test_http_server("network-body");
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("network");
    fs::create_dir_all(&plugin_dir).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        "std.print(std.httpGet(std.args().url));",
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "network",
            "description": "Network plugin",
            "version": "0.1.0",
            "permissions": ["code", "network"],
            "network_hosts": ["127.0.0.1:9"],
            "actions": [{
                "name": "Plugin Network",
                "description": "Read scoped HTTP URL",
                "when_to_use": "When validating scoped plugin HTTP reads",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-network"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let error = tools[0]
        .execute(serde_json::json!({
            "url": url,
        }))
        .unwrap_err();

    assert!(error.to_string().contains("outside plugin network scopes"));
}
