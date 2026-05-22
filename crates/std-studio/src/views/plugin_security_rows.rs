use crate::{
    ui,
    views::plugin_rows::{label_value_row, status_row},
};
use eframe::egui;
use std_egui::tokens::{Color, Space};
use std_studio::plugin_security::{boundary_summary, PluginBoundarySummary};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginSecuritySummary {
    pub(crate) status: &'static str,
    pub(crate) permissions: Vec<String>,
    pub(crate) fs_scopes: String,
    pub(crate) network_hosts: String,
    pub(crate) actions: String,
}

pub(crate) fn check_report_row(ui: &mut egui::Ui, report: &std_core::PluginCheckReport) {
    let boundary = boundary_summary(report);
    let detail = format!(
        "{} permissions={} fs={} network={}",
        boundary.actions,
        boundary.permissions.join(","),
        boundary.fs_scopes,
        boundary.network_hosts
    );
    status_row(
        ui,
        &report.plugin_name,
        boundary.status,
        &detail,
        ui::ok_bg(ui.ctx()),
    );
    boundary_panel(ui, &boundary);
}

pub(crate) fn security_summary_panel(ui: &mut egui::Ui, reports: &[std_core::PluginCheckReport]) {
    if reports.is_empty() {
        ui::empty_state(ui, std_egui::i18n::t("studio.plugins.checks.empty"));
        return;
    }
    let summary = plugin_security_summary(reports);
    status_row(
        ui,
        "Plugin boundary",
        summary.status,
        &format!("{} permissions", summary.permissions.len()),
        ui::selected_bg(ui.ctx()),
    );
    ui.horizontal_wrapped(|ui| {
        for permission in &summary.permissions {
            ui::chip(ui, permission, Color::accent_weak(ui.ctx()));
        }
    });
    label_value_row(ui, "actions", &summary.actions);
    label_value_row(ui, "fs", &summary.fs_scopes);
    label_value_row(ui, "network", &summary.network_hosts);
}

pub(crate) fn plugin_security_summary(
    reports: &[std_core::PluginCheckReport],
) -> PluginSecuritySummary {
    let mut permissions = reports
        .iter()
        .flat_map(|report| boundary_summary(report).permissions)
        .collect::<Vec<_>>();
    permissions.sort();
    permissions.dedup();
    let action_count = reports.iter().map(|report| report.actions).sum::<usize>();
    let fs_count = reports
        .iter()
        .flat_map(|report| report.fs_scopes.iter())
        .count();
    let network_count = reports
        .iter()
        .flat_map(|report| report.network_hosts.iter())
        .count();
    PluginSecuritySummary {
        status: if reports.iter().all(|report| report.status == "PASS") {
            "PASS"
        } else {
            "FAIL"
        },
        permissions,
        fs_scopes: count_label(fs_count),
        network_hosts: count_label(network_count),
        actions: format!("{action_count} actions"),
    }
}

fn boundary_panel(ui: &mut egui::Ui, boundary: &PluginBoundarySummary) {
    ui.horizontal_wrapped(|ui| {
        for permission in &boundary.permissions {
            ui::chip(ui, permission, Color::accent_weak(ui.ctx()));
        }
        ui::chip(
            ui,
            &format!("fs {}", boundary.fs_scopes),
            Color::bg_surface_2(ui.ctx()),
        );
        ui::chip(
            ui,
            &format!("net {}", boundary.network_hosts),
            Color::bg_surface_2(ui.ctx()),
        );
    });
    ui.add_space(Space::XS as f32);
}

fn count_label(count: usize) -> String {
    match count {
        0 => "none".to_string(),
        1 => "1 entry".to_string(),
        count => format!("{count} entries"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn plugin_security_summary_merges_permission_boundary() {
        let reports = vec![
            std_core::PluginCheckReport {
                manifest_path: PathBuf::from("a/plugin.json"),
                plugin_name: "a".to_string(),
                status: "PASS",
                actions: 2,
                permissions: vec![
                    std_core::plugins::PluginPermission::Code,
                    std_core::plugins::PluginPermission::FsScoped,
                ],
                fs_scopes: vec![PathBuf::from("fixtures")],
                network_hosts: vec!["api.local".to_string()],
            },
            std_core::PluginCheckReport {
                manifest_path: PathBuf::from("b/plugin.json"),
                plugin_name: "b".to_string(),
                status: "PASS",
                actions: 1,
                permissions: vec![std_core::plugins::PluginPermission::Code],
                fs_scopes: Vec::new(),
                network_hosts: Vec::new(),
            },
        ];

        let summary = plugin_security_summary(&reports);

        assert_eq!(summary.status, "PASS");
        assert_eq!(summary.permissions, vec!["Code", "FsScoped"]);
        assert_eq!(summary.actions, "3 actions");
        assert_eq!(summary.fs_scopes, "1 entry");
        assert_eq!(summary.network_hosts, "1 entry");
    }
}
