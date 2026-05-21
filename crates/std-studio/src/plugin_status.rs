use crate::plugin_security::{boundary_summary, runtime_summary};
use std_core::PluginCheckReport;
use std_egui::i18n;
use std_types::{ActionExecution, ActionPreview, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginStatusSummary {
    pub manifest_status: String,
    pub action_status: String,
    pub preview_status: String,
    pub runtime_status: String,
    pub permission_status: String,
    pub boundary_status: String,
}

pub fn summarize_plugin_status(
    reports: &[PluginCheckReport],
    actions: &[SearchResult],
    preview: Option<&ActionPreview>,
    last_execution: Option<&ActionExecution>,
) -> PluginStatusSummary {
    PluginStatusSummary {
        manifest_status: manifest_status(reports),
        action_status: format!("{} actions", actions.len()),
        preview_status: preview_status(preview),
        runtime_status: runtime_status(last_execution),
        permission_status: permission_status(reports),
        boundary_status: boundary_status(reports),
    }
}

fn manifest_status(reports: &[PluginCheckReport]) -> String {
    let pass_count = reports
        .iter()
        .filter(|report| report.status == "PASS")
        .count();
    format!("{pass_count}/{} PASS", reports.len())
}

fn preview_status(preview: Option<&ActionPreview>) -> String {
    preview
        .map(|preview| format!("{:?}", preview.action_type))
        .unwrap_or_else(|| i18n::t("studio.plugins.status.no_preview").to_string())
}

fn runtime_status(last_execution: Option<&ActionExecution>) -> String {
    last_execution
        .map(|execution| {
            let summary = runtime_summary(&execution.status, execution.output.as_ref());
            format!("{} {}", summary.status, summary.runtime)
        })
        .unwrap_or_else(|| i18n::t("studio.plugins.status.no_run").to_string())
}

fn permission_status(reports: &[PluginCheckReport]) -> String {
    let permissions = reports
        .iter()
        .flat_map(|report| boundary_summary(report).permissions)
        .collect::<Vec<_>>();
    if permissions.is_empty() {
        "ReadOnly".to_string()
    } else {
        permissions.join(",")
    }
}

fn boundary_status(reports: &[PluginCheckReport]) -> String {
    let fs_scopes = reports
        .iter()
        .map(boundary_summary)
        .filter(|summary| summary.fs_scopes != "none")
        .count();
    let network_scopes = reports
        .iter()
        .map(boundary_summary)
        .filter(|summary| summary.network_hosts != "none")
        .count();
    format!("fs={fs_scopes} network={network_scopes}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_core::plugins::PluginPermission;
    use std_types::{Action, ActionExecutionStatus, ActionType};

    #[test]
    fn plugin_status_summary_exposes_checks_preview_runtime_and_boundary() {
        let report = PluginCheckReport {
            manifest_path: "plugin.json".into(),
            plugin_name: "checked".to_string(),
            status: "PASS",
            actions: 2,
            permissions: vec![PluginPermission::Code],
            fs_scopes: vec!["fixtures".into()],
            network_hosts: vec!["api.local".to_string()],
        };
        let action = Action::new(
            "Plugin Checked",
            "Run checked",
            "When checked",
            ActionType::Command,
        );
        let action_id = action.id;
        let result = SearchResult {
            action,
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };
        let execution = ActionExecution {
            action_id,
            action_name: "Plugin Checked".to_string(),
            status: ActionExecutionStatus::Completed,
            message: "done".to_string(),
            output: Some(serde_json::json!({"runtime": "deno_core"})),
            created_at: chrono::Utc::now(),
        };

        let summary = summarize_plugin_status(&[report], &[result], None, Some(&execution));

        assert_eq!(summary.manifest_status, "1/1 PASS");
        assert_eq!(summary.action_status, "1 actions");
        assert_eq!(
            summary.preview_status,
            i18n::t("studio.plugins.status.no_preview")
        );
        assert_eq!(
            summary.runtime_status,
            format!("{} deno_core", i18n::t("studio.plugins.runtime.completed"))
        );
        assert_eq!(summary.permission_status, "Code");
        assert_eq!(summary.boundary_status, "fs=1 network=1");
    }
}
