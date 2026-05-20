use crate::ui;
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_types::MemoryRecord;

const MEMORY_ROW_HEIGHT: f32 = 72.0;
const META_ROW_HEIGHT: f32 = 52.0;

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
        egui::vec2(ui.available_width(), MEMORY_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered(), selected);
        paint_title_detail(ui, rect, &memory.title, &memory_preview(&memory.body));
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
        egui::vec2(ui.available_width(), META_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered(), true);
        paint_title_detail(ui, rect, &memory.title, &detail);
    }
    ui.add_space(Space::TWO_XS as f32);
    tag_bar(ui, &memory.scope, &memory.tags);
}

pub(crate) fn last_written(ui: &mut egui::Ui, memory: &MemoryRecord) {
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 36.0), egui::Sense::hover());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, false, false);
        ui.painter().text(
            egui::pos2(rect.left() + Space::SM as f32, rect.center().y),
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
        .take(96)
        .collect::<String>();
    if preview.len() < body.len() {
        format!("{preview}...")
    } else {
        preview
    }
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

fn paint_title_detail(ui: &mut egui::Ui, rect: egui::Rect, title: &str, detail: &str) {
    let x = rect.left() + Space::SM as f32;
    let clip = rect.shrink2(egui::vec2(Space::SM as f32, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(x, rect.top() + 18.0),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(x, rect.top() + 38.0),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_scope_and_tags(ui: &mut egui::Ui, rect: egui::Rect, scope: &str, tags: &[String]) {
    let mut x = rect.left() + Space::SM as f32;
    let y = rect.bottom() - 19.0;
    for label in std::iter::once(scope)
        .chain(tags.iter().map(String::as_str))
        .take(4)
    {
        let width = (label.len() as f32 * 7.0 + 18.0).clamp(42.0, 120.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 15.0));
        let fill = if label == scope {
            Color::accent_weak(ui.ctx())
        } else {
            Color::bg_surface_2(ui.ctx())
        };
        ui.painter()
            .rect_filled(chip_rect, egui::CornerRadius::same(Radius::SM), fill);
        ui.painter().text(
            chip_rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            Text::caption(),
            ui::strong_text(ui.ctx()),
        );
        x += width + Space::TWO_XS as f32;
    }
}
