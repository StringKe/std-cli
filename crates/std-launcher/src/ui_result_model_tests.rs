use crate::ui_result_model::{
    list_items, LauncherResultListItem, LauncherResultRowModel, TitleSegment,
};
use std_egui::{i18n, input};
use std_types::{Action, ActionId, ActionPreview, ActionType, SearchResult};

#[test]
fn list_items_insert_group_headers_without_selection_slots() {
    let command = test_result("Rebuild Index", ActionType::Command, 0.97);
    let app = test_result("Open App", ActionType::AppLaunch, 0.5);
    let items = list_items(&[command, app], None, "open", 1);

    assert_eq!(items.len(), 4);
    assert_eq!(
        items[0],
        LauncherResultListItem::Group {
            label: i18n::t("launcher.results.group.action_workflow").to_string()
        }
    );
    assert!(matches!(
        &items[1],
        LauncherResultListItem::Row(row) if row.result_index == 0
    ));
    assert!(matches!(
        &items[3],
        LauncherResultListItem::Row(row)
            if row.result_index == 1
                && row.direct_shortcut == input::launcher_result_keycap(1)
                && row.primary_shortcut.as_deref() == Some(input::enter().label().as_str())
    ));
}

#[test]
fn selected_result_row_uses_enter_and_primary_command_hint() {
    let result = test_result("Rebuild Index", ActionType::Command, 0.97);
    let preview = test_preview(result.action.id, "std index rebuild .");

    let row = LauncherResultRowModel::from_result(&result, Some(&preview), "index", 0, 3, true);

    assert_eq!(row.title, "Rebuild Index");
    assert_eq!(
        row.title_segments,
        vec![
            TitleSegment {
                text: "Rebuild ".to_string(),
                matched: false
            },
            TitleSegment {
                text: "Index".to_string(),
                matched: true
            }
        ]
    );
    assert_eq!(row.subtitle, "Refresh index");
    assert_eq!(row.group, i18n::t("launcher.results.group.action_workflow"));
    assert_eq!(row.icon_label, "CMD");
    assert_eq!(
        row.direct_shortcut.as_deref(),
        input::launcher_result_keycap(0).as_deref()
    );
    assert_eq!(
        row.primary_shortcut.as_deref(),
        Some(input::enter().label().as_str())
    );
    assert_eq!(row.action_label, "std index rebuild .");
    assert_eq!(
        row.action_hint,
        Some(format!(
            "{} std index rebuild .",
            i18n::t("launcher.action.run")
        ))
    );
    assert_eq!(row.position, "1 of 3");
}

#[test]
fn non_selected_result_rows_show_number_keycaps_until_nine() {
    let third = test_result("Open App", ActionType::AppLaunch, 0.5);
    let tenth = test_result("Open File", ActionType::Custom("file".to_string()), 0.4);

    let third_row = LauncherResultRowModel::from_result(&third, None, "app", 2, 10, false);
    let tenth_row = LauncherResultRowModel::from_result(&tenth, None, "file", 9, 10, false);

    assert_eq!(
        third_row.direct_shortcut.as_deref(),
        input::launcher_result_keycap(2).as_deref()
    );
    assert!(third_row.primary_shortcut.is_none());
    assert!(third_row.action_hint.is_none());
    assert!(tenth_row.direct_shortcut.is_none());
    assert!(tenth_row.primary_shortcut.is_none());
    assert_eq!(tenth_row.group, i18n::t("launcher.results.group.app_file"));
    assert_eq!(tenth_row.icon_label, "FIL");
}

#[test]
fn title_highlight_uses_query_only_when_name_matched() {
    let mut tag_result = test_result("WeChat", ActionType::AppLaunch, 0.8);
    tag_result.matched_fields = vec!["tags".to_string()];
    let mut fuzzy_result = test_result("Rebuild Index", ActionType::Command, 0.7);
    fuzzy_result.matched_fields = vec!["name:fuzzy".to_string()];

    assert_eq!(
        LauncherResultRowModel::from_result(&tag_result, None, "weixin", 0, 1, false),
        LauncherResultRowModel {
            title: "WeChat".to_string(),
            title_segments: vec![TitleSegment {
                text: "WeChat".to_string(),
                matched: false
            }],
            subtitle: "WeChat description".to_string(),
            match_badge: Some(i18n::t("launcher.results.match.alias").to_string()),
            kind: i18n::t("launcher.results.kind.app").to_string(),
            icon_label: "APP".to_string(),
            group: i18n::t("launcher.results.group.app_file").to_string(),
            position: "1 of 1".to_string(),
            direct_shortcut: input::launcher_result_keycap(0),
            primary_shortcut: None,
            action_hint: None,
            action_label: i18n::t("launcher.action.review_first").to_string(),
            result_index: 0,
        }
    );
    assert_eq!(
        LauncherResultRowModel::from_result(&fuzzy_result, None, "ri", 0, 1, false).title_segments,
        vec![
            TitleSegment {
                text: "R".to_string(),
                matched: true
            },
            TitleSegment {
                text: "ebu".to_string(),
                matched: false
            },
            TitleSegment {
                text: "i".to_string(),
                matched: true
            },
            TitleSegment {
                text: "ld Index".to_string(),
                matched: false
            }
        ]
    );
}

