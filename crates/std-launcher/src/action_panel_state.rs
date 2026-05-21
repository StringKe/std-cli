use crate::{LauncherFocusSection, LauncherFocusSource, LauncherState};

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
}
