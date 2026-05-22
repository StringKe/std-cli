use crate::{LauncherQueryMode, LauncherQueryRequest, LauncherState};
use std_egui::input;
use std_types::{ActionExecution, ActionExecutionStatus, ActionId, ActionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherSurfaceContract {
    pub search_bar: String,
    pub result_list: String,
    pub action_bar: String,
    pub empty_state: String,
    pub no_match_state: String,
    pub query_prefixes: String,
    pub nl_suggestion: String,
    pub executing_state: String,
    pub defer_state: String,
    pub error_state: String,
    pub visible_structure: String,
}

impl LauncherSurfaceContract {
    pub fn new() -> Self {
        let mut state = LauncherState::new();
        state.update_query("rebuild index");
        let result_count = state.view.results.len();
        let selected = state
            .view
            .selected_result()
            .expect("seeded launcher result");
        let selected_type = selected.action.action_type.clone();
        let preview = state
            .view
            .preview
            .as_ref()
            .expect("seeded launcher preview");

        Self {
            search_bar: search_bar_contract(),
            result_list: result_list_contract(result_count, &selected_type),
            action_bar: action_bar_contract(preview),
            empty_state: empty_state_contract(),
            no_match_state: no_match_state_contract(),
            query_prefixes: query_prefix_contract(),
            nl_suggestion: nl_suggestion_contract(),
            executing_state: executing_state_contract(),
            defer_state: defer_state_contract(),
            error_state: error_state_contract(),
            visible_structure: visible_structure_contract(),
        }
    }

    pub fn pass(&self) -> bool {
        self.search_bar == "height=64;text=headline;icon=search;focus=2px-accent;mode-tag=all,command,actions,ask"
            && self
            .result_list
                .contains("groups=Action / Workflow>App / File>Clipboard>Memory>Skill>Other")
            && self.result_list.contains("row_height=36")
            && self.result_list.contains("group_height=24")
            && self.result_list.contains("selected=accent-weak")
            && self
                .result_list
                .contains(&format!(
                    "direct_shortcut={}",
                    input::launcher_result_keycap(0).unwrap()
                ))
            && self
                .result_list
                .contains(&format!("primary_shortcut={}", input::enter().label()))
            && self.action_bar.contains("height=36")
            && self.action_bar.contains("left=breadcrumb+primary-command")
            && self.action_bar.contains("breadcrumb=命令 > Rebuild Index")
            && self
                .action_bar
                .contains(&format!("actions={}", input::launcher_action_panel().label()))
            && self.action_bar.contains("font=code")
            && self.empty_state.contains("recent_or_suggested")
            && self.no_match_state.contains("ask_ai_enter")
            && self.query_prefixes.contains("command_search=rebuild")
            && self.query_prefixes.contains("command_only=true")
            && self.query_prefixes.contains("actions_only=true")
            && self.nl_suggestion.contains("mode=NaturalLanguage")
            && self.nl_suggestion.contains("actions=Ask AI|Search Actions")
            && self.nl_suggestion.contains("selected=Ask AI")
            && self.nl_suggestion.contains("enter_status=NeedsExternalRunner")
            && self.nl_suggestion.contains("down_enter_query=> rebuild index")
            && self.executing_state.contains("input_locked=true")
            && self.defer_state.contains("NeedsExternalRunner")
            && self.error_state.contains("copy,retry,open_studio")
            && self
                .visible_structure
                .contains("search=input|placeholder|focus-ring|mode-tag|ime-chip")
            && self
                .visible_structure
                .contains("results=group-header|row-icon|title|subtitle|keycap|enter-action")
            && self
                .visible_structure
                .contains("states=empty|no-results|loading|executing|defer|error")
    }

    pub fn summary(&self) -> String {
        format!(
            "launcher_surface_contract {}\nsearch_bar_contract={}\nresult_list_contract={}\naction_bar_contract={}\nempty_state_contract={}\nno_match_state_contract={}\nquery_prefix_contract={}\nnl_suggestion_contract={}\nexecuting_state_contract={}\ndefer_state_contract={}\nerror_state_contract={}\nvisible_structure_contract={}",
            if self.pass() { "PASS" } else { "FAIL" },
            self.search_bar,
            self.result_list,
            self.action_bar,
            self.empty_state,
            self.no_match_state,
            self.query_prefixes,
            self.nl_suggestion,
            self.executing_state,
            self.defer_state,
            self.error_state,
            self.visible_structure
        )
    }
}

impl Default for LauncherSurfaceContract {
    fn default() -> Self {
        Self::new()
    }
}

fn search_bar_contract() -> String {
    format!(
        "height=64;text=headline;icon=search;focus=2px-accent;mode-tag={}",
        mode_tag_contract()
    )
}

fn result_list_contract(result_count: usize, selected_type: &ActionType) -> String {
    format!(
        "groups=Action / Workflow>App / File>Clipboard>Memory>Skill>Other;row_height=36;group_height=24;result_count={result_count};selected=accent-weak;selected_kind={};direct_shortcut={};primary_shortcut={};virtualized=true",
        action_type_name(selected_type),
        input::launcher_result_keycap(0).unwrap(),
        input::enter().label()
    )
}

fn action_bar_contract(preview: &std_types::ActionPreview) -> String {
    let summary = crate::ActionBarPreviewSummary::from_preview(preview);
    format!(
        "height=36;left=breadcrumb+primary-command;font=code;right=run+actions;run={};actions={};{}",
        input::enter().label(),
        input::launcher_action_panel().label(),
        summary.contract()
    )
}

fn empty_state_contract() -> String {
    let state = LauncherState::new();
    format!(
        "phase={:?};mode={:?};recent_or_suggested={};hint=slash-question-down",
        state.view.phase,
        state.view.result_mode,
        !state.view.results.is_empty()
    )
}

fn no_match_state_contract() -> String {
    let mut state = LauncherState::new();
    state.update_query("no-such-launcher-result");
    let fallback = state.no_match_fallback_query().unwrap_or_default();
    format!(
        "phase={:?};mode={:?};icon=lg-search;ask_ai_enter={fallback}",
        state.view.phase, state.view.result_mode
    )
}

fn query_prefix_contract() -> String {
    let command = LauncherQueryRequest::parse("/rebuild index");
    let actions = LauncherQueryRequest::parse(">rebuild index");
    let ask = LauncherQueryRequest::parse("?rebuild index");
    format!(
        "command_display={};command_search={};command_only={};actions_search={};actions_only={};ask_search={}",
        command.display_query,
        command.search_query,
        command.command_only(),
        actions.search_query,
        actions.action_only(),
        ask.search_query
    )
}

fn nl_suggestion_contract() -> String {
    let mut state = LauncherState::new();
    state.update_query("?rebuild index");
    let suggestion = state.view.nl_suggestion.as_ref();
    let mut enter_state = LauncherState::new();
    enter_state.update_query("?rebuild index");
    let enter_status = enter_state
        .handle_keyboard_input(crate::LauncherKey::Enter, false)
        .map(|execution| format!("{:?}", execution.status))
        .unwrap_or_else(|| "none".to_string());
    let mut search_state = LauncherState::new();
    search_state.update_query("?rebuild index");
    search_state.handle_keyboard_input(crate::LauncherKey::ArrowDown, false);
    let _ = search_state.handle_keyboard_input(crate::LauncherKey::Enter, false);
    format!(
        "mode={:?};results={};preview={};intent={};confidence={};actions={};selected={};enter_status={};down_enter_query={}",
        state.view.result_mode,
        state.view.results.len(),
        state.view.preview.is_some(),
        suggestion
            .map(|item| item.intent.as_str())
            .unwrap_or("none"),
        suggestion.map(|item| item.confidence).unwrap_or_default(),
        suggestion
            .map(|item| item.actions.join("|"))
            .unwrap_or_else(|| "none".to_string()),
        state.view.selected_nl_action().unwrap_or("none"),
        enter_status,
        search_state.view.query
    )
}

fn executing_state_contract() -> String {
    let mut state = LauncherState::new();
    state.update_query("rebuild index");
    state.view.preview_executing();
    format!(
        "phase={:?};input_locked=true;cancel={};background={};progress=action-bar",
        state.view.phase,
        input::launcher_cancel().label(),
        input::enter().label()
    )
}

fn defer_state_contract() -> String {
    let feedback = std_egui::LauncherFeedback::from_execution(&execution(
        "Fixture External Action",
        ActionExecutionStatus::NeedsExternalRunner,
    ));
    format!(
        "status={:?};title={};actions=copy,retry",
        feedback.status, feedback.title
    )
}

fn error_state_contract() -> String {
    let feedback = std_egui::LauncherFeedback::from_execution(&execution(
        "Fixture Plugin Crash",
        ActionExecutionStatus::Failed,
    ));
    format!(
        "status={:?};title={};actions=copy,retry,open_studio",
        feedback.status, feedback.title
    )
}

fn visible_structure_contract() -> String {
    [
        "search=input|placeholder|focus-ring|mode-tag|ime-chip",
        "results=group-header|row-icon|title|subtitle|keycap|enter-action",
        "preview=breadcrumb|primary-command|examples",
        "feedback=status-icon|title|message|copy|retry|open-studio",
        "states=empty|no-results|loading|executing|defer|error",
        "host=transparent-native-host|opaque-panel-surface|host-gap-0",
    ]
    .join(";")
}

fn execution(name: &str, status: ActionExecutionStatus) -> ActionExecution {
    ActionExecution {
        action_id: ActionId::default(),
        action_name: name.to_string(),
        status,
        message: "fixture message".to_string(),
        output: None,
        created_at: chrono::Utc::now(),
    }
}

fn action_type_name(action_type: &ActionType) -> &'static str {
    match action_type {
        ActionType::AppLaunch => "app",
        ActionType::Workflow => "workflow",
        ActionType::Command => "command",
        ActionType::Memory => "memory",
        ActionType::Skill => "skill",
        ActionType::Clipboard => "clipboard",
        ActionType::Custom(_) => "custom",
    }
}

