use super::*;

#[test]
fn plugin_javascript_tool_reads_scoped_file() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("reader");
    fs::create_dir_all(plugin_dir.join("data")).unwrap();
    fs::write(plugin_dir.join("data").join("note.txt"), "scoped note").unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"const input = std.args();
const body = std.readTextFile(input.path);
std.emit({ body });
"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "reader",
            "description": "Reader plugin",
            "version": "0.1.0",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Plugin Reader",
                "description": "Read scoped file",
                "when_to_use": "When validating scoped plugin file reads",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-reader"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0]
        .execute(serde_json::json!({
            "path": plugin_dir.join("data").join("note.txt"),
        }))
        .unwrap();

    assert_eq!(output["runtime"].as_str(), Some("deno_core"));
    assert_eq!(
        output["stdout"].as_str(),
        Some("{\"body\":\"scoped note\"}")
    );
    assert!(output["duration_ms"].as_u64().unwrap() < 500);
}

#[test]
fn plugin_javascript_tool_rejects_unscoped_file_read() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("reader");
    fs::create_dir_all(plugin_dir.join("data")).unwrap();
    fs::write(temp.path().join("secret.txt"), "secret").unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"std.print(std.readTextFile(std.args().path));"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "reader",
            "description": "Reader plugin",
            "version": "0.1.0",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Plugin Reader",
                "description": "Read scoped file",
                "when_to_use": "When validating scoped plugin file reads",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-reader"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let error = tools[0]
        .execute(serde_json::json!({
            "path": temp.path().join("secret.txt"),
        }))
        .unwrap_err();

    assert!(error.to_string().contains("outside plugin fs scopes"));
}

#[test]
fn plugin_javascript_tool_writes_scoped_file() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("writer");
    fs::create_dir_all(plugin_dir.join("data")).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"const input = std.args();
std.writeTextFile(input.path, input.body);
std.emit({ written: std.readTextFile(input.path) });
"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "writer",
            "description": "Writer plugin",
            "version": "0.1.0",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Plugin Writer",
                "description": "Write scoped file",
                "when_to_use": "When validating scoped plugin file writes",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-writer"]
            }]
        })
        .to_string(),
    )
    .unwrap();
    let target = plugin_dir.join("data").join("out.txt");

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let output = tools[0]
        .execute(serde_json::json!({
            "path": target,
            "body": "written note",
        }))
        .unwrap();

    assert_eq!(
        fs::read_to_string(plugin_dir.join("data").join("out.txt")).unwrap(),
        "written note"
    );
    assert_eq!(
        output["stdout"].as_str(),
        Some("{\"written\":\"written note\"}")
    );
}

#[test]
fn plugin_javascript_tool_rejects_unscoped_file_write() {
    let temp = tempfile::tempdir().unwrap();
    let plugin_dir = temp.path().join("plugins").join("writer");
    fs::create_dir_all(plugin_dir.join("data")).unwrap();
    fs::write(
        plugin_dir.join("main.js"),
        r#"std.writeTextFile(std.args().path, "blocked");"#,
    )
    .unwrap();
    fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "writer",
            "description": "Writer plugin",
            "version": "0.1.0",
            "permissions": ["code", "fs_scoped"],
            "fs_scopes": ["data"],
            "actions": [{
                "name": "Plugin Writer",
                "description": "Write scoped file",
                "when_to_use": "When validating scoped plugin file writes",
                "kind": "javascript",
                "script": "main.js",
                "timeout_ms": 1000,
                "tags": ["plugin-writer"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let tools = load_plugin_tools(&temp.path().join("plugins")).unwrap();
    let error = tools[0]
        .execute(serde_json::json!({
            "path": temp.path().join("outside.txt"),
        }))
        .unwrap_err();

    assert!(error.to_string().contains("outside plugin fs scopes"));
    assert!(!temp.path().join("outside.txt").exists());
}
