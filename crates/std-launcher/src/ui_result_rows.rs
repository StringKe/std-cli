use crate::{
    ui_metrics, ui_result_icons,
    ui_result_model::{group_header_label, LauncherResultRowModel},
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    tokens::{Color, Radius, Text},
    LauncherViewModel,
};

pub(crate) fn group_header(ui: &mut egui::Ui, group: &str) {
    let ctx = ui.ctx().clone();
    let (slot, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), ui_metrics::group_header_slot_height()),
        egui::Sense::hover(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            AccessibilityContext::from_env().launcher_result_group_label(group),
        )
    });
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

pub(crate) fn result_accessibility_label(
    model: &LauncherResultRowModel,
    view: &LauncherViewModel,
) -> String {
    let mut label = std_egui::a11y::AccessibilityContext::from_env().launcher_result_label(
        &model.title,
        &model.kind,
        model.position_number(),
        view.results.len(),
    );
    if let Some(shortcut) = model.direct_shortcut.as_deref() {
        label.push_str(&format!(", shortcut {shortcut}"));
    }
    if let Some(primary) = model.primary_shortcut.as_deref() {
        label.push_str(&format!(", press {primary} to {}", model.action_label));
    }
    label
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
    use std_types::SearchResult;

    #[test]
    fn group_header_label_preserves_readable_title_case() {
        assert_eq!(group_header_label("Action / Workflow"), "Action / Workflow");
        assert_eq!(group_header_label("  App / File  "), "App / File");
    }

    #[test]
    fn group_header_exposes_screen_reader_group_label() {
        let source = include_str!("ui_result_rows.rs");

        assert!(source.contains("launcher_result_group_label(group)"));
        assert!(source.contains("WidgetType::Label"));
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
    fn result_accessibility_label_includes_shortcuts_and_selected_action() {
        let result = SearchResult {
            action: std_types::Action::new(
                "Rebuild Index",
                "Refresh local index",
                "test",
                std_types::ActionType::Command,
            ),
            score: 1.0,
            matched_fields: vec!["name".to_string()],
        };
        let core = std_core::StdCore::default();
        let mut view = LauncherViewModel::new(&core);
        view.results = vec![result.clone()];
        let selected = LauncherResultRowModel::from_result(&result, None, 0, 1, true);
        let idle = LauncherResultRowModel::from_result(&result, None, 0, 1, false);

        let selected_label = result_accessibility_label(&selected, &view);
        let idle_label = result_accessibility_label(&idle, &view);

        assert!(selected_label.contains("shortcut"));
        assert!(selected_label.contains(&format!("press Enter to {}", selected.action_label)));
        assert!(idle_label.contains("shortcut"));
        assert!(!idle_label.contains(&format!("press Enter to {}", idle.action_label)));
    }

    #[test]
    fn result_right_affordance_uses_fixed_regions_not_wrapped_buttons() {
        let source = include_str!("ui_result_rows.rs");
        let right_body = source
            .split("fn paint_result_right")
            .nth(1)
            .and_then(|body| body.split("fn paint_result_row_background").next())
            .unwrap();

        assert!(right_body.contains("result_right_affordance_layout"));
        assert!(right_body.contains("paint_keycap"));
        assert!(right_body.contains("paint_action_hint_label"));
        assert!(!right_body.contains("right_to_left"));
        assert!(!right_body.contains("crate::ui_parts::keycap"));
    }

    #[test]
    fn selected_result_title_uses_strong_text_contract() {
        let source = include_str!("ui_result_rows.rs");

        assert!(source.contains("let text = if selected { text.strong() } else { text };"));
        assert!(source.contains(".font(Text::body())"));
    }

    #[test]
    fn result_icon_delegates_to_single_color_geometric_glyph() {
        let source = include_str!("ui_result_rows.rs");
        let icon_body = source
            .split("fn paint_result_icon")
            .nth(1)
            .and_then(|body| body.split("fn paint_result_text").next())
            .unwrap();

        assert!(icon_body.contains("ui_result_icons::paint"));
        assert!(!icon_body.contains("painter().text"));
    }
}
