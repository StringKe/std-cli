use crate::{LauncherState, LauncherWindowCommand};
use std::time::Instant;

const CLOSE_BUDGET_MS: u128 = 200;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherCloseSmokeReport {
    pub visible_before: bool,
    pub visible_after: bool,
    pub visible_after_reopen: bool,
    pub focused_after: bool,
    pub focused_after_reopen: bool,
    pub action_panel_open_after: bool,
    pub voice_active_after: bool,
    pub commands: Vec<LauncherWindowCommand>,
    pub reopen_commands: Vec<LauncherWindowCommand>,
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
        let visible_after = state.controller.visible;
        let focused_after = state.controller.focused;
        let action_panel_open_after = state.action_panel.open;
        let voice_active_after = state.controller.voice_active;
        let reopen_commands = state.handle_hotkey_toggle();
        LauncherCloseSmokeReport {
            visible_before,
            visible_after,
            visible_after_reopen: state.controller.visible,
            focused_after,
            focused_after_reopen: state.controller.focused,
            action_panel_open_after,
            voice_active_after,
            commands,
            reopen_commands,
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
            && self.visible_after_reopen
            && self.focused_after_reopen
            && self.reopen_commands
                == vec![
                    LauncherWindowCommand::ResizeToPanel,
                    LauncherWindowCommand::PositionForPanel,
                    LauncherWindowCommand::SetVisible(true),
                    LauncherWindowCommand::Focus,
                ]
            && self.close_ms <= self.budget_ms
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_close_smoke {}\nvisible_before={}\nvisible_after={}\nvisible_after_reopen={}\nfocused_after={}\nfocused_after_reopen={}\naction_panel_open_after={}\nvoice_active_after={}\ncommands={}\nreopen_commands={}\nclose_ms={}\nbudget_close_ms={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.visible_before,
            self.visible_after,
            self.visible_after_reopen,
            self.focused_after,
            self.focused_after_reopen,
            self.action_panel_open_after,
            self.voice_active_after,
            format_close_commands(&self.commands),
            format_close_commands(&self.reopen_commands),
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
        assert!(report.visible_after_reopen);
        assert!(report.focused_after_reopen);
        assert_eq!(
            report.reopen_commands,
            vec![
                LauncherWindowCommand::ResizeToPanel,
                LauncherWindowCommand::PositionForPanel,
                LauncherWindowCommand::SetVisible(true),
                LauncherWindowCommand::Focus,
            ]
        );
        assert!(report.close_ms <= report.budget_ms);
    }
}
