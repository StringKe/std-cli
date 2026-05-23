use std_studio::StudioApp;

pub(crate) struct PluginContractInput<'a> {
    pub(crate) studio: &'a StudioApp,
    pub(crate) js_runtime: &'a str,
    pub(crate) ts_runtime: &'a str,
    pub(crate) command_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginUiContracts {
    pub(crate) status_bar: String,
    pub(crate) permission_visual: String,
    pub(crate) inspector: String,
    pub(crate) visual: String,
}

impl PluginUiContracts {
    pub(crate) fn from_input(input: PluginContractInput<'_>) -> Self {
        let status_bar = status_bar_contract(input.studio);
        let permission_visual = permission_visual_contract(input.studio);
        let inspector = inspector_contract(input.studio);
        let visual = visual_contract(VisualContractInput {
            studio: input.studio,
            js_runtime: input.js_runtime,
            ts_runtime: input.ts_runtime,
            command_count: input.command_count,
            inspector: &inspector,
            status_bar: &status_bar,
            permission_visual: &permission_visual,
        });
        Self {
            status_bar,
            permission_visual,
            inspector,
            visual,
        }
    }
}

struct VisualContractInput<'a> {
    studio: &'a StudioApp,
    js_runtime: &'a str,
    ts_runtime: &'a str,
    command_count: usize,
    inspector: &'a str,
    status_bar: &'a str,
    permission_visual: &'a str,
}

fn visual_contract(input: VisualContractInput<'_>) -> String {
    let manager = &input.studio.plugin_manager;
    let status = check_statuses(manager);
    let source = manifest_source(manager);
    let audit_log = audit_log_state(manager);
    let permission_count = permission_count(manager);
    format!(
        "list=name|version|status|source|enable;list_chip_tracks=metadata|match;status={};source={};status_bar={};inspector=description|permissions|commands|audit-log;selected_inspector={};permissions={};permission_boundary={};commands={};audit_log={};runtime=js:{}|ts:{}",
        status,
        source,
        input.status_bar,
        input.inspector,
        permission_count,
        input.permission_visual,
        input.command_count,
        audit_log,
        input.js_runtime,
        input.ts_runtime
    )
}

fn inspector_contract(studio: &StudioApp) -> String {
    let manager = &studio.plugin_manager;
    crate::views::plugin_inspector_model::PluginInspectorModel::from_selection(
        manager.plugin_actions.get(manager.selected),
        &manager.check_reports,
        manager.last_execution.as_ref(),
    )
    .contract()
}

fn status_bar_contract(studio: &StudioApp) -> String {
    let manager = &studio.plugin_manager;
    let summary = std_studio::plugin_status::summarize_plugin_status(
        &manager.check_reports,
        &manager.plugin_actions,
        manager.preview.as_ref(),
        manager.last_execution.as_ref(),
    );
    format!(
        "manifest={};actions={};preview={};runtime={};permissions={};boundary={}",
        summary.manifest_status,
        summary.action_status,
        summary.preview_status,
        summary.runtime_status,
        summary.permission_status,
        summary.boundary_status
    )
}

fn permission_visual_contract(studio: &StudioApp) -> String {
    let manager = &studio.plugin_manager;
    let permission_label = permission_labels(manager).join("|");
    let checks = check_statuses(manager);
    format!(
        "manifest_checks={};permissions={};boundary_panel=permissions|fs|network|actions;runtime_panel=status|runtime|exit|duration|boundary",
        checks, permission_label
    )
}

fn check_statuses(manager: &std_egui::PluginManagerViewModel) -> String {
    manager
        .check_reports
        .iter()
        .map(|report| report.status.to_string())
        .collect::<std::collections::BTreeSet<String>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join("|")
}

fn permission_labels(manager: &std_egui::PluginManagerViewModel) -> Vec<String> {
    let labels = manager
        .check_reports
        .iter()
        .flat_map(|report| report.permissions.iter())
        .map(|permission| format!("{permission:?}"))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if labels.is_empty() {
        vec!["ReadOnly".to_string()]
    } else {
        labels
    }
}

fn permission_count(manager: &std_egui::PluginManagerViewModel) -> usize {
    manager
        .check_reports
        .iter()
        .flat_map(|report| report.permissions.iter())
        .count()
}

fn manifest_source(manager: &std_egui::PluginManagerViewModel) -> &'static str {
    if manager.manifest_paths.iter().any(|path| path.exists()) {
        "local-path"
    } else {
        "missing"
    }
}

fn audit_log_state(manager: &std_egui::PluginManagerViewModel) -> &'static str {
    if manager.last_execution.is_some() {
        "visible"
    } else {
        "missing"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std_types::{ActionExecution, ActionExecutionStatus};
    use uuid::Uuid;

    #[test]
    fn permission_contract_defaults_to_readonly_when_no_permissions_exist() {
        let mut studio = StudioApp::default();
        studio.plugin_manager = empty_manager();

        let contract = permission_visual_contract(&studio);

        assert!(contract.contains("manifest_checks="));
        assert!(contract.contains("permissions=ReadOnly"));
        assert!(contract.contains("boundary_panel=permissions|fs|network|actions"));
        assert!(contract.contains("runtime_panel=status|runtime|exit|duration|boundary"));
    }

    #[test]
    fn visual_contract_tracks_manifest_source_and_audit_log() {
        let mut studio = StudioApp::default();
        studio.plugin_manager = empty_manager();
        let plugin_dir = studio.core.config.plugins_dir().join("contract-plugin");
        std::fs::create_dir_all(&plugin_dir).unwrap();
        let manifest_path = plugin_dir.join("plugin.json");
        std::fs::write(&manifest_path, "{}").unwrap();
        studio.plugin_manager.manifest_paths.push(manifest_path);
        studio.plugin_manager.last_execution = Some(ActionExecution {
            action_id: Uuid::new_v4(),
            action_name: "Contract Plugin".to_string(),
            status: ActionExecutionStatus::Completed,
            message: "ok".to_string(),
            output: None,
            created_at: Utc::now(),
        });

        let contracts = PluginUiContracts::from_input(PluginContractInput {
            studio: &studio,
            js_runtime: "deno_core",
            ts_runtime: "deno_core",
            command_count: 2,
        });

        assert!(contracts.visual.contains("source=local-path"));
        assert!(contracts.visual.contains("audit_log=visible"));
        assert!(contracts
            .visual
            .contains("runtime=js:deno_core|ts:deno_core"));
    }

    #[test]
    fn missing_manifest_source_is_explicit() {
        let mut studio = StudioApp::default();
        studio.plugin_manager = empty_manager();

        let contracts = PluginUiContracts::from_input(PluginContractInput {
            studio: &studio,
            js_runtime: "Missing",
            ts_runtime: "Missing",
            command_count: 0,
        });

        assert!(contracts.visual.contains("source=missing"));
        assert!(contracts.visual.contains("audit_log=missing"));
    }

    fn empty_manager() -> std_egui::PluginManagerViewModel {
        std_egui::PluginManagerViewModel {
            manifest_paths: Vec::new(),
            plugin_actions: Vec::new(),
            selected: 0,
            preview: None,
            last_execution: None,
            check_reports: Vec::new(),
        }
    }
}
