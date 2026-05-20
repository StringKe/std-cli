use crate::{LauncherState, LauncherWindowCommand};
use std::time::Instant;

const CLOSE_BUDGET_MS: u128 = 200;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherCloseSmokeReport {
    pub visible_before: bool,
    pub visible_after: bool,
    pub focused_after: bool,
    pub action_panel_open_after: bool,
    pub voice_active_after: bool,
    pub commands: Vec<LauncherWindowCommand>,
    pub close_ms: u128,
    pub budget_ms: u128,
}

impl LauncherState {
    pub fn close_smoke() -> LauncherCloseSmokeReport {
        let mut state = Self::new();
        state.controller.show();
        state.open_action_panel();
        state.controller.voice_active = true;
        let visible_before = state.controller.visible;
        let started_at = Instant::now();
        let commands = state.handle_escape_hide();
        LauncherCloseSmokeReport {
            visible_before,
            visible_after: state.controller.visible,
            focused_after: state.controller.focused,
            action_panel_open_after: state.action_panel.open,
            voice_active_after: state.controller.voice_active,
            commands,
            close_ms: started_at.elapsed().as_millis(),
            budget_ms: CLOSE_BUDGET_MS,
        }
    }
}

impl LauncherCloseSmokeReport {
    pub fn pass(&self) -> bool {
        self.visible_before
            && !self.visible_after
            && !self.focused_after
            && !self.action_panel_open_after
            && !self.voice_active_after
            && self.commands == vec![LauncherWindowCommand::SetVisible(false)]
            && self.close_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_close_smoke {}\nvisible_before={}\nvisible_after={}\nfocused_after={}\naction_panel_open_after={}\nvoice_active_after={}\ncommands={}\nclose_ms={}\nbudget_close_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.visible_before,
            self.visible_after,
            self.focused_after,
            self.action_panel_open_after,
            self.voice_active_after,
            format_close_commands(&self.commands),
            self.close_ms,
            self.budget_ms
        )
    }
}

fn format_close_commands(commands: &[LauncherWindowCommand]) -> String {
    commands
        .iter()
        .map(|command| match command {
            LauncherWindowCommand::SetVisible(false) => "Visible(false)",
            LauncherWindowCommand::SetVisible(true) => "Visible(true)",
            LauncherWindowCommand::Focus => "Focus",
            LauncherWindowCommand::PositionForPanel => "PositionForPanel",
            LauncherWindowCommand::ResizeToPanel => "ResizeToPanel",
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_smoke_validates_panel_disappears_under_budget() {
        let report = LauncherState::close_smoke();

        assert!(report.pass(), "{}", report.summary());
        assert!(!report.visible_after);
        assert_eq!(
            report.commands,
            vec![LauncherWindowCommand::SetVisible(false)]
        );
        assert!(report.close_ms <= report.budget_ms);
    }
}
