use std_types::{ActionExecution, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginInspectorModel {
    pub(crate) description: String,
    pub(crate) permissions: Vec<String>,
    pub(crate) commands: Vec<String>,
    pub(crate) audit_log: String,
}

impl PluginInspectorModel {
    pub(crate) fn from_selection(
        selected: Option<&SearchResult>,
        reports: &[std_core::PluginCheckReport],
        execution: Option<&ActionExecution>,
    ) -> Self {
        let commands = selected
            .map(|result| vec![result.action.name.clone()])
            .unwrap_or_default();
        Self {
            description: selected
                .map(|result| result.action.description.clone())
                .unwrap_or_else(|| "No plugin selected".to_string()),
            permissions: selected
                .map(|result| selected_permissions(result, reports))
                .unwrap_or_default(),
            commands,
            audit_log: execution
                .map(|execution| format!("{}:{:?}", execution.action_name, execution.status))
                .unwrap_or_else(|| "missing".to_string()),
        }
    }

    pub(crate) fn contract(&self) -> String {
        format!(
            "description={};permissions={};commands={};audit_log={}",
            presence(&self.description),
            self.permissions.len(),
            self.commands.len(),
            presence(&self.audit_log)
        )
    }
}

fn selected_permissions(
    result: &SearchResult,
    reports: &[std_core::PluginCheckReport],
) -> Vec<String> {
    reports
        .iter()
        .find(|report| {
            result.action.name.contains(report.plugin_name.as_str())
                || result
                    .action
                    .description
                    .contains(report.plugin_name.as_str())
        })
        .map(|report| {
            report
                .permissions
                .iter()
                .map(|permission| format!("{permission:?}"))
                .collect()
        })
        .unwrap_or_default()
}

fn presence(value: &str) -> &'static str {
    if value == "missing" || value.trim().is_empty() {
        "missing"
    } else {
        "visible"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::path::PathBuf;
    use std_types::{Action, ActionExecution, ActionExecutionStatus, ActionType};
    use uuid::Uuid;

    #[test]
    fn inspector_model_tracks_selected_plugin_context() {
        let result = SearchResult {
            action: Action::new(
                "Plugin Example",
                "Plugin example description",
                "test",
                ActionType::Command,
            ),
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };
        let report = std_core::PluginCheckReport {
            manifest_path: PathBuf::from("example/plugin.json"),
            plugin_name: "Example".to_string(),
            status: "PASS",
            actions: 1,
            permissions: vec![std_core::plugins::PluginPermission::Code],
            fs_scopes: Vec::new(),
            network_hosts: Vec::new(),
        };
        let execution = ActionExecution {
            action_id: Uuid::new_v4(),
            action_name: "Plugin Example".to_string(),
            status: ActionExecutionStatus::Completed,
            message: "ok".to_string(),
            output: None,
            created_at: Utc::now(),
        };

        let model =
            PluginInspectorModel::from_selection(Some(&result), &[report], Some(&execution));

        assert_eq!(model.permissions, vec!["Code"]);
        assert_eq!(
            model.contract(),
            "description=visible;permissions=1;commands=1;audit_log=visible"
        );
    }
}
