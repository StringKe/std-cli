use eframe::egui;
use std_egui::tokens::{Color, Radius};

pub(crate) fn paint(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    icon_label: &str,
    selected: bool,
    ctx: &egui::Context,
) {
    let color = if selected {
        Color::accent_base(ctx)
    } else {
        Color::fg_secondary(ctx)
    };
    let stroke = egui::Stroke::new(1.5, color);
    match icon_label {
        "APP" => paint_window(ui, rect, stroke),
        "FIL" => paint_file(ui, rect, stroke),
        "WF" => paint_workflow(ui, rect, stroke),
        "MEM" => paint_document(ui, rect, stroke),
        "CLP" => paint_clipboard(ui, rect, stroke),
        "SK" => paint_skill(ui, rect, stroke),
        _ => paint_command(ui, rect, stroke),
    }
}

fn paint_window(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let inner = rect.shrink(rect.width() * 0.24);
    ui.painter().rect_stroke(
        inner,
        egui::CornerRadius::same(Radius::sm()),
        stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().line_segment(
        [
            egui::pos2(inner.left(), inner.top() + inner.height() * 0.32),
            egui::pos2(inner.right(), inner.top() + inner.height() * 0.32),
        ],
        stroke,
    );
}

fn paint_workflow(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let radius = rect.width() * 0.13;
    let left = egui::pos2(rect.left() + rect.width() * 0.32, rect.center().y);
    let right = egui::pos2(rect.right() - rect.width() * 0.32, rect.center().y);
    ui.painter().circle_stroke(left, radius, stroke);
    ui.painter().circle_stroke(right, radius, stroke);
    ui.painter().line_segment(
        [
            egui::pos2(left.x + radius, left.y),
            egui::pos2(right.x - radius, right.y),
        ],
        stroke,
    );
}

fn paint_document(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let inner = rect.shrink(rect.width() * 0.25);
    ui.painter().rect_stroke(
        inner,
        egui::CornerRadius::same(Radius::sm()),
        stroke,
        egui::StrokeKind::Inside,
    );
    for y in [0.38, 0.56, 0.74] {
        ui.painter().line_segment(
            [
                egui::pos2(
                    inner.left() + inner.width() * 0.2,
                    inner.top() + inner.height() * y,
                ),
                egui::pos2(
                    inner.right() - inner.width() * 0.2,
                    inner.top() + inner.height() * y,
                ),
            ],
            stroke,
        );
    }
}

fn paint_file(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let inner = rect.shrink(rect.width() * 0.24);
    let fold = inner.width() * 0.26;
    let points = vec![
        inner.left_top(),
        egui::pos2(inner.right() - fold, inner.top()),
        egui::pos2(inner.right(), inner.top() + fold),
        inner.right_bottom(),
        inner.left_bottom(),
        inner.left_top(),
    ];
    ui.painter().add(egui::Shape::line(points, stroke));
    ui.painter().line_segment(
        [
            egui::pos2(inner.right() - fold, inner.top()),
            egui::pos2(inner.right() - fold, inner.top() + fold),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(inner.right() - fold, inner.top() + fold),
            egui::pos2(inner.right(), inner.top() + fold),
        ],
        stroke,
    );
}

fn paint_clipboard(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let inner = rect.shrink(rect.width() * 0.25);
    ui.painter().rect_stroke(
        inner,
        egui::CornerRadius::same(Radius::sm()),
        stroke,
        egui::StrokeKind::Inside,
    );
    let clip = egui::Rect::from_center_size(
        egui::pos2(inner.center().x, inner.top()),
        egui::vec2(inner.width() * 0.48, inner.height() * 0.22),
    );
    ui.painter().rect_stroke(
        clip,
        egui::CornerRadius::same(Radius::sm()),
        stroke,
        egui::StrokeKind::Inside,
    );
    for y in [0.48, 0.66] {
        ui.painter().line_segment(
            [
                egui::pos2(
                    inner.left() + inner.width() * 0.22,
                    inner.top() + inner.height() * y,
                ),
                egui::pos2(
                    inner.right() - inner.width() * 0.22,
                    inner.top() + inner.height() * y,
                ),
            ],
            stroke,
        );
    }
}

fn paint_skill(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let center = rect.center();
    let radius = rect.width() * 0.25;
    ui.painter().circle_stroke(center, radius, stroke);
    ui.painter().line_segment(
        [
            egui::pos2(center.x - radius, center.y),
            egui::pos2(center.x + radius, center.y),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(center.x, center.y - radius),
            egui::pos2(center.x, center.y + radius),
        ],
        stroke,
    );
}

fn paint_command(ui: &mut egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let inner = rect.shrink(rect.width() * 0.28);
    ui.painter().line_segment(
        [
            egui::pos2(inner.left(), inner.top()),
            egui::pos2(inner.center().x, inner.center().y),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(inner.center().x, inner.center().y),
            egui::pos2(inner.left(), inner.bottom()),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(inner.center().x + inner.width() * 0.15, inner.bottom()),
            egui::pos2(inner.right(), inner.bottom()),
        ],
        stroke,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn result_icons_use_painter_geometry_not_text_badges() {
        let source = include_str!("ui_result_icons.rs");

        assert!(source.contains("paint_window"));
        assert!(source.contains("paint_file"));
        assert!(source.contains("paint_workflow"));
        assert!(source.contains("paint_clipboard"));
        assert!(source.contains("paint_command"));
        assert!(source.contains("Color::fg_secondary(ctx)"));
        assert!(source.contains("Color::accent_base(ctx)"));
        let implementation = source.split("#[cfg(test)]").next().unwrap();
        assert!(!implementation.contains("painter().text"));
    }
}
