use crate::{ui, views::row_metrics};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};
use std_types::{MemoryRecord, PlanStep};

struct TextPlacement {
    x: f32,
    title_y: f32,
    detail_y: f32,
}

pub(crate) fn metric_tile(ui: &mut egui::Ui, title: &str, value: usize, detail: &str) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::DASHBOARD_METRIC_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title));
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_metric_value(ui, rect, value);
        paint_title_detail_at(
            ui,
            rect,
            title,
            detail,
            TextPlacement {
                x: rect.left() + row_metrics::DASHBOARD_VALUE_TEXT_X,
                title_y: row_metrics::DASHBOARD_VALUE_TITLE_Y,
                detail_y: row_metrics::DASHBOARD_VALUE_DETAIL_Y,
            },
        );
    }
}

fn paint_metric_value(ui: &mut egui::Ui, rect: egui::Rect, value: usize) {
    let chip_rect = egui::Rect::from_min_size(
        egui::pos2(
            rect.left() + row_metrics::TEXT_INSET_X,
            rect.center().y - row_metrics::STATUS_CHIP_Y_OFFSET,
        ),
        egui::vec2(
            row_metrics::STATUS_CHIP_WIDTH,
            row_metrics::STATUS_CHIP_HEIGHT,
        ),
    );
    paint_chip(ui, chip_rect, &value.to_string(), ui::panel_alt(ui.ctx()));
}

pub(crate) fn plan_goal_row(ui: &mut egui::Ui, label: &str, goal: &str) {
    let title = format!("{label}: {goal}");
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::APP_STORAGE_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), title.as_str())
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &title,
            "planner draft",
            row_metrics::COMPACT_TITLE_Y,
            row_metrics::COMPACT_DETAIL_Y,
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn plan_step_row(ui: &mut egui::Ui, index: usize, step: &PlanStep) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), row_metrics::DASHBOARD_PLAN_ROW_HEIGHT),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &step.action_name)
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        let chip_rect = egui::Rect::from_min_size(
            egui::pos2(
                rect.left() + row_metrics::TEXT_INSET_X,
                rect.center().y - row_metrics::DASHBOARD_STEP_CHIP_Y_OFFSET,
            ),
            egui::vec2(
                row_metrics::DASHBOARD_STEP_CHIP_WIDTH,
                row_metrics::DASHBOARD_STEP_CHIP_HEIGHT,
            ),
        );
        paint_chip(
            ui,
            chip_rect,
            &(index + 1).to_string(),
            ui::selected_bg(ui.ctx()),
        );
        paint_title_detail_at(
            ui,
            rect,
            &step.action_name,
            &step.reason,
            TextPlacement {
                x: chip_rect.right() + Space::XS as f32,
                title_y: row_metrics::DASHBOARD_VALUE_TITLE_Y,
                detail_y: row_metrics::DASHBOARD_STEP_DETAIL_Y,
            },
        );
    }
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn memory_row(ui: &mut egui::Ui, memory: &MemoryRecord) {
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(
            ui.available_width(),
            row_metrics::DASHBOARD_MEMORY_ROW_HEIGHT,
        ),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &memory.title)
    });
    if ui.is_rect_visible(rect) {
        paint_row_frame(ui, rect, response.hovered());
        paint_title_detail(
            ui,
            rect,
            &memory.title,
            &memory_meta(memory),
            row_metrics::DENSE_TITLE_Y,
            row_metrics::DENSE_DETAIL_Y,
        );
        paint_memory_preview(ui, rect, &memory.body);
    }
    ui.add_space(Space::TWO_XS as f32);
}

fn paint_memory_preview(ui: &mut egui::Ui, rect: egui::Rect, body: &str) {
    let preview = preview_text(body, row_metrics::MEMORY_PREVIEW_LIMIT);
    let clip = rect.shrink2(egui::vec2(row_metrics::WIDE_CLIP_INSET_X, 0.0));
    ui.painter().with_clip_rect(clip).text(
        egui::pos2(
            rect.left() + row_metrics::TEXT_INSET_X,
            rect.bottom() - row_metrics::MEMORY_PREVIEW_Y,
        ),
        egui::Align2::LEFT_CENTER,
        preview,
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
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
    paint_title_detail_at(
        ui,
        rect,
        title,
        detail,
        TextPlacement {
            x: rect.left() + row_metrics::TEXT_INSET_X,
            title_y: y1,
            detail_y: y2,
        },
    );
}

fn paint_title_detail_at(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    title: &str,
    detail: &str,
    placement: TextPlacement,
) {
    let clip = egui::Rect::from_min_max(egui::pos2(placement.x, rect.top()), rect.right_bottom());
    let painter = ui.painter().with_clip_rect(clip);
    painter.text(
        egui::pos2(placement.x, rect.top() + placement.title_y),
        egui::Align2::LEFT_CENTER,
        title,
        Text::body(),
        ui::strong_text(ui.ctx()),
    );
    painter.text(
        egui::pos2(placement.x, rect.top() + placement.detail_y),
        egui::Align2::LEFT_CENTER,
        detail,
        Text::caption(),
        ui::muted_text(ui.ctx()),
    );
}

fn paint_chip(ui: &mut egui::Ui, rect: egui::Rect, label: &str, fill: egui::Color32) {
    ui.painter()
        .rect_filled(rect, egui::CornerRadius::same(Radius::SM), fill);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        Text::caption(),
        ui::strong_text(ui.ctx()),
    );
}

fn memory_meta(memory: &MemoryRecord) -> String {
    format!("scope={} tags={}", memory.scope, memory.tags.join(","))
}

fn preview_text(text: &str, limit: usize) -> String {
    let preview = text
        .lines()
        .next()
        .unwrap_or("")
        .chars()
        .take(limit)
        .collect::<String>();
    if text.chars().count() > limit {
        format!("{preview}...")
    } else {
        preview
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn metric_tile_uses_status_chip_not_hero_metric_number() {
        let source = include_str!("dashboard_rows.rs");
        let metric_body = source
            .split("pub(crate) fn metric_tile")
            .nth(1)
            .and_then(|body| body.split("pub(crate) fn plan_goal_row").next())
            .unwrap();

        assert!(metric_body.contains("paint_metric_value(ui, rect, value)"));
        assert!(!metric_body.contains("Text::title()"));
        assert!(source.contains("row_metrics::STATUS_CHIP_WIDTH"));
        assert!(source.contains("ui::panel_alt(ui.ctx())"));
    }

    #[test]
    fn preview_text_truncates_by_chars() {
        assert_eq!(super::preview_text("abcdef", 4), "abcd...");
        assert_eq!(super::preview_text("abc", 4), "abc");
    }
}
