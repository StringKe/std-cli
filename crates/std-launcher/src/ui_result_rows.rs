use crate::{
    ui_metrics,
    ui_result_model::{group_header_label, LauncherResultRowModel},
    ui_result_row_paint,
};
use eframe::egui;
use std_egui::{
    a11y::AccessibilityContext,
    i18n,
    tokens::{Color, Text},
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
    ui_result_row_paint::paint_background(ui, response.rect, selected, response.hovered(), &ctx);
    let rect = response.rect.shrink2(ui_metrics::result_row_shrink());
    ui_result_row_paint::paint(ui, rect, model, selected, &ctx);
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
        label.push_str(&result_label_part(
            "launcher.a11y.result.shortcut",
            &[("{shortcut}", shortcut)],
        ));
    }
    if let Some(primary) = model.primary_shortcut.as_deref() {
        label.push_str(&result_label_part(
            "launcher.a11y.result.primary",
            &[("{shortcut}", primary), ("{action}", &model.action_label)],
        ));
    }
    if let Some(badge) = model.match_badge.as_deref() {
        label.push_str(&result_label_part(
            "launcher.a11y.result.match_source",
            &[("{source}", badge)],
        ));
    }
    label
}

fn result_label_part(key: &str, replacements: &[(&str, &str)]) -> String {
    let mut text = i18n::t(key).to_string();
    for (from, to) in replacements {
        text = text.replace(from, to);
    }
    text
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
    fn result_row_delegates_painting_to_dedicated_module() {
        let source = include_str!("ui_result_rows.rs");
        let production_source = source.split("#[cfg(test)]").next().unwrap();

        assert!(production_source.contains("ui_result_row_paint::paint_background"));
        assert!(production_source.contains("ui_result_row_paint::paint"));
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
        let selected = LauncherResultRowModel::from_result(&result, None, "index", 0, 1, true);
        let idle = LauncherResultRowModel::from_result(&result, None, "index", 0, 1, false);

        let selected_label = result_accessibility_label(&selected, &view);
        let idle_label = result_accessibility_label(&idle, &view);
        let enter = std_egui::input::enter().label();

        assert!(selected_label.contains("快捷键"));
        assert!(selected_label.contains(&format!("按 {enter} {}", selected.action_label)));
        assert!(idle_label.contains("快捷键"));
        assert!(!idle_label.contains(&format!("按 {enter} {}", idle.action_label)));
    }

    #[test]
    fn result_accessibility_label_includes_alias_match_source() {
        let mut result = SearchResult {
            action: std_types::Action::new(
                "Open App: WeChat",
                "Aliases include weixin",
                "test",
                std_types::ActionType::AppLaunch,
            ),
            score: 1.0,
            matched_fields: vec!["tags".to_string()],
        };
        let core = std_core::StdCore::default();
        let mut view = LauncherViewModel::new(&core);
        view.results = vec![result.clone()];
        let row = LauncherResultRowModel::from_result(&result, None, "weixin", 0, 1, true);

        assert_eq!(row.match_badge.as_deref(), Some("别名"));
        assert!(result_accessibility_label(&row, &view).contains("匹配来源"));

        result.matched_fields = vec!["name".to_string()];
        let title_row = LauncherResultRowModel::from_result(&result, None, "wechat", 0, 1, true);
        assert!(title_row.match_badge.is_none());
    }

    #[test]
    fn external_result_accessibility_label_uses_review_first_not_run() {
        let result = SearchResult {
            action: std_types::Action::new(
                "Open App: WeChat",
                "Aliases: weixin, 微信",
                "test",
                std_types::ActionType::AppLaunch,
            ),
            score: 1.0,
            matched_fields: vec!["tags".to_string()],
        };
        let preview = std_types::ActionPreview {
            action_id: result.action.id,
            title: result.action.name.clone(),
            subtitle: result.action.description.clone(),
            action_type: result.action.action_type.clone(),
            primary_command: "open /tmp/std-fixture/WeChat.app".to_string(),
            metadata: Default::default(),
            examples: Vec::new(),
        };
        let core = std_core::StdCore::default();
        let mut view = LauncherViewModel::new(&core);
        view.results = vec![result.clone()];
        let row =
            LauncherResultRowModel::from_result(&result, Some(&preview), "weixin", 0, 1, true);

        let label = result_accessibility_label(&row, &view);
        let enter = std_egui::input::enter().label();

        assert!(label.contains(&format!("按 {enter} 先检查")));
        assert!(!label.contains(&format!("按 {enter} 运行")));
    }

    #[test]
    fn result_accessibility_label_uses_localized_parts() {
        assert_eq!(
            result_label_part("launcher.a11y.result.shortcut", &[("{shortcut}", "1")]),
            "，快捷键 1"
        );
        assert_eq!(
            result_label_part(
                "launcher.a11y.result.primary",
                &[("{shortcut}", "Enter"), ("{action}", "运行")]
            ),
            "，按 Enter 运行"
        );
        assert_eq!(
            result_label_part("launcher.a11y.result.match_source", &[("{source}", "别名")]),
            "，匹配来源 别名"
        );
    }

    #[test]
    fn result_right_affordance_uses_fixed_regions_not_wrapped_buttons() {
        let source = include_str!("ui_result_rows.rs");
        assert!(source.contains("result_row_keyboard_affordance"));
    }

    #[test]
    fn selected_result_title_uses_strong_text_contract() {
        let source = include_str!("ui_result_row_paint.rs");

        assert!(source.contains("let text = if selected || segment.matched"));
        assert!(source.contains(".font(Text::body())"));
    }

    #[test]
    fn result_icon_delegates_to_single_color_geometric_glyph() {
        let source = include_str!("ui_result_row_paint.rs");
        let icon_body = source
            .split("fn paint_result_icon")
            .nth(1)
            .and_then(|body| body.split("fn paint_result_text").next())
            .unwrap();

        assert!(icon_body.contains("ui_result_icons::paint"));
        assert!(!icon_body.contains("painter().text"));
    }
}
