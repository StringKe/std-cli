use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std_egui::tokens::{Color, Space, Text};
use std_types::MemoryRecord;

pub(crate) enum MemoryRowEvent {
    None,
    Select(usize),
}

pub(crate) fn memory_row(
    ui: &mut egui::Ui,
    index: usize,
    memory: &MemoryRecord,
    selected: bool,
) -> MemoryRowEvent {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::MEMORY_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), selected, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            &memory.title,
            &memory_preview(&memory.body),
            row_metrics::DENSE_TITLE_Y,
            row_metrics::DENSE_DETAIL_Y,
        );
        paint_scope_and_tags(ui, rect, &memory.scope, &memory.tags);
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        MemoryRowEvent::Select(index)
    } else {
        MemoryRowEvent::None
    }
}

pub(crate) fn memory_metadata(ui: &mut egui::Ui, memory: &MemoryRecord) {
    let detail = format!("id={} updated={}", memory.id, memory.updated_at);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::MEMORY_META_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), true, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            &memory.title,
            &detail,
            row_metrics::DENSE_TITLE_Y,
            row_metrics::DENSE_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
    tag_bar(ui, &memory.scope, &memory.tags);
}

pub(crate) fn last_written(ui: &mut egui::Ui, memory: &MemoryRecord) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::MEMORY_LAST_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, false, false, RowSurface::Base);
        ui.painter().text(
            egui::pos2(rect.left() + row_metrics::TEXT_INSET_X, rect.center().y),
            egui::Align2::LEFT_CENTER,
            format!("last {}", memory.title),
            Text::caption(),
            ui::strong_text(ui.ctx()),
        );
    }
}

fn tag_bar(ui: &mut egui::Ui, scope: &str, tags: &[String]) {
    ui.horizontal_wrapped(|ui| {
        ui::chip(ui, scope, ui::selected_bg(ui.ctx()));
        for tag in tags {
            ui::chip(ui, tag, ui::panel_alt(ui.ctx()));
        }
    });
}

fn memory_preview(body: &str) -> String {
    let preview = body
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(row_metrics::MEMORY_BODY_LIMIT)
        .collect::<String>();
    if preview.len() < body.len() {
        format!("{preview}...")
    } else {
        preview
    }
}

fn paint_scope_and_tags(ui: &mut egui::Ui, rect: egui::Rect, scope: &str, tags: &[String]) {
    let mut x = rect.left() + row_metrics::TEXT_INSET_X;
    let y = rect.bottom() - row_metrics::CHIP_ROW_Y_19;
    for label in std::iter::once(scope)
        .chain(tags.iter().map(String::as_str))
        .take(4)
    {
        let width = (label.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::MATCH_CHIP_MIN_WIDTH,
                row_metrics::MEMORY_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(
                width,
                row_metrics::STATUS_CHIP_HEIGHT - row_metrics::MATCH_CHIP_CHAR_WIDTH,
            ),
        );
        let fill = if label == scope {
            Color::accent_weak(ui.ctx())
        } else {
            Color::bg_surface_2(ui.ctx())
        };
        row_paint::paint_chip(ui, chip_rect, label, fill);
        x += width + row_metrics::CHIP_GAP;
    }
}
