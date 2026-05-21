use std_types::{ActionExecution, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginInspectorModel {
    pub(crate) description: String,
    pub(crate) permissions: Vec<String>,
    pub(crate) commands: Vec<String>,
    pub(crate) enable_state: String,
    pub(crate) review_prompt: String,
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
            enable_state: selected
                .map(|result| selected_enable_state(result, reports))
                .unwrap_or_else(|| "disabled".to_string()),
            review_prompt: selected
                .map(|result| selected_review_prompt(result, reports))
                .unwrap_or_else(|| "none".to_string()),
            audit_log: execution
                .map(|execution| format!("{}:{:?}", execution.action_name, execution.status))
                .unwrap_or_else(|| "missing".to_string()),
        }
    }

    pub(crate) fn contract(&self) -> String {
        format!(
            "description={};permissions={};commands={};enable_state={};review_prompt={};audit_log={}",
            presence(&self.description),
            self.permissions.len(),
            self.commands.len(),
            self.enable_state,
            presence(&self.review_prompt),
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
        .find(|report| plugin_result_matches_report(result, report))
        .map(|report| {
            report
                .permissions
                .iter()
                .map(|permission| format!("{permission:?}"))
                .collect()
        })
        .unwrap_or_default()
}

fn selected_enable_state(result: &SearchResult, reports: &[std_core::PluginCheckReport]) -> String {
    let status = selected_report_status(result, reports);
    if status == "PASS" {
        "enabled".to_string()
    } else {
        "review_required".to_string()
    }
}

fn selected_review_prompt(
    result: &SearchResult,
    reports: &[std_core::PluginCheckReport],
) -> String {
    let status = selected_report_status(result, reports);
    if status == "PASS" {
        "none".to_string()
    } else {
        format!("review permissions before enable: manifest {status}")
    }
}

fn selected_report_status(
    result: &SearchResult,
    reports: &[std_core::PluginCheckReport],
) -> String {
    reports
        .iter()
        .find(|report| plugin_result_matches_report(result, report))
        .map(|report| report.status.to_string())
        .unwrap_or_else(|| "UNKNOWN".to_string())
}

fn plugin_result_matches_report(
    result: &SearchResult,
    report: &std_core::PluginCheckReport,
) -> bool {
    let plugin = plugin_key(&report.plugin_name);
    plugin_matches(&plugin, &result.action.name)
        || plugin_matches(&plugin, &result.action.description)
        || plugin_matches(&plugin, &result.action.when_to_use)
        || result
            .action
            .examples
            .iter()
            .any(|example| plugin_matches(&plugin, example))
}

fn plugin_matches(plugin: &str, value: &str) -> bool {
    plugin_key(value).contains(plugin)
}

fn plugin_key(value: &str) -> String {
    value
        .chars()
        .filter(|char| char.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
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
        assert_eq!(model.enable_state, "enabled");
        assert_eq!(model.review_prompt, "none");
        assert_eq!(
            model.contract(),
            "description=visible;permissions=1;commands=1;enable_state=enabled;review_prompt=visible;audit_log=visible"
        );
    }

    #[test]
    fn inspector_model_matches_plugin_names_across_punctuation_and_case() {
        let result = SearchResult {
            action: Action {
                examples: vec!["main.ts".to_string()],
                ..Action::new(
                    "Plugin Studio TS Smoke",
                    "Run TypeScript plugin",
                    "When validating studio smoke",
                    ActionType::Command,
                )
            },
            score: 1.0,
            matched_fields: vec!["tags".to_string()],
        };
        let report = std_core::PluginCheckReport {
            manifest_path: PathBuf::from("studio-smoke/plugin.json"),
            plugin_name: "studio-smoke".to_string(),
            status: "PASS",
            actions: 2,
            permissions: vec![std_core::plugins::PluginPermission::Code],
            fs_scopes: Vec::new(),
            network_hosts: Vec::new(),
        };

        let model = PluginInspectorModel::from_selection(Some(&result), &[report], None);

        assert_eq!(model.permissions, vec!["Code"]);
    }

    #[test]
    fn inspector_model_surfaces_review_required_enable_prompt() {
        let result = SearchResult {
            action: Action::new(
                "Plugin Unchecked",
                "Unchecked plugin",
                "When validating plugin review prompt",
                ActionType::Command,
            ),
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };

        let model = PluginInspectorModel::from_selection(Some(&result), &[], None);

        assert_eq!(model.enable_state, "review_required");
        assert_eq!(
            model.review_prompt,
            "review permissions before enable: manifest UNKNOWN"
        );
    }
}
