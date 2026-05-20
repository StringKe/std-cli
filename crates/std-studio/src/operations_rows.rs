use crate::ui;
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

const GATE_ROW_HEIGHT: f32 = 58.0;

pub(crate) fn gate_row(ui: &mut egui::Ui, label: &str, value: &str, detail: &str) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), GATE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(ui, rect, label, value, detail);
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn completion_chip_bar(ui: &mut egui::Ui, labels: &[&str]) {
    ui.horizontal_wrapped(|ui| {
        for label in labels {
            ui::chip(ui, label, ui::warn_bg(ui.ctx()));
        }
    });
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

fn paint_title_detail(ui: &mut egui::Ui, rect: egui::Rect, label: &str, value: &str, detail: &str) {
    let text_x = rect.left() + Space::SM as f32;
    let clip = rect.shrink2(egui::vec2(Space::SM as f32, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_x, rect.top() + 15.0),
        egui::Align2::LEFT_CENTER,
        label,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(text_x, rect.top() + 32.0),
        egui::Align2::LEFT_CENTER,
        value,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(text_x, rect.top() + 49.0),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}
