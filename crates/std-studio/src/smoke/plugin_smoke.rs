use super::plugin_contracts::{PluginContractInput, PluginUiContracts};
use std_studio::StudioApp;

pub(crate) struct PluginManagerSmoke {
    pub(crate) js_status: String,
    pub(crate) ts_status: String,
    pub(crate) manifest_checks: usize,
    pub(crate) permissions: String,
    pub(crate) action_count: usize,
    pub(crate) preview_kind: String,
    pub(crate) js_runtime: String,
    pub(crate) ts_runtime: String,
    pub(crate) status_bar_contract: String,
    pub(crate) permission_visual_contract: String,
    pub(crate) inspector_contract: String,
    pub(crate) visual_contract: String,
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
    std::fs::write(
        plugin_dir.join("main.ts"),
        r#"type SmokeOutput = { plugin: string; status: string };
const output: SmokeOutput = { plugin: "studio-ts-smoke", status: "ok" };
std.emit(output);"#,
    )?;
    std::fs::write(plugin_dir.join("plugin.json"), smoke_plugin_manifest())?;
    studio.reload_plugins()?;
    let all_action_count = studio.plugin_manager.plugin_actions.len();
    let actions = studio.search_plugins("studio");
    studio.search_plugins("studio-js-smoke");
    let js_preview_kind = studio
        .plugin_manager
        .preview
        .as_ref()
        .map(|preview| format!("{:?}", preview.action_type))
        .unwrap_or_else(|| "Missing".to_string());
    let js_status = run_selected_status(studio);
    let js_runtime = selected_runtime(studio);
    studio.search_plugins("studio-ts-smoke");
    let ts_status = run_selected_status(studio);
    let ts_runtime = selected_runtime(studio);
    let contracts = PluginUiContracts::from_input(PluginContractInput {
        studio,
        js_runtime: &js_runtime,
        ts_runtime: &ts_runtime,
        command_count: all_action_count,
    });
    let permissions = studio
        .plugin_manager
        .check_reports
        .iter()
        .find(|report| report.plugin_name == "studio-smoke")
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
        js_status,
        ts_status,
        manifest_checks: studio.plugin_manager.check_reports.len(),
        permissions,
        action_count: actions.len(),
        preview_kind: js_preview_kind,
        js_runtime,
        ts_runtime,
        status_bar_contract: contracts.status_bar,
        permission_visual_contract: contracts.permission_visual,
        inspector_contract: contracts.inspector,
        visual_contract: contracts.visual,
    })
}

fn run_selected_status(studio: &mut StudioApp) -> String {
    studio
        .run_selected_plugin()
        .map(|execution| format!("{:?}", execution.status))
        .unwrap_or_else(|| "Missing".to_string())
}

fn selected_runtime(studio: &StudioApp) -> String {
    studio
        .plugin_manager
        .last_execution
        .as_ref()
        .and_then(|execution| execution.output.as_ref())
        .and_then(|output| output.get("runtime"))
        .and_then(|runtime| runtime.as_str())
        .unwrap_or("Missing")
        .to_string()
}

fn smoke_plugin_manifest() -> String {
    serde_json::json!({
        "name": "studio-smoke",
        "description": "Studio smoke plugin",
        "permissions": ["code"],
        "actions": [
            {
                "name": "Plugin Studio JS Smoke",
                "description": "Run Studio JavaScript smoke plugin",
                "when_to_use": "When validating std-studio JavaScript smoke",
                "kind": "javascript",
                "script": "main.js",
                "tags": ["studio-smoke", "studio-js-smoke"]
            },
            {
                "name": "Plugin Studio TS Smoke",
                "description": "Run Studio TypeScript smoke plugin",
                "when_to_use": "When validating std-studio TypeScript smoke",
                "kind": "typescript",
                "script": "main.ts",
                "tags": ["studio-smoke", "studio-ts-smoke"]
            }
        ]
    })
    .to_string()
}
