use crate::ui;
use eframe::egui;
use std::path::Path;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_types::{ActionExecutionStatus, ActionPreview, SearchResult};

const MANIFEST_ROW_HEIGHT: f32 = 54.0;
const ACTION_ROW_HEIGHT: f32 = 62.0;
const STATUS_ROW_HEIGHT: f32 = 54.0;

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
        egui::vec2(ui.available_width(), MANIFEST_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered(), false);
        paint_title_detail(ui, rect, title, &detail, 18.0, 37.0);
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
        egui::vec2(ui.available_width(), ACTION_ROW_HEIGHT),
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
        paint_row_frame(ui, rect, response.hovered(), selected);
        paint_title_detail(
            ui,
            rect,
            &result.action.name,
            &result.action.description,
            17.0,
            36.0,
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
    let detail = format!(
        "permissions={} fs_scopes={} network_hosts={}",
        report
            .permissions
            .iter()
            .map(|permission| format!("{permission:?}"))
            .collect::<Vec<_>>()
            .join(","),
        report.fs_scopes.len(),
        report.network_hosts.len()
    );
    status_row(
        ui,
        &format!("{} actions={}", report.plugin_name, report.actions),
        report.status,
        &detail,
        ui::ok_bg(ui.ctx()),
    );
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
) {
    status_row(
        ui,
        name,
        &format!("{status:?}"),
        message,
        plugin_status_fill(ui.ctx(), status),
    );
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
        egui::vec2(ui.available_width(), STATUS_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered(), false);
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(rect.left() + Space::XS as f32, rect.center().y - 11.0),
            egui::vec2(96.0, 22.0),
        );
        paint_status_chip(ui, chip_rect, status, fill);
        let text_rect = egui::Rect::from_min_max(
            egui::pos2(chip_rect.right() + Space::XS as f32, rect.top()),
            rect.right_bottom(),
        );
        paint_title_detail_at(ui, text_rect, title, detail, 18.0, 37.0);
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

fn paint_row_frame(ui: &mut egui::Ui, rect: egui::Rect, hovered: bool, selected: bool) {
    let fill = if selected {
        Color::accent_weak(ui.ctx())
    } else if hovered {
        Color::bg_surface_3(ui.ctx())
    } else {
        Color::bg_surface_1(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(Radius::SM),
        egui::Stroke::new(1.0, Color::stroke_divider(ui.ctx())),
        egui::StrokeKind::Inside,
    );
    if selected {
        let strip = egui::Rect::from_min_max(
            rect.left_top(),
            egui::pos2(rect.left() + 3.0, rect.bottom()),
        );
        ui.painter().rect_filled(
            strip,
            egui::CornerRadius::same(Radius::SM),
            Color::accent_base(ui.ctx()),
        );
    }
}

fn paint_title_detail(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    y1: f32,
    y2: f32,
) {
    let text_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + Space::SM as f32, rect.top()),
        rect.right_bottom(),
    );
    paint_title_detail_at(ui, text_rect, title, detail, y1, y2);
}

fn paint_title_detail_at(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    y1: f32,
    y2: f32,
) {
    let clip = rect.shrink2(egui::vec2(Space::TWO_XS as f32, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(rect.left(), rect.top() + y1),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(rect.left(), rect.top() + y2),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_match_chips(ui: &mut egui::Ui, rect: egui::Rect, fields: &[String]) {
    let mut x = rect.left() + Space::SM as f32;
    let y = rect.bottom() - 17.0;
    for field in fields.iter().take(3) {
        let width = (field.len() as f32 * 7.0 + 18.0).clamp(42.0, 96.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 14.0));
        paint_status_chip(ui, chip_rect, field, Color::bg_surface_2(ui.ctx()));
        x += width + Space::TWO_XS as f32;
    }
}

fn paint_status_chip(ui: &mut egui::Ui, rect: egui::Rect, status: &str, fill: egui::Color32) {
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        status,
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
}

fn plugin_status_fill(ctx: &egui::Context, status: &ActionExecutionStatus) -> egui::Color32 {
    match status {
        ActionExecutionStatus::Completed => ui::ok_bg(ctx),
        ActionExecutionStatus::Failed => ui::warn_bg(ctx),
        ActionExecutionStatus::NeedsExternalRunner => ui::warn_bg(ctx),
    }
}
