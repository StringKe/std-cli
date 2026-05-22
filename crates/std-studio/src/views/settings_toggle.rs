use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std_egui::{
    i18n,
    tokens::{Color, ControlSize, Radius, Space, Text},
};

pub(crate) enum ToggleRowEvent {
    None,
    Toggle(bool),
}

pub(crate) fn toggle_row(
    ui: &mut egui::Ui,
    title: &str,
    detail: &str,
    enabled: bool,
) -> ToggleRowEvent {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::SETTINGS_TOGGLE_ROW_HEIGHT,
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Checkbox,
            ui.is_enabled(),
            format!("{title} {detail} {}", toggle_state_label(enabled)),
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), enabled, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            title,
            detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
        paint_toggle(ui, rect, enabled);
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        ToggleRowEvent::Toggle(!enabled)
    } else {
        ToggleRowEvent::None
    }
}

fn paint_toggle(ui: &mut egui::Ui, rect: egui::Rect, enabled: bool) {
    let ctx = ui.ctx().clone();
    let toggle_rect = egui::Rect::from_center_size(
        egui::pos2(
            rect.right() - ControlSize::switch_right_inset(),
            rect.center().y,
        ),
        ControlSize::switch_size(),
    );
    let fill = if enabled {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_2(&ctx)
    };
    let stroke = if enabled {
        Color::accent_base(&ctx)
    } else {
        Color::stroke_border(&ctx)
    };
    ui.painter().rect(
        toggle_rect,
        egui::CornerRadius::same(Radius::SM),
        fill,
        egui::Stroke::new(1.0, stroke),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        toggle_rect.center(),
        egui::Align2::CENTER_CENTER,
        toggle_state_label(enabled),
        Text::caption(),
        ui::strong_text(&ctx),
    );
}

fn toggle_state_label(enabled: bool) -> &'static str {
    if enabled {
        i18n::t("studio.settings.toggle.on")
    } else {
        i18n::t("studio.settings.toggle.off")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn toggle_row_uses_token_painted_click_target_not_checkbox_widget() {
        let source = include_str!("settings_toggle.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("toggle_row"));
        assert!(implementation.contains("paint_toggle"));
        assert!(implementation.contains("WidgetType::Checkbox"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("ControlSize::switch_size()"));
        assert!(!implementation.contains("ui.checkbox"));
        let forbidden_inline_size = ["egui::", "vec2(48.0, 24.0)"].concat();
        assert!(!implementation.contains(&forbidden_inline_size));
    }
}
