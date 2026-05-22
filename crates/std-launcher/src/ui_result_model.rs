use std_egui::{i18n, input};
use std_types::{ActionPreview, ActionType, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LauncherResultListItem {
    Group { label: String },
    Row(Box<LauncherResultRowModel>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherResultRowModel {
    pub(crate) title: String,
    pub(crate) title_segments: Vec<TitleSegment>,
    pub(crate) subtitle: String,
    pub(crate) match_badge: Option<String>,
    pub(crate) kind: String,
    pub(crate) icon_label: String,
    pub(crate) group: String,
    pub(crate) position: String,
    pub(crate) direct_shortcut: Option<String>,
    pub(crate) primary_shortcut: Option<String>,
    pub(crate) action_hint: Option<String>,
    pub(crate) action_label: String,
    pub(crate) result_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TitleSegment {
    pub(crate) text: String,
    pub(crate) matched: bool,
}

impl LauncherResultRowModel {
    pub(crate) fn from_result(
        result: &SearchResult,
        preview: Option<&ActionPreview>,
        query: &str,
        index: usize,
        total: usize,
        selected: bool,
    ) -> Self {
        let direct_shortcut = input::launcher_result_keycap(index);
        Self {
            title: result.action.name.clone(),
            title_segments: title_segments(result, query),
            subtitle: result_subtitle(result, preview),
            match_badge: match_badge(result),
            kind: action_kind(&result.action.action_type).to_string(),
            icon_label: action_icon_label(&result.action.action_type).to_string(),
            group: action_group(result),
            position: format!("{} of {total}", index + 1),
            direct_shortcut,
            primary_shortcut: selected.then(|| input::enter().label()),
            action_hint: selected.then(|| selected_action_hint(preview, &result.action.name)),
            action_label: selected_action_label(preview),
            result_index: index,
        }
    }

    pub(crate) fn position_number(&self) -> usize {
        self.position
            .split_once(" of ")
            .and_then(|(number, _)| number.parse().ok())
            .unwrap_or(0)
    }
}

pub(crate) fn group_count(results: &[SearchResult]) -> usize {
    results
        .iter()
        .map(action_group)
        .collect::<std::collections::BTreeSet<_>>()
        .len()
}

pub(crate) fn list_items(
    results: &[SearchResult],
    preview: Option<&ActionPreview>,
    query: &str,
    selected_index: usize,
) -> Vec<LauncherResultListItem> {
    let mut items = Vec::new();
    let mut last_group = String::new();
    for (index, result) in results.iter().enumerate() {
        let selected = index == selected_index;
        let row_preview = selected.then_some(preview).flatten();
        let model = LauncherResultRowModel::from_result(
            result,
            row_preview,
            query,
            index,
            results.len(),
            selected,
        );
        if model.group != last_group {
            last_group.clone_from(&model.group);
            items.push(LauncherResultListItem::Group {
                label: model.group.clone(),
            });
        }
        items.push(LauncherResultListItem::Row(Box::new(model)));
    }
    items
}

fn selected_action_hint(preview: Option<&ActionPreview>, fallback: &str) -> String {
    let command = preview
        .map(|preview| preview.primary_command.as_str())
        .filter(|command| !command.trim().is_empty())
        .unwrap_or(fallback);
    format!("{} {command}", i18n::t("launcher.action.run"))
}

fn selected_action_label(preview: Option<&ActionPreview>) -> String {
    preview
        .map(|preview| preview.primary_command.as_str())
        .filter(|command| !command.trim().is_empty())
        .unwrap_or(i18n::t("launcher.action.run"))
        .to_string()
}

fn result_subtitle(result: &SearchResult, preview: Option<&ActionPreview>) -> String {
    preview
        .map(|preview| preview.subtitle.as_str())
        .filter(|subtitle| !subtitle.trim().is_empty())
        .unwrap_or(result.action.description.as_str())
        .to_string()
}

fn match_badge(result: &SearchResult) -> Option<String> {
    if result
        .matched_fields
        .iter()
        .any(|field| field == "tags" || field == "tags:fuzzy")
    {
        Some(i18n::t("launcher.results.match.alias").to_string())
    } else if result
        .matched_fields
        .iter()
        .any(|field| field == "description" || field == "when_to_use")
    {
        Some(i18n::t("launcher.results.match.detail").to_string())
    } else {
        None
    }
}

fn title_segments(result: &SearchResult, query: &str) -> Vec<TitleSegment> {
    let title = result.action.name.as_str();
    let query = query.trim();
    if query.is_empty() || !matches_title(result) {
        return vec![TitleSegment {
            text: title.to_string(),
            matched: false,
        }];
    }
    contiguous_title_segments(title, query).unwrap_or_else(|| fuzzy_title_segments(title, query))
}

fn matches_title(result: &SearchResult) -> bool {
    result
        .matched_fields
        .iter()
        .any(|field| field == "name" || field == "name:fuzzy")
}

fn contiguous_title_segments(title: &str, query: &str) -> Option<Vec<TitleSegment>> {
    let title_lower = title.to_lowercase();
    let query_lower = query.to_lowercase();
    let start = title_lower.find(&query_lower)?;
    let end = start + query_lower.len();
    Some(split_title_by_ranges(title, &[(start, end)]))
}

fn fuzzy_title_segments(title: &str, query: &str) -> Vec<TitleSegment> {
    let ranges = fuzzy_match_ranges(title, query);
    if ranges.is_empty() {
        return vec![TitleSegment {
            text: title.to_string(),
            matched: false,
        }];
    }
    split_title_by_ranges(title, &ranges)
}

fn fuzzy_match_ranges(title: &str, query: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut query_chars = query
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .flat_map(char::to_lowercase);
    let mut target = query_chars.next();
    for (index, ch) in title.char_indices() {
        let Some(expected) = target else {
            break;
        };
        if ch.to_lowercase().any(|value| value == expected) {
            ranges.push((index, index + ch.len_utf8()));
            target = query_chars.next();
        }
    }
    if target.is_some() {
        Vec::new()
    } else {
        ranges
    }
}

fn split_title_by_ranges(title: &str, ranges: &[(usize, usize)]) -> Vec<TitleSegment> {
    let mut segments = Vec::new();
    let mut cursor = 0;
    for &(start, end) in ranges {
        if start > cursor {
            segments.push(TitleSegment {
                text: title[cursor..start].to_string(),
                matched: false,
            });
        }
        segments.push(TitleSegment {
            text: title[start..end].to_string(),
            matched: true,
        });
        cursor = end;
    }
    if cursor < title.len() {
        segments.push(TitleSegment {
            text: title[cursor..].to_string(),
            matched: false,
        });
    }
    segments
}

pub(crate) fn action_group(result: &SearchResult) -> String {
    match &result.action.action_type {
        ActionType::AppLaunch => i18n::t("launcher.results.group.app_file").to_string(),
        ActionType::Workflow => i18n::t("launcher.results.group.action_workflow").to_string(),
        ActionType::Command => i18n::t("launcher.results.group.action_workflow").to_string(),
        ActionType::Clipboard => i18n::t("launcher.results.group.clipboard").to_string(),
        ActionType::Memory => i18n::t("launcher.results.group.memory").to_string(),
        ActionType::Skill => i18n::t("launcher.results.group.skill").to_string(),
        ActionType::Custom(kind) if kind == "file" => {
            i18n::t("launcher.results.group.app_file").to_string()
        }
        ActionType::Custom(_) => i18n::t("launcher.results.group.other").to_string(),
    }
}

pub(crate) fn action_kind(action_type: &ActionType) -> &str {
    match action_type {
        ActionType::AppLaunch => i18n::t("launcher.results.kind.app"),
        ActionType::Workflow => i18n::t("launcher.results.kind.workflow"),
        ActionType::Command => i18n::t("launcher.results.kind.command"),
        ActionType::Memory => i18n::t("launcher.results.kind.memory"),
        ActionType::Skill => i18n::t("launcher.results.kind.skill"),
        ActionType::Clipboard => i18n::t("launcher.results.kind.clipboard"),
        ActionType::Custom(kind) if kind == "file" => i18n::t("launcher.results.kind.file"),
        ActionType::Custom(_) => i18n::t("launcher.results.kind.custom"),
    }
}

pub(crate) fn action_icon_label(action_type: &ActionType) -> &str {
    match action_type {
        ActionType::AppLaunch => "APP",
        ActionType::Workflow => "WF",
        ActionType::Command => "CMD",
        ActionType::Memory => "MEM",
        ActionType::Skill => "SK",
        ActionType::Clipboard => "CLP",
        ActionType::Custom(kind) if kind == "file" => "FIL",
        ActionType::Custom(_) => "ACT",
    }
}

pub(crate) fn group_header_label(group: &str) -> String {
    group.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_types::{Action, ActionId};

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
                action_label: i18n::t("launcher.action.run").to_string(),
                result_index: 0,
            }
        );
        assert_eq!(
            LauncherResultRowModel::from_result(&fuzzy_result, None, "ri", 0, 1, false)
                .title_segments,
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
}
