use std_egui::input;
use std_types::{Action, ActionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionPanelItem {
    Run,
    Defer,
    OpenInStudio,
    CopyCommand(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionPanel {
    pub open: bool,
    pub selected: usize,
    pub action_name: String,
    pub query: String,
    pub items: Vec<ActionPanelItem>,
}

impl ActionPanel {
    pub fn closed() -> Self {
        Self {
            open: false,
            selected: 0,
            action_name: String::new(),
            query: String::new(),
            items: Vec::new(),
        }
    }

    pub fn open_for(&mut self, action: &Action) {
        self.open = true;
        self.selected = 0;
        self.action_name = action.name.clone();
        self.query.clear();
        self.items = action_panel_items(action);
    }

    pub fn close(&mut self) {
        self.open = false;
        self.selected = 0;
        self.query.clear();
    }

    pub fn move_selection(&mut self, delta: isize) {
        let visible_len = self.visible_items().len();
        if visible_len == 0 {
            self.selected = 0;
            return;
        }
        let next = self.selected.saturating_add_signed(delta);
        self.selected = next.min(visible_len - 1);
    }

    pub fn jump_selection(&mut self, first: bool) {
        let visible_len = self.visible_items().len();
        if visible_len == 0 {
            self.selected = 0;
            return;
        }
        self.selected = if first { 0 } else { visible_len - 1 };
    }

    pub fn update_query(&mut self, query: impl Into<String>) {
        self.query = query.into().trim().to_lowercase();
        self.selected = 0;
    }

    pub fn selected_item(&self) -> Option<&ActionPanelItem> {
        if !self.open {
            return None;
        }
        self.visible_items().get(self.selected).copied()
    }

    pub fn visible_items(&self) -> Vec<&ActionPanelItem> {
        if self.query.is_empty() {
            return self.items.iter().collect();
        }
        self.items
            .iter()
            .filter(|item| item.matches(&self.query))
            .collect()
    }
}

impl ActionPanelItem {
    pub fn title(&self) -> &str {
        match self {
            Self::Run => "Run",
            Self::Defer => "Defer",
            Self::OpenInStudio => "Open in Studio",
            Self::CopyCommand(_) => "Copy command",
        }
    }

    pub fn shortcut_label(&self) -> String {
        match self {
            Self::Run => input::enter().label(),
            Self::Defer => input::launcher_defer().label(),
            Self::OpenInStudio => input::launcher_open_studio().label(),
            Self::CopyCommand(_) => input::launcher_copy_command().label(),
        }
    }

    fn matches(&self, query: &str) -> bool {
        self.title().to_lowercase().contains(query)
            || self.shortcut_label().to_lowercase().contains(query)
    }
}

fn action_panel_items(action: &Action) -> Vec<ActionPanelItem> {
    let mut items = vec![ActionPanelItem::Run];
    if action.action_type.needs_external_runner() {
        items.push(ActionPanelItem::Defer);
    }
    if studio_supported(action) {
        items.push(ActionPanelItem::OpenInStudio);
    }
    if let Some(command) = primary_command(action) {
        items.push(ActionPanelItem::CopyCommand(command));
    }
    items
}

fn studio_supported(action: &Action) -> bool {
    !matches!(action.action_type, ActionType::Custom(_))
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
