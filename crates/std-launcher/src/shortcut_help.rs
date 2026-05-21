use std_egui::input;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherShortcutHelpRow {
    pub key: String,
    pub action: &'static str,
}

pub fn launcher_shortcut_help_visible(query: &str) -> bool {
    query.trim() == "?"
}

pub fn launcher_shortcut_help_rows() -> Vec<LauncherShortcutHelpRow> {
    vec![
        LauncherShortcutHelpRow {
            key: format!(
                "{} / {}",
                input::arrow_up().label(),
                input::arrow_down().label()
            ),
            action: "Move selection",
        },
        LauncherShortcutHelpRow {
            key: format!(
                "{} / {}",
                input::mod_arrow_up().label(),
                input::mod_arrow_down().label()
            ),
            action: "Jump to first or last result",
        },
        LauncherShortcutHelpRow {
            key: format!("{} / {}", input::tab().label(), input::shift_tab().label()),
            action: "Move focus between search, results, and actions",
        },
        LauncherShortcutHelpRow {
            key: input::enter().label(),
            action: "Run selected primary action",
        },
        LauncherShortcutHelpRow {
            key: input::launcher_action_panel().label(),
            action: "Open Action Panel",
        },
        LauncherShortcutHelpRow {
            key: launcher_index_range_label(),
            action: "Run result by index",
        },
        LauncherShortcutHelpRow {
            key: input::launcher_delete_previous_token().label(),
            action: "Delete previous query token",
        },
        LauncherShortcutHelpRow {
            key: "? text".to_string(),
            action: "Ask natural language planner",
        },
        LauncherShortcutHelpRow {
            key: input::escape().label(),
            action: "Clear query or hide Launcher",
        },
    ]
}

pub fn launcher_shortcut_help_summary() -> String {
    let rows = launcher_shortcut_help_rows()
        .iter()
        .map(|row| format!("{}={}", row.key, row.action))
        .collect::<Vec<_>>()
        .join("|");
    format!(
        "launcher_shortcut_help rows={};trigger=?;action_panel={};enter={};token_delete={};rows={rows}",
        launcher_shortcut_help_rows().len(),
        input::launcher_action_panel().label(),
        input::enter().label(),
        input::launcher_delete_previous_token().label()
    )
}

fn launcher_index_range_label() -> String {
    format!(
        "{}..{}",
        input::launcher_result_keycap(0).unwrap_or_else(|| "1".to_string()),
        input::launcher_result_keycap(8).unwrap_or_else(|| "9".to_string())
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortcut_help_is_only_for_lone_question_mark() {
        assert!(launcher_shortcut_help_visible("?"));
        assert!(launcher_shortcut_help_visible(" ? "));
        assert!(!launcher_shortcut_help_visible("? rebuild"));
        assert!(!launcher_shortcut_help_visible(""));
    }

    #[test]
    fn shortcut_help_rows_cover_docs20_launcher_keys() {
        let summary = launcher_shortcut_help_summary();

        for required in [
            format!(
                "{} / {}",
                input::arrow_up().label(),
                input::arrow_down().label()
            ),
            format!(
                "{} / {}",
                input::mod_arrow_up().label(),
                input::mod_arrow_down().label()
            ),
            format!("{} / {}", input::tab().label(), input::shift_tab().label()),
            input::enter().label(),
            input::launcher_action_panel().label(),
            launcher_index_range_label(),
            input::launcher_delete_previous_token().label(),
            "? text".to_string(),
            input::escape().label(),
        ] {
            assert!(summary.contains(&required), "{required}");
        }
    }

    #[test]
    fn shortcut_help_rows_use_platform_labels_not_mod_placeholders() {
        let summary = launcher_shortcut_help_summary();

        assert!(!summary.contains("Mod+"));
        assert!(summary.contains(&input::launcher_action_panel().label()));
        assert!(summary.contains(&launcher_index_range_label()));
    }
}
