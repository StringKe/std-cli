use crate::LauncherWindowCommand;
use std_types::ActionExecutionStatus;

pub const SEARCH_BUDGET_MS: u128 = 16;
pub const PREVIEW_BUDGET_MS: u128 = 16;
pub const TRIGGER_BUDGET_MS: u128 = 80;
pub const HOTKEY_BUDGET_MS: u128 = 80;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherSmokeReport {
    pub query: String,
    pub preview_title: String,
    pub execution_status: ActionExecutionStatus,
    pub feedback_title: String,
    pub performance: LauncherPerformanceReport,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherWindowSmokeReport {
    pub hidden_commands: Vec<LauncherWindowCommand>,
    pub shown_commands: Vec<LauncherWindowCommand>,
    pub final_visible: bool,
    pub focused: bool,
    pub elapsed_ms: u128,
    pub budget_ms: u128,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherPerformanceReport {
    pub hotkey_budget_ms: u128,
    pub search_budget_ms: u128,
    pub preview_budget_ms: u128,
    pub trigger_budget_ms: u128,
    pub last_search_ms: u128,
    pub last_preview_ms: u128,
    pub last_trigger_ms: u128,
    pub result_count: usize,
}

impl LauncherPerformanceReport {
    pub fn pass(&self) -> bool {
        self.last_search_ms <= self.search_budget_ms
            && self.last_preview_ms <= self.preview_budget_ms
            && self.last_trigger_ms <= self.trigger_budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_perf {}\nsearch_ms={}\npreview_ms={}\ntrigger_ms={}\nresults={}\nbudget_search_ms={}\nbudget_preview_ms={}\nbudget_trigger_ms={}\nbudget_hotkey_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.last_search_ms,
            self.last_preview_ms,
            self.last_trigger_ms,
            self.result_count,
            self.search_budget_ms,
            self.preview_budget_ms,
            self.trigger_budget_ms,
            self.hotkey_budget_ms
        )
    }
}

impl LauncherSmokeReport {
    pub fn summary(&self) -> String {
        format!(
            "launcher_smoke {}\nquery={}\npreview={}\nstatus={:?}\nfeedback={}\n{}",
            if self.performance.pass() {
                "PASS"
            } else {
                "FAIL"
            },
            self.query,
            self.preview_title,
            self.execution_status,
            self.feedback_title,
            self.performance.summary()
        )
    }
}

impl LauncherWindowSmokeReport {
    pub fn pass(&self) -> bool {
        self.hidden_commands == vec![LauncherWindowCommand::SetVisible(false)]
            && self.shown_commands
                == vec![
                    LauncherWindowCommand::ResizeToPanel,
                    LauncherWindowCommand::PositionForPanel,
                    LauncherWindowCommand::SetVisible(true),
                    LauncherWindowCommand::Focus,
                ]
            && self.final_visible
            && self.focused
            && self.elapsed_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_window_smoke {}\nhidden_commands={}\nshown_commands={}\nfinal_visible={}\nfocused={}\nelapsed_ms={}\nbudget_window_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            format_window_commands(&self.hidden_commands),
            format_window_commands(&self.shown_commands),
            self.final_visible,
            self.focused,
            self.elapsed_ms,
            self.budget_ms
        )
    }
}

pub fn format_window_commands(commands: &[LauncherWindowCommand]) -> String {
    commands
        .iter()
        .map(|command| match command {
            LauncherWindowCommand::SetVisible(true) => "Visible(true)",
            LauncherWindowCommand::SetVisible(false) => "Visible(false)",
            LauncherWindowCommand::Focus => "Focus",
            LauncherWindowCommand::PositionForPanel => "PositionForPanel",
            LauncherWindowCommand::ResizeToPanel => "ResizeToPanel",
        })
        .collect::<Vec<_>>()
        .join(",")
}
