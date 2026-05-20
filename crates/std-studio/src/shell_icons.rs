use eframe::egui;
use std_egui::tokens::Color;
use std_studio::StudioPane;

pub(crate) fn paint_pane_icon(ui: &egui::Ui, rect: egui::Rect, pane: StudioPane, selected: bool) {
    let color = if selected {
        Color::accent_base(ui.ctx())
    } else {
        Color::fg_secondary(ui.ctx())
    };
    let stroke = egui::Stroke::new(1.5, color);
    match pane {
        StudioPane::Dashboard => paint_square(ui, rect, stroke),
        StudioPane::Workflows => paint_flow(ui, rect, stroke),
        StudioPane::Apps => paint_grid(ui, rect, stroke),
        StudioPane::Memory => paint_book(ui, rect, stroke),
        StudioPane::Plugins => paint_plug(ui, rect, stroke),
        StudioPane::Analysis => paint_chart(ui, rect, stroke),
        StudioPane::History => paint_clock(ui, rect, stroke),
        StudioPane::Operations => paint_terminal(ui, rect, stroke),
        StudioPane::Settings => paint_sliders(ui, rect, stroke),
    }
}

fn paint_square(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    ui.painter().rect_stroke(
        rect.shrink(2.0),
        egui::CornerRadius::same(2),
        stroke,
        egui::StrokeKind::Inside,
    );
}

fn paint_flow(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let y = rect.center().y;
    let left = egui::pos2(rect.left() + 2.0, y);
    let mid = rect.center();
    let right = egui::pos2(rect.right() - 2.0, y);
    ui.painter().line_segment([left, right], stroke);
    ui.painter().circle_stroke(left, 2.0, stroke);
    ui.painter().circle_stroke(mid, 2.0, stroke);
    ui.painter().circle_stroke(right, 2.0, stroke);
}

fn paint_grid(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    for x in [rect.left() + 3.0, rect.left() + 9.0] {
        for y in [rect.top() + 3.0, rect.top() + 9.0] {
            let cell = egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(4.0, 4.0));
            ui.painter().rect_stroke(
                cell,
                egui::CornerRadius::same(1),
                stroke,
                egui::StrokeKind::Inside,
            );
        }
    }
}

fn paint_book(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    ui.painter().rect_stroke(
        rect.shrink(2.0),
        egui::CornerRadius::same(2),
        stroke,
        egui::StrokeKind::Inside,
    );
    ui.painter().line_segment(
        [
            egui::pos2(rect.center().x, rect.top() + 2.0),
            egui::pos2(rect.center().x, rect.bottom() - 2.0),
        ],
        stroke,
    );
}

fn paint_plug(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    ui.painter().line_segment(
        [
            egui::pos2(rect.left() + 5.0, rect.top() + 3.0),
            egui::pos2(rect.left() + 5.0, rect.top() + 7.0),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(rect.right() - 5.0, rect.top() + 3.0),
            egui::pos2(rect.right() - 5.0, rect.top() + 7.0),
        ],
        stroke,
    );
    ui.painter().rect_stroke(
        egui::Rect::from_min_max(
            egui::pos2(rect.left() + 4.0, rect.top() + 7.0),
            egui::pos2(rect.right() - 4.0, rect.bottom() - 4.0),
        ),
        egui::CornerRadius::same(2),
        stroke,
        egui::StrokeKind::Inside,
    );
}

fn paint_chart(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    let base = rect.bottom() - 2.0;
    for (index, height) in [4.0, 8.0, 12.0].into_iter().enumerate() {
        let x = rect.left() + 3.0 + index as f32 * 5.0;
        ui.painter()
            .line_segment([egui::pos2(x, base), egui::pos2(x, base - height)], stroke);
    }
}

fn paint_clock(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    ui.painter().circle_stroke(rect.center(), 6.0, stroke);
    ui.painter().line_segment(
        [rect.center(), egui::pos2(rect.center().x, rect.top() + 4.0)],
        stroke,
    );
    ui.painter().line_segment(
        [
            rect.center(),
            egui::pos2(rect.right() - 4.0, rect.center().y),
        ],
        stroke,
    );
}

fn paint_terminal(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    ui.painter().line_segment(
        [
            egui::pos2(rect.left() + 3.0, rect.top() + 5.0),
            egui::pos2(rect.left() + 7.0, rect.center().y),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(rect.left() + 7.0, rect.center().y),
            egui::pos2(rect.left() + 3.0, rect.bottom() - 5.0),
        ],
        stroke,
    );
    ui.painter().line_segment(
        [
            egui::pos2(rect.left() + 9.0, rect.bottom() - 4.0),
            egui::pos2(rect.right() - 3.0, rect.bottom() - 4.0),
        ],
        stroke,
    );
}

fn paint_sliders(ui: &egui::Ui, rect: egui::Rect, stroke: egui::Stroke) {
    for (index, y) in [rect.top() + 4.0, rect.center().y, rect.bottom() - 4.0]
        .into_iter()
        .enumerate()
    {
        ui.painter().line_segment(
            [
                egui::pos2(rect.left() + 2.0, y),
                egui::pos2(rect.right() - 2.0, y),
            ],
            stroke,
        );
        let knob_x = rect.left() + 5.0 + index as f32 * 3.0;
        ui.painter()
            .circle_filled(egui::pos2(knob_x, y), 1.8, stroke.color);
    }
}
