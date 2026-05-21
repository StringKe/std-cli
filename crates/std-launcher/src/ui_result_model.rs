use std_egui::i18n;
use std_types::{ActionPreview, ActionType, SearchResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LauncherResultListItem {
    Group { label: String },
    Row(LauncherResultRowModel),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct LauncherResultRowModel {
    pub(crate) title: String,
    pub(crate) subtitle: String,
    pub(crate) kind: String,
    pub(crate) icon_label: String,
    pub(crate) group: String,
    pub(crate) position: String,
    pub(crate) shortcut: Option<String>,
    pub(crate) action_hint: Option<String>,
    pub(crate) action_label: String,
    pub(crate) result_index: usize,
}

impl LauncherResultRowModel {
    pub(crate) fn from_result(
        result: &SearchResult,
        preview: Option<&ActionPreview>,
        index: usize,
        total: usize,
        selected: bool,
    ) -> Self {
        let shortcut = if selected {
            Some("Enter".to_string())
        } else if index < 9 {
            Some(format!("Mod+{}", index + 1))
        } else {
            None
        };
        Self {
            title: result.action.name.clone(),
            subtitle: result_subtitle(result, preview),
            kind: action_kind(&result.action.action_type).to_string(),
            icon_label: action_icon_label(&result.action.action_type).to_string(),
            group: action_group(result),
            position: format!("{} of {total}", index + 1),
            shortcut,
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

    pub(crate) fn position_total(&self) -> usize {
        self.position
            .split_once(" of ")
            .and_then(|(_, total)| total.parse().ok())
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
        items.push(LauncherResultListItem::Row(model));
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

pub(crate) fn action_group(result: &SearchResult) -> String {
    match &result.action.action_type {
        ActionType::AppLaunch => i18n::t("launcher.results.group.app_file").to_string(),
        ActionType::Workflow => i18n::t("launcher.results.group.action_workflow").to_string(),
        ActionType::Command => i18n::t("launcher.results.group.action_workflow").to_string(),
        ActionType::Skill => i18n::t("launcher.results.group.memory_skill").to_string(),
        ActionType::Clipboard => i18n::t("launcher.results.group.clipboard").to_string(),
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
        ActionType::Skill => "SK",
        ActionType::Clipboard => "CLP",
        ActionType::Custom(kind) if kind == "file" => "FIL",
        ActionType::Custom(_) => "ACT",
    }
}

pub(crate) fn group_header_label(group: &str) -> String {
    group.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std_types::{Action, ActionId};

    #[test]
    fn list_items_insert_group_headers_without_selection_slots() {
        let command = test_result("Rebuild Index", ActionType::Command, 0.97);
        let app = test_result("Open App", ActionType::AppLaunch, 0.5);
        let items = list_items(&[command, app], None, 1);

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
                if row.result_index == 1 && row.shortcut.as_deref() == Some("Enter")
        ));
    }

    #[test]
    fn selected_result_row_uses_enter_and_primary_command_hint() {
        let result = test_result("Rebuild Index", ActionType::Command, 0.97);
        let preview = test_preview(result.action.id, "std index rebuild .");

        let row = LauncherResultRowModel::from_result(&result, Some(&preview), 0, 3, true);

        assert_eq!(row.title, "Rebuild Index");
        assert_eq!(row.subtitle, "Refresh index");
        assert_eq!(row.group, i18n::t("launcher.results.group.action_workflow"));
        assert_eq!(row.icon_label, "CMD");
        assert_eq!(row.shortcut.as_deref(), Some("Enter"));
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

        let third_row = LauncherResultRowModel::from_result(&third, None, 2, 10, false);
        let tenth_row = LauncherResultRowModel::from_result(&tenth, None, 9, 10, false);

        assert_eq!(third_row.shortcut.as_deref(), Some("Mod+3"));
        assert!(third_row.action_hint.is_none());
        assert!(tenth_row.shortcut.is_none());
        assert_eq!(tenth_row.group, i18n::t("launcher.results.group.app_file"));
        assert_eq!(tenth_row.icon_label, "FIL");
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
