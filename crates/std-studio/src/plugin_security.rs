use std_core::{plugins::PluginPermission, PluginCheckReport};
use std_types::ActionExecutionStatus;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginBoundarySummary {
    pub status: &'static str,
    pub permissions: Vec<String>,
    pub fs_scopes: String,
    pub network_hosts: String,
    pub actions: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginRuntimeSummary {
    pub status: String,
    pub runtime: String,
    pub exit_code: String,
    pub duration: String,
    pub boundary: String,
}

pub fn boundary_summary(report: &PluginCheckReport) -> PluginBoundarySummary {
    PluginBoundarySummary {
        status: report.status,
        permissions: permission_labels(&report.permissions),
        fs_scopes: count_or_list(
            report
                .fs_scopes
                .iter()
                .map(|path| path.display().to_string()),
        ),
        network_hosts: count_or_list(report.network_hosts.iter().cloned()),
        actions: format!("{} actions", report.actions),
    }
}

pub fn runtime_summary(
    status: &ActionExecutionStatus,
    output: Option<&serde_json::Value>,
) -> PluginRuntimeSummary {
    let runtime = output
        .and_then(|value| value.get("runtime"))
        .and_then(|value| value.as_str())
        .unwrap_or("deferred")
        .to_string();
    PluginRuntimeSummary {
        status: format!("{status:?}"),
        exit_code: number_field(output, "exit_code"),
        duration: duration_field(output),
        boundary: runtime_boundary(status, &runtime),
        runtime,
    }
}

fn permission_labels(permissions: &[PluginPermission]) -> Vec<String> {
    let labels = permissions
        .iter()
        .map(|permission| format!("{permission:?}"))
        .collect::<Vec<_>>();
    if labels.is_empty() {
        vec!["ReadOnly".to_string()]
    } else {
        labels
    }
}

fn count_or_list(values: impl Iterator<Item = String>) -> String {
    let values = values.filter(|value| !value.is_empty()).collect::<Vec<_>>();
    match values.len() {
        0 => "none".to_string(),
        1 => values[0].clone(),
        count => format!("{count} entries"),
    }
}

fn number_field(output: Option<&serde_json::Value>, key: &str) -> String {
    output
        .and_then(|value| value.get(key))
        .and_then(|value| value.as_i64())
        .map(|value| value.to_string())
        .unwrap_or_else(|| "none".to_string())
}

fn duration_field(output: Option<&serde_json::Value>) -> String {
    output
        .and_then(|value| value.get("duration_ms"))
        .and_then(|value| value.as_u64())
        .map(|value| format!("{value} ms"))
        .unwrap_or_else(|| "none".to_string())
}

fn runtime_boundary(status: &ActionExecutionStatus, runtime: &str) -> String {
    match status {
        ActionExecutionStatus::NeedsExternalRunner => "external runner deferred".to_string(),
        ActionExecutionStatus::Completed | ActionExecutionStatus::Failed => {
            format!("{runtime} controlled runtime")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn boundary_summary_exposes_permissions_and_scopes() {
        let report = PluginCheckReport {
            manifest_path: PathBuf::from("plugin.json"),
            plugin_name: "checked".to_string(),
            status: "PASS",
            actions: 2,
            permissions: vec![PluginPermission::Code, PluginPermission::FsScoped],
            fs_scopes: vec![PathBuf::from("fixtures")],
            network_hosts: vec!["api.local".to_string(), "cdn.local".to_string()],
        };

        let summary = boundary_summary(&report);

        assert_eq!(summary.permissions, vec!["Code", "FsScoped"]);
        assert_eq!(summary.fs_scopes, "fixtures");
        assert_eq!(summary.network_hosts, "2 entries");
        assert_eq!(summary.actions, "2 actions");
    }

    #[test]
    fn runtime_summary_exposes_controlled_runtime() {
        let output = serde_json::json!({
            "runtime": "deno_core",
            "exit_code": 0,
            "duration_ms": 12
        });

        let summary = runtime_summary(&ActionExecutionStatus::Completed, Some(&output));

        assert_eq!(summary.status, "Completed");
        assert_eq!(summary.runtime, "deno_core");
        assert_eq!(summary.exit_code, "0");
        assert_eq!(summary.duration, "12 ms");
        assert_eq!(summary.boundary, "deno_core controlled runtime");
    }
}
