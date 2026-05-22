use crate::{
    ui_metrics, ui_result_icons,
    ui_result_model::{LauncherResultRowModel, TitleSegment},
};
use eframe::egui;
use std_egui::tokens::{Color, Radius, Space, Text};

pub(crate) fn paint(
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

pub(crate) fn paint_background(
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
    ui_result_icons::paint(ui, layout.icon_rect, &model.icon_label, selected, ctx);
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
            render_title_segments(ui, &model.title_segments, selected, ctx);
        });
    });
    let subtitle_rect = painter.text(
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
    if let Some(badge) = model.match_badge.as_deref() {
        let badge_pos = egui::pos2(
            layout.subtitle_pos.x + subtitle_rect.width() + Space::XS as f32,
            layout.subtitle_pos.y,
        );
        painter.text(
            badge_pos,
            egui::Align2::LEFT_CENTER,
            badge,
            Text::caption(),
            Color::accent_base(ctx),
        );
    }
}

fn render_title_segments(
    ui: &mut egui::Ui,
    segments: &[TitleSegment],
    selected: bool,
    ctx: &egui::Context,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        for segment in segments {
            let color = title_segment_color(segment.matched, selected, ctx);
            let text = egui::RichText::new(&segment.text)
                .font(Text::body())
                .color(color);
            let text = if selected || segment.matched {
                text.strong()
            } else {
                text
            };
            ui.label(text);
        }
    });
}

fn title_segment_color(matched: bool, selected: bool, ctx: &egui::Context) -> egui::Color32 {
    if matched || selected {
        Color::fg_primary(ctx)
    } else {
        Color::fg_tertiary(ctx)
    }
}

fn paint_result_right(
    ui: &mut egui::Ui,
    layout: &crate::ui_metrics_results::LauncherResultRowLayout,
    model: &LauncherResultRowModel,
) {
    let affordance =
        ui_metrics::result_right_affordance_layout(layout.right_rect, model.action_hint.is_some());
    if let Some(shortcut) = model.direct_shortcut.as_deref() {
        paint_keycap(ui, affordance.direct_keycap, shortcut);
    }
    if model.action_hint.is_some() {
        if let Some(rect) = affordance.action_label {
            paint_action_hint_label(ui, rect, &model.action_label);
        }
        if let (Some(rect), Some(shortcut)) =
            (affordance.primary_keycap, model.primary_shortcut.as_deref())
        {
            paint_keycap(ui, rect, shortcut);
        }
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

fn paint_action_hint_label(ui: &mut egui::Ui, rect: egui::Rect, label: &str) {
    let ctx = ui.ctx().clone();
    let painter = ui.painter().with_clip_rect(rect);
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        Text::caption(),
        Color::fg_secondary(&ctx),
    );
}

fn paint_keycap(ui: &mut egui::Ui, rect: egui::Rect, label: &str) {
    let ctx = ui.ctx().clone();
    let key_rect = rect.shrink2(egui::vec2(2.0, 6.0));
    ui.painter().rect_filled(
        key_rect,
        egui::CornerRadius::same(Radius::sm()),
        Color::bg_surface_0(&ctx),
    );
    ui.painter().rect_stroke(
        key_rect,
        egui::CornerRadius::same(Radius::sm()),
        egui::Stroke::new(1.0, Color::stroke_border(&ctx)),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        key_rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        Text::caption(),
        Color::fg_secondary(&ctx),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn selected_result_title_uses_primary_text_even_without_name_match() {
        let ctx = egui::Context::default();

        assert_eq!(
            title_segment_color(false, true, &ctx),
            Color::fg_primary(&ctx)
        );
        assert_eq!(
            title_segment_color(true, false, &ctx),
            Color::fg_primary(&ctx)
        );
        assert_eq!(
            title_segment_color(false, false, &ctx),
            Color::fg_tertiary(&ctx)
        );
    }

    #[test]
    fn right_affordance_uses_fixed_regions_not_wrapped_buttons() {
        let source = include_str!("ui_result_row_paint.rs");
        let right_body = source
            .split("fn paint_result_right")
            .nth(1)
            .and_then(|body| body.split("fn result_row_background_color").next())
            .unwrap();

        assert!(right_body.contains("result_right_affordance_layout"));
        assert!(right_body.contains("paint_keycap"));
        assert!(right_body.contains("paint_action_hint_label"));
        assert!(!right_body.contains("right_to_left"));
        assert!(!right_body.contains("crate::ui_parts::keycap"));
    }
}
