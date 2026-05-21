use std_egui::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LauncherShortcutHelpRow {
    pub key: &'static str,
    pub action: &'static str,
}

pub fn launcher_shortcut_help_visible(query: &str) -> bool {
    query.trim() == "?"
}

pub fn launcher_shortcut_help_rows() -> Vec<LauncherShortcutHelpRow> {
    vec![
        LauncherShortcutHelpRow {
            key: "Up / Down",
            action: "Move selection",
        },
        LauncherShortcutHelpRow {
            key: "Mod+Up / Mod+Down",
            action: "Jump to first or last result",
        },
        LauncherShortcutHelpRow {
            key: "Tab / Shift+Tab",
            action: "Move focus between search, results, and actions",
        },
        LauncherShortcutHelpRow {
            key: "Enter",
            action: "Run selected primary action",
        },
        LauncherShortcutHelpRow {
            key: "Mod+K",
            action: "Open Action Panel",
        },
        LauncherShortcutHelpRow {
            key: "Mod+1..9",
            action: "Run result by index",
        },
        LauncherShortcutHelpRow {
            key: "Mod+Backspace",
            action: "Delete previous query token",
        },
        LauncherShortcutHelpRow {
            key: "? text",
            action: "Ask natural language planner",
        },
        LauncherShortcutHelpRow {
            key: "Esc",
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
            "Up / Down",
            "Mod+Up / Mod+Down",
            "Tab / Shift+Tab",
            "Enter",
            "Mod+K",
            "Mod+1..9",
            "Mod+Backspace",
            "? text",
            "Esc",
        ] {
            assert!(summary.contains(required), "{required}");
        }
    }
}
