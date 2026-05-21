use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std::path::Path;
use std_egui::tokens::{Color, Space, Text};
use std_studio::plugin_security::{boundary_summary, runtime_summary, PluginBoundarySummary};
use std_types::{ActionExecutionStatus, ActionPreview, SearchResult};

pub(crate) enum PluginActionRowEvent {
    None,
    Select(usize),
}

pub(crate) fn manifest_row(ui: &mut egui::Ui, path: &Path) {
    let title = path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .or_else(|| path.file_stem().and_then(|name| name.to_str()))
        .unwrap_or("plugin");
    let detail = path.display().to_string();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::PLUGIN_MANIFEST_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_title_detail(
            ui,
            rect,
            title,
            &detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn action_row(
    ui: &mut egui::Ui,
    index: usize,
    result: &SearchResult,
    selected: bool,
) -> PluginActionRowEvent {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::PLUGIN_ACTION_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Button,
            ui.is_enabled(),
            &result.action.name,
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), selected, RowSurface::Base);
        row_paint::paint_title_detail(
            ui,
            rect,
            &result.action.name,
            &result.action.description,
            row_metrics::PLUGIN_ACTION_TITLE_Y,
            row_metrics::PLUGIN_ACTION_DETAIL_Y,
        );
        paint_match_chips(ui, rect, &result.matched_fields);
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        PluginActionRowEvent::Select(index)
    } else {
        PluginActionRowEvent::None
    }
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
    runtime_row(ui, "actions", &summary.actions);
    runtime_row(ui, "fs", &summary.fs_scopes);
    runtime_row(ui, "network", &summary.network_hosts);
}

pub(crate) fn preview_panel(ui: &mut egui::Ui, preview: &ActionPreview) {
    status_row(
        ui,
        &preview.title,
        &format!("{:?}", preview.action_type),
        &format!(
            "{} examples={}",
            preview.primary_command,
            preview.examples.len()
        ),
        ui::selected_bg(ui.ctx()),
    );
    for (key, value) in &preview.metadata {
        metadata_row(ui, key, value);
    }
}

pub(crate) fn execution_panel(
    ui: &mut egui::Ui,
    name: &str,
    status: &ActionExecutionStatus,
    message: &str,
    output: Option<&serde_json::Value>,
) {
    let runtime = runtime_summary(status, output);
    status_row(
        ui,
        name,
        &runtime.status,
        message,
        plugin_status_fill(ui.ctx(), status),
    );
    runtime_row(ui, "runtime", &runtime.runtime);
    runtime_row(ui, "exit", &runtime.exit_code);
    runtime_row(ui, "duration", &runtime.duration);
    runtime_row(ui, "boundary", &runtime.boundary);
}

pub(crate) fn output_view(ui: &mut egui::Ui, output: &serde_json::Value) {
    let mut body = output.to_string();
    ui.add_sized(
        [ui.available_width(), 120.0],
        egui::TextEdit::multiline(&mut body).interactive(false),
    );
}

pub(crate) fn status_row(
    ui: &mut egui::Ui,
    title: &str,
    status: &str,
    detail: &str,
    fill: egui::Color32,
) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::PLUGIN_STATUS_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + Space::XS as f32,
                rect.center().y - row_metrics::STATUS_CHIP_Y_OFFSET,
            ),
            egui::vec2(
                row_metrics::PLUGIN_STATUS_CHIP_WIDTH,
                row_metrics::STATUS_CHIP_HEIGHT,
            ),
        );
        row_paint::paint_chip(ui, chip_rect, status, fill);
        let text_rect = egui::Rect::from_min_max(
            egui::pos2(chip_rect.right() + Space::XS as f32, rect.top()),
            rect.right_bottom(),
        );
        row_paint::paint_title_detail_at(
            ui,
            text_rect,
            title,
            detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

fn metadata_row(ui: &mut egui::Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(key)
                .font(Text::caption())
                .color(ui::muted_text(ui.ctx())),
        );
        ui.label(
            egui::RichText::new(value)
                .font(Text::caption())
                .color(ui::strong_text(ui.ctx())),
        );
    });
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

fn runtime_row(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .font(Text::caption())
                .color(ui::muted_text(ui.ctx())),
        );
        ui.label(
            egui::RichText::new(value)
                .font(Text::caption())
                .color(ui::strong_text(ui.ctx())),
        );
    });
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PluginSecuritySummary {
    pub(crate) status: &'static str,
    pub(crate) permissions: Vec<String>,
    pub(crate) fs_scopes: String,
    pub(crate) network_hosts: String,
    pub(crate) actions: String,
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

fn count_label(count: usize) -> String {
    match count {
        0 => "none".to_string(),
        1 => "1 entry".to_string(),
        count => format!("{count} entries"),
    }
}

fn paint_match_chips(ui: &mut egui::Ui, rect: egui::Rect, fields: &[String]) {
    let mut x = rect.left() + Space::SM as f32;
    let y = rect.bottom() - row_metrics::MATCH_CHIP_BOTTOM_INSET;
    for field in fields.iter().take(3) {
        let width = (field.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::MATCH_CHIP_MIN_WIDTH,
                row_metrics::MATCH_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(width, row_metrics::MATCH_CHIP_HEIGHT),
        );
        row_paint::paint_chip(ui, chip_rect, field, Color::bg_surface_2(ui.ctx()));
        x += width + row_metrics::CHIP_GAP;
    }
}

fn plugin_status_fill(ctx: &egui::Context, status: &ActionExecutionStatus) -> egui::Color32 {
    match status {
        ActionExecutionStatus::Completed => ui::ok_bg(ctx),
        ActionExecutionStatus::Failed => ui::warn_bg(ctx),
        ActionExecutionStatus::NeedsExternalRunner => ui::warn_bg(ctx),
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
