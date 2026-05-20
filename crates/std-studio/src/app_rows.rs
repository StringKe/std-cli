use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std::path::Path;
use std_egui::tokens::{Color, Space};
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
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
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
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
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
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
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
        row_paint::paint_chip(ui, chip_rect, label, fill);
        x += width + row_metrics::CHIP_GAP;
    }
}

fn app_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("App")
        .to_string()
}
