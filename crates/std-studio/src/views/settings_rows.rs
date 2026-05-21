use crate::views::{
    row_metrics,
    row_paint::{self, RowSurface},
    settings_model::SettingsCategory,
};
use eframe::egui;
use std_core::shortcuts::ShortcutSpec;
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
        row_metrics::settings_path_row_size(ui.available_width()),
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

pub(crate) enum ShortcutRowEvent {
    None,
    Reset(&'static str),
}

pub(crate) fn shortcut_row(ui: &mut egui::Ui, shortcut: &ShortcutSpec) -> ShortcutRowEvent {
    let (rect, response) = ui.allocate_exact_size(
        row_metrics::settings_path_row_size(ui.available_width()),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            shortcut_a11y_label(shortcut),
        )
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        let detail = format!(
            "{} / {} / {}",
            shortcut.scope.label(),
            shortcut.source.label(),
            shortcut.default_binding
        );
        row_paint::paint_inset_title_detail(
            ui,
            rect,
            shortcut.action,
            &detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
        let shortcut_pos = egui::pos2(
            rect.right() - row_metrics::SETTINGS_SHORTCUT_BINDING_X,
            rect.center().y,
        );
        ui.painter().text(
            shortcut_pos,
            egui::Align2::LEFT_CENTER,
            &shortcut.binding,
            egui::TextStyle::Monospace.resolve(ui.style()),
            std_egui::tokens::Color::fg_secondary(ui.ctx()),
        );
    }
    let reset_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.right() - row_metrics::SETTINGS_SHORTCUT_RESET_WIDTH,
            rect.top(),
        ),
        egui::Vec2::new(row_metrics::SETTINGS_SHORTCUT_RESET_WIDTH, rect.height()),
    );
    let reset_response = ui.put(
        reset_rect,
        egui::Button::new(i18n::t("studio.settings.hotkey.reset")),
    );
    ui.add_space(Space::TWO_XS as f32);
    if shortcut.resettable && reset_response.clicked() {
        ShortcutRowEvent::Reset(shortcut.id)
    } else {
        ShortcutRowEvent::None
    }
}

fn shortcut_a11y_label(shortcut: &ShortcutSpec) -> String {
    format!(
        "{}, {}, {}, source {}, default {}",
        i18n::t("studio.settings.hotkey.row"),
        shortcut.action,
        shortcut.binding,
        shortcut.source.label(),
        shortcut.default_binding
    )
}
