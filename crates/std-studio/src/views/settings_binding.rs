use crate::views::{
    row_metrics,
    row_paint::{self, RowSurface},
};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

const BINDING_CONTROL_HEIGHT: f32 = 32.0;
const BINDING_CONTROL_HALF_HEIGHT: f32 = 16.0;

pub(crate) struct BindingRowOutput {
    pub(crate) save_clicked: bool,
}

pub(crate) fn binding_editor_row(
    ui: &mut egui::Ui,
    label: &str,
    detail: &str,
    save_label: &str,
    value: &mut String,
) -> BindingRowOutput {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::SETTINGS_BINDING_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            format!("{label} {detail} {value}"),
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            label,
            detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
    }
    let edit_rect = binding_edit_rect(rect);
    let save_rect = binding_save_rect(rect);
    ui.put(
        edit_rect,
        egui::TextEdit::singleline(value)
            .font(Text::code())
            .text_color(Color::fg_primary(ui.ctx()))
            .desired_width(edit_rect.width()),
    );
    let save_clicked = ui
        .put(
            save_rect,
            egui::Button::new(save_label)
                .fill(Color::accent_weak(ui.ctx()))
                .stroke(egui::Stroke::new(1.0, Color::accent_base(ui.ctx())))
                .corner_radius(egui::CornerRadius::same(Radius::SM)),
        )
        .clicked();
    ui.add_space(Space::TWO_XS as f32);
    BindingRowOutput { save_clicked }
}

fn binding_edit_rect(rect: egui::Rect) -> egui::Rect {
    let right_width = row_metrics::SETTINGS_BINDING_EDIT_WIDTH;
    let right = rect.right() - row_metrics::TEXT_INSET_X;
    let left = (right - right_width).max(rect.left() + 220.0);
    egui::Rect::from_min_max(
        egui::pos2(left, rect.center().y - BINDING_CONTROL_HALF_HEIGHT),
        egui::pos2(
            right - row_metrics::SETTINGS_BINDING_SAVE_WIDTH - Space::XS as f32,
            rect.center().y + BINDING_CONTROL_HALF_HEIGHT,
        ),
    )
}

fn binding_save_rect(rect: egui::Rect) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(
            rect.right() - row_metrics::TEXT_INSET_X - row_metrics::SETTINGS_BINDING_SAVE_WIDTH,
            rect.center().y - BINDING_CONTROL_HALF_HEIGHT,
        ),
        egui::vec2(
            row_metrics::SETTINGS_BINDING_SAVE_WIDTH,
            BINDING_CONTROL_HEIGHT,
        ),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn binding_editor_row_wraps_text_edit_in_token_surface() {
        let source = include_str!("settings_binding.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("binding_editor_row"));
        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("row_paint::paint_row_frame"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("TextEdit::singleline"));
        assert!(implementation.contains("save_label"));
        assert!(!implementation.contains("ui.text_edit_singleline"));
    }
}
