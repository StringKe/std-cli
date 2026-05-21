use crate::views::{
    row_metrics,
    row_paint::{self, RowSurface},
    settings_model::SettingsCategory,
};
use eframe::egui;
use std_core::shortcuts::ShortcutSpec;
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};

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

pub(crate) fn theme_mode_control(ui: &mut egui::Ui, current: &str) -> Option<&'static str> {
    let current = normalized_theme_mode(current);
    let mut selected = None;
    ui.horizontal(|ui| {
        for mode in ["system", "dark", "light"] {
            let response = theme_mode_button(ui, mode, mode == current);
            if response.clicked() {
                selected = Some(mode);
            }
        }
    });
    selected
}

fn normalized_theme_mode(value: &str) -> &str {
    match value.trim().to_ascii_lowercase().as_str() {
        "dark" => "dark",
        "light" => "light",
        _ => "system",
    }
}

fn theme_mode_button(ui: &mut egui::Ui, mode: &'static str, selected: bool) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if selected {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_1(&ctx)
    };
    let stroke = if selected {
        egui::Stroke::new(1.0, Color::accent_base(&ctx))
    } else {
        egui::Stroke::new(1.0, Color::stroke_divider(&ctx))
    };
    ui.add(
        egui::Button::new(
            egui::RichText::new(theme_mode_label(mode))
                .font(Text::body())
                .color(Color::fg_primary(&ctx)),
        )
        .fill(fill)
        .stroke(stroke)
        .corner_radius(egui::CornerRadius::same(Radius::SM)),
    )
}

fn theme_mode_label(mode: &str) -> &'static str {
    match mode {
        "dark" => i18n::t("studio.settings.theme.dark"),
        "light" => i18n::t("studio.settings.theme.light"),
        _ => i18n::t("studio.settings.theme.system"),
    }
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

#[cfg(test)]
mod tests {
    #[test]
    fn theme_mode_control_uses_segmented_buttons_not_text_input() {
        let source = include_str!("settings_rows.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("theme_mode_control"));
        assert!(implementation.contains("[\"system\", \"dark\", \"light\"]"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("Color::accent_base"));
        assert!(!implementation.contains("text_edit_singleline(&mut self.settings_theme)"));
    }
}
