use crate::ui;
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_index::{ComponentDigest, IndexCoverage, IndexCoverageItem, IndexDocument};

const OVERVIEW_ROW_HEIGHT: f32 = 88.0;
const COMPONENT_ROW_HEIGHT: f32 = 74.0;
const COVERAGE_ROW_HEIGHT: f32 = 78.0;

pub(crate) fn document_overview_row(ui: &mut egui::Ui, document: &IndexDocument) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), OVERVIEW_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            &document.overview.name,
        )
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &document.overview.name,
            &document.overview.path.display().to_string(),
            18.0,
            38.0,
        );
        paint_document_chips(ui, rect, document);
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn component_row(ui: &mut egui::Ui, component: &ComponentDigest) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), COMPONENT_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    let title = component.path.display().to_string();
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title.as_str())
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(ui, rect, &title, &component.purpose, 18.0, 38.0);
        paint_component_chips(ui, rect, component);
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn coverage_item_row(ui: &mut egui::Ui, item: &IndexCoverageItem) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), COVERAGE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &item.name)
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        let detail = format!(
            "components={} relations={} history={}",
            item.component_count, item.relation_count, item.history_count
        );
        paint_title_detail(ui, rect, &item.name, &detail, 18.0, 38.0);
        paint_coverage_chips(ui, rect, &item.coverage);
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn coverage_summary(
    ui: &mut egui::Ui,
    total: usize,
    complete: usize,
    incomplete: usize,
) {
    ui.horizontal_wrapped(|ui| {
        ui::chip(ui, &format!("total={total}"), ui::panel_alt(ui.ctx()));
        ui::chip(ui, &format!("complete={complete}"), ui::ok_bg(ui.ctx()));
        ui::chip(
            ui,
            &format!("incomplete={incomplete}"),
            ui::warn_bg(ui.ctx()),
        );
    });
}

fn paint_document_chips(ui: &mut egui::Ui, rect: egui::Rect, document: &IndexDocument) {
    let chips = [
        format!("{:?}", document.overview.kind),
        format!("components={}", document.components.len()),
        format!("relations={}", document.relations.len()),
        format!("history={}", document.history.len()),
    ];
    paint_chips(
        ui,
        rect.left() + Space::SM as f32,
        rect.bottom() - 21.0,
        &chips,
    );
}

fn paint_component_chips(ui: &mut egui::Ui, rect: egui::Rect, component: &ComponentDigest) {
    let mut chips = vec![
        component.language.clone(),
        format!("size={}", component.size_bytes),
    ];
    if !component.symbols.is_empty() {
        chips.push(format!("symbols={}", component.symbols.len()));
    }
    paint_chips(
        ui,
        rect.left() + Space::SM as f32,
        rect.bottom() - 19.0,
        &chips,
    );
}

fn paint_coverage_chips(ui: &mut egui::Ui, rect: egui::Rect, coverage: &IndexCoverage) {
    let chips = [
        ("overview", coverage.entity_overview),
        ("components", coverage.component_digest),
        ("relations", coverage.symbol_relation_index),
        ("history", coverage.historical_context),
    ];
    let mut x = rect.left() + Space::SM as f32;
    let y = rect.bottom() - 19.0;
    for (label, pass) in chips {
        let width = (label.len() as f32 * 7.0 + 18.0).clamp(58.0, 104.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 15.0));
        paint_chip(
            ui,
            chip_rect,
            label,
            if pass {
                ui::ok_bg(ui.ctx())
            } else {
                ui::warn_bg(ui.ctx())
            },
        );
        x += width + Space::TWO_XS as f32;
    }
}

fn paint_chips(ui: &mut egui::Ui, start_x: f32, y: f32, labels: &[String]) {
    let mut x = start_x;
    for label in labels.iter().take(4) {
        let width = (label.len() as f32 * 7.0 + 18.0).clamp(52.0, 132.0);
        let chip_rect = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(width, 15.0));
        paint_chip(ui, chip_rect, label, Color::bg_surface_2(ui.ctx()));
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
