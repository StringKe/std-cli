use crate::views::workflow_builder_metrics;
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

const FIELD_LABEL_WIDTH: f32 = 104.0;

pub(crate) fn text_field_row(ui: &mut egui::Ui, label: &str, value: &mut String) {
    field_frame(ui).show(ui, |ui| {
        ui.horizontal(|ui| {
            field_label(ui, label);
            let response = ui.add_sized(
                [
                    ui.available_width(),
                    workflow_builder_metrics::PROPERTY_SINGLE_LINE_HEIGHT,
                ],
                egui::TextEdit::singleline(value).font(Text::body()),
            );
            response.widget_info(|| {
                egui::WidgetInfo::labeled(
                    egui::WidgetType::TextEdit,
                    ui.is_enabled(),
                    field_a11y_label(label, value),
                )
            });
        });
    });
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn parameters_field_row(ui: &mut egui::Ui, label: &str, value: &mut String) {
    field_frame(ui).show(ui, |ui| {
        field_label(ui, label);
        let response = ui.add_sized(
            workflow_builder_metrics::parameter_editor_size(ui.available_width()),
            egui::TextEdit::multiline(value).font(Text::code()),
        );
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::TextEdit,
                ui.is_enabled(),
                field_a11y_label(label, value),
            )
        });
    });
    ui.add_space(Space::TWO_XS as f32);
}

pub(crate) fn index_field(ui: &mut egui::Ui, label: &str, value: &mut String) {
    field_label(ui, label);
    let response = ui.add_sized(
        workflow_builder_metrics::step_index_size(),
        egui::TextEdit::singleline(value).font(Text::code()),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::TextEdit,
            ui.is_enabled(),
            field_a11y_label(label, value),
        )
    });
}

pub(crate) fn property_button(ui: &mut egui::Ui, label: &str, emphasized: bool) -> egui::Response {
    let ctx = ui.ctx().clone();
    let fill = if emphasized {
        Color::accent_weak(&ctx)
    } else {
        Color::bg_surface_1(&ctx)
    };
    let stroke = if emphasized {
        Color::accent_base(&ctx)
    } else {
        Color::stroke_divider(&ctx)
    };
    ui.add(
        egui::Button::new(
            egui::RichText::new(label)
                .font(Text::caption())
                .color(Color::fg_primary(&ctx)),
        )
        .fill(fill)
        .stroke(egui::Stroke::new(1.0, stroke))
        .corner_radius(egui::CornerRadius::same(Radius::sm())),
    )
}

pub(crate) fn fields_contract() -> &'static str {
    "properties=token-field-rows;inputs=step-name|parameters-json|index;actions=add|update|move-up|move-down|remove;primary=add|update"
}

fn field_frame(ui: &egui::Ui) -> egui::Frame {
    let ctx = ui.ctx();
    egui::Frame::new()
        .fill(Color::bg_surface_1(ctx))
        .stroke(egui::Stroke::new(1.0, Color::stroke_divider(ctx)))
        .corner_radius(egui::CornerRadius::same(Radius::sm()))
        .inner_margin(egui::Margin::symmetric(Space::XS, Space::TWO_XS))
}

fn field_label(ui: &mut egui::Ui, label: &str) {
    let ctx = ui.ctx().clone();
    ui.add_sized(
        [
            FIELD_LABEL_WIDTH,
            workflow_builder_metrics::PROPERTY_LABEL_HEIGHT,
        ],
        egui::Label::new(
            egui::RichText::new(label)
                .font(Text::caption())
                .color(Color::fg_secondary(&ctx)),
        ),
    );
}

fn field_a11y_label(label: &str, value: &str) -> String {
    let value = if value.trim().is_empty() {
        "empty"
    } else {
        value.trim()
    };
    format!("{label}, text box, value {value}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workflow_builder_fields_use_token_rows_not_bare_inputs() {
        let source = include_str!("workflow_builder_fields.rs");
        let implementation = source.split("#[cfg(test)]").next().unwrap();

        assert!(implementation.contains("field_frame"));
        assert!(implementation.contains("Color::bg_surface_1"));
        assert!(implementation.contains("Color::accent_weak"));
        assert!(implementation.contains("TextEdit::singleline"));
        assert!(implementation.contains("TextEdit::multiline"));
        assert!(implementation.contains("WidgetType::TextEdit"));
        assert!(implementation.contains("field_a11y_label"));
        assert!(!implementation.contains("ui.text_edit_singleline"));
    }

    #[test]
    fn workflow_builder_field_a11y_label_exposes_value() {
        assert_eq!(
            field_a11y_label("Step name", "Collect context"),
            "Step name, text box, value Collect context"
        );
        assert_eq!(
            field_a11y_label("Parameters", "  "),
            "Parameters, text box, value empty"
        );
    }
}
