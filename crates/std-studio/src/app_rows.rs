use crate::ui;
use eframe::egui;
use std::path::Path;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_types::SearchResult;

const SEARCH_ROW_HEIGHT: f32 = 78.0;
const REGISTERED_ROW_HEIGHT: f32 = 60.0;

pub(crate) enum AppRowEvent {
    None,
    Select(String),
}

pub(crate) fn search_result_row(ui: &mut egui::Ui, result: &SearchResult) -> AppRowEvent {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), SEARCH_ROW_HEIGHT),
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
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &result.action.name,
            &result.action.description,
            18.0,
            38.0,
        );
        paint_search_chips(ui, rect, result);
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        AppRowEvent::Select(result.action.name.replace("Open App: ", ""))
    } else {
        AppRowEvent::None
    }
}

pub(crate) fn registered_app_row(ui: &mut egui::Ui, path: &Path) -> AppRowEvent {
    let name = app_name(path);
    let detail = path.display().to_string();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), REGISTERED_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), name.as_str())
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(ui, rect, &name, &detail, 19.0, 40.0);
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        AppRowEvent::Select(name)
    } else {
        AppRowEvent::None
    }
}

pub(crate) fn storage_row(ui: &mut egui::Ui, path: &Path) {
    let detail = path.display().to_string();
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 42.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), "storage")
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(ui, rect, "storage", &detail, 16.0, 33.0);
    }
}

fn paint_search_chips(ui: &mut egui::Ui, rect: egui::Rect, result: &SearchResult) {
    let mut chips = vec![
        format!("score={:.2}", result.score),
        "external runner".to_string(),
    ];
    chips.extend(result.matched_fields.iter().take(3).cloned());
    let mut x = rect.left() + Space::SM as f32;
    let y = rect.bottom() - 19.0;
    for (index, label) in chips.iter().enumerate() {
        let width = (label.len() as f32 * 7.0 + 18.0).clamp(54.0, 130.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 15.0));
        let fill = if index == 1 {
            ui::warn_bg(ui.ctx())
        } else {
            Color::bg_surface_2(ui.ctx())
        };
        paint_chip(ui, chip_rect, label, fill);
        x += width + Space::TWO_XS as f32;
    }
}

fn paint_row_frame(ui: &mut egui::Ui, rect: egui::Rect, hovered: bool) {
    let fill = if hovered {
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

fn paint_title_detail(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    y1: f32,
    y2: f32,
) {
    let x = rect.left() + Space::SM as f32;
    let clip = rect.shrink2(egui::vec2(Space::SM as f32, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(x, rect.top() + y1),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(x, rect.top() + y2),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_chip(ui: &mut egui::Ui, rect: egui::Rect, label: &str, fill: egui::Color32) {
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
}

fn app_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("App")
        .to_string()
}
