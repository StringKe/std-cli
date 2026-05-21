use std_types::{ActionType, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginListRowModel {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) status: String,
    pub(crate) source: String,
    pub(crate) enable: String,
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
        Self {
            name,
            version: "0.1.0".to_string(),
            status,
            source: source_label(result),
            enable: enable_label(&result.action.action_type),
            detail: result.action.description.clone(),
        }
    }

    #[cfg(test)]
    pub(crate) fn contract(&self) -> String {
        format!(
            "name={};version={};status={};source={};enable={}",
            self.name, self.version, self.status, self.source, self.enable
        )
    }
}

fn plugin_result_matches_report(
    result: &SearchResult,
    report: &std_core::PluginCheckReport,
) -> bool {
    result.action.name.contains(report.plugin_name.as_str())
        || result
            .action
            .description
            .contains(report.plugin_name.as_str())
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
            "name=Studio TS Smoke;version=0.1.0;status=PASS;source=local-path;enable=enabled"
        );
        assert!(row.detail.contains("Run studio-smoke TypeScript plugin"));
    }
}
