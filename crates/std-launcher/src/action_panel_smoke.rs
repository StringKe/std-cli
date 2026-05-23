use crate::{LauncherKey, LauncherState};
use std::time::Instant;
use std_core::{StdConfig, StdCore};
use std_egui::i18n;
use std_types::ActionExecutionStatus;

const ACTION_PANEL_BUDGET_MS: u128 = 50;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherActionPanelSmokeReport {
    pub query: String,
    pub opened: bool,
    pub focus_section: String,
    pub action_count: usize,
    pub selected_title: String,
    pub external_primary_title: String,
    pub external_order: String,
    pub default_enter_status: ActionExecutionStatus,
    pub explicit_run_status: ActionExecutionStatus,
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
            external_primary_title: external_order_evidence().primary_title,
            external_order: external_order_evidence().order,
            default_enter_status: external_run_evidence().default_enter_status,
            explicit_run_status: external_run_evidence().explicit_run_status,
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
            && self.selected_title == i18n::t("launcher.action.run")
            && self.external_primary_title == i18n::t("launcher.action.review_first")
            && self.external_order == external_order_contract()
            && self.default_enter_status == ActionExecutionStatus::NeedsExternalRunner
            && self.explicit_run_status == ActionExecutionStatus::NeedsExternalRunner
            && self.open_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_action_panel_smoke {}\nquery={}\nopened={}\nfocus_section={}\naction_count={}\nselected_title={}\nexternal_primary_title={}\nexternal_order={}\ndefault_enter_status={:?}\nexplicit_run_status={:?}\nopen_ms={}\nbudget_action_panel_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.query,
            self.opened,
            self.focus_section,
            self.action_count,
            self.selected_title,
            self.external_primary_title,
            self.external_order,
            self.default_enter_status,
            self.explicit_run_status,
            self.open_ms,
            self.budget_ms
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExternalOrderEvidence {
    primary_title: String,
    order: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExternalRunEvidence {
    default_enter_status: ActionExecutionStatus,
    explicit_run_status: ActionExecutionStatus,
}

fn external_order_contract() -> String {
    [
        i18n::t("launcher.action.review_first"),
        i18n::t("launcher.action.run"),
        i18n::t("launcher.action.defer"),
        i18n::t("launcher.action.open_in_studio"),
        i18n::t("launcher.action.copy_command"),
    ]
    .join(">")
}

fn external_order_evidence() -> ExternalOrderEvidence {
    let mut state = external_fixture_state();
    state.update_query("ActionPanelSmokeApp");
    state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let titles = state
        .action_panel
        .items
        .iter()
        .map(|item| item.title().to_string())
        .collect::<Vec<_>>();

    ExternalOrderEvidence {
        primary_title: titles
            .first()
            .cloned()
            .unwrap_or_else(|| "none".to_string()),
        order: titles.join(">"),
    }
}

fn external_run_evidence() -> ExternalRunEvidence {
    let mut default_state = external_fixture_state();
    default_state.update_query("ActionPanelSmokeApp");
    default_state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    let default_enter_status = default_state
        .handle_keyboard_input(LauncherKey::Enter, false)
        .map(|execution| execution.status)
        .unwrap_or(ActionExecutionStatus::Failed);

    let mut run_state = external_fixture_state();
    run_state.update_query("ActionPanelSmokeApp");
    run_state.handle_keyboard_input(LauncherKey::ActionPanel, false);
    run_state.handle_keyboard_input(LauncherKey::ArrowDown, false);
    let explicit_run_status = run_state
        .handle_keyboard_input_by_user(LauncherKey::Enter, false)
        .map(|execution| execution.status)
        .unwrap_or(ActionExecutionStatus::Failed);

    ExternalRunEvidence {
        default_enter_status,
        explicit_run_status,
    }
}

fn external_fixture_state() -> LauncherState {
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
    LauncherState::with_core(core)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_panel_smoke_validates_mod_k_budget_contract() {
        let report = LauncherState::action_panel_smoke("index");

        assert!(report.pass(), "{}", report.summary());
        assert_eq!(report.focus_section, "ActionPanel");
        assert_eq!(report.selected_title, i18n::t("launcher.action.run"));
        assert_eq!(
            report.external_primary_title,
            i18n::t("launcher.action.review_first")
        );
        assert_eq!(report.external_order, external_order_contract());
        assert_eq!(
            report.default_enter_status,
            ActionExecutionStatus::NeedsExternalRunner
        );
        assert_eq!(
            report.explicit_run_status,
            ActionExecutionStatus::NeedsExternalRunner
        );
        assert!(report.open_ms <= report.budget_ms);
    }
}
