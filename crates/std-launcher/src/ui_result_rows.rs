use crate::{
    ui_metrics,
    ui_parts::keycap,
    ui_result_model::{group_header_label, LauncherResultRowModel},
};
use eframe::egui;
use std_egui::{
    tokens::{Color, Radius, Space, Text},
    LauncherViewModel,
};

pub(crate) fn group_header(ui: &mut egui::Ui, group: &str) {
    let ctx = ui.ctx().clone();
    let (slot, _response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ui_metrics::group_header_slot_height()),
        egui::Sense::hover(),
    );
    let divider = ui_metrics::group_divider_rect(slot.width(), slot.min);
    let label_pos = egui::pos2(
        slot.left(),
        slot.center().y + ui_metrics::group_header_label_offset_y(),
    );
    ui.painter().rect_filled(
        divider,
        egui::CornerRadius::ZERO,
        Color::stroke_border(&ctx),
    );
    ui.painter().text(
        label_pos,
        egui::Align2::LEFT_CENTER,
        group_header_label(group),
        Text::footnote(),
        Color::fg_tertiary(&ctx),
    );
}

pub(crate) fn result_row(ui: &mut egui::Ui, model: &LauncherResultRowModel) -> egui::Response {
    let ctx = ui.ctx().clone();
    let selected = model.action_hint.is_some();
    let response = ui.allocate_response(
        ui_metrics::result_row_size(ui.available_width()),
        egui::Sense::click(),
    );
    paint_result_row_background(ui, response.rect, selected, response.hovered(), &ctx);
    let rect = response.rect.shrink2(ui_metrics::result_row_shrink());
    paint_result_row(ui, rect, model, selected, &ctx);
    response
}

pub(crate) fn section_header(ui: &mut egui::Ui, title: &str, detail: &str) {
    let ctx = ui.ctx().clone();
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(title)
                .font(Text::body())
                .color(Color::fg_primary(&ctx))
                .strong(),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(egui::RichText::new(detail).font(Text::footnote()));
        });
    });
    ui.add_space(Space::two_xs() as f32);
}

pub(crate) fn result_accessibility_label(
    model: &LauncherResultRowModel,
    view: &LauncherViewModel,
) -> String {
    std_egui::a11y::AccessibilityContext::from_env().launcher_result_label(
        &model.title,
        &model.kind,
        model.position_number(),
        view.results.len(),
    )
}

fn paint_result_row(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    model: &LauncherResultRowModel,
    selected: bool,
    ctx: &egui::Context,
) {
    let layout = ui_metrics::result_row_layout(rect);
    paint_result_icon(ui, &layout, model, selected, ctx);
    paint_result_text(ui, &layout, model, selected, ctx);
    paint_result_right(ui, &layout, model);
}

fn paint_result_icon(
    ui: &mut egui::Ui,
    layout: &crate::ui_metrics_results::LauncherResultRowLayout,
    model: &LauncherResultRowModel,
    selected: bool,
    ctx: &egui::Context,
) {
    let fill = if selected {
        Color::accent_weak(ctx)
    } else {
        Color::bg_surface_2(ctx)
    };
    ui.painter().rect_filled(
        layout.icon_rect,
        egui::CornerRadius::same(Radius::sm()),
        fill,
    );
    ui.painter().text(
        layout.icon_rect.center() + ui_metrics::result_icon_text_offset_y(),
        egui::Align2::CENTER_CENTER,
        &model.icon_label,
        Text::caption(),
        Color::fg_secondary(ctx),
    );
}

fn paint_result_text(
    ui: &mut egui::Ui,
    layout: &crate::ui_metrics_results::LauncherResultRowLayout,
    model: &LauncherResultRowModel,
    selected: bool,
    ctx: &egui::Context,
) {
    let painter = ui.painter().with_clip_rect(layout.text_clip);
    ui.scope_builder(egui::UiBuilder::new().max_rect(layout.text_clip), |ui| {
        ui.scope_builder(egui::UiBuilder::new().max_rect(layout.title_rect), |ui| {
            let text = egui::RichText::new(&model.title)
                .font(Text::body())
                .color(Color::fg_primary(ctx));
            let text = if selected { text.strong() } else { text };
            ui.add(egui::Label::new(text).truncate());
        });
    });
    painter.text(
        layout.subtitle_pos,
        egui::Align2::LEFT_CENTER,
        &model.subtitle,
        Text::footnote(),
        if selected {
            Color::fg_secondary(ctx)
        } else {
            Color::fg_tertiary(ctx)
        },
    );
}

fn paint_result_right(
    ui: &mut egui::Ui,
    layout: &crate::ui_metrics_results::LauncherResultRowLayout,
    model: &LauncherResultRowModel,
) {
    ui.scope_builder(egui::UiBuilder::new().max_rect(layout.right_rect), |ui| {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if model.action_hint.is_some() {
                if let Some(shortcut) = model.primary_shortcut.as_deref() {
                    keycap(ui, shortcut);
                }
                action_hint_label(ui, &model.action_label);
            }
            if let Some(shortcut) = model.direct_shortcut.as_deref() {
                keycap(ui, shortcut);
            }
        });
    });
}

fn paint_result_row_background(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    selected: bool,
    hovered: bool,
    ctx: &egui::Context,
) {
    if let Some(fill) = result_row_background_color(selected, hovered, ctx) {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(Radius::md()), fill);
    }
}

fn result_row_background_color(
    selected: bool,
    hovered: bool,
    ctx: &egui::Context,
) -> Option<egui::Color32> {
    if selected {
        Some(Color::accent_weak(ctx))
    } else if hovered {
        Some(Color::bg_surface_2(ctx))
    } else {
        None
    }
}

fn action_hint_label(ui: &mut egui::Ui, label: &str) {
    let ctx = ui.ctx().clone();
    ui.label(
        egui::RichText::new(label)
            .font(Text::caption())
            .color(Color::fg_secondary(&ctx)),
    );
}

#[cfg(test)]
pub(crate) fn result_row_keyboard_affordance(
    model: &LauncherResultRowModel,
) -> (String, String, &str) {
    (
        model
            .direct_shortcut
            .clone()
            .unwrap_or_else(|| "none".to_string()),
        model
            .primary_shortcut
            .clone()
            .unwrap_or_else(|| "none".to_string()),
        model.action_label.as_str(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui_result_model::group_header_label;

    #[test]
    fn group_header_label_is_uppercase() {
        assert_eq!(group_header_label("Action / Workflow"), "ACTION / WORKFLOW");
    }

    #[test]
    fn result_row_background_uses_selected_hover_and_idle_layers() {
        let ctx = egui::Context::default();

        assert_eq!(
            result_row_background_color(true, true, &ctx),
            Some(Color::accent_weak(&ctx))
        );
        assert_eq!(
            result_row_background_color(false, true, &ctx),
            Some(Color::bg_surface_2(&ctx))
        );
        assert_eq!(result_row_background_color(false, false, &ctx), None);
    }

    #[test]
    fn selected_result_title_uses_strong_text_contract() {
        let source = include_str!("ui_result_rows.rs");

        assert!(source.contains("let text = if selected { text.strong() } else { text };"));
        assert!(source.contains(".font(Text::body())"));
    }
}
