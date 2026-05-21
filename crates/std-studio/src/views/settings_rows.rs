use crate::views::{
    row_metrics,
    row_paint::{self, RowSurface},
    settings_model::SettingsCategory,
};
use eframe::egui;
use std_egui::{i18n, tokens::Space};

pub(crate) enum SettingsCategoryEvent {
    None,
    Select(SettingsCategory),
}

pub(crate) fn category_row(
    ui: &mut egui::Ui,
    category: SettingsCategory,
    selected: bool,
) -> SettingsCategoryEvent {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::SETTINGS_CATEGORY_ROW_HEIGHT,
        ),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            i18n::t(category.title_key()),
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), selected, RowSurface::Base);
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            i18n::t(category.title_key()),
            i18n::t(category.detail_key()),
            row_metrics::COMPACT_TITLE_Y,
            row_metrics::COMPACT_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        SettingsCategoryEvent::Select(category)
    } else {
        SettingsCategoryEvent::None
    }
}

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
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
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
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_inset_title_detail(
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
