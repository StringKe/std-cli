use super::*;

#[test]
fn studio_plugin_manager_loads_manifest_check_reports() {
    let mut studio = test_studio();
    let plugin_dir = studio.core.config.plugins_dir().join("checked");
    std::fs::create_dir_all(&plugin_dir).unwrap();
    std::fs::write(
        plugin_dir.join("plugin.json"),
        serde_json::json!({
            "name": "checked",
            "description": "Checked plugin",
            "permissions": ["shell"],
            "actions": [{
                "name": "Plugin Checked",
                "description": "Run checked plugin",
                "when_to_use": "When validating Studio plugin checks",
                "kind": "shell",
                "command": "printf checked",
                "tags": ["studio-plugin-check"]
            }]
        })
        .to_string(),
    )
    .unwrap();

    let manager = studio.reload_plugins().unwrap();

    assert_eq!(manager.manifest_paths.len(), 1);
    assert_eq!(manager.check_reports.len(), 1);
    assert_eq!(manager.check_reports[0].status, "PASS");
    assert_eq!(manager.check_reports[0].plugin_name, "checked");
}
