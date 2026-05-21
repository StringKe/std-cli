use crate::{ActionPanelItem, LauncherFocusSection, LauncherFocusSource, LauncherState};
use std_types::ActionExecution;

impl LauncherState {
    pub fn open_action_panel(&mut self) -> bool {
        let Some(result) = self.view.selected_result() else {
            self.action_panel.close();
            return false;
        };
        self.action_panel.open_for(&result.action);
        self.focus_section = LauncherFocusSection::ActionPanel;
        self.focus_source = LauncherFocusSource::Keyboard;
        true
    }

    pub fn close_action_panel(&mut self) {
        self.action_panel.close();
        self.focus_section = LauncherFocusSection::Results;
        self.focus_source = LauncherFocusSource::Keyboard;
    }

    pub fn move_action_panel_selection(&mut self, delta: isize) {
        self.action_panel.move_selection(delta);
    }

    pub fn jump_action_panel_selection(&mut self, first: bool) {
        self.action_panel.jump_selection(first);
    }

    pub fn update_action_panel_query(&mut self, query: impl Into<String>) {
        self.action_panel.update_query(query);
    }

    pub fn type_action_panel_query(&mut self, ch: char) {
        if !self.action_panel.open || self.focus_section != LauncherFocusSection::ActionPanel {
            return;
        }
        let mut query = self.action_panel.query.clone();
        query.push(ch);
        self.update_action_panel_query(query);
    }

    pub fn trigger_action_panel_selection(&mut self) -> Option<ActionExecution> {
        self.trigger_action_panel_selection_with_external_runner(false)
    }

    pub fn trigger_action_panel_selection_by_user(&mut self) -> Option<ActionExecution> {
        self.trigger_action_panel_selection_with_external_runner(true)
    }

    fn trigger_action_panel_selection_with_external_runner(
        &mut self,
        allow_external_runner: bool,
    ) -> Option<ActionExecution> {
        match self.action_panel.selected_item()?.clone() {
            ActionPanelItem::Run => {
                self.trigger_selected_with_external_runner(allow_external_runner)
            }
            ActionPanelItem::ReviewFirst => {
                self.trigger_selected_with_external_runner(allow_external_runner)
            }
            ActionPanelItem::Defer => self.trigger_selected(),
            ActionPanelItem::OpenInStudio => {
                self.open_selected_action_in_studio()?;
                None
            }
            ActionPanelItem::CopyCommand(command) => Some(self.complete_action_panel_copy(command)),
        }
    }
}
