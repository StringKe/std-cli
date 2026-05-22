use crate::{
    ui,
    views::{
        plugin_inspector_model::PluginInspectorModel,
        plugin_list_model::PluginListRowModel,
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std::path::Path;
use std_egui::tokens::{Color, Space, Text};
use std_types::{ActionPreview, SearchResult};

pub(crate) enum PluginActionRowEvent {
    None,
    Select(usize),
}

pub(crate) fn manifest_row(ui: &mut egui::Ui, path: &Path) {
    let title = path
        .parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .or_else(|| path.file_stem().and_then(|name| name.to_str()))
        .unwrap_or("plugin");
    let detail = path.display().to_string();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::PLUGIN_MANIFEST_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        row_paint::paint_title_detail(
            ui,
            rect,
            title,
            &detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn action_row(
    ui: &mut egui::Ui,
    index: usize,
    result: &SearchResult,
    reports: &[std_core::PluginCheckReport],
    selected: bool,
) -> PluginActionRowEvent {
    let model = PluginListRowModel::from_result(result, reports);
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::PLUGIN_LIST_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &model.name)
    });
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), selected, RowSurface::Base);
        row_paint::paint_title_detail(
            ui,
            rect,
            &model.name,
            &model.detail,
            row_metrics::PLUGIN_LIST_TITLE_Y,
            row_metrics::PLUGIN_LIST_DETAIL_Y,
        );
        paint_plugin_list_chips(ui, rect, &model, PluginChipTrack::Metadata);
        paint_match_chips(ui, rect, &result.matched_fields, PluginChipTrack::Match);
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        PluginActionRowEvent::Select(index)
    } else {
        PluginActionRowEvent::None
    }
}

pub(crate) fn inspector_context_panel(ui: &mut egui::Ui, model: &PluginInspectorModel) {
    status_row(
        ui,
        "Inspector",
        "SELECTED",
        &model.description,
        ui::selected_bg(ui.ctx()),
    );
    label_value_row(ui, "permissions", &model.permissions.join(","));
    label_value_row(ui, "commands", &model.commands.join(","));
    label_value_row(ui, "enable", &model.enable_state);
    label_value_row(ui, "review", &model.review_prompt);
    label_value_row(ui, "audit log", &model.audit_log);
}

pub(crate) fn preview_panel(ui: &mut egui::Ui, preview: &ActionPreview) {
    status_row(
        ui,
        &preview.title,
        &format!("{:?}", preview.action_type),
        &format!(
            "{} examples={}",
            preview.primary_command,
            preview.examples.len()
        ),
        ui::selected_bg(ui.ctx()),
    );
    for (key, value) in &preview.metadata {
        label_value_row(ui, key, value);
    }
}

pub(crate) fn status_row(
    ui: &mut egui::Ui,
    title: &str,
    status: &str,
    detail: &str,
    fill: egui::Color32,
) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::PLUGIN_STATUS_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + Space::XS as f32,
                rect.center().y - row_metrics::STATUS_CHIP_Y_OFFSET,
            ),
            egui::vec2(
                row_metrics::PLUGIN_STATUS_CHIP_WIDTH,
                row_metrics::STATUS_CHIP_HEIGHT,
            ),
        );
        row_paint::paint_chip(ui, chip_rect, status, fill);
        let text_rect = egui::Rect::from_min_max(
            egui::pos2(chip_rect.right() + Space::XS as f32, rect.top()),
            rect.right_bottom(),
        );
        row_paint::paint_title_detail_at(
            ui,
            text_rect,
            title,
            detail,
            row_metrics::PLUGIN_META_TITLE_Y,
            row_metrics::PLUGIN_META_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn label_value_row(ui: &mut egui::Ui, key: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(key)
                .font(Text::caption())
                .color(ui::muted_text(ui.ctx())),
        );
        ui.label(
            egui::RichText::new(value)
                .font(Text::caption())
                .color(ui::strong_text(ui.ctx())),
        );
    });
}

enum PluginChipTrack {
    Metadata,
    Match,
}

fn paint_plugin_list_chips(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    model: &PluginListRowModel,
    track: PluginChipTrack,
) {
    let chips = [
        format!("v{}", model.version),
        model.status.clone(),
        model.source.clone(),
        model.enable.clone(),
        model.enable_state.clone(),
    ];
    let mut x = rect.left() + Space::SM as f32;
    let y = plugin_chip_y(rect, track);
    for chip in chips {
        let width = (chip.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::MATCH_CHIP_MIN_WIDTH,
                row_metrics::MATCH_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(width, row_metrics::MATCH_CHIP_HEIGHT),
        );
        row_paint::paint_chip(ui, chip_rect, &chip, Color::bg_surface_2(ui.ctx()));
        x += width + row_metrics::CHIP_GAP;
    }
}

fn paint_match_chips(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    fields: &[String],
    track: PluginChipTrack,
) {
    let mut x = rect.left() + Space::SM as f32;
    let y = plugin_chip_y(rect, track);
    for field in fields.iter().take(3) {
        let width = (field.len() as f32 * row_metrics::MATCH_CHIP_CHAR_WIDTH
            + row_metrics::MATCH_CHIP_TEXT_PAD)
            .clamp(
                row_metrics::MATCH_CHIP_MIN_WIDTH,
                row_metrics::MATCH_CHIP_MAX_WIDTH,
            );
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(x, y),
            egui::vec2(width, row_metrics::MATCH_CHIP_HEIGHT),
        );
        row_paint::paint_chip(ui, chip_rect, field, Color::bg_surface_2(ui.ctx()));
        x += width + row_metrics::CHIP_GAP;
    }
}

fn plugin_chip_y(rect: egui::Rect, track: PluginChipTrack) -> f32 {
    let inset = match track {
        PluginChipTrack::Metadata => row_metrics::PLUGIN_META_CHIP_BOTTOM_INSET,
        PluginChipTrack::Match => row_metrics::PLUGIN_MATCH_CHIP_BOTTOM_INSET,
    };
    rect.bottom() - inset
}

#[cfg(test)]
mod tests {
    #[test]
    fn plugin_list_chips_use_separate_metadata_and_match_tracks() {
        let source = include_str!("plugin_rows.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("PluginChipTrack::Metadata"));
        assert!(implementation.contains("PluginChipTrack::Match"));
        assert!(implementation.contains("PLUGIN_META_CHIP_BOTTOM_INSET"));
        assert!(implementation.contains("PLUGIN_MATCH_CHIP_BOTTOM_INSET"));
        assert!(implementation.contains("paint_plugin_list_chips(ui, rect, &model,"));
        assert!(implementation.contains("paint_match_chips(ui, rect, &result.matched_fields,"));
    }

    #[test]
    fn plugin_rows_keep_list_preview_and_shared_status_primitives() {
        let source = include_str!("plugin_rows.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("fn manifest_row"));
        assert!(implementation.contains("fn action_row"));
        assert!(implementation.contains("fn preview_panel"));
        assert!(implementation.contains("fn status_row"));
        assert!(implementation.contains("fn label_value_row"));
        assert!(!implementation.contains("fn output_view"));
        assert!(!implementation.contains("fn plugin_security_summary"));
    }
}
