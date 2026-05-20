use crate::ui;
use eframe::egui;
use std::path::{Path, PathBuf};
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};

const FILE_ROW_HEIGHT: f32 = 58.0;
const STATUS_ROW_HEIGHT: f32 = 52.0;
const PATH_ROW_HEIGHT: f32 = 42.0;

pub(crate) enum WorkflowFileAction {
    None,
    Select(PathBuf),
    Open(PathBuf),
}

pub(crate) fn workflow_file_row(
    ui: &mut egui::Ui,
    path: &Path,
    selected: bool,
) -> WorkflowFileAction {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), FILE_ROW_HEIGHT),
        egui::Sense::click(),
    );
    let title = workflow_label(path);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &title)
    });
    let open_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - 58.0, rect.center().y - 13.0),
        egui::vec2(52.0, 26.0),
    );
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered(), selected);
        paint_file_text(ui, rect, &title, &path.display().to_string());
        paint_open_control(ui, open_rect, response.hovered());
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        let Some(pointer) = response.interact_pointer_pos() else {
            return WorkflowFileAction::None;
        };
        if open_rect.contains(pointer) {
            WorkflowFileAction::Open(path.to_path_buf())
        } else {
            WorkflowFileAction::Select(path.to_path_buf())
        }
    } else {
        WorkflowFileAction::None
    }
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
            egui::vec2(88.0, 22.0),
        );
        paint_status_chip(ui, chip_rect, status, fill);
        let text_x = chip_rect.right() + Space::XS as f32;
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 18.0),
            egui::Align2::LEFT_CENTER,
            title,
            Text::body(),
            ui::strong_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 36.0),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn path_row(ui: &mut egui::Ui, label: &str, path: &Path) {
    let detail = path.display().to_string();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), PATH_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered(), false);
        let text_x = rect.left() + Space::SM as f32;
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 15.0),
            egui::Align2::LEFT_CENTER,
            label,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.top() + 32.0),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::strong_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn workflow_summary(ui: &mut egui::Ui, title: &str, status: &str, steps: usize) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(Text::body())
                .color(ui::strong_text(ui.ctx())),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{status} steps={steps}"))
                    .font(Text::caption())
                    .color(ui::muted_text(ui.ctx())),
            );
        });
    });
    ui.add_space(Space::TWO_XS as f32);
}

fn workflow_label(path: &Path) -> String {
    path.parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .or_else(|| path.file_stem().and_then(|name| name.to_str()))
        .unwrap_or("workflow")
        .to_string()
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
}

fn paint_file_text(ui: &mut egui::Ui, rect: egui::Rect, title: &str, detail: &str) {
    let text_x = rect.left() + Space::SM as f32;
    let right_limit = rect.right() - 68.0;
    let clip = egui::Rect::from_min_max(
        egui::pos2(text_x, rect.top()),
        egui::pos2(right_limit, rect.bottom()),
    );
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_x, rect.top() + 19.0),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(text_x, rect.top() + 39.0),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_open_control(ui: &mut egui::Ui, rect: egui::Rect, hovered: bool) {
    let fill = if hovered {
        Color::bg_surface_2(ui.ctx())
    } else {
        Color::bg_surface_0(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(Radius::SM),
        egui::Stroke::new(1.0, Color::stroke_border(ui.ctx())),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        i18n::t("studio.workflows.open"),
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
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
