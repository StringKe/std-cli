use crate::{
    ui,
    views::{
        row_metrics,
        row_paint::{self, RowSurface},
    },
};
use eframe::egui;
use std::path::{Path, PathBuf};
use std_egui::{
    i18n,
    tokens::{Color, Radius, Space, Text},
};

pub(crate) enum WorkflowFileAction {
    None,
    Select(PathBuf),
    Open(PathBuf),
}

pub(crate) fn workflow_file_row(
    ui: &mut egui::Ui,
    path: &Path,
    selected: bool,
) -> WorkflowFileAction {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::FILE_ROW_HEIGHT),
        egui::Sense::click(),
    );
    let title = workflow_label(path);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), &title)
    });
    let open_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.right() - row_metrics::FILE_ACTION_RIGHT_INSET,
            rect.center().y - row_metrics::FILE_ACTION_Y_OFFSET,
        ),
        egui::vec2(
            row_metrics::FILE_ACTION_WIDTH,
            row_metrics::FILE_ACTION_HEIGHT,
        ),
    );
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), selected, RowSurface::Base);
        paint_file_text(ui, rect, &title, &path.display().to_string());
        paint_open_control(ui, open_rect, response.hovered());
    }
    ui.add_space(Space::TWO_XS as f32);
    if response.clicked() {
        let Some(pointer) = response.interact_pointer_pos() else {
            return WorkflowFileAction::None;
        };
        if open_rect.contains(pointer) {
            WorkflowFileAction::Open(path.to_path_buf())
        } else {
            WorkflowFileAction::Select(path.to_path_buf())
        }
    } else {
        WorkflowFileAction::None
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
        egui::vec2(ui.available_width(), row_metrics::STATUS_ROW_HEIGHT),
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
                row_metrics::STATUS_CHIP_WIDTH,
                row_metrics::STATUS_CHIP_HEIGHT,
            ),
        );
        row_paint::paint_chip(ui, chip_rect, status, fill);
        let text_x = chip_rect.right() + Space::XS as f32;
        ui.painter().text(
            egui::pos2(text_x, rect.top() + row_metrics::STATUS_TITLE_Y),
            egui::Align2::LEFT_CENTER,
            title,
            Text::body(),
            ui::strong_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.top() + row_metrics::STATUS_DETAIL_Y),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn path_row(ui: &mut egui::Ui, label: &str, path: &Path) {
    let detail = path.display().to_string();
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::PATH_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), false, RowSurface::Base);
        let text_x = rect.left() + row_metrics::TEXT_INSET_X;
        ui.painter().text(
            egui::pos2(text_x, rect.top() + row_metrics::PATH_TITLE_Y),
            egui::Align2::LEFT_CENTER,
            label,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
        ui.painter().text(
            egui::pos2(text_x, rect.top() + row_metrics::PATH_DETAIL_Y),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::strong_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn workflow_summary(ui: &mut egui::Ui, title: &str, status: &str, steps: usize) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(Text::body())
                .color(ui::strong_text(ui.ctx())),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{status} steps={steps}"))
                    .font(Text::caption())
                    .color(ui::muted_text(ui.ctx())),
            );
        });
    });
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn builder_step_summary(ui: &mut egui::Ui, name: &str, steps: usize) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(name)
                .font(Text::body())
                .color(ui::strong_text(ui.ctx())),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("steps={steps}"))
                    .font(Text::caption())
                    .color(ui::muted_text(ui.ctx())),
            );
        });
    });
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn builder_step_row(
    ui: &mut egui::Ui,
    index: usize,
    name: &str,
    detail: &str,
    selected: bool,
) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::BUILDER_STEP_ROW_HEIGHT),
        egui::Sense::click(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), name));
    if ui.is_rect_visible(rect) {
        row_paint::paint_row_frame(ui, rect, response.hovered(), selected, RowSurface::Raised);
        paint_builder_grabber(ui, rect);
        if selected {
            paint_builder_selected_rail(ui, rect);
        }
        let index_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + Space::XS as f32 + row_metrics::BUILDER_GRABBER_WIDTH,
                rect.center().y - row_metrics::STATUS_CHIP_Y_OFFSET,
            ),
            egui::vec2(
                row_metrics::BUILDER_INDEX_WIDTH,
                row_metrics::BUILDER_INDEX_HEIGHT,
            ),
        );
        paint_builder_step_index(ui, index_rect, index + 1, selected);
        let text_x = index_rect.right() + Space::XS as f32;
        let type_chip_width = builder_type_chip_width(detail);
        let type_chip_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.right() - Space::XS as f32 - type_chip_width,
                rect.center().y - row_metrics::STATUS_CHIP_Y_OFFSET,
            ),
            egui::vec2(type_chip_width, row_metrics::STATUS_CHIP_HEIGHT),
        );
        paint_builder_type_chip(ui, type_chip_rect, detail, selected);
        let text_clip = egui::Rect::from_min_max(
            egui::pos2(text_x, rect.top()),
            egui::pos2(type_chip_rect.left() - Space::XS as f32, rect.bottom()),
        );
        let painter = ui.painter().with_clip_rect(text_clip);
        painter.text(
            egui::pos2(text_x, rect.top() + row_metrics::STEP_TITLE_Y),
            egui::Align2::LEFT_CENTER,
            name,
            Text::body(),
            ui::strong_text(ui.ctx()),
        );
        painter.text(
            egui::pos2(text_x, rect.top() + row_metrics::STEP_DETAIL_Y),
            egui::Align2::LEFT_CENTER,
            detail,
            Text::caption(),
            ui::muted_text(ui.ctx()),
        );
    }
    ui.add_space(Space::TWO_XS as f32);
    response
}

