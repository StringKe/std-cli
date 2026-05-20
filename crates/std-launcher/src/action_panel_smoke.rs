use crate::{LauncherKey, LauncherState};
use std::time::Instant;

const ACTION_PANEL_BUDGET_MS: u128 = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherActionPanelSmokeReport {
    pub query: String,
    pub opened: bool,
    pub focus_section: String,
    pub action_count: usize,
    pub selected_title: String,
    pub open_ms: u128,
    pub budget_ms: u128,
}

impl LauncherState {
    pub fn action_panel_smoke(query: &str) -> LauncherActionPanelSmokeReport {
        let mut state = Self::new();
        state.controller.show();
        state.update_query(query);
        let started_at = Instant::now();
        state.handle_keyboard_input(LauncherKey::ActionPanel, false);
        LauncherActionPanelSmokeReport {
            query: state.view.query.clone(),
            opened: state.action_panel.open,
            focus_section: format!("{:?}", state.focus_section),
            action_count: state.action_panel.items.len(),
            selected_title: state
                .action_panel
                .selected_item()
                .map(|item| item.title().to_string())
                .unwrap_or_else(|| "none".to_string()),
            open_ms: started_at.elapsed().as_millis(),
            budget_ms: ACTION_PANEL_BUDGET_MS,
        }
    }
}

impl LauncherActionPanelSmokeReport {
    pub fn pass(&self) -> bool {
        self.opened
            && self.focus_section == "ActionPanel"
            && self.action_count >= 2
            && self.selected_title == "Run"
            && self.open_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_action_panel_smoke {}\nquery={}\nopened={}\nfocus_section={}\naction_count={}\nselected_title={}\nopen_ms={}\nbudget_action_panel_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.query,
            self.opened,
            self.focus_section,
            self.action_count,
            self.selected_title,
            self.open_ms,
            self.budget_ms
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_panel_smoke_validates_mod_k_budget_contract() {
        let report = LauncherState::action_panel_smoke("index");

        assert!(report.pass(), "{}", report.summary());
        assert_eq!(report.focus_section, "ActionPanel");
        assert_eq!(report.selected_title, "Run");
        assert!(report.open_ms <= report.budget_ms);
    }
}
