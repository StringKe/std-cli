use crate::{LauncherKey, LauncherState};
use std::time::Instant;
use std_core::{StdConfig, StdCore};

const ACTION_PANEL_BUDGET_MS: u128 = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherActionPanelSmokeReport {
    pub query: String,
    pub opened: bool,
    pub focus_section: String,
    pub action_count: usize,
    pub selected_title: String,
    pub external_primary_title: String,
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
            external_primary_title: external_primary_title(),
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
            && self.external_primary_title == "Review first"
            && self.open_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_action_panel_smoke {}\nquery={}\nopened={}\nfocus_section={}\naction_count={}\nselected_title={}\nexternal_primary_title={}\nopen_ms={}\nbudget_action_panel_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.query,
            self.opened,
            self.focus_section,
            self.action_count,
            self.selected_title,
            self.external_primary_title,
            self.open_ms,
            self.budget_ms
        )
    }
}

fn external_primary_title() -> String {
    let root = std::env::temp_dir().join(format!(
        "std-launcher-action-panel-smoke-{}",
        std::process::id()
    ));
    let config = StdConfig {
        data_dir: root.join("data"),
        ..StdConfig::default()
    };
    let app = config.apps_dir().join("ActionPanelSmokeApp.app");
    let _ = std::fs::create_dir_all(app.join("Contents").join("MacOS"));
    let _ = std::fs::write(app.join("Contents").join("MacOS").join("fixture"), "bin");
    let core = StdCore::with_config(config);
    let _ = core.register_local_content_actions();
    let mut state = LauncherState::with_core(core);
    state.update_query("ActionPanelSmokeApp");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let title = state
        .action_panel
        .selected_item()
        .map(|item| item.title().to_string())
        .unwrap_or_else(|| "none".to_string());
    let _ = std::fs::remove_dir_all(root);
    title
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
        assert_eq!(report.external_primary_title, "Review first");
        assert!(report.open_ms <= report.budget_ms);
    }
}