pub(crate) fn builder_step_visual_contract() -> &'static str {
    "steps=list|selected-row|keyboard-reorder|grabber-6px|selected-accent-rail-4px|type-chip"
}

fn workflow_label(path: &Path) -> String {
    path.parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .or_else(|| path.file_stem().and_then(|name| name.to_str()))
        .unwrap_or("workflow")
        .to_string()
}

fn paint_builder_step_index(ui: &mut egui::Ui, rect: egui::Rect, number: usize, selected: bool) {
    let fill = if selected {
        Color::accent_base(ui.ctx())
    } else {
        Color::bg_surface_1(ui.ctx())
    };
    let text_color = if selected {
        Color::bg_surface_0(ui.ctx())
    } else {
        Color::fg_secondary(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        number.to_string(),
        Text::caption(),
        text_color,
    );
}

fn paint_builder_grabber(ui: &mut egui::Ui, rect: egui::Rect) {
    let x = rect.left() + Space::TWO_XS as f32;
    let top = rect.center().y - 8.0;
    let color = Color::fg_tertiary(ui.ctx());
    for offset in [0.0, 5.0, 10.0] {
        let center = egui::pos2(x + row_metrics::BUILDER_GRABBER_WIDTH / 2.0, top + offset);
        ui.painter().circle_filled(center, 1.0, color);
    }
}

fn paint_builder_selected_rail(ui: &mut egui::Ui, rect: egui::Rect) {
    let rail = egui::Rect::from_min_max(
        rect.left_top(),
        egui::pos2(
            rect.left() + row_metrics::BUILDER_SELECTED_RAIL_WIDTH,
            rect.bottom(),
        ),
    );
    ui.painter().rect_filled(
        rail,
        egui::CornerRadius::same(Radius::SM),
        Color::accent_base(ui.ctx()),
    );
}

fn paint_builder_type_chip(ui: &mut egui::Ui, rect: egui::Rect, detail: &str, selected: bool) {
    let fill = if selected {
        Color::accent_weak(ui.ctx())
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
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn builder_type_chip_width(detail: &str) -> f32 {
    let measured = detail.chars().count() as f32 * 7.0 + 18.0;
    measured.clamp(
        row_metrics::BUILDER_TYPE_CHIP_MIN_WIDTH,
        row_metrics::BUILDER_TYPE_CHIP_MAX_WIDTH,
    )
}

fn paint_file_text(ui: &mut egui::Ui, rect: egui::Rect, title: &str, detail: &str) {
    let text_x = rect.left() + row_metrics::TEXT_INSET_X;
    let right_limit = rect.right() - row_metrics::FILE_RIGHT_CLIP_INSET;
    let clip = egui::Rect::from_min_max(
        egui::pos2(text_x, rect.top()),
        egui::pos2(right_limit, rect.bottom()),
    );
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(text_x, rect.top() + row_metrics::FILE_TITLE_Y),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(text_x, rect.top() + row_metrics::FILE_DETAIL_Y),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_open_control(ui: &mut egui::Ui, rect: egui::Rect, hovered: bool) {
    let fill = if hovered {
        Color::bg_surface_2(ui.ctx())
    } else {
        Color::bg_surface_0(ui.ctx())
    };
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(Radius::SM),
        egui::Stroke::new(1.0, Color::stroke_border(ui.ctx())),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        i18n::t("studio.workflows.open"),
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_step_visual_contract_matches_docs22_step_list() {
        assert_eq!(
            builder_step_visual_contract(),
            "steps=list|selected-row|keyboard-reorder|grabber-6px|selected-accent-rail-4px|type-chip"
        );
    }

    #[test]
    fn builder_type_chip_width_is_bounded() {
        assert_eq!(
            builder_type_chip_width("Run"),
            row_metrics::BUILDER_TYPE_CHIP_MIN_WIDTH
        );
        assert_eq!(
            builder_type_chip_width("VeryLongWorkflowStepType"),
            row_metrics::BUILDER_TYPE_CHIP_MAX_WIDTH
        );
    }
}
