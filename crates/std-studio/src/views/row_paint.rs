use crate::{ui, views::row_metrics};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

#[derive(Clone, Copy)]
pub(crate) enum RowSurface {
    Base,
    Raised,
}

pub(crate) fn paint_row_frame(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    hovered: bool,
    selected: bool,
    surface: RowSurface,
) {
    ui.painter().rect_filled(
        rect,
        egui::CornerRadius::same(Radius::SM),
        row_fill(ui, hovered, selected, surface),
    );
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(Radius::SM),
        egui::Stroke::new(1.0, Color::stroke_divider(ui.ctx())),
        egui::StrokeKind::Inside,
    );
    if selected {
        paint_selected_strip(ui, rect);
    }
}

pub(crate) fn paint_title_detail(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    title_y: f32,
    detail_y: f32,
) {
    let text_rect = egui::Rect::from_min_max(
        egui::pos2(rect.left() + Space::SM as f32, rect.top()),
        rect.right_bottom(),
    );
    paint_title_detail_at(ui, text_rect, title, detail, title_y, detail_y);
}

pub(crate) fn paint_title_detail_at(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    title_y: f32,
    detail_y: f32,
) {
    let clip = rect.shrink2(egui::vec2(row_metrics::CLIP_INSET_X, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(rect.left(), rect.top() + title_y),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(rect.left(), rect.top() + detail_y),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

pub(crate) fn paint_chip(ui: &mut egui::Ui, rect: egui::Rect, label: &str, fill: egui::Color32) {
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

fn row_fill(ui: &egui::Ui, hovered: bool, selected: bool, surface: RowSurface) -> egui::Color32 {
    if selected {
        Color::accent_weak(ui.ctx())
    } else if hovered {
        Color::bg_surface_3(ui.ctx())
    } else {
        match surface {
            RowSurface::Base => Color::bg_surface_1(ui.ctx()),
            RowSurface::Raised => Color::bg_surface_2(ui.ctx()),
        }
    }
}

fn paint_selected_strip(ui: &mut egui::Ui, rect: egui::Rect) {
    let strip = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2(
            rect.left() + row_metrics::SELECTED_STRIP_WIDTH,
            rect.bottom(),
        ),
    );
    ui.painter().rect_filled(
        strip,
        egui::CornerRadius::same(Radius::SM),
        Color::accent_base(ui.ctx()),
    );
}
