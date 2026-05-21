use crate::LauncherState;
use std_egui::{
    i18n::{self, Locale},
    input,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutingSemantics {
    pub(crate) search_text: String,
    pub(crate) input_enabled: bool,
    pub(crate) cancel_shortcut: String,
    pub(crate) background_shortcut: String,
}

pub(crate) fn executing_semantics(query: &str) -> ExecutingSemantics {
    let mut state = LauncherState::new();
    state.update_query(query);
    state.view.preview_executing();
    let title = state
        .view
        .preview
        .as_ref()
        .map(|preview| preview.title.clone())
        .unwrap_or_else(|| "selected action".to_string());
    ExecutingSemantics {
        search_text: format!(
            "{} {}",
            i18n::translate(Locale::EnUs, "launcher.search.running"),
            title
        ),
        input_enabled: false,
        cancel_shortcut: format!(
            "{} {}",
            i18n::translate(Locale::EnUs, "launcher.action.cancel"),
            input::launcher_cancel().label()
        ),
        background_shortcut: format!(
            "{} {}",
            i18n::translate(Locale::EnUs, "launcher.action.background"),
            input::enter().label()
        ),
    }
}
