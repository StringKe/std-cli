use std_types::{ActionType, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginListRowModel {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) source: String,
    pub(crate) enable: String,
    pub(crate) enable_state: String,
    pub(crate) review_prompt: String,
    pub(crate) detail: String,
}

impl PluginListRowModel {
    pub(crate) fn from_result(
        result: &SearchResult,
        reports: &[std_core::PluginCheckReport],
    ) -> Self {
        let name = result
            .action
            .name
            .strip_prefix("Plugin ")
            .unwrap_or(&result.action.name)
            .to_string();
        let status = reports
            .iter()
            .find(|report| plugin_result_matches_report(result, report))
            .map(|report| report.status.to_string())
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let enable_state = enable_state(&status, &result.action.action_type);
        Self {
            name,
            version: "0.1.0".to_string(),
            status: status.clone(),
            source: source_label(result),
            enable: enable_label(&result.action.action_type),
            review_prompt: review_prompt(&status, &enable_state),
            enable_state,
            detail: result.action.description.clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn contract(&self) -> String {
        format!(
            "name={};version={};status={};source={};enable={};enable_state={};review_prompt={}",
            self.name,
            self.version,
            self.status,
            self.source,
            self.enable,
            self.enable_state,
            self.review_prompt
        )
    }
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

fn source_label(result: &SearchResult) -> String {
    result
        .action
        .examples
        .first()
        .map(|example| {
            if example.ends_with(".js") || example.ends_with(".ts") {
                "local-path"
            } else {
                "command"
            }
        })
        .unwrap_or("local-path")
        .to_string()
}

fn enable_label(action_type: &ActionType) -> String {
    if matches!(action_type, ActionType::Command) {
        "enabled".to_string()
    } else {
        "disabled".to_string()
    }
}

fn enable_state(status: &str, action_type: &ActionType) -> String {
    if !matches!(action_type, ActionType::Command) {
        return "disabled".to_string();
    }
    if status == "PASS" {
        "enabled".to_string()
    } else {
        "review_required".to_string()
    }
}

fn review_prompt(status: &str, enable_state: &str) -> String {
    if enable_state == "review_required" {
        format!("review permissions before enable: manifest {status}")
    } else {
        "none".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std_types::{Action, ActionType};

    #[test]
    fn plugin_list_row_model_exposes_docs_columns() {
        let action = Action {
            examples: vec!["main.ts".to_string()],
            ..Action::new(
                "Plugin Studio TS Smoke",
                "Run studio-smoke TypeScript plugin",
                "When validating plugin list",
                ActionType::Command,
            )
        };
        let result = SearchResult {
            action,
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

        let row = PluginListRowModel::from_result(&result, &[report]);

        assert_eq!(
            row.contract(),
            "name=Studio TS Smoke;version=0.1.0;status=PASS;source=local-path;enable=enabled;enable_state=enabled;review_prompt=none"
        );
        assert!(row.detail.contains("Run studio-smoke TypeScript plugin"));
    }

    #[test]
    fn plugin_list_row_requires_review_before_enabling_unchecked_plugin() {
        let action = Action::new(
            "Plugin Unchecked",
            "Unchecked plugin",
            "When validating review prompt",
            ActionType::Command,
        );
        let result = SearchResult {
            action,
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };

        let row = PluginListRowModel::from_result(&result, &[]);

        assert_eq!(row.status, "UNKNOWN");
        assert_eq!(row.enable, "enabled");
        assert_eq!(row.enable_state, "review_required");
        assert_eq!(
            row.review_prompt,
            "review permissions before enable: manifest UNKNOWN"
        );
    }
}
