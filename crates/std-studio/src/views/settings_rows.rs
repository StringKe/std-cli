use crate::{ui, views::row_metrics};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

pub(crate) fn config_path_row(ui: &mut egui::Ui, path: &str) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::SETTINGS_CONFIG_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), path));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            "config",
            path,
            row_metrics::PATH_TITLE_Y,
            row_metrics::SETTINGS_CONFIG_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn resolved_path_row(ui: &mut egui::Ui, key: &str, value: &str) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::SETTINGS_PATH_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), key));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            key,
            value,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
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
    let text_x = rect.left() + row_metrics::TEXT_INSET_X;
    let clip = rect.shrink2(egui::vec2(row_metrics::WIDE_CLIP_INSET_X, 0.0));
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_x, rect.top() + y1),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(text_x, rect.top() + y2),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}
