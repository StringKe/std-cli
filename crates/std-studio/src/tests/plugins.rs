use super::*;
use crate::plugin_security::{boundary_summary, runtime_summary};
use std_types::ActionExecutionStatus;

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
    let boundary = boundary_summary(&manager.check_reports[0]);
    assert_eq!(boundary.permissions, vec!["Shell"]);
    assert_eq!(boundary.fs_scopes, "none");
    assert_eq!(boundary.network_hosts, "none");
    assert_eq!(boundary.actions, "1 actions");
}

#[test]
fn studio_plugin_runtime_summary_reports_js_ts_controlled_runtime() {
    let js_output = serde_json::json!({
        "runtime": "deno_core",
        "exit_code": 0,
        "duration_ms": 18
    });

    let summary = runtime_summary(&ActionExecutionStatus::Completed, Some(&js_output));

    assert_eq!(
        summary.status,
        std_egui::i18n::t("studio.plugins.runtime.completed")
    );
    assert_eq!(summary.runtime, "deno_core");
    assert_eq!(summary.exit_code, "0");
    assert_eq!(summary.duration, "18 ms");
    assert_eq!(summary.boundary, "deno_core controlled runtime");
}

#[test]
fn studio_plugin_toolbar_search_has_textbox_semantics() {
    let body = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("views")
            .join("plugins.rs"),
    )
    .unwrap();

    assert!(body.contains("WidgetType::TextEdit"));
    assert!(body.contains("plugin_query_a11y_label"));
    assert!(body.contains("Plugin search, text box, value"));
    assert!(body.contains("query.trim().is_empty()"));
}

#[test]
fn studio_plugin_toolbar_exposes_docs22_install_actions_without_native_dialog() {
    let body = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("views")
            .join("plugins.rs"),
    )
    .unwrap();
    let catalog = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("std-egui")
            .join("src")
            .join("i18n")
            .join("catalog")
            .join("studio")
            .join("plugins.rs"),
    )
    .unwrap();

    assert!(body.contains("studio.plugins.install.path"));
    assert!(body.contains("studio.plugins.install.registry"));
    assert!(body.contains("studio.plugins.install.path.status"));
    assert!(body.contains("studio.plugins.install.registry.status"));
    assert!(catalog.contains("Install from path"));
    assert!(catalog.contains("Install from registry"));
    assert!(catalog.contains("registry install is phase 2"));
    assert!(!body.contains("FileDialog"));
    assert!(!body.contains("rfd::"));
    assert!(!body.contains("open::that"));
}
