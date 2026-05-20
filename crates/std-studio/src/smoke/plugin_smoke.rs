use std_studio::StudioApp;

pub(crate) struct PluginManagerSmoke {
    pub(crate) status: String,
    pub(crate) manifest_checks: usize,
    pub(crate) permissions: String,
    pub(crate) action_count: usize,
    pub(crate) preview_kind: String,
    pub(crate) runtime: String,
}

pub(crate) fn run_plugin_manager_smoke(
    studio: &mut StudioApp,
) -> Result<PluginManagerSmoke, Box<dyn std::error::Error>> {
    let plugin_dir = studio.core.config.plugins_dir().join("studio-smoke");
    std::fs::create_dir_all(&plugin_dir)?;
    std::fs::write(
        plugin_dir.join("main.js"),
        r#"std.emit({ plugin: "studio-smoke", status: "ok" });"#,
    )?;
    std::fs::write(plugin_dir.join("plugin.json"), smoke_plugin_manifest())?;
    studio.reload_plugins()?;
    let actions = studio.search_plugins("studio-smoke");
    let preview_kind = studio
        .plugin_manager
        .preview
        .as_ref()
        .map(|preview| format!("{:?}", preview.action_type))
        .unwrap_or_else(|| "Missing".to_string());
    let status = studio
        .run_selected_plugin()
        .map(|execution| format!("{:?}", execution.status))
        .unwrap_or_else(|| "Missing".to_string());
    let runtime = studio
        .plugin_manager
        .last_execution
        .as_ref()
        .and_then(|execution| execution.output.as_ref())
        .and_then(|output| output.get("runtime"))
        .and_then(|runtime| runtime.as_str())
        .unwrap_or("Missing")
        .to_string();
    let permissions = studio
        .plugin_manager
        .check_reports
        .first()
        .map(|report| {
            report
                .permissions
                .iter()
                .map(|permission| format!("{permission:?}"))
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_else(|| "Missing".to_string());

    Ok(PluginManagerSmoke {
        status,
        manifest_checks: studio.plugin_manager.check_reports.len(),
        permissions,
        action_count: actions.len(),
        preview_kind,
        runtime,
    })
}

fn smoke_plugin_manifest() -> String {
    serde_json::json!({
        "name": "studio-smoke",
        "description": "Studio smoke plugin",
        "permissions": ["code"],
        "actions": [{
            "name": "Plugin Studio Smoke",
            "description": "Run Studio smoke plugin",
            "when_to_use": "When validating std-studio smoke",
            "kind": "javascript",
            "script": "main.js",
            "tags": ["studio-smoke"]
        }]
    })
    .to_string()
}
