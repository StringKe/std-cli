use crate::{ui, views::row_metrics};
use eframe::egui;
use std::path::Path;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_types::SearchResult;

pub(crate) enum AppRowEvent {
    None,
    Select(String),
}

pub(crate) fn search_result_row(ui: &mut egui::Ui, result: &SearchResult) -> AppRowEvent {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::APP_SEARCH_ROW_HEIGHT),
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
            row_metrics::DENSE_TITLE_Y,
            row_metrics::DENSE_DETAIL_Y,
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
        egui::vec2(ui.available_width(), row_metrics::APP_REGISTERED_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), name.as_str())
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &name,
            &detail,
            row_metrics::TALL_TITLE_Y,
            row_metrics::TALL_DETAIL_Y,
        );
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
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::APP_STORAGE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), "storage")
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            "storage",
            &detail,
            row_metrics::COMPACT_TITLE_Y,
            row_metrics::PATH_DETAIL_Y,
        );
    }
}

fn paint_search_chips(ui: &mut egui::Ui, rect: egui::Rect, result: &SearchResult) {
    let mut chips = vec![
        format!("score={:.2}", result.score),
        "external runner".to_string(),
    ];
    chips.extend(result.matched_fields.iter().take(3).cloned());
    let mut x = rect.left() + row_metrics::TEXT_INSET_X;
    let y = rect.bottom() - row_metrics::CHIP_ROW_Y_19;
    for (index, label) in chips.iter().enumerate() {
        let width = (label.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::APP_CHIP_MIN_WIDTH,
                row_metrics::APP_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(
                width,
                row_metrics::STATUS_CHIP_HEIGHT - row_metrics::MATCH_CHIP_CHAR_WIDTH,
            ),
        );
        let fill = if index == 1 {
            ui::warn_bg(ui.ctx())
        } else {
            Color::bg_surface_2(ui.ctx())
        };
        paint_chip(ui, chip_rect, label, fill);
        x += width + row_metrics::CHIP_GAP;
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
    let x = rect.left() + row_metrics::TEXT_INSET_X;
    let clip = rect.shrink2(egui::vec2(row_metrics::WIDE_CLIP_INSET_X, 0.0));
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