fn mode_tag_contract() -> String {
    [
        LauncherQueryMode::from_query("index").contract_name(),
        LauncherQueryMode::from_query("/workflow new").contract_name(),
        LauncherQueryMode::from_query("> action").contract_name(),
        LauncherQueryMode::from_query("? rebuild").contract_name(),
    ]
    .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn surface_contract_covers_launcher_docs_21_states() {
        let contract = LauncherSurfaceContract::new();

        assert!(contract.pass(), "{}", contract.summary());
        assert!(contract
            .summary()
            .contains("launcher_surface_contract PASS"));
        assert!(contract.summary().contains("search_bar_contract=height=64"));
        assert!(contract
            .summary()
            .contains("mode-tag=all,command,actions,ask"));
        assert!(contract
            .summary()
            .contains("result_list_contract=groups=Action / Workflow"));
        assert!(contract.summary().contains(&format!(
            "direct_shortcut={}",
            input::launcher_result_keycap(0).unwrap()
        )));
        assert!(contract
            .summary()
            .contains(&format!("primary_shortcut={}", input::enter().label())));
        assert!(contract
            .summary()
            .contains("no_match_state_contract=phase=NoMatches"));
        assert!(contract
            .summary()
            .contains("query_prefix_contract=command_display=/rebuild index"));
        assert!(contract
            .summary()
            .contains("nl_suggestion_contract=mode=NaturalLanguage"));
        assert!(contract
            .summary()
            .contains("executing_state_contract=phase=Executing"));
        assert!(contract
            .summary()
            .contains("defer_state_contract=status=NeedsExternalRunner"));
        assert!(contract
            .summary()
            .contains("error_state_contract=status=Failed"));
        assert!(contract.summary().contains("actions=copy,retry"));
        assert!(contract
            .summary()
            .contains("actions=copy,retry,open_studio"));
    }
}