#[test]
fn app_alias_matches_show_alias_subtitle_without_selected_preview() {
    let mut result = test_result("Open App: WeChat", ActionType::AppLaunch, 0.8);
    result.action.description =
        "Aliases: WeChat, Weixin, 微信 / Path: /tmp/std-fixture/WeChat.app".to_string();
    result.matched_fields = vec!["tags".to_string()];

    let row = LauncherResultRowModel::from_result(&result, None, "微信", 0, 1, false);

    assert_eq!(
        row.subtitle,
        "Aliases: WeChat, Weixin, 微信 / Path: /tmp/std-fixture/WeChat.app"
    );
    assert_eq!(
        row.match_badge,
        Some(i18n::t("launcher.results.match.alias").to_string())
    );
}

#[test]
fn app_alias_result_row_keeps_multilingual_names_keyboard_visible() {
    let mut result = test_result("Open App: WeChat", ActionType::AppLaunch, 0.95);
    result.action.description =
        "Aliases: WeChat, Weixin, 微信 / Path: /tmp/std-fixture/WeChat.app".to_string();
    result.matched_fields = vec!["tags".to_string()];

    let row = LauncherResultRowModel::from_result(&result, None, "weixin", 0, 3, true);

    assert_eq!(row.kind, i18n::t("launcher.results.kind.app"));
    assert_eq!(row.icon_label, "APP");
    assert_eq!(
        row.match_badge,
        Some(i18n::t("launcher.results.match.alias").to_string())
    );
    assert!(row.subtitle.contains("WeChat"));
    assert!(row.subtitle.contains("Weixin"));
    assert!(row.subtitle.contains("微信"));
    assert_eq!(row.direct_shortcut, input::launcher_result_keycap(0));
    assert_eq!(row.primary_shortcut, Some(input::enter().label()));
    assert_eq!(row.action_label, i18n::t("launcher.action.review_first"));
    let expected_hint = format!(
        "{} Open App: WeChat",
        i18n::t("launcher.action.review_first")
    );
    assert_eq!(row.action_hint, Some(expected_hint));
}

#[test]
fn app_alias_result_row_uses_review_first_even_with_preview_command() {
    let mut result = test_result("Open App: WeChat", ActionType::AppLaunch, 0.95);
    result.action.description =
        "Aliases: WeChat, Weixin, 微信 / Path: /tmp/std-fixture/WeChat.app".to_string();
    result.matched_fields = vec!["tags".to_string()];
    let preview = ActionPreview {
        action_id: result.action.id,
        title: result.action.name.clone(),
        subtitle: result.action.description.clone(),
        action_type: result.action.action_type.clone(),
        primary_command: "open -a WeChat".to_string(),
        metadata: std::collections::HashMap::new(),
        examples: vec!["open -a WeChat".to_string()],
    };

    let row = LauncherResultRowModel::from_result(&result, Some(&preview), "weixin", 0, 1, true);

    assert_eq!(row.action_label, i18n::t("launcher.action.review_first"));
    assert_eq!(
        row.action_hint,
        Some(format!(
            "{} open -a WeChat",
            i18n::t("launcher.action.review_first")
        ))
    );
}

fn test_result(name: &str, action_type: ActionType, score: f32) -> SearchResult {
    SearchResult {
        action: Action::new(name, format!("{name} description"), "test", action_type),
        score,
        matched_fields: vec!["name".to_string()],
    }
}

fn test_preview(action_id: ActionId, command: &str) -> ActionPreview {
    ActionPreview {
        action_id,
        title: "Rebuild Index".to_string(),
        subtitle: "Refresh index".to_string(),
        action_type: ActionType::Command,
        primary_command: command.to_string(),
        metadata: std::collections::HashMap::new(),
        examples: vec![command.to_string()],
    }
}
