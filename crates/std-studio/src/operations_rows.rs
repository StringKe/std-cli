use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface, ThreeTextRows},
    },
};
use eframe::egui;
use std_egui::tokens::Space;

pub(crate) fn gate_row(ui: &mut egui::Ui, label: &str, value: &str, detail: &str) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::OPS_GATE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_caption_body_caption(
            ui,
            rect,
            label,
            value,
            detail,
            ThreeTextRows {
                top_y: row_metrics::OPS_LABEL_Y,
                body_y: row_metrics::OPS_VALUE_Y,
                bottom_y: row_metrics::OPS_DETAIL_Y,
            },
        );
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
