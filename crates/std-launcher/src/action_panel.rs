use std_types::{Action, ActionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionPanelItem {
    Run,
    Defer,
    CopyCommand(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionPanel {
    pub open: bool,
    pub selected: usize,
    pub action_name: String,
    pub items: Vec<ActionPanelItem>,
}

impl ActionPanel {
    pub fn closed() -> Self {
        Self {
            open: false,
            selected: 0,
            action_name: String::new(),
            items: Vec::new(),
        }
    }

    pub fn open_for(&mut self, action: &Action) {
        self.open = true;
        self.selected = 0;
        self.action_name = action.name.clone();
        self.items = action_panel_items(action);
    }

    pub fn close(&mut self) {
        self.open = false;
        self.selected = 0;
    }

    pub fn move_selection(&mut self, delta: isize) {
        if self.items.is_empty() {
            self.selected = 0;
            return;
        }
        let next = self.selected.saturating_add_signed(delta);
        self.selected = next.min(self.items.len() - 1);
    }

    pub fn selected_item(&self) -> Option<&ActionPanelItem> {
        if !self.open {
            return None;
        }
        self.items.get(self.selected)
    }
}

impl ActionPanelItem {
    pub fn title(&self) -> &str {
        match self {
            Self::Run => "Run",
            Self::Defer => "Defer",
            Self::CopyCommand(_) => "Copy command",
        }
    }

    pub fn shortcut(&self) -> &str {
        match self {
            Self::Run => "Enter",
            Self::Defer => "Shift+Enter",
            Self::CopyCommand(_) => "Cmd+C",
        }
    }
}

fn action_panel_items(action: &Action) -> Vec<ActionPanelItem> {
    let mut items = vec![ActionPanelItem::Run];
    if action.action_type.needs_external_runner() {
        items.push(ActionPanelItem::Defer);
    }
    if let Some(command) = primary_command(action) {
        items.push(ActionPanelItem::CopyCommand(command));
    }
    items
}

fn primary_command(action: &Action) -> Option<String> {
    match &action.action_type {
        ActionType::AppLaunch => action.examples.first().cloned(),
        ActionType::Command => action.examples.first().cloned(),
        ActionType::Workflow => action.examples.first().cloned(),
        ActionType::Skill => action.examples.first().cloned(),
        ActionType::Clipboard => Some(action.description.clone()),
        ActionType::Custom(kind) if kind == "file" => action.examples.first().cloned(),
        ActionType::Custom(_) => action.examples.first().cloned(),
    }
}
